use ::std::env;

#[derive(Debug, PartialEq, Eq)]
pub enum DesktopEnvironment {
  Unknown,
  Cinnamon,
  Enlightenment,
  Gnome,
  Kde,
  Lxde,
  Lxqt,
  MacOs,
  Mate,
  Unity,
  Windows,
  Xfce,
}

#[cfg(target_os = "macos")]
pub fn detect() -> DesktopEnvironment {
  DesktopEnvironment::MacOs
}

#[cfg(target_os = "windows")]
pub fn detect() -> DesktopEnvironment {
  DesktopEnvironment::Windows
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn detect() -> DesktopEnvironment {
  match env::var("XDG_CURRENT_DESKTOP") {
    Err(_) => DesktopEnvironment::Unknown,
    Ok(current_desktop) => {
      match current_desktop.as_str() {
        "Cinnamon" => DesktopEnvironment::Cinnamon,
        "ENLIGHTENMENT" => DesktopEnvironment::Enlightenment,
        "GNOME" => DesktopEnvironment::Gnome,
        "KDE" => DesktopEnvironment::Kde,
        "LXDE" => DesktopEnvironment::Lxde,
        "LXQt" => DesktopEnvironment::Lxqt,
        "MATE" => DesktopEnvironment::Mate,
        "Unity" => DesktopEnvironment::Unity,
        "XFCE" => DesktopEnvironment::Xfce,
        _ => DesktopEnvironment::Unknown,
      }
    }
  }
}
