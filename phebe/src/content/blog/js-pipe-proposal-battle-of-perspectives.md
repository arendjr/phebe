---
title: "JavaScript Pipe Operator Proposal: A Battle of Perspectives"
description: "A deep dive into the pros and cons of the proposals for a JavaScript pipe operator"
pubDate: "Sep 19 2021"
---

Over the past few days, plenty has been written about JavaScript’s pipe operator proposal. I read several [opinion](https://jamesdigioia.com/hack-pipe-for-functional-programmers-how-i-learned-to-stop-worrying-and-love-the-placeholder/) [pieces](https://benlesh.com/posts/tc39-pipeline-proposal-hack-vs-f-sharp/) with opposing viewpoints, lots of comments on them, and then there was an update on [the proposal](https://github.com/tc39/proposal-pipeline-operator) itself. And yet, I found I was unable to form a satisfactory opinion for myself. It intrigued me.

I realized the reason I was unable to form a clear opinion was because there are two proposals — two camps — and I saw merit to both. Both came from different perspectives however, and as a person who likes to look at things from different perspectives, I found it hard to pick one side over the other.

This post is my attempt to lay out the most important perspectives as I see them, and form my own opinion through the process.

## A library author’s perspective

First of all, I maintain an open-source TypeScript library called [text-clipper](https://github.com/arendjr/text-clipper). So naturally, I am interested to see how text-clipper might be affected by the pipe proposals. The library only exports a single function:

```ts
function clip(text: string, maxLength: number, options?: object): string
```

The Hack proposal looks like it is well-suited to take advantage of this signature out-of-the-box, whereas if I wanted to take advantage of the F# proposal, I would need to expose a secondary signature:

```ts
function clip(maxLength: number, options?: object): (text: string) => string
```

It would be a minor adjustment, and it could even be retrofitted into the existing single function, but the fact remains Hack has some advantage over F# here. I know there are other libraries, such as Ramda and RxJS, that already embrace a functional style for which the advantage would go in the other direction, but without any data to back this up, my impression is that today’s JavaScript ecosystem at large would profit from Hack more than F#.

That said, there is a secondary proposal for partial function application, that works well with the F# proposal. Pair them together and it would be just as convenient as Hack, plus have the advantage that partial application is useful in other scenarios than just pipes as well. More on that in the next section...

## A language design perspective

I’m not really a language designer, but having experience in quite a few languages does make me interested in comparing their features. And especially how those features affect the users of those languages, because well, that affects me.

Just before, I already mentioned the partial application proposal. While I don’t want to dive into that proposal too much to stay on topic, it is interesting to mention because the Hack proposal effectively combines partial application through the ^ character and the pipe operator into a single proposal.

From a language design perspective this is problematic: it ties two orthogonal features together. Because of this, its users are forced to use partial application when using the pipe operator, even when they don’t need it. This creates some visual noise, but is otherwise not a big deal. But the other downside is that partial application is no longer available outside the pipe operator. This is a real shame, because it would certainly be nice to have in other scenarios as well.

For what it’s worth, the Hack proposal recognizes that it would be a shame to not have partial application as a generic feature, so in order not to miss out, it decides to double down: it introduces *yet another* operator (`+>`) to allow the creation of lambda functions with Hack-style partial applications that leads to such syntactic gems as this: `a.map(+> ^ + ^)`

At this point I was reminded of the famous quote from [Jurassic Park](https://www.quotes.net/mquote/49960):

> "Yeah, but your scientists were so preoccupied with whether or not they could, they didn't stop to think if they should."

From this perspective I’m going to side with F#. It’s focused and doesn’t try to lump orthogonal features together in such a way that it requires an even bigger soup of operators to make partial application generally applicable again.

## A maintainability perspective

As a lead developer, one of the most important aspects I care about for projects I work on is maintainability. To me, that means that I can write code, come back to it a year later, and still make sense of it. It means I can hand it over to co-workers and they can make sense of it and improve it with little friction. And when they do improve it, their contributions should be easy to review.

In my experience, this does work well if everyone just does whatever they feel like, so an important part in optimizing that process is tooling. I use Prettier and ESLint extensively to avoid bikeshedding: I want code to be high quality without wasting time on arguments about style. It’s better to settle on a style that is well-readable (even if it isn’t your favourite) than to leave everyone doing what they like and then argue during the review phase when the code is already written.

The pipe operator proposal however, is a prime opportunity for bikeshedding! After all, regardless of which proposal wins, it gives us additional style options, which means new linting rules must be right around the corner.

Where before we would write:

```js
fn(a)
```

We can now write:

```js
a |> fn
```

Or, if Hack wins:

```js
a |> fn(^)
```

Is one better than the other? Should we restrict which one people use? In my opinion, the answers to those questions are *arguably* and *yes*. I don’t really care which of these two you end up favouring, but I’d rather set up guidance upfront than arguing about it afterwards.

Now, looking at the two proposals, what might potential linting rules look like? Personally, and regardless of which proposal, I would start with a lint rule that only allows the pipe operator if there’s more than a single function call in your pipeline. But from the perspective of maintainability and readability, can we foresee other rules we might want to introduce?

Again, regardless of which proposal, I would like to restrict piping into complex lambda expressions, as I think that would defeat the purpose of the operator. Anything else?

Frankly, for the F# proposal, that’s all I can think of.

For the Hack proposal however… where to start? The proposal allows piping into any kind of expression. And while this is great for function calls with arbitrary arguments, I strongly feel this quickly overshoots its usefulness when it comes to other expressions. In one of the [Reddit discussions](https://www.reddit.com/r/programming/comments/pobeuk/tc39_pipeline_operator_hack_vs_f/hd260ru/?context=3) where I was playing devil’s advocate for the Hack proposal, I reduced a pipeline to:

```js
"PIPE" |> ^.split('') |> ^[0]
```

It doesn’t violate any of the rules I came up with so far, but I sure as hell would not want anyone in my team to write code like that. You might say that’s common sense, and nobody would write code like that, but is it really that easy?

Looking at the proposal itself, we see more examples of how the syntax would work:

```js
value |> {foo: ^}
value |> `${^}`
```

These are all things we could already do, in a more straightforward way, without the pipe operator. I’m starting to think I would like to see an allowlist for expressions that may be used with the pipe operator, and at this point I’m inclined to let it only include function calls. But maybe I’m overreacting. Would I be throwing the baby out with the bath water?

Let’s look at the [real-world examples](https://github.com/tc39/proposal-pipeline-operator#real-world-examples) provided by the proposal itself. First example:

```js
// Status quo
var minLoc = Object.keys( grunt.config( "uglify.all.files" ) )[ 0 ];

// With pipes
var minLoc = grunt.config('uglify.all.files') |> Object.keys(^)[0];
```

Ah, that’s nice! Even though it’s only a single pipe, the part after the pipe makes it immediately obvious that we’re taking the first key without the need for a named function. The status quo here wasn’t actually too bad, but I like how the Hack proposal works here. +1 for Hack.

Next example:

```js
// Status quo
const json = await npmFetch.json(npa(pkgs[0]).escapedName, opts);

// With pipes
const json = pkgs[0] |> npa(^).escapedName |> await npmFetch.json(^, opts);
```

Ehm, okay. I suppose you could do this. It’s different, but is it better? Frankly, I don’t think this is a great example for the pipeline operator at all. I don’t like how it hides the `await` at the end of the line, and I don’t like how we have to scan forward to find the caret and perform a mental substitution to understand what we’re doing. -1 for Hack.

Next example:

```js
// Status quo
return filter(obj, negate(cb(predicate)), context);

// With pipes
return cb(predicate) |> _.negate(^) |> _.filter(obj, ^, context);
```

Frankly, I don’t like this much either. The negation is clear enough, but I’m having trouble reasoning backwards how the filter function is being applied. It’s as if I’m losing my context along the pipeline. Maybe it’s just because I’m not used to it yet, but here I would again vote -1 for Hack.

Next one:

```js
// Status quo
return xf['@@transducer/result'](obj[methodName](bind(xf['@@transducer/step'], xf), acc));

// With pipes
return xf
|> bind(^['@@transducer/step'], ^)
|> obj[methodName](^, acc)
|> xf['@@transducer/result'](^);
```

What am I looking at? I will say the pipes make the calling order clear, but that doesn’t really help me to understand what this code is doing. Basically, we’ve added a lot of verbosity to turn gibberish into still-bad code. The status quo code was bad, but the pipes aren’t fixing it. Again, -1 for Hack.

I could go through the other examples as well (and I did), but I’ll just skip to the result here: if these are supposed to show what we can do with the Hack operator, I’d rather have a language without any pipe operator, than one with Hack.

Frankly, at this point I’m a little bewildered. Surely the Hack operator *can* be used for good, the *first* example was good, but with so many bad examples I’m starting to wonder what problem we were trying to solve again, and I wonder if this proposal is still written with the right priorities in mind. So I read back into the proposal’s motivation, and read this:

> “It is often simply too tedious and wordy to write code with a long sequence of temporary, single-use variables. It is arguably even tedious and visually noisy for a human to read, too.”

Well, I was indeed arguing about the readability, and my opinion is that most examples with Hack are *not* any more readable than their status quo counterparts. Even less so, for someone not familiar with their style. But this motivation seems to suggest readability wasn’t the primary concern, writability was. And as someone who cares about maintainability over writability, I oppose this motivation.

From a maintainability perspective, I’m starting to believe the Hack proposal might actually do more harm than good.

Note I haven’t talked about the F# proposal for a while here. That’s because the proposal is much simpler. It’s focused on unary functions and leaves it at that. It wouldn’t have made the bad examples better, but that’s because it doesn’t try to. And in my opinion, that’s a good thing. Just because we get pipe operators, doesn’t mean we have to shoehorn the entire language into it.

I think it also explains *why* the F# proposal is more elegant than the Hack proposal. Because while pipes indeed allow you to omit naming of intermediate results, the names of the unary functions still clearly describe the steps taken. Thus the code stays self-documenting. Contrast this to Hack where neither the results nor the steps are self-documenting anymore, and I don't see the latter adding much value over the status quo.

## An emotional perspective

While I thought I had a clear idea of why the Hack proposal was more practical to use, it never felt quite right. It’s always hard to articulate such feelings. Maybe it was because the caret felt like a red flag. Maybe I already had a nagging feeling that arbitrary expressions would lead to a rabbit hole, something I hopefully demonstrated in a more concrete manner above.

But apart from my own feelings on the matter, I also see an emotional outcry from people on Reddit. They’re calling the Hack proposal a mistake, even if they cannot articulate well why. I’m getting the impression (though I have no data to back this up) the people that were most looking forward to the pipeline operator, are those that were hoping for the F# proposal. Instead, we might be getting the Hack proposal which compromises to those that don’t really seem to care about either the operator, or the maintainability of the code it produces.

It’s hard to put into better words, so I’ll just add my voice to the choir: the Hack proposal feels like a mistake.

## Summing it up

When I started writing this post I couldn’t really make up my mind between the two proposals. Hack seemed more practical, but didn’t feel quite right. F# seemed to have nicer syntax, but it was less generally applicable. Those observations haven’t changed, but if you read this far, I think my preference is clear now.

But I am also a pragmatist, so the burning question is: does the practicality of the Hack proposal weigh up against the various arguments against it? I am going to say No, and favour F# instead. And after this deep dive I would even say I would rather not have any pipe operator than have to deal with Hack, for the reasons I explained above: not because the operator is inherently bad (it *can* be used for good), but because I see it as a major footgun that’s too easy to abuse. If the proposal’s champions have such a hard time coming up with convincing examples, how can I expect junior or medior developers to use such an operator responsibly? How am I going to explain how you can improve your code with such an operator, when I don’t even see most of the examples in the proposal itself as an improvement?

I just hope it's not yet too late, and the F# proposal can still be championed. It would be a more modest addition to the language. One that is only applicable in some scenarios. And that's okay. Because at least it handles those *well*, and at least we can agree that for those scenarios, it's actually an improvement.
