# Quick informations

NeoNex is an incredibly powerful and reusable piece of software, designed to be extended without changing its library source code.
That's being said, you can litterally implement a new platform for NeoNex, without changing its source code, but implementing
this support in your own workspace.

This flexibility is completely zero runtime overhead. In fact, `neonex_ecs` is a standalone crate, which offer a completely static, type driven plugin system, which is flexible enough to assemble plugins from external crates into one big ECS app. This ECS may appear limited at first, but as soon as you start using it, you quickly understand that you don't need to have more to create any piece of software. The ECS is alloc-free and no_std, expanding its platform compatibility. It doesn't make assumptions about hardware, but only about type systems, that gets optimized away into a performant loop.

`neonex_core` contains NeoNexInstance, Config and Platform: 3 ZSTs which statically contain information to change the behavior of NeoNex, switch its backends, run its loop, and even more, while having no runtime overhead due to configuring or allocation (everything is static and known by the compiler to make aggressive optimizations). This crate is standalone and can be used to create criss-platform and performant terminal apps, leveraging ratatui.

`neonex_logic` contains the platform-agnostic logic of NeoNex. It can't be used as a standalone crate, because it relies on `neonex_core` for logic compile-time configuration, platform agnosticism (so use of Platform items, defined when initializing NeoNexInstance). Note that `neonex_logic` is dependent of `neonex_core`, but `neonex_core` is not a dependency of `neonex_logic`. The crate exposes the NeoNexLogic plugin, which, once imported in NeoNex ECS, requires `neonex_core` plugins to work correctly (ensured at compile-time).

`neonex_ecs` is a completely compile-time ECS, which main goal is to implement a compile-time plugin system, so that plugins can require each other and can access each other data.

## Roadmap upon the MVP stage

- [x] trait-based customizations (NeoNexInstance, NeoNexConfig)
- [x] platform-specific configuration (NeoNexPlatform, DesktopPlatform, MobilePlatform, WebPlatform)
- [ ] CCBS -> Connected to SCS save/retrieve/update mechanisms
- [ ] SSM Full implementation
- [ ] Minimal dynamic terminal support (fork/contribute to [bevy_ratatui](https://github.com/cxreiff/bevy_ratatui))
- [ ] pluggable plugins (no priority, but trivial)
- [x] Finish StartupConfigSet (SCS) implementations
- [ ] Implement logic for `neonex-teminal`: ASCII art, animations, custom GUI, ...
- [ ] Document everything in-code
- [ ] Publish drafts, design documents, graphs
- [ ] Implement Github CI and automate information sources (e.g. dependency graphs, with detailed feature-gates and targets)
- [ ] Publish neonex lib with all of its descendents (e.g. neonex-core)
- [ ] Web page about NeoNex