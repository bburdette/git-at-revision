# git-at-revision

This is a little commandline gadget to clone a git repo as of a certain
revision (SHA).  Unless you already did that in which case nothing happens.

get-at-revision *sha* *repo-url* *target-directory*
  
    does the target exist?
      yes - 
        is it the right revision?
          yes - 
            is it modified?
              yes - 
                reset, or error.
              no - 
                success.
          no - 
            is it modified?
              yes - error.
              no - 
                checkout revision.  success?
                  yes - return.
                  no - 
                    git pull, retry checkout.  success?
                      no - fail.
                      yes - success.
      no - 
        clone repo.  success?
          yes - checkout revision.  success?
            yes - success.
            no - error.
          no - 
            error.

