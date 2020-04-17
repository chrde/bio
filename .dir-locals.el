;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((nil . ((counsel-etags-update-tags-backend . (lambda
                                                (src-dir)
                                                (shell-command "rusty-tags emacs")))
         (counsel-etags-tags-file-name . "rusty-tags.emacs")
         (counsel-compile-local-builds . ("cargo check" "cargo run" "cargo test" "cargo clippy")))))
