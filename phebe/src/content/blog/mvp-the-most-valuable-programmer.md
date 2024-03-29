---
title: "MVP: The Most Valuable Programmer"
description: "The Most Valuable Programmer isn't a concrete concept. Rather, it's a goal that you strive towards for becoming the best version of yourself."
pubDate: "Apr 10 2023"
---

The abbreviation MVP usually stands for Minimal Viable Product, at least if
you're working in the field of software engineering. But today I want to talk
about a different kind of MVP: *The Most Valuable Programmer*.

Just like the Minimal Viable Product, the Most Valuable Programmer isn't a
concrete concept. Rather, it's a goal that you strive towards. Also, it's not
about being the most valuable among your peers. Instead, it's about becoming the
best version of yourself. Let me elaborate...

## A Younger Me

The other day I was reminded of a conversation I had with a senior developer probably close to 15 years ago. I don't remember exactly what had gotten into my head, but I had taken it upon myself to micro-optimize a few large PHP files. And this was in the days before we used op-caching, so files were parsed over and over for every request. In order to optimize this I changed all the double-quoted strings to be single-quoted instead, because I had read somewhere that parsing those was 11% faster (or something to that effect) due to the lack of escape sequences.

All of this had resulted in a diff that was rather huge and chock-full of tedious changes. And it was this senior developer who had been given the pleasure of reviewing those. He gave me a bit of a scolding for creating such a large diff without prior discussion, which had caused him quite the headache, but he was surprisingly polite about it. It might have been my saving grace that he assumed that I had used an automated formatter on the code, because as he mentioned, nobody in their right mind would spend the time to do such a thing by hand. I agreed.

But this was also before the days that automated formatters were really commonplace and I had in fact done everything by hand. Oh silly me. I knew very well that it was not in my best interest to admit it at the time, but I had absolutely been stupid enough to have wasted several hours replacing string quotes and making other tedious changes, and in turn had wasted my senior's time reviewing all of that.

There might even be some interesting pedagogical aspect to how this lesson was taught. Had he really assumed I was using an auto-formatter, or was he just giving me the benefit of the doubt? Had the lesson stuck precisely *because* his polite snark had brought shame upon myself? Regardless, the lesson was learned and it helped me take one of those many steps towards becoming a better programmer. A more valuable programmer, if you will.

## Code versus Value

I have been coding for about 28 years now and I used to take great pride in my code. I would love to draw diagrams of architecture, I had a rigid coding style, followed meticulous interface guidelines, and always had performance on the top of my mind. It's really no wonder I was proud of my code, because it truly was a thing of beauty. Or so I thought.

I suppose it might have been a luxury that was afforded to me because I started programming even before high school. It allowed me to perfect my art without any pressure from employment, and frankly without any consideration for what really matters. I learned immensely from those early years of programming, but I think my appreciation of the code itself might have been something that held me back for many years after.

If you ever want to become a better programmer, please take this advice: **Don't even *try* to become the best programmer.** Nobody will agree what it means to be the best programmer anyway, so it's a futile goal. Chasing of wind. Instead, try to become the most *valuable* programmer. Value is still a rather abstract concept, but at least it can be tied to more concrete goals, such as business value.

I think one of my biggest mistakes was an abstract idea that I believed in for many years: The idea that code is valuable. It's not. **Code is a liability.** Once code is written it needs to be reviewed, it needs to be maintained, it may need to be debugged, or rewritten, or even thrown away. But once it's there, it becomes a time sink. There is no value in code. There is only value in value. That may be a tautology, but it's so fundamental that it bears repeating: You gain value from solving the problems that your code solves, not from the code itself. The less code you need to solve your problem, the better.

Consider this: If you're an engineer you get hired to solve problems. Code may be your weapon of choice, but you don't get paid to deliver code. You get paid to solve problems. The more problems you solve, the more value you deliver. The more code you deliver, the more of a burden you become...

