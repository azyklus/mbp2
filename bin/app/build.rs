const FRONTEND_DIR: &str = "../../web";

fn main() {
   println!("cargo:rerun-if-changed={}", FRONTEND_DIR);
   println!("cargo:rerun-if-changed={}/index.html", FRONTEND_DIR);

   // Build our frontend.
   Command::new("trunk")
      .args(&["build", "--release"])
      .current_dir(FRONTEND_DIR)
      .status()
      .expect("could not build frontend");

   // Take note of our current working directory and change to our frontend dir.
   let Some(root) = env::current_dir().ok() else { unreachable!() };
   env::set_current_dir(FRONTEND_DIR).expect("failed to set working dir");

   // Copy static assets to our web 'dist' directory.
   let options = CopyOptions::new();
   dir::create("dist/static", true).expect("could not create destination dir");
   dir::copy("static", "dist", &options).expect("could not copy static assets");
   fs::copy("robots.txt", "dist/robots.txt").unwrap();

   // Return our current working directory to the project root.
   env::set_current_dir(root).expect("failed to set working dir");
}

use fs_extra::{dir, dir::CopyOptions};
use std::{env, fs, process::Command};
