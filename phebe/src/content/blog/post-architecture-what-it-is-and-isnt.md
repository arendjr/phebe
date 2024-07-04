---
title: "Post-Architecture: What It Is And What It Isn't"
description: "On the implications of defining the architecture after you build the product -- part II"
pubDate: "Jul 04 2024"
mastodon:
  toot: "112727923803808013"
---

After my previous post
[introducing Post-Architecture](/blog/2024/06/post-architecture),
I received a bunch of positive feedback, as well as enquiries from people
wanting to know more. So I figured a follow-up was in order.

To recap: Post-Architecture is a method of defining architecture incrementally,
rather than designing it upfront. It prioritizes velocity during the early
phases of development and minimizes the risk of making the wrong architectural
decisions.

# Goals

The main goal of Post-Architecture is to enable a faster, more open and
iterative approach to software engineering, including architecture, by avoiding
architectural constraints early on in the process.

I think you could say that Post-Architecture is to Architecture what Agile is to
Waterfall. Like Agile, it's not a silver bullet, but an approach that may be
beneficial to certain teams in certain circumstances. In the previous post I
mentioned startups as an environment where this approach works particularly
well. Also like Agile, Post-Architecture embraces the fact that we don't know
everything upfront and we may need to pivot at certain points in time. But where
Agile concerns itself with the development _process_, Post-Architecture defines
a set of values for code architecture that works best in combination with such a
process.

So the reason I picked Agile for this analogy isn't coincidental: I think there
is a high level of correlation between the two. Are you using Agile methodology
or Lean development? You probably want to embrace Post-Architecture as well.

# Post-Architecture as a Set of Values

More than anything, Post-Architecture is about a mindset: It's about being okay
with the idea that not all of the code you write is subject to a predefined
architecture. With Post-Architecture, architecture _trails_ product development.
This is a simple consequence of the values it embraces:

