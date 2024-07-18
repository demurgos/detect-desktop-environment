#![deny(missing_docs)]
//! Desktop environment detection
//!
//! This crate implements automatic detection for the current desktop environment.
//!
//! See [`DesktopEnvironment`] for supported desktop environments.
//!
//! The environment can be detected using [`DesktopEnvironment::detect`]:
//!
//! ```rust
//! use detect_desktop_environment::DesktopEnvironment;
//!
//! match DesktopEnvironment::detect() {
//!   Some(de) => println!("detected desktop environment: {de:?}"),
//!   None => println!("failed to detect desktop environment"),
//! }
//! ```

/// Desktop environments supported by `detect-desktop-environment`.
// If adding new environment, please keep them sorted alphabetically and use `PascalCase`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DesktopEnvironment {
  /// Cinnamon, the default desktop environment for Linux Mint.
  ///
  /// - <https://en.wikipedia.org/wiki/Cinnamon_(desktop_environment)>
  Cinnamon,
  /// COSMIC, the default desktop environment for Linux Pop!_OS.
  ///
  /// Note: This corresponds to the classic COSMIC based on GNOME, not the Rust
  /// [COSMIC-epoch](https://github.com/pop-os/cosmic-epoch). Please send a PR if you can
  /// test how to detect cosmic-epoch.
  ///
  /// - <https://github.com/pop-os/cosmic>
  Cosmic,
  /// Deepin desktop environment
  ///
  /// - <https://www.deepin.org/index/en>
  Dde,
  /// EDE Desktop
  ///
  /// - <https://edeproject.org/>
  Ede,
  /// Endless OS desktop
  ///
  /// - <https://www.endlessos.org/os>
  Endless,
  /// Enlightenment desktop environment.
  ///
  /// - <https://en.wikipedia.org/wiki/Enlightenment_(software)>
  Enlightenment,
  /// Gnome, the default environment for many major Linux distributions.
  ///
  /// - <https://en.wikipedia.org/wiki/GNOME>
  Gnome,
  /// Hyprland tiling window manager
  ///
  /// - <https://hyprland.org/>
  Hyprland,
  /// KDE Plasma, the Kool Desktop Environment.
  ///
  /// - <https://kde.org/plasma-desktop/>
  Kde,
  /// LXDE
  ///
  /// - <https://www.lxde.org/>
  Lxde,
  /// LXQt
  ///
  /// - <https://lxqt-project.org/>
  Lxqt,
  /// MacOs, the environment for Apple's OS
  MacOs,
  /// MATE
  ///
  /// - <https://mate-desktop.org/>
  Mate,
  /// Legacy menu systems
  ///
  /// Listed in [Freedesktop Desktop Environments](https://specifications.freedesktop.org/menu-spec/latest/apb.html).
  // Please send a PR if you have more details or better ideas about how to handle this value.
  Old,
  /// Elementary OS Desktop Environment
  ///
  /// - <https://elementary.io/>
  Pantheon,
  /// Razor-qt
  ///
  /// Discontinued Desktop Environment, this is an ancestor of LXQt.
  ///
  ///
  /// - <https://github.com/Razor-qt/razor-qt>
  Razor,
  /// ROX Desktop
  ///
  /// - <https://rox.sourceforge.net/desktop/>
  Rox,
  /// Sway tiling window manager
  ///
  /// - <https://swaywm.org/>
  Sway,
  /// TrinityDesktopEnvironment
  ///
  /// - <https://www.trinitydesktop.org/>
  Tde,
  /// Unity, the legacy desktop environment for Ubuntu
  ///
  /// - <https://en.wikipedia.org/wiki/Unity_%28user_interface%29>
  Unity,
  /// Windows, the environments for Microsoft's OS
  Windows,
  /// Xfce
  ///
  /// - <https://xfce.org/>
  Xfce,
}

impl DesktopEnvironment {
  /// Detect the current desktop environment
  ///
  /// If the current desktop environment can't be detected, `None` is returned.
  pub fn detect() -> Option<Self> {
    Self::detect_impl()
  }

