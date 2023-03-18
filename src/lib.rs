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
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DesktopEnvironment {
  /// Cinnamon, the default desktop environment for Linux Mint.
  ///
  /// - <https://en.wikipedia.org/wiki/Cinnamon_(desktop_environment)>
  Cinnamon,
  /// Enlightenment desktop environment.
  ///
  /// - <https://en.wikipedia.org/wiki/Enlightenment_(software)>
  Enlightenment,
  /// Gnome, the default environment for many major Linux distributions.
  ///
  /// This also covers the COSMIC variant used by Pop!_OS.
  ///
  /// - <https://en.wikipedia.org/wiki/GNOME>
  /// - <https://github.com/pop-os/cosmic>
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