* **[It's better to fail fast than to spend more time trying to succeed](https://www.forbes.com/sites/sunniegiles/2018/04/30/how-to-fail-faster-and-why-you-should/)**. When you
  are early in the process of trying out a new business, or even just a new
  feature, you may not yet know the final direction you end up taking. The idea
  for your business or feature may even fail entirely. The sooner you find out
  what _doesn't_ work, the less effort you will waste, and the sooner you can
  try something else that might work instead.

  Even if the thing you are working on _does_ work out, there might be
  difficulties along the way that need to be resolved before you can succeed.
  You want to discover those as early in the process as you can.
* **[Code is a liability](/blog/2023/04/mvp-the-most-valuable-programmer/#code-versus-value)**,
  especially during early development. If at any point in the development
  process you run into difficulties, and you realize you need to change gears,
  you may need to pivot and adjust your approach. Any code written up to that
  point may no longer suffice, and you may need to update it, or throw parts of
  it away. The less code you have, and the smaller your investment thus far, the
  easier this will be.
* **Validate before you commit**. Committing to a specific architecture is a
  decision with a lot of impact, especially when you pick the wrong one. It may
  feel like a chosen architecture is a safe bet, because it is widely used by
  other projects. But how do you know it will fit _your_ project? If your
  requirements are well-defined upfront you can probably still make a reasonably
  sound bet, but in an agile environment? You better validate your ideas before
  committing to a specific architecture.

  Implementing a single use case is not enough to validate any given approach.
  But by the time you're implementing three similar ones that could benefit from
  a common abstraction? Now you're onto something.
* **[Perfect is the enemy of good](https://www.betterup.com/blog/perfect-is-the-enemy-of-good)**.
  Striving for perfection prevents you from making meaningful progress. Since
  Post-Architecture prioritizes velocity, it takes the position that progress —
  any kind of progress — is preferred over time optimizing towards an abstract
  idea of perfection. It will be hard to find any two people agreeing on the
  same definition of perfection anyway. It's okay if it means you need to spend
  extra time cleaning things up later, as long as it helps you discover more
  fundamental pitfalls early on.

Taken together, these values should hopefully make it clear _why_ you should
postpone major architectural decisions, as well as give an idea as to when it
_is_ the right time to start refining your architecture.

# Post-Architecture is an Iterative Process

Assuming your product or feature doesn't fail immediately, you still need to
make a conscious effort towards defining your architecture. Post-Architecture
is not _no_ architecture.

[Someone on programming.dev](https://programming.dev/comment/10497316) called it
_postponed_ architecture, and they have a point. It very much _is_ postponed
architecture, but that's not the whole story to it. Because by postponing your
architectural decisions, it means you will be subject to a different set of
constraints compared to if you had defined your architecture upfront. Most
notably, potentially a lot of code will have already been written. How can you
define an architecture, when the code you've written thus far looks like it
abides by anything but?

I will give more technical guidance on this in a follow-up post, but the key
point is: You iterate. Don't feel the need to fix everything at once. Make small
incremental improvements that fix concrete painpoints. This also reduces the
risk of breaking things in the process.

What you want to avoid is that you get to a point where you have to halt feature
development for a longer period of time because you are now in a "refine your
architecture" phase. It's hard to speak in absolutes, but usually when an
improvement takes more than a few days, I would question my approach and wonder
if it can be done in smaller steps instead.

# Post-Architecture is Not an Excuse

It's easy to think that since we want to prioritize velocity, it means we can
make a lot of sacrifices to achieve that high velocity. This is **not** what
Post-Architecture is about. While Post-Architecture indeed trades upfront
architecture for a postponed architecture, it's not about sacrificing anything.
Post-Architecture is not an excuse.

Code quality should very much be preserved. We should acknowledge that keeping a
high level of maintainability across your codebase can actually be harder if
your architecture is not yet well-defined. So while Post-Architecture posits
that your architecture will become more fixed over time (and assuming your
project isn't canceled, should even lead to a _more suitable_ architecture over
time), it means you need other means to preserve code quality while it's still
in flux. Use linters, automated formatters, document your code, draw a diagram
if necessary, do proper code review, and especially document anything that
_feels off_. If your code becomes a mess, refining its architecture will be a
messy process as well.

Quick 'n dirty can sometimes be fine, but don't use Post-Architecture as an
excuse to deliver a hackjob. Where to draw the line? As long as someone else can
still easily understand what you did and easily improve on it, you're probably
on the safe side.

# Post-Architecture is Not a Methodology

I really don't care if you write Post-Architecture or post-architecture. I also
don't have a defined set of guidelines that you can follow to become a certified
Post-Architect.

I think every programmer can operate in the spirit of Post-Architecture if they
have the right mindset. To excel in it is merely a matter of experience. And to
gain experience you need to make mistakes. I know I made my share. And that's
alright, because this is a mindset that embraces mistakes as long as you learn
from them — as long as you keep making forward momentum. Just try to make sure
your mistakes aren't huge ones, so keep iterating in small increments ;)

In a sense, this may feel like a cop-out. By refusing to define a methodology, I
effectively refuse to define what good Post-Architecture looks like. If I wanted
to bait my audience, I can always refute counter-examples of where
post-architecture-like approaches failed and _a posteriori_ claim that those
cases wouldn't fit my definition anyway. A true
[no true Scotsman](https://en.wikipedia.org/wiki/No_true_Scotsman) defense.
Alas, I don't intend to argue that Post-Architecture is better or worse than
upfront Architecture, I merely believe its approach is beneficial in _some_
situations.

You could say that Post-Architecture represents an antithesis to the classical
ideas around Software Architecture. There are indeed situations in which all the
recognized best practices of Software Architecture can be thrown as caution to
the wind. But just as it takes skill to recognize when to best apply a given
practice or code pattern, so too does it take skill to recognize when to cast it
aside. Post-Architecture is merely here to say you are not crazy for going off
the beaten path, to guide you as you do so, and to minimize your risk as you
venture out on your own.

![It Depends](../../images/it_depends.png)

*Image courtesy of [Daniel Paschal](https://www.linkedin.com/pulse/true-memes-development-depends-daniel-paschal/)*

# How to Apply Post-Architecture

Thanks for making it this far! If you're still with me after all this writing,
I imagine I've struck a nerve and this is a topic you're interested in. But I
can also imagine you're getting tired of all the abstract talk and are looking
for more concrete, practical pointers on how to really _apply_
post-architecture. Some more technical guidance so to say.

I have taken my time explaining the abstract, so that hopefully the technical
advice can be placed in the right context. But more technical advice _is_
coming... in the next post in this series. Stay tuned!