  /// Test if the desktop environment is based on the GTK framework
  ///
  /// See <https://en.wikipedia.org/wiki/Category:Desktop_environments_based_on_GTK>
  ///
  /// ```
  /// use detect_desktop_environment::DesktopEnvironment;
  ///
  /// // All matching desktop environments:
  /// assert!(DesktopEnvironment::Cinnamon.gtk());
  /// assert!(DesktopEnvironment::Cosmic.gtk());
  /// assert!(DesktopEnvironment::Gnome.gtk());
  /// assert!(DesktopEnvironment::Lxde.gtk());
  /// assert!(DesktopEnvironment::Mate.gtk());
  /// assert!(DesktopEnvironment::Unity.gtk());
  /// assert!(DesktopEnvironment::Xfce.gtk());
  /// assert!(DesktopEnvironment::Pantheon.gtk());
  /// assert!(DesktopEnvironment::Dde.gtk());
  ///
  /// // Non-GTK examples
  /// assert!(!DesktopEnvironment::Kde.gtk());
  /// assert!(!DesktopEnvironment::Windows.gtk());
  /// ```
  pub const fn gtk(self) -> bool {
    use DesktopEnvironment::*;
    matches!(self, Cinnamon | Cosmic | Dde | Gnome | Lxde | Mate | Pantheon | Unity | Xfce)
  }

  /// Test if the desktop environment is based on the Qt framework
  ///
  /// ```
  /// use detect_desktop_environment::DesktopEnvironment;
  ///
  /// // All matching desktop environments:
  /// assert!(DesktopEnvironment::Kde.qt());
  /// assert!(DesktopEnvironment::Lxqt.qt());
  /// assert!(DesktopEnvironment::Razor.qt());
  /// assert!(DesktopEnvironment::Tde.qt());
  ///
  /// // Non-Qt examples
  /// assert!(!DesktopEnvironment::Gnome.qt());
  /// assert!(!DesktopEnvironment::Windows.qt());
  /// ```
  pub const fn qt(self) -> bool {
    use DesktopEnvironment::*;
    matches!(self, Kde | Lxqt | Razor | Tde)
  }

  #[cfg(target_os = "macos")]
  fn detect_impl() -> Option<Self> {
    Some(DesktopEnvironment::MacOs)
  }

  #[cfg(target_os = "windows")]
  fn detect_impl() -> Option<Self> {
    Some(DesktopEnvironment::Windows)
  }

  #[cfg(not(any(target_os = "macos", target_os = "windows")))]
  fn detect_impl() -> Option<Self> {
    std::env::var("XDG_CURRENT_DESKTOP").ok().as_deref().and_then(Self::from_xdg_current_desktop)
  }

  /// Parse the desktop environment from the name registered with Freedesktop.org
  ///
  /// See <https://specifications.freedesktop.org/menu-spec/latest/apb.html>
  ///
  /// Returns `None` if the desktop is not registered.
  ///
  /// This function is strictly restricted to the DEs registered with Freedesktop, for a more
  /// complete list use [`DesktopEnvironment::from_xdg_name`]. Note that the check follows the
  /// spec and is case-sensitive.
  ///
  /// ```
  /// use detect_desktop_environment::DesktopEnvironment;
  ///
  /// assert_eq!(Some(DesktopEnvironment::Kde), DesktopEnvironment::from_freedesktop("KDE"));
  /// assert_eq!(None, DesktopEnvironment::from_freedesktop("kde")); // must be uppercase
  /// assert_eq!(None, DesktopEnvironment::from_freedesktop("SWAY")); // not registered
  /// assert_eq!(None, DesktopEnvironment::from_freedesktop("unknown_de"));
  /// ```
  pub fn from_freedesktop(name: &str) -> Option<Self> {
    // the patterns in the match below are ordered to match the order in the freedesktop table
    match name {
      "GNOME" => Some(DesktopEnvironment::Gnome),
      "GNOME-Classic" => Some(DesktopEnvironment::Gnome),
      "GNOME-Flashback" => Some(DesktopEnvironment::Gnome),
      "KDE" => Some(DesktopEnvironment::Kde),
      "LXDE" => Some(DesktopEnvironment::Lxde),
      "LXQt" => Some(DesktopEnvironment::Lxqt),
      "MATE" => Some(DesktopEnvironment::Mate),
      "Razor" => Some(DesktopEnvironment::Razor),
      "ROX" => Some(DesktopEnvironment::Rox),
      "TDE" => Some(DesktopEnvironment::Tde),
      "Unity" => Some(DesktopEnvironment::Unity),
      "XFCE" => Some(DesktopEnvironment::Xfce),
      "EDE" => Some(DesktopEnvironment::Ede),
      "Cinnamon" => Some(DesktopEnvironment::Cinnamon),
      "Pantheon" => Some(DesktopEnvironment::Pantheon),
      "DDE" => Some(DesktopEnvironment::Dde),
      "Endless" =>Some(DesktopEnvironment::Endless),
      "Old" =>Some(DesktopEnvironment::Old),
      _ => None,
    }
  }

