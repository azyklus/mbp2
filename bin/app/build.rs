const FRONTEND_DIR: &str = "../../web";

fn main() {
   println!("cargo:rerun-if-changed={}", FRONTEND_DIR);
   println!("cargo:rerun-if-changed={}/index.html", FRONTEND_DIR);

   Command::new("trunk")
      .args(&["build", "--release"])
      .current_dir(FRONTEND_DIR)
      .status()
      .expect("could not build frontend");
}

use std::process::Command;
