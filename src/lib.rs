#![feature(custom_derive, plugin, try_from)]
#![plugin(serde_macros)]

#[macro_use] extern crate error_chain;
extern crate clap;
extern crate serde;
extern crate serde_json;

mod errors;

use clap::ArgMatches;
use errors::*;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::path::PathBuf;

pub struct RepositoryManager<'a> {
  pub matches: &'a ArgMatches<'a>,
  pub manifest: Manifest
}

struct ManagerInternals {
  manifest: Manifest
}

impl<'a> RepositoryManager<'a> {
  pub fn new(matches: &'a ArgMatches<'a>) -> Result<Self> {
    let internals = try!(RepositoryManager::verify_matches(matches));
    Ok(RepositoryManager {
      matches: matches,
      manifest: internals.manifest
    })
  }

  fn verify_matches(matches: &'a ArgMatches<'a>) -> Result<ManagerInternals> {
    let manifest = if matches.is_present("manifest") {
      let manifest_path = match matches.value_of("manifest") {
        Some(m) => m,
        None => return Err("no manifest".into())
      };
      try!(Manifest::try_from(PathBuf::from(&manifest_path)))
    } else if matches.is_present("list") {
      let list = match matches.values_of("list") {
        Some(l) => l,
        None => return Err("no list".into())
      };
      list
        .map(|item| ManifestRepository {
          name: item.to_owned(),
          description: String::new(),
          collaborators: Vec::new()
        })
        .collect()
    } else {
      return Err("no manifest or list".into());
    };
    Ok(ManagerInternals {
      manifest: manifest
    })
  }

  pub fn create(&self) -> Result<()> {
    unimplemented!();
  }

  pub fn open(&self) -> Result<()> {
    unimplemented!();
  }

  pub fn close(&self) -> Result<()> {
    unimplemented!();
  }

  pub fn delete(&self) -> Result<()> {
    unimplemented!();
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
  pub repositories: Vec<ManifestRepository>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestRepository {
  pub name: String,
  pub description: String,
  pub collaborators: Vec<String>
}

impl TryFrom<PathBuf> for Manifest {
  type Err = Error;

  fn try_from(path: PathBuf) -> Result<Manifest> {
    if !path.is_file() {
      return Err(format!("{} does not point to a file", path.to_string_lossy()).into());
    }
    let mut data = String::new();
    try!(try!(File::open(path)).read_to_string(&mut data));
    let manifest: Manifest = try!(serde_json::from_str(&data));
    Ok(manifest)
  }
}

impl FromIterator<ManifestRepository> for Manifest {
  fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=ManifestRepository> {
    let repos: Vec<ManifestRepository> = iter.into_iter().collect();
    Manifest {
      repositories: repos
    }
  }
}
