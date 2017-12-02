extern crate detect_desktop_environment;

use detect_desktop_environment::{DesktopEnvironment, detect};

fn main() {
  let de: DesktopEnvironment = detect();
  println!("{:?}", de);
}
