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
  /// Enlightenment desktop environment.
  ///
  /// - <https://en.wikipedia.org/wiki/Enlightenment_(software)>
  Enlightenment,
  /// Gnome, the default environment for many major Linux distributions.
  ///
  /// - <https://en.wikipedia.org/wiki/GNOME>
  Gnome,
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
  /// <https://mate-desktop.org/>
  Mate,
  /// Unity, the legace desktop environment for Ubuntu
  ///
  /// <https://en.wikipedia.org/wiki/Unity_%28user_interface%29>
  Unity,
  /// Windows, the environments for Microsoft's OS
  Windows,
  /// Xfce
  ///
  /// <https://xfce.org/>
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
  ///
  /// // Non-GTK examples
  /// assert!(!DesktopEnvironment::Kde.gtk());
  /// assert!(!DesktopEnvironment::Windows.gtk());
  /// ```
  pub const fn gtk(self) -> bool {
    use DesktopEnvironment::*;
    matches!(self, Cinnamon | Cosmic | Gnome | Lxde | Mate | Unity | Xfce)
  }

  /// Test if the desktop environment is based on the Qt framework
  ///
  /// ```
  /// use detect_desktop_environment::DesktopEnvironment;
  ///
  /// // All matching desktop environments:
  /// assert!(DesktopEnvironment::Kde.qt());
  /// assert!(DesktopEnvironment::Lxqt.qt());
  ///
  /// // Non-Qt examples
  /// assert!(!DesktopEnvironment::Gnome.qt());
  /// assert!(!DesktopEnvironment::Windows.qt());
  /// ```
  pub const fn qt(self) -> bool {
    use DesktopEnvironment::*;
    matches!(self, Kde | Lxqt)
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
    std::env::var("XDG_CURRENT_DESKTOP")
      .ok()
      .as_deref()
      .and_then(|de| match de {
        "Cinnamon" => Some(DesktopEnvironment::Cinnamon),
        "ENLIGHTENMENT" => Some(DesktopEnvironment::Enlightenment),
        "GNOME" => Some(DesktopEnvironment::Gnome),
        "KDE" => Some(DesktopEnvironment::Kde),
        "LXDE" => Some(DesktopEnvironment::Lxde),
        "LXQt" => Some(DesktopEnvironment::Lxqt),
        "MATE" => Some(DesktopEnvironment::Mate),
        "pop:GNOME" => Some(DesktopEnvironment::Gnome),
        "Unity" => Some(DesktopEnvironment::Unity),
        "X-Cinnamon" => Some(DesktopEnvironment::Cinnamon),
        "XFCE" => Some(DesktopEnvironment::Xfce),
        _ => None,
      })
  }
}
