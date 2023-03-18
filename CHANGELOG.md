# Next

- **[Breaking change]**: Mark the enum `DesktopEnvironment` as non-exhaustive. This allows adding new environments in
  the future without introducing breaking changes (@notgull, [#2](https://github.com/demurgos/detect-desktop-environment/pull/2))
- **[Breaking change]**: Remove `DesktopEnvironment::Unknown`, replaced by `detect` returning an `Option`.
- **[Breaking change]** Require Rust 1.56.0.
- **[Feature]**: Detect the `COSMIC` DE from Pop!_OS (@Pixelstormer, [#1](https://github.com/demurgos/detect-desktop-environment/pull/1))

# 0.2.0 (2017-12-02)

- **[Breaking change]** Move the `detect` function to the `DesktopEnvironment` namespace
- **[Fix]** Detect `Cinammon`

# 0.1.0 (2017-12-02)

- **[Feature]** First release