So how do you avoid becoming a burden, and how do you become a valuable programmer that solves many problems? **Prioritize.** And a good trick to teach yourself to prioritize is a change of mindset: You're not trying to become a valuable programmer. You are a valuable programmer. You are your own most valuable asset. And what are your scarcest resources? Time and energy. There's only so many hours in a day, and there's only so much energy you can muster. Don't waste it on code formatting, but help out solving the problems the business or your project needs solving most.

## Code Style

I suppose I could have stopped there, but I had all these topics in mind and I'm not going to waste them. And since they're all topics that most programmers should answer for themselves at some point, we might as well get on with it. We already touched on the subject of code style, so it's a natural place to start. It's also a good one to highlight a fundamental contradiction in all of this: Many things are simultaneously important, and they all deserve our attention. Prioritization isn't merely picking up what's most important and dropping everything else. It's about finding a balance that ensures your basic needs are met so that you can spend most of your time focusing on the things that matter most.

Code style is important for many reasons. We want our code to be readable so our peers can review it and so that we ourselves can still understand it if we have to dive back in later. If everyone in a team follows their own style it tends to distract from what the code is trying to achieve. Code written in a different style from what you're used to is harder to read because it goes against your expectations. Compare it two people speaking in different dialects: They might both be speaking English, but it becomes harder to focus on the message.

But ultimately, it doesn't matter so much which dialect you speak, so long as everyone speaks the same. For software, that means agreeing on a code style and staying consistent. There are countless debates to be found on all the minutiae so I'm not going to repeat them here. Make a choice that makes you happy as a team and stick with it.

And make sure you use automation to verify your style. No better way to avoid wasting other people's time on nitpicking than to let the machines do it.

## Correctness

Oh, the joys of correctness. Both of utmost importance, for obvious reasons, and a potentially endless timesink, for much more subtle ones. Making sure your code is correct is one of the primary responsibilities of a programmer. Bugs may bite your users, and that's not good for business. Not to mention that haunting them down is also a nasty time-consuming job that nobody likes to do. Better to avoid them in the first place.

So our code should always be correct, right? Well, it depends.

For example, let's assume you're writing a script to handle some automation within a repository. Maybe the script wouldn't be able to handle file names that are not valid UTF8. That's a bummer, and you can argue that's not very correct. But if none of the files in your repository would cause it any trouble, it's certainly correct *enough*.

That's a very different story from when you're building a client application that you distribute to end users and which needs to be able to handle arbitrary paths on their machines. People may use all kinds of locales, and sooner or later you might run into someone with file names that aren't valid UTF8. The threshold for correctness may differ very much per situation.

In general, I think it makes sense to say that the programs we write should produce correct results for all the inputs that may be reasonably expected. Maybe you work in an industry where bugs can create life-threatening situations, in which case you probably have a very strict interpretation of what may be "reasonably expected". But going beyond the requirements for your problem domain is often a recipe for writing lots of code with little value. Small chance anyone will thank you for it.

## Dryness

DRY stands for Don't Repeat Yourself. Rather than copy-pasting code and modifying tiny bits to fit different use cases, it's usually better to write code that's a bit more reusable and can be used for both use cases. But this by itself presents another trap that junior programmers may fall into.

Mantras, when taken to any extreme, usually lead to them being applied to situations where they backfire. DRY was invented to ease maintainability. After all, if you later need to update the code, you likely only need to update it in one place instead of hunting down all the places it was copied to and possibly missing some. That's great and all, but if you keep on extending a single function with various options and branches to make it cover an increasing amount of use cases, that function itself will become a hazard to maintainability.

In this specific case, it would probably be better to split a large function into smaller ones. Then you can compose them back into larger, use-case-specific ones, even if that introduces a bit of boilerplate. But in the more general sense, always try to question what the purpose is for a given guideline. Following guidelines isn't bad, but learn to recognize when it's a good time to step away from them.

## Performance

Many a programmer's darling. If nobody appreciates the beauty of your code, at least you can revel in how many allocations you saved. I know — I once replaced hundreds of quotes because supposedly that made parsing them 11% faster on code that was never a bottleneck in the first place.

