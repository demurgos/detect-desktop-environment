extern crate detect_desktop_environment;

use detect_desktop_environment::DesktopEnvironment;

fn main() {
  match DesktopEnvironment::detect() {
    Some(de) => println!("detected desktop environment: {de:?}"),
    None => println!("failed to detect desktop environment"),
  }
}
