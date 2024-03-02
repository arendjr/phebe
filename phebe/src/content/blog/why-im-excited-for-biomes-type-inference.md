---
title: "Why I'm excited for Biome's type inference"
description: "An overview of what Biome's upcoming type inference is planned to do, and why I'm excited for it."
pubDate: "Jan 28 2024"
---

A few weeks ago, Biome published their
[2024 roadmap](https://biomejs.dev/blog/roadmap-2024/). There's a lot of good
stuff in there, but one thing has me particularly curious: type inference for
implementing lint rules that rely on TypeScript type information.

There's a lot to unpack in that single sentence. What is Biome actually trying
to achieve here?

First of all, I would like to stress that they're not trying to create a full
implementation of the TypeScript compiler. And not even a full implementation of
its type-checking capabilities either. For those, users are expected to continue
to use the official TypeScript compiler. So what _are_ they trying to do then?

Biome is a linter (among other things) and as such they're competing with
ESLint. One of the most popular plugins for ESLint is
[typescript-eslint](https://typescript-eslint.io/), which is able to use
TypeScript type information to implement lint rules that go beyond what ESLint
could offer out of the box. I like to call those "type-informed" lint rules.

Biome has already implemented some of the lint rules from the typescript-eslint
project, such as
[`useConsistentArrayType`](https://biomejs.dev/linter/rules/use-consistent-array-type).
That rule is specific to the TypeScript syntax, but it is not a type-informed
rule.

An example of a type-informed rule is
[`await-thenable`](https://typescript-eslint.io/rules/await-thenable). What's
the difference? While `useConsistentArrayType` operates on the TypeScript
syntax, `await-thenable` relies on actual type information to know which
expressions evaluate to something thenable.

Let's take this statement from the `await-thenable` example:

```js
await createValue();
```

Depending on what `createValue` contains, the rule will either report a
diagnostic, or not. Is `createValue` an `async` function, or another function
returning a `Promise`? Then it's fine. But if it's not, then a diagnostic is
reported. And that's the kind of decision making that requires type information,
which only type-informed rules can do.

So today, Biome is able to implement TypeScript rules that operate at a syntax
level, but type-informed rules are still out of its reach. The typescript-eslint
project solved this problem by talking to the real TypeScript compiler to
extract type information from it. Biome could do something similar, but there's
a catch: It comes with a
[significant performance](https://www.joshuakgoldberg.com/blog/rust-based-javascript-linters-fast-but-no-typed-linting-right-now/#type-checked-linting-performance)
overhead. For some users, that overhead is worth it. But Biome is attracting
users from other tools on the premise of being magnitudes faster. Going the same
route as typescript-eslint wouldn't provide a very compelling reason to switch.

So instead, Biome intends to implement a minimal subset of type inference (the
part that tracks what type a given expression evaluates to) so that it is able
to implement type-informed lint rules of its own. Its approach would be limited
by nature, but that's all right: Users are still expected to use the TypeScript
compiler for type safety. The linter is "merely" an additional safety net, and
it doesn't need to be perfect. If it can catch some user's mistakes, while being
much faster than alternative solutions, it would be a very useful addition to its current offering.

## Looking ahead

So far, I've only explained what Biome's goals for 2024 are. There's no
guarantee they will be delivered in time, of course, but generally I think they
make a lot of sense. But what makes me more excited than the near-term is the
potential longer term impact: Could this lead to the implementation of a full
TypeScript compiler in Rust?

Building a TypeScript compiler in Rust is a monumental task, but that hasn't
stopped people from trying. Most notably there are the
[`stc`](https://stc.dudy.dev/) and [Ezno](https://github.com/kaleidawave/ezno)
projects. Both have been at it for over a year, but neither is showing signs of
offering "good enough" compatibility to replace the TypeScript compiler anytime
soon. That's not a dig at them, because I applaud their efforts. But the reason
I think they face an uphill battle is because it's very hard to convince people
to switch if you only offer 70% or 80% compatibility, no matter how much faster
your offering is. And getting to even that level of compatibility seems to be
well over a year of effort on its own. Not to mention that even when you get to
90% compatibility, you *still* have
[90% of the work](https://en.wikipedia.org/wiki/Ninety%E2%80%93ninety_rule)
ahead of you.

Imagine the situation where you're building a TypeScript reimplementation from
scratch, with nary an end in sight, and no users. And while you're working on
it, the TypeScript team keeps churning out new versions and your target keeps
slipping away again. I don't think anyone can fault the `stc` developers for
giving up on their project, or the Ezno developers for not making fast enough
progress.

Where Biome is different is that they intentionally *don't* aim to make a
reimplementation of the TypeScript compiler. For them, 70% compatibility on type
inference alone would be enough to offer valuable type-informed lint rules.
Assuming they get there, they will also be likely to continue to improve. And
maybe more importantly, because they're already offering value to users, the
developers will be more motivated to keep up the work.

And meanwhile, almost as a side-effect of building increasingly good type
inference, the team will be building the foundation on which a true type checker
could be built. It may take years to get there, maybe they'll never get there.
So maybe this is all just hopeful thinking on my part. But it is indeed my hope
that one day we may actually see a TypeScript reimplementation written in Rust.
And I suspect that Biome may be the most promising route to get there.

## Discussion

Of course there's still plenty of challenges ahead before Biome's type inference
becomes good enough that it could power a true type checker with sufficient
compatibility that users can use at a replacement. First they'll need
compatibility with `.d.ts` files, NPM compatibility, various module resolution
modes, and plenty of non-trivial stuff. Is it feasible? Other challenges I
hadn't thought of? Or do you have suggestions that might help the Biome project?

Join the [Biome Discord server](https://discord.gg/BypW39g6Yc) to discuss, or
reply to the
[thread on Mastodon](https://mstdn.social/@arendjr/111834501316186795).