Just realize, that unless you work on the Linux kernel or some special embedded domain, obsessing about performance is wasting your own energy and not delivering any real kind of value.

That's not to say that performance isn't important (all of these topics are), but delivering good performance is again very much about picking your battles. Optimize your critical path, if you have any. Batch requests instead of bombarding your API or database with dozens or hundreds of requests. But you don't add any value trying to optimize things that are fast enough anyway.

## Adding Value

We've covered plenty of examples to show that restraint is good. Don't go overboard and you're halfway there on your path to becoming a valuable programmer. But where *should* you focus your energy? How *do* you add value?

Here is a random list of ideas that's by no means authorative...

- Try to understand the business' motivation for functional requirements. Once you understand the problem domain well, you may be able to offer simpler alternatives that take less effort to implement.
- Identify unaddressed problem areas. These can be technical such as common causes for bugs, but may as well be process-related or organizational, such as causes for reduced velocity or team morale. Do your research, then propose solutions. Offer yourself as willing to address them. Many times you'll find you're not the first to notice them, but it may just take someone willing to put in the effort.
- Take your time reviewing your co-worker's code. When looking at a pull request try to understand what problem they are trying to solve, and whether their solution makes sense in that context. Can you think of anything they might have missed? This is also a great opportunity for knowledge sharing. Don't just point out what they missed, but if it's a system you're familiar with, you may also be able to offer some background as to why things are the way they are. You might even think of ways to improve the maintainability so the next person won't miss the same thing.
- Communicate. Make sure others know what you're working on, and have some sense of what others are doing. If others are unaware of your work, they also cannot offer advise. You might think you have a good idea and want to surprise your peers with a working solution. But in an organization, surprises are rarely good, and you don't want your good idea to interfere with someone else's.

## Don't Forget Yourself

Thanks for making it this far. Hopefully I can offer you one last gem, as for some this might be the most important advice of this piece: Don't forget yourself.

I mentioned before that time and energy are your scarcest resources. I also mentioned that prioritization is about finding a balance that ensures your basic needs are met. If you run out of time, you might miss a deadline and that's not good for business. If you run out of energy, you may risk a burnout and that's not good for anyone, least of all you.

But before you run out of either, you usually enter a negative spiral that can be recognized. If you have little time, it causes stress that can cause you to burn energy faster. If you're low on energy, you start to lack motivation and you start taking more time to complete basic tasks. If you notice these signs, it's a very clear indication your basic needs are not met and **you need to speak up**. If your manager has to ask why a deadline was missed it's too late. And if you have to take sick leave to recover from burnout it is definitely too late.

There are many ways to prevent this negative spiral or to step out of it once you notice the early signs. First of all, don't over-promise as it's a surefire way of taking on more work than you can handle. But also if you notice a particular task is draining your motivation, ask your peers for help, or put it on the backburner instead of forcing yourself to finish it at once. And if you feel a deadline is unreasonable, tell your manager well in advance. Don't beat yourself up if you cannot make it.

Make sure you take time for yourself, your family and/or your hobbies. For me personally, reading up on and experimenting with technology used to be very much of a pastime, though nowadays I often find myself writing fiction<a href="#ref-1"><sup>[1]</sup></a> instead. I love spending time with my wife and my son, and I can be perfectly content not thinking about work or programming at all.

None of that compromises the idea of being a valuable programmer. You need to relax and stay healthy to be happy. Only then can you keep up the energy to keep improving yourself. Happy programmers tend to be more productive<a href="#ref-2"><sup>[2]</sup></a>.

And after all, you are your own most valuable programmer, so take care of yourself.

Love, Arend.


<a id="ref-1">[1]</a> [Aron Silver — an author of little renown](https://aronsilver.com)

<a id="ref-2">[2]</a> [Happiness and the Productivity of Software Engineers](https://link.springer.com/chapter/10.1007/978-1-4842-4221-6_10)
