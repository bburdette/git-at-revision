use std::env;
use std::process::Command;

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

  let dewit = Command::new("git")
    .args(&["clone", repo.as_str(), target.as_str()])
    .output()
    .expect("failed to execute 'git' command");

  dewit.stdout;
  Ok(())
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
}
