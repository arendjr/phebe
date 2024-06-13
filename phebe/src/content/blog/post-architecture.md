---
title: "Post-Architecture: An Open Approach to Software Engineering"
description: "On the implications of defining the architecture after you build the product"
pubDate: "Jun 13 2024"
---

For a while I described myself as a software engineer and architect. But the
word "architect" always struck me as a bit of a misnomer; the main reason I used
it was for lack of a better word. Today I think I found that word:
post-architect.

The reason "architect" felt off to me is twofold: First there is a bit of a
stigma around it. The term ["architecture astronauts"](https://en.wikipedia.org/wiki/Architecture_astronaut) quickly comes to mind. Now I'm not a big believer in
putting people into boxes, and using a term that will encourage others to put me
into a box isn't a great start. But the second part of it is that I don't even
believe in software architecture as a separate discipline per se. Or at least,
that's not how I practice it.

There's a lot of different architectures for different software projects. How do
you pick the right one? If you're building the next CRUD application, you may
have an easy time here, but if you're working in a startup and you're building
something more exploratory, the decision can be a challenging one. A gamble
even. For me, there's certain software engineering *principles* I gravitate
towards, but I certainly don't have a favorite architecture. I don't even
particularly like designing an architecture. What do I like? I like building
products and solving problems.

At this point, you might almost start to think architecture isn't really my jam,
so why did I put it on my resume to begin with? Well, I do have hands-on
experience
[building](https://fiberplane.com/blog/announcing-fp-bindgen)
[complex](https://fiberplane.com/blog/a-deep-dive-into-fiberplane-s-operational-transformation)
[software](https://fiberplane.com/blog/writing-redux-reducers-in-rust)
[products](https://fiberplane.com/blog/creating-a-rich-text-editor-using-rust-and-react)
from the ground up
*and developing and maintaining them for many years after*. That means living
with the technology choices I make for many years, and adapting the software to
remain scalable, maintainable and extensible. If that doesn't involve
architecture, I don't know what else does.

Back when I was younger, I used to have a more lofty attitude towards
architecture. It appeared as if good architecture was the Holy Grail of software
engineering that once attained would guide engineering principles, solve
technical debt, and be, on its own, a thing of beauty. Except that it didn't
really turn out to be that unattainable, and when I held it in my hands it
turned out that it was just a cup. Another tool to be used like any other, and
the mystical search had just been smoke and mirrors.

As I wrote in [MVP: The Most Valuable Programmer](/blog/2023/04/mvp-the-most-valuable-programmer):
"The less code you need to solve your problem, the better." If you start out by
creating an architecture, you will start by writing a lot of code and solving
very few problems. Moreover, you'll be solving problems _you don't have yet_.

When I started my previous job, I was asked what architecture I was going to
use. The job before that I had indeed started with nice diagrams and overviews.
But in hindsight, that had provided little value: Both were quickly outdated
when reality turned out to be more intricate. So this time I made a conscious
decision to not bother. I answered: I'll just keep it simple and we'll take it
from there.

And that turned out to be the best decision I could have made. Most of us have
heard of the [KISS principle](https://en.wikipedia.org/wiki/KISS_principle), but
this was the biggest validation of it I have witnessed in my career. Not only
did we build an impressive architecture, we built it without foresight. Because
it wasn't needed. All along the way, we just built the pieces that we needed and
moved on. And every time pain points showed up, we refactored and improved.
Lather, rinse, repeat.

To an architecture astronaut, parts of the functionality we delivered may have
looked like a hackjob. But that was okay, because whenever this was the case, it
meant that part of the codebase had no fixed architecture yet. And as long as
the architecture isn't fixed yet, you can't violate it. It's open. Open to
modification, open to improvement, open to refactoring. Even open to be thrown
away if the functionality isn't validated.

Of course, the deeper you went into the system, the more fixed the architecture
became. That's natural. But it became fixed _after_ validation, after
implementing various use cases, after refactorings to make sure it would handle
future use cases as well as the ones that had come before. A single use case
should not define an architecture. And at least if you care about exploratory
development, architecture should not lead product development, it should trail
it. First you define your use cases, then you build implementations. And then
you validate. Architecture comes after.

None of that is to say there are no projects that would benefit from a more
traditional approach to architecture. If you're doing a waterfall project and/or
your requirements are well-defined upfront, this post may not really apply to
you. But for startup environments like the ones I've witnessed, I think this
approach deserves a word on its own.

This is what I will call Post-Architecture.
