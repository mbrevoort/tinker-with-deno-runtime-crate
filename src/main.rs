use std::env;

mod lib;

fn main() {
  // Expect the path to a single bundled JS source file to execute
  let args: Vec<String> = env::args().collect();
  let source_path: &str = &args[1][..];
  lib::run_local(source_path);
}
