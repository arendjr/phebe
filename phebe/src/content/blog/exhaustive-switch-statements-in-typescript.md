---
title: "Writing exhaustive switch statements in TypeScript"
description: "A little trick that can help make switch statements more type-safe"
pubDate: "Mar 15 2024"
mastodon:
  toot: "112099141927078946"
---

As long as [pattern matching](https://github.com/tc39/proposal-pattern-matching) is not yet part of TypeScript, developers sometimes need to get creative to avoid a common pitfall in the language: how do you make sure a switch statement has covered all the variants in a union type?

Today, I’d like to share my favorite approach.

Let’s look at the following code example:

```ts
type WrittenNumber = "one" | "two" | "three";

function printAsDigit(number: WrittenNumber) {
    let digit;
    switch (number) {
        case "one":
            digit = 1;
            break;
        case "two":
            digit = 2;
            break;
        case "three":
            digit = 3;
            break;
    }

    console.log(`The number is ${digit}.`);
}
```

Looks innocent enough, right? But what if we added the variant "four" to `WrittenNumber`? Ideally, we would like it if TypeScript could catch it when we forget to update the `switch` statement. Can we make it so?

If you’re using `typescript-eslint`, there’s a rule that gets you covered: https://typescript-eslint.io/rules/switch-exhaustiveness-check/
But if you’re not using ESLint, are you out of luck?

Not necessarily! With a simple refactor we can make the code look a bit cleaner *and* use TypeScript itself to our advantage. Look at this version:

```ts
type WrittenNumber = "one" | "two" | "three";

function printAsDigit(number: WrittenNumber) {
    const digit = numberToDigit(number);

    console.log(`The number is ${digit}.`);
}

function numberToDigit(number: WrittenNumber): number {
    switch (number) {
        case "one":
            return 1;
        case "two":
            return 2;
        case "three":
            return 3;
    }
}
```

Not only have we saved ourselves from the ugly `break` statements, and replaced a `let` variable with `const` (generally, if you can do with fewer mutable variables, that’s preferred!), we have also increased our type safety. The trick is that `numberToDigit()` is explicitly annotated as returning a `number`. Because of this, if the `switch` statement missed a variant, TypeScript will warn us!

Only caveat: If one of the cases in a switch statement has a legit reason for producing `undefined`, the type safety benefit will be lost.

But even then I really like the added elegance of putting switch statements in their own functions. Maybe I could make a [Biome](https://biomejs.dev) rule to enforce this as a pattern? 🤔

Let me know your thoughts on [Mastodon](https://mstdn.social/@arendjr/112099141927078946)!
