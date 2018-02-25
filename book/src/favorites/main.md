# my favorites
On this talk I will present my favorites features in the language.
Buld system/style

## Rust build system

So arguably build system is not really a feautre of the language, but I would feel bad If I did not mention Rust' build system.

So I will just briefly mention the features that come to mind.
* Can directly link with shared libraries - and operate on objects - without intermediate layer.
* Compiles into machine code.
* Uses LLVM as a backend.
* Build system - cargo - supports dependencies.
* Supports compilation configuration - something similar to #IFDEF.
* Allows for noncolliding inclusion of multiple version of the same libraries within one target binary. - For example you could have boost 1.55 and boost 1.56 compiled into exe and you'd know that all is fine.
* Allows to specify dependencies via: [crates.io ( something like nexus), path, github ( branches, commits.., forks ), ..]
* Has supports for workspaces - meaning you can split one project into subprojects.