---
title: "Biome's Approach to Multi-File Analysis"
description: "Explaining My Vision for Biome Multi-File Analysis"
pubDate: "Nov 28 2024"
github:
  url: "https://github.com/biomejs/biome/discussions/4653"
---

Currently, Biome's linter analyzes files one by one. This is fast and easy to reason about; it works well. But it also comes with some significant limitations: Any information that we require for analysis that cannot be found within the same file is simply not available. Sure, we can carve out some exceptions like parsing `package.json` before we start the analysis, but this approach doesn't scale well with our long-desired [Monorepo support](https://github.com/biomejs/biome/issues/2228).

It also doesn't scale to other use cases we would like to implement:
* Resolving import statements
* Extracting type information
* GraphQL schema validation
* Project-level dead code analysis
* ... and probably more

So we want true [Multi-file support](https://github.com/biomejs/biome/issues/3307), and I've also been experimenting lately to see what it would take to make this possible. Since then, I've developed a bit more of a vision on how I think we should tackle this. That doesn't mean I have all the answers available -- there will certainly be challenges still -- but I think I see a feasible path towards implementing this.

In fact, I think there are generally **two** paths towards implementing this. Both these paths have their pros and cons, so in order to understand the tradeoffs and the reasons _why_ I'm proposing the path that I'm proposing, I will first explain the path I'm _not_ proposing.

## The Road Not Taken

Multi-file analysis is certainly a difficult problem. The main reason for this is that it requires _holistic_ analysis of a project or even repository with multiple projects. Loading an entire repository into memory is expensive, so if you need to do that, you typically only want to do it once. So you want to keep it in memory, which implies caching. And if the user changes a file, we want to update that cache in a clever way. Great, we've now arrived at [one of the hardest problems in computer science](https://surfingcomplexity.blog/2022/11/25/cache-invalidation-really-is-one-of-the-hardest-things-in-computer-science/).

Of course, Biome is not unique in needing this kind of cache. It's pretty much the same problem any language server for an IDE is faced with. Being written in Rust, we may turn to the crate ecosystem and see if there is any prior art that we can make use of. And good news, there is! There's almost an abundance even, but I think it's fair to say that many in the community currently consider [Salsa](https://github.com/salsa-rs/salsa) to be among the state of the art for this kind of technology. And it surely does look very appealing, even if it's still considered experimental. I'm not going to go into its architecture in-depth here, but suffice to say I would be excited to have a chance to work with it.

So why am I not proposing we adopt Salsa for Biome? Two simple reasons:
* Biome already has a pretty well-established architecture, and this would need significant overhaul if we decided to adopt Salsa
* We don't have the manpower to commit to an overhaul which I would expect a full-time engineer to take several months at the very least

So that puts us in a tough position, unless we can come up with a simpler alternative.

## Biome's Constraints

So we are constrained in manpower, but still want to implement a solution that gives us multi-file analysis. Thankfully, it is well-established that knowing one's constraints is a fertile breeding ground for [creativity and innovation](https://hbr.org/2021/04/innovation-starts-with-defining-the-right-constraints).

What are our constraints?

* It must be simple, so it's not too much work to get there from our current architecture. And simplicity has another key advantage, which is that it lowers the barrier to entry for new contributors.
* It needs to scale with Biome's use cases mentioned earlier. But notably, we don't need to build a general purpose solution.
* We cannot break any of Biome's clients, including the CLI, our LSP and the online playground. Although it's acceptable if the playground doesn't offer all the features.
* It must be fast because it's what our users expect from us. That said, it doesn't need to be _the fastest_ -- functionality beats performance.

Can we come up with a solution that satisfies these constraints? I believe we can.

## The Road to Multi-File Support

### Services

Biome's architecture already accounts for the notion of services. One example is the `SemanticModel`, which abstracts over a parsed syntax tree and tracks variable assignments and usages, so we can reason about them semantically. Another example is the `PackageJson` that we inject so lint rules can detect imports for dependencies that haven't been declared, for instance.

It's relatively easy to imagine the introduction of new services such as a `DependencyGraph`, which can be used to track resolved paths of import statements. Or a `ProjectLayout`, which doesn't just contain a single `package.json`, but a layout of all projects and their manifests within the repository, so as to facilitate monorepo support.

The challenge is not really in adding these services, since we have the facilities for that. It's in how to populate the data for these services -- and in how to keep it up to date.

### Multi-Phase Analysis

One thing we can't seem to avoid is the requirement for multi-phase analysis. In short, this means having one phase for extracting the information that goes into the services, and another for the actual analysis that can make use of that information.

On a per-file basis these phases may be far removed from one another. After all, before the analysis phase on a file can be performed, we need to have completed the extraction phase on all the files that this file depends on. And possibly including the files _those_ files depend on, and so on.

### Dependency Graph

I already mentioned Biome will probably have a `DependencyGraph` service. This service can track which imports resolve to which files. This information is vital for lint rules that may want to warn about trying to import non-existing files, or detecting import cycles, and so on.

In theory, a dependency graph can also be used to determine from which files we need to have completed the extraction phase before we can start the analysis phase on a given file. But this is where I suggest we greatly deviate from a Salsa-like approach: We are not going to use our dependency graph for this at all.

Instead, Biome will simply traverse _all_ files relevant to a repository as part of the extraction phase. And when everything is done, it will run the analysis phase on _all_ files we are interested in. This greatly simplifies the amount of bookkeeping we need to do and means our approach to caching can be much simpler than it needs to be otherwise.

### File System Traversal

Currently, Biome makes clients such as the CLI or LSP responsible for file system traversal. This worked well with single-file analysis, since it meant that every file could be unloaded from the Biome daemon once it was processed. This also obsoleted the need for complex caching.

But this approach is not going to work well with multi-phase analysis, since we will need to traverse files twice: once for the extraction phase and once for the analysis phase. Submitting each file to the daemon and unloading them, then resubmitting and unloading would be a waste of performance (not to mention it would complicate our own implementation). Rather, we should move the process as a whole into the daemon process. This is probably where most of the complexity will be for us to implement multi-file analysis, regardless of approach taken.

### File Watching

Since we are going to persist service data within the daemon, the daemon also needs to become responsible for file watching. Thankfully, there's a solid [Rust crate](https://crates.io/crates/notify) to help us with the implementation.

### Workspace Caching

The `Workspace` is Biome's interface to the services implemented by our daemon and the data it stores internally. The `WorkspaceServer` is our implementation of this interface that resides within the daemon. If you look inside this implementation, you can see there is cached storage for open files and their syntax trees, as well as other project-level information. This is the place where we will be storing more and more data that we need for the new services to introduce.

But the implementation of the workspace server is not easy to work with. Currently, you [can see](https://github.com/biomejs/biome/blob/main/crates/biome_service/src/workspace/server.rs#L44) there are a bunch of `RwLock` and `DashMap` wrappers there, which are needed to synchronize access to this data from multiple threads that operate within the daemon. Unfortunately, it wouldn't be the first time we ran into deadlock issues here. And with the introduction of even more data structures, which we also need to update as a result of file watches, I fear it would become more and more difficult for us to manage this state effectively and error-free.

This is why I'm [preparing to simplify](https://github.com/biomejs/biome/pull/4624) this implementation with the introduction of optimistic parsing and [`papaya`](https://github.com/ibraheemdev/papaya), a lock-free concurrent hash map. If we can simplify the data structures, and we have less need to worry about deadlocks, I hope we can create a simple and manageable solution even in the absence of a detailed query framework.

### Performance Implications

There are lots of performance implications to this proposal. In general I think it is fair to say that Biome *will* become slower, simply because it will have more work to do. Especially when we get to the point where we implement our own type inference, which we still hope to do at some point in the future, there will be a significant impact. That said, there is also reason for optimism.

The proposal to simply traverse *all* files before even starting the analysis phase naturally means there is a bottleneck. But in a CI environment, where you typically scan your entire repository, you would always traverse all files anyway. If you use an IDE, the traversal would happen at startup, while analysis after that remains fast. And if you use the CLI, only your first invocation would pay the price, while future invocations can use the cache. And if you use an IDE _and_ the CLI together, they can even reuse their cache. And the idea of moving the file system traversal into the workspace server is even expected to improve performance for that aspect.

Also, we should consider that both analysis phases are very much parallelizable. And thanks to the absence of fine-grained dependency graphs and the use of lock-free data structures, threads are going to spend *very* little time waiting for one another.

Maybe Biome will not have the fastest multi-file analysis possible. But at least I believe we can create a competitive offering despite our constraints.

## Help Wanted

Biome is an open-source project and is open to contributions. If you think you can help out with any of the things discussed here, feel free to [reach out](https://biomejs.dev/chat)! We welcome your involvement. If you feel you do not have the time or skill to help us with technical implementation, please consider [sponsoring us](https://github.com/biomejs/biome/#funding).