  /// Parse the XDG desktop environment name
  ///
  /// This is an extended variant of [`DesktopEnvironment::from_freedesktop`]. It supports all
  /// registered Freedesktop names, as well as some extra unregistered names. This is the
  /// recommended method to parse names from the list in the env var `XDG_CURRENT_DESKTOP`.
  ///
  /// Returns `None` if the name is unknown.
  ///
  /// ```
  /// use detect_desktop_environment::DesktopEnvironment;
  ///
  /// assert_eq!(Some(DesktopEnvironment::Kde), DesktopEnvironment::from_xdg_name("KDE")); // freedesktop DE
  /// assert_eq!(None, DesktopEnvironment::from_xdg_name("kde")); // must be uppercase
  /// assert_eq!(Some(DesktopEnvironment::Sway), DesktopEnvironment::from_xdg_name("SWAY")); // not registered
  /// assert_eq!(None, DesktopEnvironment::from_xdg_name("unknown_de"));
  /// ```
  pub fn from_xdg_name(name: &str) -> Option<Self> {
    if let Some(de) = Self::from_freedesktop(name) {
      return Some(de);
    }

    // keep the patterns sorted alphabetically
    match name {
      "ENLIGHTENMENT" => Some(DesktopEnvironment::Enlightenment),
      "Hyprland" => Some(DesktopEnvironment::Hyprland),
      "SWAY" => Some(DesktopEnvironment::Sway),
      "X-Cinnamon" => Some(DesktopEnvironment::Cinnamon),
      _ => None,
    }
  }

  /// Retrieve the desktop environment from the format used by `XDG_CURRENT_DESKTOP`.
  ///
  /// `XDG_CURRENT_DESKTOP` is a colon separated list of information about the current desktop
  /// environment.
  /// See: <https://specifications.freedesktop.org/mime-apps-spec/1.0.1/ar01s02.html>
  ///
  /// Returns `None` if the resolution fails.
  /// Duplicate entries are allowed as long as they correspond to same Desktop Environment.
  pub fn from_xdg_current_desktop(xdg_current_desktop: &str) -> Option<Self> {
    let mut resolved: Option<DesktopEnvironment> = None;

    for part in xdg_current_desktop.split(':') {
      let de = match Self::from_xdg_name(part) {
        Some(de) => de,
        None => {
          // We ignore parsing errors as we don't really control which values are possible.
          // Some of the entries don't even represent a DE (e.g. `ubuntu:GNOME`, where `ubuntu` is
          // a distro, not a DE)
          // If you want more control over this, open an issue to discuss it.
          continue
        },
      };
      match resolved {
        None => {
          // first successfully parsed DE, store it but keep iterating to check for conflicts
          resolved = Some(de)
        },
        Some(prev) => {
          // a DE was already parsed previously, duplicates are allowed but a conflict causes
          // immediate rejection with `None`.
          // If you want more control over this, open an issue to discuss it.
          if de != prev {
            return None
          }
        }
      }
    }

    resolved
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn linux_tests() {
    // Cases without colon
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("Cinnamon"),
      Some(DesktopEnvironment::Cinnamon)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("ENLIGHTENMENT"),
      Some(DesktopEnvironment::Enlightenment)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("GNOME"),
      Some(DesktopEnvironment::Gnome)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("KDE"),
      Some(DesktopEnvironment::Kde)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("LXDE"),
      Some(DesktopEnvironment::Lxde)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("LXQt"),
      Some(DesktopEnvironment::Lxqt)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("MATE"),
      Some(DesktopEnvironment::Mate)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("Unity"),
      Some(DesktopEnvironment::Unity)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("X-Cinnamon"),
      Some(DesktopEnvironment::Cinnamon)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("XFCE"),
      Some(DesktopEnvironment::Xfce)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("TDE"),
      Some(DesktopEnvironment::Tde)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("DDE"),
      Some(DesktopEnvironment::Dde)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("Pantheon"),
      Some(DesktopEnvironment::Pantheon)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("SWAY"),
      Some(DesktopEnvironment::Sway)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("Hyprland"),
      Some(DesktopEnvironment::Hyprland)
    );

    // Colon splitting
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("ubuntu:GNOME"),
      Some(DesktopEnvironment::Gnome)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("ubuntu:KDE"),
      Some(DesktopEnvironment::Kde)
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("pop:GNOME"),
      Some(DesktopEnvironment::Gnome)
    );

    // Mixed messages
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("KDE:GNOME"),
      None
    );
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("ubuntu:KDE:GNOME"),
      None
    );

    // Strange cases
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("GNOME:GNOME"),
      Some(DesktopEnvironment::Gnome)
    );

    // Empty string
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop(""),
      None
    );

    // Unknown Desktop Environment
    assert_eq!(
      DesktopEnvironment::from_xdg_current_desktop("foo"),
      None
    );
  }
}
