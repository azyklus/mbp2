const FRONTEND_DIR: &str = "../../web";
const DATABASE_DIR: &str = "../database";

fn main() {
   println!("cargo:rerun-if-changed={}", FRONTEND_DIR);
   println!("cargo:rerun-if-changed={}/index.html", FRONTEND_DIR);

   Command::new("trunk")
      .args(&["build", "--release"])
      .current_dir(FRONTEND_DIR)
      .status()
      .expect("could not build frontend");

   Command::new("cp")
      .args(&["-r", "static", "dist/static"])
      .current_dir(FRONTEND_DIR)
      .status()
      .expect("could not copy static files");

   Command::new("nimble")
      .args(&["build", "--mm:orc"])
      .current_dir(DATABASE_DIR)
      .status()
      .expect("could not build database service");

   Command::new("cp")
      .args(&["-r", "./mbp2_database", format!("{}/dist/mbp2_database", FRONTEND_DIR).as_str()])
      .current_dir(DATABASE_DIR)
      .status()
      .expect("could not copy database program to dist folder");
}

use std::process::Command;
