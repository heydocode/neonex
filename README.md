# Quick informations

Now, a work-in-progress is made to accomplish the designed architecture in-code: pluggable [bevy](https://bevy.org/) plugins, Custom Command Behavior Set (CCBS), trait-based powerful but zero runtime overhead customization system, platform-specific configurations and preferences, Startup Configs, Startup Status Messages (SSM), ......... the list is long!

The purpose of the project is to provide a futurist stylized virtual assistant, located in a terminal (native or virtual, to support mobile and web too). Even if the assistant aims to be AI-free and be especially useful for quick but powerful and performant automatizations, some plans in the future tend to include AIs in this project, so that they can create CCBS according to your wishes and build runtime (and so, unhappily less performant) automatizations, built in [Rhai](https://rhai.rs/), for all of its features, that would be useful for preemtible automatizations that would in theory be added with AIs.

If you're curious about the project, you can DM me in discord (@heydo_code)

If you want to contribute, that's not (yet) the right time: I have to build the basis of NeoNex before accepting any contributions: all of the systems mentioned in the first paragraph. It'll be short, and I'll surely do a showcase post in the [bevy's discord](https://discord.com/invite/bevy), as NeoNex is dependent on bevy and as NeoNex leverages new ideas into bevy, what could be interesting for other bevy users.

## Roadmap before the MVP stage

- [x] trait-based customizations (NeoNexInstance, NeoNexConfig)
- [x] platform-specific configuration (NeoNexPlatform, DesktopPlatform, MobilePlatform, WebPlatform)
- [ ] CCBS -> Connected to SCS save/retrieve/update mechanisms
- [ ] SSM Full implementation
- [ ] Minimal dynamic terminal support (fork/contribute to [bevy_ratatui](https://github.com/cxreiff/bevy_ratatui))
- [ ] pluggable plugins (no priority, but trivial)
- [ ] Finish StartupConfigSet (SCS) implementations
- [ ] Implement logic for `neonex-teminal`: ASCII art, animations, custom GUI, ...
- [ ] Document everything in-code
- [ ] Publish drafts, design documents, graphs
- [ ] Implement Github CI and automate information sources (e.g. dependency graphs, with detailed feature-gates and targets)
- [ ] Publish neonex lib with all of its descendents (e.g. neonex-core)
- [ ] Web page about NeoNex
