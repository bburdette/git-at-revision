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
    Ok(success) => {
      if success {
        println!("success!");
        std::process::exit(0);
      } else {
        std::process::exit(1);
      }
    }
    Err(e) => {
      println!("error: {:?}", e);
      std::process::exit(1);
    }
  }
}

fn dothethings() -> Result<bool, String> {
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name

  // with zero args, print the syntax.
  if iter.len() == 0 {
    println!("command syntax: ");
    println!("gitcloneasof <revision> <repo> <targetdir> <--dirtyok>");
    return Ok(false);
  }

  let revision = into_err(iter.next(), "revision not found")?;
  let repo = into_err(iter.next(), "repo not found")?;
  let target = into_err(iter.next(), "target not found")?;
  let dirtyok = iter.next() == Some("--dirtyok".to_string());

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

  // dirty check function.
  // git diff-index --quiet HEAD --
  let checkdirty = || -> bool {
    let wat = Command::new("git")
      .args(&[dirarg.as_str(), "diff-index", "--quiet", "HEAD", "--"])
      .output()
      .expect("failed to execute 'git' command");
    println!("wat: {:?}", wat);
    !wat.status.success()
  };

  if !dirtyok {
    println!("dirty? {}", checkdirty());
    return Ok(false);
  } else {
    println!("not checking for a dirty repo!");
  }

  if checkrev()? {
    println!("revision matches for repo: {}", repo);
    Ok(true)
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
      Ok(true)
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
        Ok(true)
      } else {
        Err(format!(
          "unable to check out specified revision for repo: {}",
          repo
        ))
      }
    }
  }
}
