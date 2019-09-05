use std::env;
use std::process::Command;
use std::str;

use std::path::Path;

/*

gitcloneasof <revision> <repo> <targetdir> <--dirtyok>

*/

/*
TODO:
  - fail when the repo is dirty.
  - also check for untracked files.
  - if the target has no .git (like its the parent dir), uh?  maybe fail?
    - could just print a better error message.
    - or, somehow the parent dir should be enough?
  - the temptation to modify the gitdeps/ repos will be strong!
    how to deal with this?
    dirty flag to build script.
    production disables dirty flag.
    git hooks?
*/

fn into_err<A, E>(opt: Option<A>, err: E) -> Result<A, E> {
  match opt {
    Some(val) => Ok(val),
    None => Err(err),
  }
}

fn main() {
  match dothethings() {
    Ok(_) => println!("success!"),
    Err(e) => {
      println!("error: {:?}", e);
      println!("command syntax: ");
      println!("gitcloneasof <revision> <repo> <targetdir> <--dirtyok>");
    }
  }
}

fn dothethings() -> Result<(), String> {
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name

  let revision = into_err(iter.next(), "revision not found")?;
  let repo = into_err(iter.next(), "repo not found")?;
  let target = into_err(iter.next(), "target not found")?;

  let dirarg = format!("--git-dir={}/.git", target).to_string();
  let worktreearg = format!("--work-tree={}", target).to_string();

  if Path::new(target.as_str()).exists() {
    println!("path exists!");
  } else {
    println!("'{}' doesnt exist, cloning! {}", target, dirarg);
    // clone!
    Command::new("git")
      .args(&["clone", repo.as_str(), target.as_str()])
      .output()
      .expect("failed to execute 'git' command");

    println!("cloned repo: {}:", repo);
  };

  // function to check the revision
  let checkrev = || -> Result<bool, String> {
    let current_rev = Command::new("git")
      .args(&[dirarg.as_str(), "rev-parse", "HEAD"])
      .output()
      .expect("failed to execute 'git' command");

    let revstring = match str::from_utf8(&current_rev.stdout) {
      Ok(rs) => Ok(rs),
      Err(_) => Err("utf8 conversion error in revision string!"),
    }?;
    Ok(revstring.trim() == revision.as_str())
  };

  if checkrev()? {
    println!("revision matches for repo: {}", repo);
    Ok(())
  } else {
    // they don't match, do a checkout.
    let checkout = Command::new("git")
      .args(&[
        dirarg.as_str(),
        worktreearg.as_str(),
        "checkout",
        revision.trim(),
      ])
      .output()
      .expect("failed to execute 'git' command");

    println!("checkout result: {:?}!", checkout);

    if checkrev()? {
      println!("success!");
      Ok(())
    } else {
      // ok try a fetch and then a checkout.
      let fetch = Command::new("git")
        .args(&[dirarg.as_str(), worktreearg.as_str(), "fetch"])
        .output()
        .expect("failed to execute 'git' command");

      println!("git fetch result: {:?}", fetch);

      let checkout = Command::new("git")
        .args(&[
          dirarg.as_str(),
          worktreearg.as_str(),
          "checkout",
          revision.trim(),
        ])
        .output()
        .expect("failed to execute 'git' command");

      println!("checkout2 result: {:?}!", checkout);

      if checkrev()? {
        println!("success!");
        Ok(())
      } else {
        Err(format!(
          "unable to check out specified revision for repo: {}",
          repo
        ))
      }
    }
  }
}
