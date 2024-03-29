---
title: "How I made text-clipper the fastest HTML clipping library"
description: "An overview of JavaScript optimization techniques for optimizing text analyzers"
pubDate: "Feb 2 2016"
---

**Update:** Redditor /u/ugwe43to874nf4 gave another [interesting insight](https://www.reddit.com/r/javascript/comments/522di5/how_i_made_textclipperjs_the_fastest_html/d7hidml?st=isyhsmtb&sh=0a2df58e) in response to this blog. In short, he suggested the final performance improvement might be more due to skipping large blocks in one go, rather than the fact I was doing so using regular expressions. This prompted me to a little bit more investigation, which you can find below...

**tl;dr:** Regular expressions will pretty much always outperform you, but if you have to evaluate single characters use `charCodeAt()` rather than `charAt()`.

When I started working on [text-clipper](https://github.com/arendjr/text-clipper) a few weeks back, my main focus was on correctness. I wanted to be very sure the library would work correctly no matter what HTML you threw at it, provided it was valid according to the HTML5 spec. Of course there were already some clipping/trimming/truncating libraries for HTML out there, but they all seemed to rely on regular expressions to parse the HTML and well, HTML is just a little more complicated than regular expressions can handle, which meant they all would miss certain edge cases the HTML5 spec said are valid.

But I also had a secondary concern, which is that I wanted text-clipper to be bloody fast. The fastest, if possible. Surely a minimal state machine that evaluates the text to clip only once could be faster than a soup of regular expressions, right? Sure, my state machine is written in JavaScript, while regular expressions are optimized in C, so it would be an interesting testament to the performance of the JIT compiler, but at least on face value it seems my state machine would have to perform less work. So, after my initial implementation, I ran a little benchmark:

```
text-clipper x 35,338 ops/sec ±4.25% (53 runs sampled)
html-truncate x 95,116 ops/sec ±4.39% (55 runs sampled)
trim-html x 40,879 ops/sec ±2.69% (49 runs sampled)
truncate-html x 1,177 ops/sec ±5.83% (53 runs sampled)
Fastest is html-truncate
```

So the performance was similar to trim-html, but still 2.5 times slower than html-truncate. At least it was much faster than truncate-html, which uses jQuery internally and involves actual DOM parsing to do the work.

Now, html-truncate uses regular expressions to find the HTML tags relatively efficiently, so it explains its good score, but to be honest I was a little disappointed text-clipper wasn't any faster than trim-html which runs a whole bunch of regular expressions on the entire string and then splits it all into an array which is traversed next. I knew regular expressions were very fast, but are they so fast you can do 3 global searches over the input string and split it into an array and still come out (slightly) ahead against a minimal state machine that evaluates every character once in JavaScript? It appears so...

So yesterday I spent some time trying to optimize text-clipper, and with a little experimentation and some common sense, I realized I had made one big mistake: I used `string.charAt(index)` to take the characters from the string and evaluated them one by one. Even if [inline strings](https://blog.mozilla.org/javascript/2014/07/21/slimmer-and-faster-javascript-strings-in-firefox/) can be used, this will give some overhead and every time I compare that character against another, the JS engine has to assume I'm doing a comparison between two random strings. So I switched the implementation to use `string.charCodeAt(index)` instead, and I instantly achieved a >75% performance improvement ([commit](https://github.com/arendjr/text-clipper/commit/45d8913002ca7f3db57fb0942c5bd9ccbcc23067)):

```
text-clipper x 62,848 ops/sec ±3.56% (52 runs sampled)
html-truncate x 88,262 ops/sec ±2.39% (50 runs sampled)
trim-html x 43,259 ops/sec ±2.42% (50 runs sampled)
truncate-html x 1,173 ops/sec ±3.87% (55 runs sampled)
Fastest is html-truncate
```

Note that different runs of the benchmark unfortunately are not always consistent. I had closed Chrome, but maybe there are still some background processes interfering, so please be aware the fluctuation may be more than benchmark.js reported. When validating my results, I always re-ran the benchmark a few times to make sure I wasn't looking at a random outlier.

Okay, so now trim-html was comfortably left behind, and if I could pull a similar feat, I might be able to beat html-truncate as well. So I went on...

After that I came up with a few smaller optimizations, that basically resolved around doing fewer comparisons in the main loop or making the comparisons cheaper. I won a few percent moving one block of code so that its `if`-statement didn't have to be evaluated for every character and replacing two range checks with a single bitmasked check ([commit](https://github.com/arendjr/text-clipper/commit/0bf072a76b4626bf3865074ab0cdd09dc81db7ee)). Another ~10% was won by no longer keeping track of good places to break the string (text-clipper prefers to not break in the middle of a word), but doing a little backtrack for that at the end ([commit](https://github.com/arendjr/text-clipper/commit/cf94961f6d24f74996ca508b7e31a17fcb655e3e)). Finally, I created two specialized inline loops for processing HTML entities ([commit](https://github.com/arendjr/text-clipper/commit/bc5407faf15681af5035c7e7fc480ed7635dc8c3)) and one for processing HTML tags ([commit](https://github.com/arendjr/text-clipper/commit/d6c01b05bd886b1d4e61ff57c4e14781f67b7052)) which yielded roughly 5% and 10% performance gains, respectively.

```
text-clipper x 81,248 ops/sec ±3.36% (55 runs sampled)
html-truncate x 91,289 ops/sec ±2.94% (50 runs sampled)
trim-html x 38,015 ops/sec ±0.86% (72 runs sampled)
truncate-html x 1,042 ops/sec ±5.01% (51 runs sampled)
Fastest is html-truncate
```

At this point text-clipper had almost caught up with truncate-html. But I had another realization. I had been focusing rather heavily on one specific input string that did contain a lot of different HTML tags. This was an intentional decision in the beginning because it would be more indicative of a worst-case input, but that wouldn't be very typical input. So how were the results if I used an input string with only a few HTML tags?

```
text-clipper x 191,700 ops/sec ±3.13% (54 runs sampled)
html-truncate x 142,627 ops/sec ±3.87% (52 runs sampled)
trim-html x 55,776 ops/sec ±1.75% (49 runs sampled)
truncate-html x 1,337 ops/sec ±4.92% (49 runs sampled)
Fastest is text-clipper
```

Okay, great. So text-clipper is already ahead in those cases. But I had one more idea that I wanted to try out. As we learned in the beginning, regular expressions are bloody fast. The other libraries relying on them are having pretty good performance, without having to care too much about performance for the rest. So could we use a regular expression in text-clipper, not for the parsing, but for finding the first interesting character the parser is looking for? So I made one final [commit](https://github.com/arendjr/text-clipper/commit/783d3b3f8ff283183a1e88b3de7dc61320d5d05f), and the results were impressive...

First with the original, bad-case string:

```
text-clipper x 89,645 ops/sec ±2.84% (54 runs sampled)
html-truncate x 90,654 ops/sec ±4.57% (49 runs sampled)
trim-html x 36,977 ops/sec ±1.55% (77 runs sampled)
truncate-html x 1,107 ops/sec ±6.15% (47 runs sampled)
Fastest is text-clipper,html-truncate
```

Okay, so text-clipper and html-truncate are now within margin of error of each other (and multiple runs confirm they are pretty much tied there), but the biggest difference was with a more typical string:

```
text-clipper x 426,806 ops/sec ±4.33% (48 runs sampled)
html-truncate x 146,281 ops/sec ±3.40% (53 runs sampled)
trim-html x 55,098 ops/sec ±3.05% (50 runs sampled)
truncate-html x 1,503 ops/sec ±3.62% (48 runs sampled)
Fastest is text-clipper
```

More than twice as fast as the previous commit, and almost triple the performance of html-truncate! ~~I think that's as good as it's going to get for now :)~~ I thought that was the most of it, but once another redditor pointed out I had skipped one important step in my conclusion, I investigated a bit further...

In the previous step I saw a large performance gain, but was it because of the regular expression I was now using, or was it simply because I wasn't concatenating the result string one character after another anymore? So I had another close look, and quickly it became obvious I had still left quite some room for improvement: Why was I even building the result as I went along at all? It became apparent I only need to search for the position *where* to clip, and then do the clipping if and where necessary and no more. So I first removed all the building of the result string and ran the benchmark again:

Bad-case input:

```
text-clipper x 106,197 ops/sec ±3.53% (48 runs sampled)
html-truncate x 87,662 ops/sec ±3.14% (49 runs sampled)
trim-html x 43,243 ops/sec ±2.86% (50 runs sampled)
truncate-html x 1,198 ops/sec ±4.66% (50 runs sampled)
Fastest is text-clipper
```

More typical input:

```
text-clipper x 589,891 ops/sec ±3.35% (55 runs sampled)
html-truncate x 148,986 ops/sec ±2.32% (49 runs sampled)
trim-html x 51,572 ops/sec ±2.63% (54 runs sampled)
truncate-html x 1,423 ops/sec ±4.05% (51 runs sampled)
Fastest is text-clipper
```

Another roughly 20-40% improvement depending on the input. Okay, but how does this compare to when we remove the regular expression and iterate manually again?

Bad-case input:

```
text-clipper x 105,661 ops/sec ±3.36% (55 runs sampled)
html-truncate x 87,098 ops/sec ±3.62% (51 runs sampled)
trim-html x 40,277 ops/sec ±2.77% (53 runs sampled)
truncate-html x 1,161 ops/sec ±4.89% (50 runs sampled)
Fastest is text-clipper
```

More typical input:

```
text-clipper x 346,503 ops/sec ±3.69% (53 runs sampled)
html-truncate x 154,289 ops/sec ±2.26% (49 runs sampled)
trim-html x 53,631 ops/sec ±3.19% (55 runs sampled)
truncate-html x 1,402 ops/sec ±4.73% (56 runs sampled)
Fastest is text-clipper
```

Pretty much no difference for the bad-case input, but the typical case got significantly worse.

But, there's one advantage to *not* using a regular expression that might be important to some. If the input string is very long, you might want to break early once you discover `maxLength` characters, but the regular expression I'm using can't do that. By definition, the algorithm scales linearly with the input string, but if you don't use the regular expression, you can cap it to a constant. So how do both variations perform with an input string of 1 million uninteresting characters?

With regular expression:

```
text-clipper x 575 ops/sec ±2.52% (48 runs sampled)
(html-truncate didn't complete)
trim-html x 60.38 ops/sec ±4.44% (46 runs sampled)
truncate-html x 59.80 ops/sec ±3.64% (46 runs sampled)
Fastest is text-clipper
```

Without regular expression:

```
text-clipper x 336,121 ops/sec ±3.42% (53 runs sampled)
(html-truncate didn't complete)
trim-html x 57.67 ops/sec ±3.43% (48 runs sampled)
truncate-html x 60.11 ops/sec ±2.78% (43 runs sampled)
Fastest is text-clipper
```

As you can see, all the algorithms slow down to quite an extent. Html-truncate, that otherwise did so well, doesn't even complete anymore within 5 minutes or so before I killed it. The happy exception is text-clipper without regular expression which seems hardly affected because it can break early.

Now this does leave me in a bit of a tough spot. Do I sacrifice typical performance for the worst-case or the other way around? For now I've decided to let the regular expression in and have the best average performance. After all, strings with a million characters are very unlikely, and even the occasional newline character would be enough to prevent that worst case from happening. And even if it does occur, at a million characters text-clipper can still handle over 500 such strings in a second, so it just seems the potential for a DOS is really small.

So to sum it up:

- Using `charCodeAt()` instead of `charAt()` provides a very big gain if you're writing a parser.
- By carefully optimizing how many `if`-statements you're doing you can shave off tens of percents from a hot loop.
- Don't underestimate the performance of regular expressions (but be aware of their limitations).
- And finally: Share your results and learn from others' insights :)
