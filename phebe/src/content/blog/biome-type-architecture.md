---
title: "Biome Type Inference: A Look Behind The Scenes"
description: "An in-depth exploration of the architecture powering Biome's type inference."
pubDate: "May 12, 2099"
mastodon:
  toot: ""
---

A little over a month ago, Biome [announced](https://biomejs.dev/blog/vercel-partners-biome-type-inference/) their partnership to work with
[Vercel](https://vercel.com/) on improving their type inference.

Concretely, this meant I was contracted to implement type inference in Biome for
the purpose of enabling the
[`noFloatingPromises`](https://next.biomejs.dev/linter/rules/no-floating-promises/)
rule, as well as a similar, but upcoming
[`noMisusedPromises`](https://github.com/biomejs/biome/issues/5541) rule.

So why is this newsworthy? What challenges did we face enabling such rules?
And what is still to come?

If these questions interest you, this post is for you. I'll start by explaining
the challenges we face, the constraints we operate under, and finally move onto
the technical details on how we're tackling all this.

## The Challenge of Type Inference

Nowadays, if you are familiar with the JavaScript scene, you probably recognise
this simple reality: If you want type safety, you turn to TypeScript. TypeScript
is like JavaScript, but with types.

So what does it mean when we say TypeScript has types? Doesn't JavaScript have
types too?

And yes, JavaScript does have types. It even has a `typeof` operator that can
tell us the type of any given value. So what is it really that sets TypeScript
apart from JavaScript?

Well, two things:
* TypeScript can _annotate_ type expectations.
* TypeScript can _validate_ type assignments.

I'll illustrate both with an example.

### Type Annotations

The first thing TypeScript does that JavaScript doesn't is to provide the
ability to annotate variables and parameters with the _type_ they expect. Let's
first look at an example _without_ type annotations:

```js
function greet(subject) {
    console.log(`Hello, ${subject}`);
}

greet('World');
```

Looking at this code, we can understand that `greet` accepts a `string` as the
type of its `subject` parameter. After all, try the following in any JavaScript
console:

```js
> typeof 'World'
"string"
```

Well, indeed. The type of `'World'` is a `string`.

But when we look at the function signature of `greet`, this is not exactly
obvious. We can look at the _body_ of the function, then look at the
`console.log()` statement, and understand that yes, `subject` is being used to
display part of a `string` message. And while in this example, that's not
exactly rocket science, imagine if `greet` had been 10 or 20, or even a 100
lines long.

We don't want to look at function bodies to understand what type a parameter
has, we want it laid out for us. That's why TypeScript allows us to write:

```ts
function greet(subject: string) {
    console.log(`Hello, ${subject}`);
}
```

Such a simple and subtle change. But now we don't need to look at the function
body anymore to understand that yes, `subject` should be a `string`.

Those few extra characters, `: string`, that's what we call a type annotation.
And it tells us, immediately, what the type of a given variable or parameter is.

### Type Validation

Compared to type annotations, type validation may be considered the other side
of the coin. Because if type annotations tell us what a variable _should_ be,
how do we know what it _actually is_?

That's where type validation comes into play. Thanks to the type annotation in
our `greet` function, we, _and the computer_, know that the `subject` parameter
is of type `string`. That means we know we should pass `string`s to it, and
_the computer_ can validate that's what we do.

The computer in this case is `tsc`, or the TypeScript Compiler. While TypeScript
is a language, which dictates us what we can and can't do, it's the compiler
that enforces that we follow the rules.

So what happens if we were to call `greet` like this instead:

```ts
greet(1);
```

The TypeScript Compiler will complain and tell us:

```
Argument of type 'number' is not assignable to parameter of type 'string'.
```

Well, fair enough. It's trying to prevent us from making mistakes, which is
generally a good thing. In fact, that's pretty much the entirety of the reason
for TypeScript's existence. By detecting type errors early, meaning during
compilation instead of at runtime, we can discover mistakes more early, and thus
save ourselves time during development.

Thank you, TypeScript.

### So What's the Challenge with Inference?

Type inference is the art of figuring out what type _something_ evaluates to.
In the examples above that was easy. The type of `'World'` is a `string`, and
the type of the `subject` parameter is also a `string`. In the first case we
know this because `'World'` is a _string literal_, and those are always of type
`string`. In the second case we knew it because of the type annotation.

Similarly, we learned that the type of `1` is `number`. But what's the type of
`1 + 1`? It doesn't take a lot of imagination to figure out it's also `number`.
But what about `1 + ' ounce'`?

See where we're going here?

> *Reader nods understandingly.*

Thankfully, even though the results are not always intuitive, JavaScript defines
pretty well what each kind of expression should evaluate to. For operators, such
as `+`, we can find references what every combination of operands should
evaluate to.

But what about `myFunction()`? This is what we call a _call expression_, and it
means it will call the function `myFunction`, and it will evaluate to the result
of that function.

JavaScript has no way of knowing what `myFunction()` evaluates to, unless it's
a predefined function written down in some standard. But assuming `myFunction`
is defined in someone's own codebase, the answer to what `myFunction()`
evaluates to should be sought in that codebase as well.

TypeScript's annotations make this easier again. Assume the following function
signature:

```ts
function myFunction(): Promise<number> {
    return Promise.resolve(1);
}
```

Great! We know the answer! `myFunction()` evaluates to something of type
`Promise<number>`.

> _"That was easy! So where's the challenge?"_

The problem is that TypeScript's type system is **complicated**. So far, we've
barely scratched the surface, and this blog post is too short to go down into
all of the dark corners. But just to scare you with some imposing terminology,
imagine how the above could be complicated with _generics_, _mapped types_,
_conditional types_, _interfaces_ and _inheritance_, _template literal types_,
_method overloads_, _unions_ and _intersections_, _declaration files_, and more.

If those words don't sound imposing enough on their own, consider that their
meanings aren't even formally defined. TypeScript doesn't have a specification
that says exactly how all those feature are supposed to work. Instead, the rule
of the land is: Whatever the TypeScript Compiler says is right, is right.

Except when a bug report is filed, and accepted by the TypeScript team, in which
case what is right may change when the next version of TypeScript is released.

So now you hopefully see the real problem: TypeScript is a complex moving target
that is not even strictly defined to begin with. As such, it is no wonder that
most tools have hardly made an effort to attempt to interpret TypeScript's type
system. It's a complex endeavour and 100% compatibility seems an ever-elusive
goal.

### What Is Type Inference Really?

> _"But... didn't you say you were contracted by Vercel to implement type
inference in Biome? How are you going to do that if it's an ever-elusive goal?"_

Yes, I did say that 100% compatibility was an ever-elusive goal. Thankfully,
Biome is a linter, not a type checker. As mentioned before, TypeScript offers
two things: type _annotations_ and type _validation_.

Type validation falls into the domain of a type checker. It's what the
TypeScript Compiler is good at, and we as Biome developers expect our users to
keep using it for the foreseeable future. So type validation is _not_ part of
type inference, and we don't even need to strive for compatibility on that
front.

Type annotations, on the other hand, are a tool to make type inference _easier_,
not harder. Type inference can be performed on arbitrary JavaScript expressions.
Having annotations just makes it easier, although admittedly, supporting all the
type system features of TypeScript does add complexity again. But the basics of
TypeScript are pretty well defined at this point, even if it lacks an official
specification.

And because Biome is a linter, not a type checker, it doesn't require 100%
compatibility to achieve value to its users. Seen in the context of rules such
as
[`noFloatingPromises`](https://next.biomejs.dev/linter/rules/no-floating-promises/),
if Biome can infer 80% of the instances where a floating promise would occur, it
can offer 80% of the value such a rule could theoretically provide. And to put
things into perspective, not even the TypeScript Compiler can offer
[100% correctness](https://github.com/microsoft/TypeScript/wiki/TypeScript-Design-Goals#non-goals)
here.

So to sum things up:
* Type inference attempts to figure out what type a given expression evaluates
  to.
* Type annotations help to narrow down applicable types.
* A high degree of TypeScript compatibility is desired to conform to user
  expectations.
* A 100% compatibility with the TypeScript Compiler is not required for
  providing (significant) value.

So what this means is: For Biome, which doesn't need to do type checking, mere
inference, an independent implementation that is very similar to TypeScript's
compiler _may_ be worthwile, even if it doesn't offer perfect compatibility with
`tsc`.

### Why Is It Worthwhile?

> _"All right, so you made a case that Biome_ could _deliver something
worthwhile... But why not just use `tsc` if it's already there?"_

Two reasons:
* `tsc` is slow. That's especially the case today. But the TypeScript team has
  also announced a port to Go, which will supposedly speed things up. How fast
  it will really be remains to be seen however: Even if Biome were to integrate
  with a native-speed version of `tsc`, unless it were a shared library that
  could directly utilise our data structures, there would be duplicated parsing
  of source files, duplicated analysis, and so on.
* `tsc` is difficult to integrate into Biome. Biome is written in Rust, and can
  easily utilise libraries written in Rust or even C. But integrating a project
  written with Node.js means we have to launch an external process and integrate
  through IPC (Inter-Process Communication), which generally involves quite some
  complication. And moreover, when Biome needs to launch an external process, it
  shifts the burden onto our users to make sure that process can even be
  launched. In a CI environment, that means our users are responsible for making
  sure the executable is installed. We can try to help them to make things easy
  to set up, but we'll have to pay for the effort as well as the brunt when
  things don't work as they're supposed to. And unfortunately, it appears the Go
  port will offer little relief there.

## Biome Type Architecture

> _"Okay, so you really decided to do it yourself. But how?"_

Not so fast there. Before I'm going to dive into our architecture, I'll attempt
to explain the constraints that led us to make the choices we did...

### Architecture Constraints

The main thing to understand about Biome is that we put our **User Experience**
front and center. Whether it's our
[Rule Pillars](https://biomejs.dev/linter/#rule-pillars), our Batteries-Included
approach, the
[`biome migrate`](https://biomejs.dev/guides/migrate-eslint-prettier/) command
for users coming from other tools, or our focus on IDE support, we know that
without users we are nowhere.

And it's precisely this last point, our IDE support, that's so important in this
story. IDE support was already an important consideration in our
[approach to multi-file support](https://github.com/biomejs/biome/discussions/4653),
and this seeps through into our type inference architecture.

For many tools, such as bundlers, it is sufficient to optimise the performance
for CLI usage. Development servers may have an interest in optimising hot-reload
performance as well, but they tend to do so by pushing responsibility to the
client instead of rebuilding their bundles faster.

For Biome, priorities are different: If a user changes file A, they want the
diagnostics for file B to update in their IDE regardless of whether it has
dependencies on file A or not. Updates need to happen near-instantaneously, and
the IDE is not a client we can offload responsibility to.

### Module Graph

Biome's _module graph_ is central to our multi-file support and is designed with
these considerations in mind. And our type architecture is built upon this
module graph. The module graph is effectively just a
[fancy hash map](https://github.com/ibraheemdev/papaya/) that contains entries
for every module (every JS/TS file in a repository), including metadata such as
which other modules that module depends on, which symbols it exports, and yes,
also which types it contains.

The key constraint the module graph operates under is this: No module may copy
or clone data from another module, not even if that data is behind an
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
The reason for this is simple: Because of our focus on IDE support, we maintain
the idea that any module in the module graph may be updated at any point in time
due to a user action. Whenever that happens, we shouldn't have trouble figuring
out which other modules need their data to be invalidated, which might happen if
modules were to copy each other's data.

Some other tools use complex systems to track dependencies between modules, both
explicit dependencies as well as implicit ones, so they can do very granular
cache invalidation. With Biome we're trying radical simplicity instead: Just
make sure we don't have such dependencies between entries in our module graph.
So far, that appears to be working well enough, but naturally it comes with its
own challenges.

### Type Data Structures

> _"You're finally getting to the interesting stuff, aren't you?"_

I guess that depends. If like me, you're a nerd for the technical nitty gritty,
yeah, we're getting there :)

In Biome, the most basic data structure for type information is a giant `enum`,
called `TypeData`:

```rs
enum TypeData {
    Unknown,
    Boolean,
    Number,
    String
    Null,
    Undefined,
    Function(Box<Function>),
    Object(Box<Object>),
    Class(Box<Class>),
    Reference(Box<TypeReference>),
    // many, many more...
}
```

This enum has many different variants in order to cover all the different kinds
of types that are supported by TypeScript. But a few are specifically
interesting to mention here:

* `TypeData::Unknown` is important because our implementation of type inference
  is only a partial implementation. Whenever something is not implemented, we
  default to `Unknown` to indicate that, well, the type is unknown. This is
  practically identical to the `unknown` keyword that exists in TypeScript, but
  we do have a separate `TypeData::UnknownKeyword` variant for that so that we
  can distinguish between situations where our inference falls short versus
  situations where we _can't_ infer because the user explicitly used `unknown`.
  They're semantically identical, so the difference is only for measuring the
  effectiveness of our inference.
* Complex types such as `TypeData::Function` and `TypeData::Object` carry extra
  information, such as definitions of function parameters and object properties.
  Because function parameters and object properties themselves also have a type,
  we can recognise that `TypeData` is potentially a circular data structure.
* Rather than allowing the data structure itself to become circular/recursive,
  we use `TypeReference` to refer to other types. And because we try to avoid
  duplicating types if we can, we have `TypeData::Reference` to indicate a type
  is nothing but a reference to another type.

### Why Use Type References?

> _"You mentioned types could become circular/recursive. But is that a problem?
Why can't you use something like `Arc` to reuse `TypeData` instances and let
them reference each other directly?"_

Glad you asked! Yes, we _could_ use `Arc` and let types reference each other
directly. But remember that module graph I mentioned? If a type from module A
were to reference a type from module B, and we'd store the type from module B
behind an `Arc`, then what would happen if module B were replaced in our module
graph?

The result would be that the module graph would have an updated version of
module B, but the types in module A would hang on to old versions of those
types, because the `Arc` would keep those old versions alive. Of course we could
try to mitigate that, but solutions tend to become either very complex or very
slow, and possibly both.

We wanted simplicity, so we opted to sidestep this problem using
`TypeReference`s instead.

But even though the constraints of our module graph were our primary reason for
choosing to use type references, they have other advantages too:

* By not putting the type data behind `Arc`s, we can store data for multiple
  types in a linear vector. This improves data locality, and with it,
  performance.
* Storing type data in a vector also makes it more convenient to see which types
  have been registered, which in turn helps with debugging and test snapshots.
* Not having to deal with recursive data structures made some of our algorithms
  easier to reason about as well. If we want to perform some action on every
  type, we just run it on the vector instead of needing to traverse a graph
  while tracking which parts of the graph have already been visited.

### Type Resolution Phases

> _"I get it, you like type references. But how do they work?"_

Which kind?

> _"Huh?"_

You see, type references come in multiple variants:

```rs
enum TypeReference {
    Qualifier(TypeReferenceQualifier),
    Resolved(ResolvedTypeId),
    Import(TypeImportQualifier),
    Unknown,
}
```

There isn't a singular answer to how type references work. The reason for this
is that _type resolution_, the process of resolving type references, works in
multiple phases. 

Biome recognises three levels of type inference, and has different resolution
phases to support those...

#### Local Inference

_Local inference_ is when we look at an expression and derive a type definition.
For example, consider this seemingly trivial example:

```js
a + b
```

It looks like this should be easy, but because local inference doesn't have any
context such as definitions from surrounding scopes, it will never be able to
understand what `a` or `b` refers to.

> _"That doesn't sound very useful. If you know neither `a` nor `b`, how can you
make anything more useful of this expression than `TypeData::Unknown`?"_

It's true that local inference cannot resolve this to a _concrete_ type. But
with the help of type references, we can rewrite the expression into something
useful:

```rs
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(TypeReferenceQualifier::from_path("a")),
    right: TypeReference::from(TypeReferenceQualifier::from_path("b"))
})
```

Local inference doesn't do any type resolution yet, it only creates type
references. So in most cases we won't know a concrete type yet, but it still
provides a useful starting point for later inference.

#### Module-Level ("Thin") Inference

_Module-level inference_, or as I sometimes like to call it: "thin inference",
allows us to put those types from the local inference phase into context. This
is where we look at a module as a whole, take its import and export definitions,
look at the scopes that are created, as well as the types derived using local
inference, and apply another round of inference to it.

Within the scope of a module, we do our first round of type resolution: We take
all the references of the variant `TypeReference::Qualifier` (the only ones
created thus far), and attempt to look them up in the relevant scopes. If a
local scope declaration is found, we consider the type _resolved_ and convert
the reference into a `TypeReference::Resolved` variant with an associated
`ResolvedTypeId` structure, which looks like this:

```rs
struct ResolvedTypeId(ResolverId, TypeId)
```

Both `ResolverId` and `TypeId` are a `u32` internally, so this is a really
compact representation for referencing another type, not bigger than a regular
64-bit pointer. The `TypeId` is a literal index into a vector where types are
stored, while the `ResolverId` is a slightly more complex identifier that allows
us to determine _which_ vector we need to look in, because every module will
have its own vector (and there are a few more places to look besides).

> _"But what if the type reference qualifier could not be found in one of the
scopes?"_

Good to see you're still paying attention!

Another possibility is that the qualifier references a binding from an
_import statement_, such as `import { a } from "./a.ts"`. In this case, we
cannot fully resolve the type yet, because thin inference cannot look beyond the
boundaries of its own module. But we can mark this case as an explicit import
reference. This is what the `TypeReference::Import` variant is for.

And if the qualifier exists neither as a local declaration, nor as an imported
binding, then we know it must come from the global scope, where we can find
predefined bindings such as `Array` and `Promise`, or the `window` object. If a
global reference is found, it also gets converted to a `TypeReference::Resolved`
variant, where the `ResolverId` can be used to indicate this type can be looked
up from a vector of predefined types.

But ultimately, if not even a global declaration was found, then we're at a loss
and fall back to `TypeReference::Unknown`.

### Full Inference

_Full inference_ is where we can tie all the loose ends together. It's where we
have the entire module graph at our disposal, so that whenever we run into an
unresolved `TypeReference::Import` variant, we can resolve it on the spot, at
which point it becomes a `TypeReference::Resolved` variant again.

Today, results from our full inference cannot be cached for the same reason
we've seen before: Such a cache would get stale the moment a module is replaced,
and we don't wan't to have complex cache invalidation schemes. But we may have
one more trick up our sleeve...

> _"Hey, do tell! What's the trick?"_

One moment, dear reader, I will get to that, but first I have to explain
about...

### Type Resolvers

The thing about having all these type references all over the place is that you
need to perform explicit type resolution to follow these references. That's why
we have _type resolvers_. I use plural, because we have a whole bunch of them.
There's a `TypeResolver` trait, and at this point we already have 6
implementations of it.

That may sound worse than it is though. As of today, the implementations are:

* **`HardcodedSymbolResolver`**. This one is purely for test purposes.
* **`GlobalsResolver`**. This is the one that is responsible for resolving
  globals such as `Promise` and `Array`. The way we do this is still rather
  primitive with hardcoded, predefined symbols. At some point we probably should
  be able to use TypeScript's own global `.d.ts` files, such as
  [es2023.array.d.ts](https://github.com/microsoft/TypeScript/blob/main/src/lib/es2023.array.d.ts),
  directly.
* **`JsModuleInfoCollector`**. This one is responsible for collecting
  information about a module, and for performing our module-level inference.
* **`JsModuleInfo`**. Once the `JsModuleInfoCollector` has done its job, a
  `JsModuleInfo` instance is created, which is stored as an entry in our module
  graph. But this data structure also implements `TypeResolver` so that our full
  inference can access the module's types too.
* **`ScopedResolver`**. This is the one that is responsible for our actual full
  inference. Its named as it is because it is the only resolver that can really
  resolve things in any arbitrary scope. Compare this to the
  `JsModuleInfoCollector` which only cares about the global scope of a module,
  because at least so far that's all we need to determine types of exports
  (we don't determine the return type of functions without annotations yet, and
  it's not yet decided when or if we'll do this).
* **`ScopeRestrictedRegistrationResolver`** may sound impressive, but is but a
  helper for `ScopedResolver` to conveniently set the correct scope ID on
  certain references, so that when the time comes for the `ScopedResolver` to
  resolve it, it will still know which scope should be used for resolving it.

I've mentioned before that types are stored in vectors. Those type vectors are
stored inside the structures that implement `TypeResolver`, and with the
exception of `ScopeRestrictedRegistrationResolver`, they all have their own
internal storage for types.

> _"Super interesting! But I'm still waiting for that trick you were teasing."_

Yeah, well. By now you can hopefully see that while type resolvers are somewhat
of a necessity because of our choice to use type references pervasively, they
also offer us quite a bit of flexibility.

One potential area of flexibility -- as of yet unexplored -- is that it is
entirely feasible to imagine a type resolver that _does_ cache the results of
our full inference. Such a resolver would be very hard to get right for our
language server, where modules may be updated at any time, but it might work
very well for our CLI where we can assume this doesn't happen.

### Flattening

Apart from type resolution, there's one other, last important piece to type\
inference: _type flattening_.

Let's look at the `a + b` expression again. After local inference, it was
interpreted as this:

```rs
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(TypeReferenceQualifier::from_path("a")),
    right: TypeReference::from(TypeReferenceQualifier::from_path("b"))
})
```

But at some point, supposedly one of the resolvers is going to be able to
resolve `a` and `b`, and the expression becomes something such as:

```rs
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(ResolvedTypeId(/* resolver ID and type ID */)),
    right: TypeReference::from(ResolvedTypeId(/* resolver ID and type ID */))
})
```

At this point we know the actual types we are dealing with. If the types for
both `left` and `right` resolve to `TypeData::Number`, the entire expression can
be _flattened_ to `TypeData::Number`, because that's the result of adding two
numbers. And in most other cases it will become `TypeData::String` instead.

## What's Next

Biome type architecture is still _very_ young. I've been working on this for
about 6 weeks since the first code was written. The module graph was already
there before that, but not a single type-related structure was present in
Biome's codebase.

Even so, preliminary results on one of Vercel's codebases shows that our
inference is able to detect about 40% of the cases that should be flagged by our
`noFloatingPromises` rule. No false positives are being reported as of yet.

While a 40% success rate doesn't sound particularly impressive, we have a clear
indication where we need to focus next: Currently our path resolution only works
for _relative_ paths. So `import ... from "./foo.ts";` works, but
`import ... from "#/foo"` with a path alias won't. Nor does
`import ... from "foo"` work with external libraries. With such major
limitations, it's no surprise that we have a limited success rate.

But looking at the bright side, we're only 6 weeks in, and we clearly know where
to go next: At this moment the limiting factor isn't even our _type_ resolution,
it's our _import_ resolution.

And thankfully, that's not the only thing we're doing either. Our core
contributor [Siketyan](https://github.com/siketyan) has already implemented a
new
[`useExhaustiveSwitchCases`](https://next.biomejs.dev/linter/rules/use-exhaustive-switch-cases/)
rule too, improving some of our inference in the process. Such efforts reinforce
one another, and as we attract more people working on these type-informed rules,
our inference only stands to become better and better.

## Wrapping Up

> _"Thanks. I thought you were never getting there."_

Thank you for reading all this way. Hopefully this has provided an informative
look behind the scenes on Biome's type inference. I'll get back to hacking on
our type inference and our import resolution :)

> _"Before you go, I have one question: Do you think Biome will ever implement
type_ checking _too?"_

Heh, that's a great question! To be honest, it's hard to give a definitive
answer, because I can't look into the future. What I can say is that it is
certainly outside the scope of my current contract.

That said, there is a rather natural path towards implementing type checking
too. TypeScript has _conditional types_, which look something like
`A extends B ? C : D` and which can be read as: If type A can be assigned to
type B, use type C, otherwise use D. And that condition is awefully close to
type _validation_. And if we have type validation, we're awefully close to
becoming a type _checker_ as well.

So, who knows, we may venture into type checking territory some day. But
remember that full compatibility with `tsc` is a rather elusive goal for any
type checker, so please temper your expectations and keep in mind you will
definitely still be using `tsc` for the foreseeable future.

Thanks again for reading! And if you have any other questions, feel free to
leave a comment here. Or reach out to us on
[our Discord](https://biomejs.dev/chat) in the `#type-inference` channel.
