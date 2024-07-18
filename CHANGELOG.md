# Next

- **[Fix]** Fix spelling when matching `GNOME-Classic`. ([@bash](https://github.com/bash), [#11](https://github.com/demurgos/detect-desktop-environment/pull/11))

# 1.1.0 (2024-04-16)

- **[Feature]** Detect more environments: Deepin DE, EDE, Endless OS, Hyprland, Old (legacy menus), Pantheon (Elementary OS), Razor, ROX, Sway, Trinity DE ([@nagua](https://github.com/nagua), [#6](https://github.com/demurgos/detect-desktop-environment/pull/6))
- **[Fix]** Fix parsing of `XDG_CURERNT_DESKTOP` ([@nagua](https://github.com/nagua), [#6](https://github.com/demurgos/detect-desktop-environment/pull/6))

# 1.0.0 (2023-03-18)

- **[Breaking change]** Mark the enum `DesktopEnvironment` as non-exhaustive. This allows adding new environments in
  the future without introducing breaking changes ([@notgull](https://github.com/notgull), [#2](https://github.com/demurgos/detect-desktop-environment/pull/2))
- **[Breaking change]** Remove `DesktopEnvironment::Unknown`, replaced by `detect` returning an `Option`.
- **[Breaking change]** Require Rust 1.56.
- **[Feature]** Detect the `COSMIC` DE from Pop!_OS ([@Pixelstormer](https://github.com/Pixelstormer), [#1](https://github.com/demurgos/detect-desktop-environment/pull/1))
- **[Feature]** Implement `Clone`, `Copy`, `PartialOrd`, `Ord`, `Hash`.
- **[Feature]** Add the methods `gtk` and `qt` to test the GUI framework used by the desktop framework.
- **[Internal]** Rename the main branch to `main`.
- **[Internal]** Configure Continuous Integration through GitHub CI.
- **[Internal]** Update README: MSRV policy, badges, documentation link

# 0.2.0 (2017-12-02)

- **[Breaking change]** Move the `detect` function to the `DesktopEnvironment` namespace
- **[Fix]** Detect `Cinammon`

# 0.1.0 (2017-12-02)

- **[Feature]** First release
