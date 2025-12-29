# NeoNex

> [!NOTE]
> This project is under active development and not ready for production use.

## Quick information

`NeoNex` is a modular and cross-platform terminal interface built on top of Bevy ECS and underlying ratatui Backend to draw on screen.

Currently supported backends are:
- [`Softatui`](https://github.com/gold-silver-copper/soft_ratatui) (All platforms supported by bevy)
- [`Crossterm`](https://lib.rs/crates/ratatui-crossterm) (Desktop)
- [`tui-uefi`](https://github.com/reubeno/tui-uefi) (UEFI - Unstable/WIP)

NeoNex is designed as a configurable, cross-platform terminal-based launcher and configuration frontend that runs before an application’s main runtime. 

Depending on the target platform, you can make it invoke:
- Another Bevy App, within the same `main` function (stable and reliable, though no self-update).
- Executables, and also update them as needed (stable on Windows and Linux, but not really on MacOS due to code signing and notarization).
- Wasm modules (stable)
- Dynamically linked applications (desktop-only, unstable and unreliable)

In all the cases, the launcher can pass arbitrary data to the invoked application through platform-appropriate mechanisms.

## Extensibility

You can literally implement a `NeoNexConfig` for such an unusual device, which a GameBoy Advance is, **entirely in userspace code, without editing the crate's source code**.

*Note: supporting embedded platforms will only be possible once the project updates to stable ratatui v0.30.0, but the project's structure allows such deep customization in the userspace code.*

## Roadmap upon the MVP stage

- [x] trait-based customizations (NeoNexInstance, NeoNexConfig)
- [x] platform-specific configuration (NeoNexPlatform, DesktopPlatform, MobilePlatform, WebPlatform)
- [x] platform-appropriate communication mechanisms - cross-platform tmp config file. Not available on UEFI.
- ~~[ ] Minimal dynamic terminal support (fork/contribute to [bevy_ratatui](https://github.com/cxreiff/bevy_ratatui))~~
- ~~[ ] pluggable plugins (no priority, but trivial)~~
- [x] Finish StartupConfigSet (SCS) implementations
- ~~[ ] Implement logic for `neonex-terminal`: ASCII art, animations, custom GUI, ...~~
- [ ] Document everything in-code
- [ ] Reorganize `NeoNex` so that it creates separate crates only for platform-specific logic
- [ ] Publish drafts, design documents, graphs
- [ ] Implement Github CI and automate information sources (e.g. dependency graphs, with detailed feature-gates and targets)
- ~~[ ] Publish neonex lib with all of its descendents (e.g. neonex-core)~~
- [ ] Add XR support with three modes: floating terminal window in VR, fullscreen terminal, and floating terminal window in AR.
- [ ] Web page about NeoNex
