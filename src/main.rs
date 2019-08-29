use std::env;
use std::process::Command;
use std::str;

use std::path::Path;

/*

gitcloneasof <revision> <repo> <target>

*/

fn into_err<A,E>(opt : Option<A>, err : E) -> Result<A,E> {
  match opt {
    Some(val) => Ok(val),
    None => Err(err),
  }
}

fn main() {
  match dothethings() {
    Ok(_) => println!("success!"),
    Err(e) => println!("error: {:?}", e),
  }
}

fn dothethings() -> Result<(),String> {
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name

  let revision = into_err(iter.next(),"revision not found")?;
  let repo = into_err(iter.next(),"repo not found")?;
  let target = into_err(iter.next(),"target not found")?;

  let dirarg = format!("--git-dir={}/.git", target).to_string();

  if Path::new(target.as_str()).exists() {
    println!("path exists!");
  }
  else {
    println!("path doesnt exists, cloning!i {}", dirarg);
    // clone!
    let clone = Command::new("git")
      .args(&["clone", repo.as_str(), target.as_str()])
      .output()
      .expect("failed to execute 'git' command");

  }

  // check the revision
  let current_rev = Command::new("git")
    .args(&[dirarg.as_str(),"rev-parse", "HEAD"])
    .output()
    .expect("failed to execute 'git' command");

  let revstring = match str::from_utf8(&current_rev.stdout) {
    Ok(rs) => Ok(rs),
    Err(_) => Err("utf8 conversion error in revision string!"),
  }?;
  if revstring.trim() == revision.as_str() {
    println!("they're equal")
  }
  else
  {
    // they don't match, do a checkout.
    let checkout = Command::new("git")
      .args(&["checkout", target.as_str()])
      .output()
      .expect("failed to execute 'git' command");

    println!("clone result: {:?}!", checkout)
  }

  println!("revstring: {:?}", revstring);  println!("currentrev: {:?}", current_rev);

  Ok(())
}
  /*
  match iter.next() {
    Some(revision) => {
    println!("revision: {} ", revision);
      match iter.next() {
        Some(repo) => {
          println!("repo: {} ", repo);
          match iter.next() {
            Some(target) => {
              println!("target: {}", target);
            }
            None => {
              println!("nah");
            }
          }
        }
        None => {
          println!("nah");
        }
      }
    }
    None => {
      println!("nah");
    }
  }
  */
