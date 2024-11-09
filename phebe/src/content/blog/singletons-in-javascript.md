---
title: "Singletons in JavaScript"
description: "Various ways of using singletons in JavaScript, with their pros and cons"
pubDate: "Nov 9 2024"
mastodon:
  toot: "113453406338204758"
---

## Introduction

The [singleton pattern](https://en.wikipedia.org/wiki/Singleton_pattern) is a bit of a controversial
pattern, sometimes even considered an anti-pattern.

There are good uses for it (mostly convenience), but before using them you should consider these
downsides:

* Singletons can make it harder to write unit tests for code that uses them.
* If you have multiple singletons that call into one another, you can easily find yourself
  entangling their concerns. If you find it becomes hard to track what happens where, you should
  probably reconsider your approach.

So be mindful when using them, but as long as you avoid the pitfalls, a singleton can sometimes be
the simplest tool for the job. In this post, I'll present four different ways we can use one in
JavaScript.

## Using module state

```js
let privateState;

export function publicFunction() {
    // Do stuff with `privateState`.
}
```

Wait, what's that? That doesn't look like a singleton at all! Indeed there are no classes, so there
also isn't a singleton instance of one. This example simply relies on the realization that
JavaScript has proper modules, and we can use module state to the same effect as state inside a
singleton instance.

This super simple example has all the same benefits as a singleton: Namely, convenient access to
functions that can modify private state. And naturally, it shares the same downsides as well.

## Using a class and an instance function

```js
let instance;

export function getMyInstance() {
    if (!instance) {
        instance = new MyClass();
    }
    return instance;
}

class MyClass {
    #privateState;

    publicMethod() {
        // Do stuff with `#privateState`.
    }
}
```

Okay, now we have something that looks more like a traditional singleton. It adds a bunch of
ceremony compared to the previous example, and you should carefully consider whether that ceremony
is worth it: You now have a class instance, but what's the purpose of that instance when the idea
behind a singleton is that there can only be a single instance, which you _shouldn't_ pass around?

The main benefit of this approach is that it may make lazy initialization a little easier.

One thing to note: Because we don't export `MyClass`, we prevent consumers of the singleton from
creating their own instances, which is exactly what we want.

## Using a class with a static instance method

The previous example could also be rewritten using a static instance method:

```js
export class MyClass {
    static #instance;

    #privateState;

    static getInstance() {
        if (!MyClass.#instance) {
            MyClass.#instance = new MyClass();
        }
        return MyClass.#instance;
    }

    publicMethod() {
        // Do stuff with `#privateState`.
    }
}
```

Personally I find `static` to be mostly a redundant concept in JavaScript, since as the previous
example shows, we already have module-level variables that can do the same thing. But I understand
some people like them for familiarity reasons, or because they feel it connects the variable to
the class more.

Unfortunately, because the class is now exported, code outside the module can now use
`new MyClass()` to create instances, which is what we don't want.

If you use TypeScript, that disadvantage is easily remedied by adding a `private constructor() {}`,
but if you use plain JS you can't. At the same time, if you use plain JS, you don't have static
typing either, so I guess it's par for the course.

## Using a class with a static instance method and a guarded constructor

But if you really insist on using plain JavaScript and also on using static methods _and_ guarding
the constructor, you can do so like this:

```js
export class MyClass {
    static #instance;
    static #instantiating = false;

    #privateState;

    static getInstance() {
        if (!MyClass.#instance) {
            MyClass.#instantiating = true;
            MyClass.#instance = new MyClass();
            MyClass.#instantiating = false;
        }
        return MyClass.#instance;
    }

    constructor() {
        if (!MyClass.#instantiating) {
            throw new Error("Use MyClass.getInstance() instead");
        }
    }

    publicMethod() {
        // Do stuff with `#privateState`.
    }
}
```

## Recommendations

As we can see, there's plenty of flavors to choose from.

My personal recommendation would be to stick with the first or second example, depending on your
needs. If you use TypeScript, you may opt to use the third, but it's mostly a matter of taste.

As I explained in my post on
[Post-Architecture](/blog/2024/07/post-architecture-premature-abstraction-is-the-root-of-all-evil/),
I have a strong preference towards simplicity. So if you feel tempted by number 4, I would ask you
to reconsider and look at number 2 again :)

One other thing I would explicitly recommend **against** doing is this: Some people add an instance
check inside the constructor itself, and return the existing instance from there if it exists. This
changes the semantics of the constructor, since `new MyClass()` would suddenly no longer return a
_new_ instance. That's misleading to your caller.

To protect yourself against bad practices like that, I recommend using Biome's
[`noConstructorReturn` rule](https://biomejs.dev/linter/rules/no-constructor-return/).
