extern crate git2;
extern crate rustc_version;

use git2::{DescribeFormatOptions, DescribeOptions, Repository};
use rustc_version::version_matches;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

fn get_version() -> String {
  let profile = env::var("PROFILE").unwrap();
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let mut info = Vec::new();
  info.push(format!("profile: {}", profile));
  if let Ok(repo) = Repository::open(&manifest_dir) {
    let version = repo.describe(DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true))
      .and_then(|x| x.format(Some(DescribeFormatOptions::new().dirty_suffix("-dirty"))));
    match version {
      Ok(v) => info.push(format!("git: {}", v)),
      Err(_) => {}
    }
  };
  info.join("\n")
}

fn main() {
  if !version_matches(">= 1.13.0") {
    writeln!(&mut io::stderr(), "tenjava cli requires at least Rust 1.13.0").unwrap();
    exit(1);
  }
  let version = get_version();
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("extra_version_info.rs");
  let mut f = File::create(&dest_path).unwrap();
  f.write_all(format!("
      fn extra_version_info() -> &'static str {{
          \"{}\"
      }}
  ",
                       version)
      .as_bytes())
    .unwrap();
}

