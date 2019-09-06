# git-at-revision

This is a little commandline gadget to clone a git repo as of a certain
revision (SHA).  Unless you already did that in which case nothing happens.

Its not on crates.io yet, so clone this repo and then build it with the rust build tool cargo:

>  cargo install --path .

Usage: 
>  git-at-revision \<sha\> \<repo-url\> \<target-directory\> --dirtyok

program logic is more or less like this:
  
    does the target dir exist?
      yes - 
        is it the right revision?
          yes - 
            is it modified, or are there untracked files?
              yes - 
                error, unless --dirtyok.
              no - 
                success.
          no - 
            checkout revision.  success?
              yes - return.
              no - 
                git fetch, retry checkout.  success?
                  no - fail.
                  yes - success.
      no - 
        clone repo.  success?
          yes - checkout revision.  success?
            yes - success.
            no - error.
          no - 
            error.

