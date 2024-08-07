---
title: "Post-Architecture: Premature Abstraction Is the Root of All Evil"
description: "Practical tips that allow you to build an evolving architecture"
pubDate: "Jul 18 2024"
mastodon:
  toot: "112806380832819305"
---

At this point, I've written a fair bit about Post-Architecture. Previous posts
focused more on the values underpinning Post-Architecture and why following a
Post-Architecture approach may be beneficial to you:

* [Post-Architecture: Post-Architecture: An Open Approach to Software Engineering](/blog/2024/06/post-architecture)
* [Post-Architecture: What It Is And What It Isn't](/blog/2024/07/post-architecture-what-it-is-and-isnt)

To recap: Post-Architecture is a method of defining architecture incrementally,
rather than designing it upfront. It prioritizes velocity during the early
phases of development and minimizes the risk of making the wrong architectural
decisions.

With this post I want to take a bit of a different direction, instead focusing
on how we can apply Post-Architecture in a practical sense. To do so, I want to
use this post to dive into the shadow side of abstraction. People who work with
code architecture a lot often view abstraction as a useful tool to model complex
problems. And they're not wrong. Where it goes wrong however is when people try
to apply abstractions prematurely. In fact, I would adapt one of
[Donald Knuth's famous quotes](https://effectiviology.com/premature-optimization/)
to say:

> Premature abstraction is the root of all evil.

If abstraction is applied too soon in the process, it has a tendency to hinder
rather than help maintainability. So even though abstraction is regarded as a
useful tool for maintainability, it's very much a double-edged sword. Make one
wrong decision, and you achieve the opposite of what you intended.

Of course I'm not the first to realize this. I'm even not the first to make this
particular adaptation of the quote. I found
[this blog post](https://zackoverflow.dev/writing/premature-abstraction) which
effectively comes to the same conclusions. Go ahead and read it, it's nice and
short :)

This post won't be as short. I want to take a deep dive into how we can change
our mindsets so that we don't fall for the false appeal of early abstraction.
The overall theme here is that you should strive towards simplification, since
it will allow you to do faster iteration and it will put you in a better
position to refactor when necessary.

I also want to explore some ideas on what kind of architecture can be useful to
preserve most of that initial simplicity.

# Embracing Procedural Programming

So if it is simplicity we want, and avoiding unnecessary abstractions, how
simple does it get? I usually try not to speak in absolutes, but I'm willing to
say it can get _very_ simple indeed, without losing anything of value. We'll
go back to the basics of procedural programming, before seeing which properties
we want to add as we scale back up again.

But before we get there, let me indulge with a little story...

## What I Learned From C++

When I was in university, I learned programming in my first year with a course
in C++. Although "learned" wasn't quite the right word, because I had been
programming for about 7 years at that point, but I was happy they taught us C++.
Why? At that point I had dabbled my feet into BASIC, Pascal, C, Assembly, some
PHP4, and indeed C++. In my mind, C++ was where it was at: You had classes and
other high-level abstractions, and still regained full power over what your
program was doing. You could even drop down into Assembly for the parts where it
really mattered (I was also quite into making my own video games in the early
days, and in the DOS era that meant doing your own video drivers, so this was
somewhat of a thing).

Being taught C++ in university made me feel validated in my choice, and I've
been "enjoying" the language for many years since. The reason I put "enjoying"
between quotes is because this was very much a Stockholm syndrome kinda thing.
I appreciated the language from my values as a game developer, even though
fortunately I never made a career out of it (I still make the
[occassional game](https://apps.apple.com/us/app/sudoku-pi/id6467504425)
though). But it was that same appreciation that made me blind to its flaws for
many years. In fact, I think it took me roughly another 15 years before I could
see the flaws in C++ that are nowadays so obvious for anyone to see.

What flaws am I talking about? I could write many blogs on this topic alone, and
many others already have, so I'm going to spare you the most of it. But there is
one particular idea, and it's not even specific to C++, that stands out the most
to me. It's just that the reason why it stands out to me in particular is
_because_ I had been taught C++ in my first year of university. Back then
object-oriented was all the rage, Java was just released (but it was _slow_,
best not to go there!) and C++ was the obvious improvement over the primitive
language called C, which can only do procedural programming. The point is, if
you started writing a new program, you would be _insane_ not to do so using the
latest object-oriented insights.

I remember having a discussion on this very topic with my uncle about two
decades ago, where he proclaimed that all this object-oriented stuff would just
get in the way, and he was much happier to just use plain C instead.
_How could he not get it?_ I didn't get it.

It's kind of understandable that when the first "real programming" classes
you're taught jump straight into full-blown object-oriented programming
paradigms, you never really stop to wonder why those things are the way they
are. As I said, it took me another 15 years before I pulled back the curtain far
enough to realize that those early teachings were not universal truths at all,
but merely highly opinionated guidance, some of which hasn't really stood the
test of time.

## Object-Oriented Code vs. Post-Architecture

Don't worry, I'm not going to turn this into an anti-object-oriented-programming
rant. But I do like to take time elaborating on this topic, since to this day I
still see plenty of people who are going through this very same journey of
realization. Just know that if you want to learn to embrace simplicity,
object-oriented programming is **not** where it is at.

Take a step back.

Look at this little nugget of code (I'll use TypeScript syntax throughout this
post for the sake of simplicity):

```ts
class Foo {
  private readonly bar: Bar;

  constructor(bar: Bar) {
    this.bar = bar;
  }

  doFoo(arg: string) {
    this.bar.doFoo(arg);
  }
}
```

Can you tell me what is wrong with this?

There is nothing _fundamentally_ wrong with this piece of code, and yet, if you
truly want to embrace post-architecture, few things could be more fundamental.

The real problem with the `class Foo` above is that it is utterly and entirely
_unnecessary_. This class represents an abstraction, and what does this
abstraction achieve? It allows us to write code like this:

```ts
const foo = new Foo(bar);
foo.doFoo("Hello");
```

But without this abstraction, we could have written the following instead:

```ts
bar.doFoo("string");
```

It would do the exact same thing. No extra code (one line less even), and no
unnecessary abstraction. So we should ask ourselves the question: Is the latter
in any way worse than the former?

If you had asked me that question 20 years ago, I might have tried to come up
with arguments about how the former represented _good design_ because it was
following code patterns, maybe I would have thought up a reason about
encapsulation or extensibility, or I don't know. We all like to laugh at Java
developers and their `AbstractSingletonProxyFactoryBean` classes, and of course
it's a relief to know I wasn't the only one — or even the worst — falling into
this trap. But we should be honest, many of us were guilty of overcomplicating
our codebases at one point or another.

Some also think that a class-based approach helps with testability. For
instance, consider the following class:

```ts
type DateProvider = { now: () => number };

class RelativeTimeFormatter {
  private dateProvider: DateProvider;

  constructor(dateProvider: DateProvider) {
    this.dateProvider = dateProvider;
  }

  format(date: Date): string {
    const now = this.dateProvider.now();
    const time = date.getTime();
    if (time - now < 60_000) {
      return "Just now";
    } else if (time - now < 60 * 60_000) {
      return `${Math.floor((time - now) / 60_000)} minutes ago`;
    } else if (time - now < 24 * 60 * 60_000) {
      return `${Math.floor((time - now) / (60 * 60_000))} hours ago`;
    } else {
      return date.toLocaleString();
    }
  }
}
```

Now we can create unit tests for arbitrary dates by injecting a custom date
provider. A fine example of dependency injection.

Except the same could be achieved through a much simpler function:

```ts
function formatRelativeTime(date: Date, now = Date.now()): string {
  const time = date.getTime();
  if (time - now < 60_000) {
    return "Just now";
  } else if (time - now < 60 * 60_000) {
    return `${Math.floor((time - now) / 60_000)} minutes ago`;
  } else if (time - now < 24 * 60 * 60_000) {
    return `${Math.floor((time - now) / (60 * 60_000))} hours ago`;
  } else {
    return date.toLocaleString();
  }
}
```

So nowadays I'll be more outspoken: In both these examples you should prefer the
latter. It's simpler, there's less code to maintain, and it's easier to extend
or refactor, since there's less code to begin with.

That doesn't mean design patterns don't have their uses, but it does mean you
should only use them when you need them. Any abstraction you don't need is one
too many.

And to stress that last sentence one more time: It's worse to pick the wrong
abstraction than to pick _no_ abstraction. Premature abstraction is the root of
all evil.

## Functional Programming vs. Post-Architecture

So if object-oriented programming isn't where it's at, maybe functional
programming is? Not necessarily.

Even though I've taken a liking towards some of the values of functional
programming, I only endorse functional programming so far as it sticks to the
basics. Just like with object-oriented patterns, feel free to use more if you
have a need for it, but don't pull out the big guns just because you have them.

Since I mentioned I like to make games sometimes, let me apply a quote from
John Carmack here:

> A large fraction of the flaws in software development are due to programmers
> not fully understanding all the possible states their code may execute in.
> [...] No matter what language you work in, programming in a functional style
> provides benefits. You should do it whenever it is convenient, and you should
> think hard about the decision when it isn’t convenient.
>
> \- https://www.gamedeveloper.com/programming/in-depth-functional-programming-in-c-

So if you're not using a functional language, say TypeScript, what does it
mean to use a functional style? I think the main things you should keep as
guidelines are:

* Stick to procedural programming, i.e. use plain functions as your building
  blocks.
* Make sure your functions are pure (i.e. don't mutate their inputs or global
  state, or produce other side-effects) wherever feasible.

I guess you could say you'll end up with procedural programming with a
functional flavor. For example:

```ts
/// BAD: This mutates `input`, which may be unexpected by the caller.
function foo(input) {
  input.bar = "output";
  return input;
}

/// GOOD: `input` is preserved and a new object is created for the output.
function foo(input) {
  return { ...input, bar: "output" };
}

/// BAD: You probably have no need for this type of "currying" in TS:
const sum = (a) => (b) => {
  return a + b;
};

/// GOOD: Simple and to the point.
function sum(a, b) {
  return a + b;
}
```

Using pure functions really has a tremendous amount of benefit:

* They avoid [action-at-a-distance](https://en.wikipedia.org/wiki/Action_at_a_distance_%28computer_programming%29).
* They are trivial to unit test.
* They're safe to use in a multi-threading environment.
* They are easy to compose into bigger functions, so when you do need a bit of
  abstraction, you can do so using the simplest tools at your disposal.

Summing up, I think for any fresh (post-)architecture, the best thing to do is
to start with the absolute basics. Embrace the basics of procedural programming,
with a touch of functional flavor, avoid unnecessary abstractions, and focus on
the minimal set of things you need to get a working solution.

And when you do need to expand your abstractions, feel free to choose either an
object-oriented or a more functional programming path, as long as it fits your
programming domain. To find out which path may be more beneficial to you,
consider the following quote:

> OO makes code understandable by encapsulating moving parts. FP makes code
> understandable by minimizing moving parts.
>
> \- https://www.johndcook.com/blog/2010/11/03/object-oriented-vs-functional-programming/

In this light I would say that if you _can_ minimize the moving parts, you
should, and a more functional path is probably beneficial. But in some software
systems it's inevitable to have many moving parts, and a more object-oriented
path may be more beneficial instead.

# Focus On Data Structures Over Code Patterns

It's been a while since I ran into this quote, but it's one that's stuck with me
and that I would like to repeat here. It's from a person you may have heard of,
called Linus Torvalds:

> I will, in fact, claim that the difference between a bad programmer and a good
> one is whether he considers his code or his data structures more important.
> Bad programmers worry about the code. Good programmers worry about data
> structures and their relationships.
>
> \- https://lwn.net/Articles/193245/

The quote's from 2006, but I remember it because it made me think. Better said,
it puzzled me. It's been a while, but I remember this post triggering me into
thinking something along the lines of, "but without code, how are you using your
data structures?" _Of course you have to consider your code first!_ Make a good
abstraction, and the data structure behind it can be swapped at will.

I truly was a [sweet summer child](https://en.wiktionary.org/wiki/sweet_summer_child).

The thing is, most [abstractions are leaky](https://www.joelonsoftware.com/2002/11/11/the-law-of-leaky-abstractions/) and that goes very much for the data structures
they attempt to hide. Often, an abstraction doesn't truly hide the data
structures underneath, but it is bound by the limitations of the initial data
structure(s) used to implement it. You want to refactor and use a new data
structure? Chances are you need a new abstraction.

So in the early stages of design, you really should focus on getting your data
structures right. Refactoring the code around it is much easier.

# Identify Data Flows

Almost every non-trivial program has data flows that can be identified. Are you
querying a database and returning an API response? That's a data flow. Are you
rendering data from a server in a UI? That's another. Even user input in a
client application can be modeled as a data flow from a
`(clientState, userEvent)` tuple to a new `clientState`.

The beauty of data flows is that you can think of them in terms of input and
output. What else has input and output? A pure function. Any data flow can be
implemented entirely using pure functions. Maybe they'll become complex
functions at some point, and over time you'll certainly have many of them, but
fundamentally your problem can be entirely solved through pure functions alone.
And pure functions are easy to test and they are relatively easy to understand,
since they don't have (surprising) side-effects.

What's more, since data flows that are implemented with pure functions are
relatively easy to manage, they largely avoid the need for more complex
architecture to manage them. This is the ultimate spirit of Post-Architecture:
To use simple, straight-forward tools to solve our problems so that we can not
only postpone potentially risky architectural decisions, but also avoid
unnecessary architecture entirely where possible.

# Push State Outward

Earlier I wrote that "if you _can_ minimize the moving parts, you should". Maybe
this raised a simple question: How? Before trying to answer that question, let's
first clarify what we mean by "moving parts". At least that question has an
easy answer: Anything that is stateful can be considered a moving part. This
especially includes I/O, since after all, what is I/O but the exchange of state
with the outside world? So to minimize moving parts, we should reduce the places
where there is statefulness. But that can be easier said than done.

Generally though, there's a simple rule to keep in mind: If you want to make a
system simpler, there's only one place where you can move statefulness: Out. Try
to move it anywhere else and you're just moving the problem and/or making it
worse, so the only place it can reasonably go is out. Now of course you
(probably) cannot push the state entirely out of your program, so the best you
can hope for is to push it to the outer shell, and stick to as much pure code
inside.

This pattern of programming is called
[Functional Core, Imperative Shell](https://kennethlange.com/functional-core-imperative-shell/).
It's a very useful pattern for separating the responsibilies of I/O and state
management from the pure data flows that handle your internal logic.

If you're working with Rust or Python, you may be more likely to have heard of
the [Sans-IO architecture](https://www.firezone.dev/blog/sans-io), which is a
variation on the same theme. It's more focused on pushing I/O out, while solving
the challenges of the remaining state management by using the right data
structures as part of building an explicit state machine.

If you're more familiar with frontend development, the
[Redux Toolkit](https://redux-toolkit.js.org/) is a good example of a tool that
applies pure data flows to client-side state management. By keeping most state
in a central store, you again reduce the moving parts in your codebase. The
trick to working with Redux effectively is to try to put most of your logic in
your reducers, which are pure functions that are very easy to test. The Redux
store even provides a useful boundary if you ever need to evolve the core logic
of your client towards a Sans-IO architecture by
[writing your Redux reducers in Rust](https://fiberplane.com/blog/writing-redux-reducers-in-rust).

It's also no coincidence that Redux itself is inspired by the
[Elm Architecture](https://guide.elm-lang.org/architecture/), the architecture
used by the Elm programming language, a functional language widely praised for
its pleasant developer experience. A quote from the page I just linked:

> This architecture seems to emerge naturally in Elm. Rather than someone
> inventing it, early Elm programmers kept discovering the same basic patterns
> in their code. It was kind of spooky to see people ending up with
> well-architected code without planning ahead!

Sounds familiar?

# Wrapping Up

By now we are getting to the point where you can hopefully see the reason I
placed so much emphasis on procedural programming at the beginning of this post.
At the earliest stages of development, when you're just building your first
proof-of-concept, you don't want to worry about what is going to be your
functional core, or what is going to end up in your imperative shell. You just
build something that works. But by sticking to the basics, especially at the
start, you retain the freedom to move in either direction. When the time comes,
you can identify your data flows and move pure functions into what will evolve
into your functional core. While other code, where statefulness cannot be
avoided might benefit from an asynchronous approach, or object-oriented
wrappers.

Some codebases may not really have a large functional core inside them, because
they're intrinsically doing I/O all over the place. Sometimes you have to accept
that, yet even in such a system you often can find some pure islands.

Ultimately your architecture will be shaped largely by the problem domain that
you're working in, and that's inevitable. What you do have a hand in however, is
making sure your architecture will actually reflect that problem domain, with as
few over-engineered distractions as possible. In order to achieve that, you
keep things simple, and stick to the basics where you can.

Don't start out with complex code patterns, or other unnecessary abstractions.
Don't worry too much about your code, just make sure your data structures are
right. Let your architecture emerge, rather than defining it upfront.

If this sounds like something you can do, you are hereby a certified
Post-Architect ;p
