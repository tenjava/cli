extern crate clap;
extern crate dotenv;

use clap::{Arg, ArgGroup, ArgMatches, App, AppSettings, SubCommand};
use dotenv::dotenv;

include!(concat!(env!("OUT_DIR"), "/extra_version_info.rs"));

fn get_name() -> String {
  option_env!("CARGO_PKG_NAME").unwrap_or("unknown_name").to_owned()
}

fn get_version(basic: bool) -> String {
  let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown_version").to_owned();
  if basic {
    version
  } else {
    let extra_version_info = extra_version_info();
    format!("{}\n\n{}", version, extra_version_info)
  }
}

fn get_options<'a>() -> ArgMatches<'a> {
  let version = get_version(true);
  App::new(get_name())
    .version(version.as_ref())
    .global_setting(AppSettings::SubcommandRequiredElseHelp)
    .global_setting(AppSettings::VersionlessSubcommands)
    .global_setting(AppSettings::DeriveDisplayOrder)
    .subcommand(SubCommand::with_name("repositories")
      .aliases(&["repository", "repo", "repos"])
      .about("repository management")
      .arg(Arg::with_name("organization")
        .help("the organization on which to manage repositories")
        .required(true)
        .takes_value(true)
        .value_name("organization"))
      .arg(Arg::with_name("live")
        .long("live")
        .help("actually sends the requests to GitHub"))
      .arg(Arg::with_name("manifest")
        .help("a manifest json file containing the repositories on which to operate")
        .short("m"))
      .arg(Arg::with_name("list")
        .help("a list of repositories on which to operate, specified on the command line")
        .short("l"))
      .group(ArgGroup::with_name("manifest_or_list")
        .args(&["manifest", "list"])
        .required(true))
      .subcommand(SubCommand::with_name("create")
        .about("creates repositories for contestants"))
      .subcommand(SubCommand::with_name("open")
        .about("opens a list of repositories, allowing contributions to be pushed"))
      .subcommand(SubCommand::with_name("close")
        .about("closes a list of repositories, preventing contributions from being pushed")))
    .get_matches()
}

fn inner() -> i32 {
  let matches = get_options();

  dotenv().ok();
  0
}

fn main() {
  let exit_code = inner();
  std::process::exit(exit_code);
}
