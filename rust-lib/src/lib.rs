#![deny(clippy::all)]

use keyring::{Entry, Result};
use std::env;

#[macro_use]
extern crate napi_derive;

fn credential_entry(service_name: String, author_name: String) -> Result<Entry> {
  match env::consts::OS {
      "windows" => {
        let target = format!("{}/{}", service_name, author_name);
        Entry::new_with_target(&target, &service_name, &author_name)
      }
      _ => Entry::new(&service_name, &author_name)
  }

}

#[napi]
pub fn set_password(
  service_name: String,
  author_name: String,
  password: String,
) -> napi::Result<()> {
  // TODO: impl validation
  let entry = match credential_entry(service_name, author_name) {
    Ok(entry) => entry,
    Err(err) => {
      eprintln!("Couldn't create entry: {err}");
      std::process::exit(1);
    }
  };
  match entry.set_password(&password) {
    Ok(()) => {
      println!("Credential set successfully.");
      Ok(())
    }
    Err(err) => {
      eprintln!("Failed to set credential: {err}");
      std::process::exit(1);
    }
  }
}

#[napi]
pub fn get_password(service_name: String, author_name: String) -> napi::Result<String> {
  let entry = match credential_entry(service_name, author_name) {
    Ok(entry) => entry,
    Err(err) => {
      eprintln!("Couldn't create entry: {err}");
      std::process::exit(1);
    }
  };

  match entry.get_password() {
    Ok(password) => Ok(password),
    Err(err) => {
      eprintln!("Credential not found: {err}");
      std::process::exit(1);
    }
  }
}
