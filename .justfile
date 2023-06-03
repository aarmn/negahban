#!/usr/bin/env just --justfile

alias cmp_now := compare_head_to_now
alias cmp_head := compare_before_to_head
alias rev_last := revert_to_head
alias rev_semi := revert_to_semihead

#>>>>>>>>>>>>>>>#
# Im a git n00b #
#>>>>>>>>>>>>>>>#

# doc comment
compare_head_to_now FILE="":
    GIT_EXTERNAL_DIFF=difft git diff HEAD {{FILE}}

# doc comment
compare_before_to_head FILE="":
    GIT_EXTERNAL_DIFF=difft git diff HEAD HEAD^ {{FILE}}

# revert (takes a soft or hard)
# doc comment
revert_to_head SH="":
    

# doc comment
revert_to_semihead SH="":

#<<<<<<<<<<<<<<<#
# Im a git n00b #
#<<<<<<<<<<<<<<<#

# add files, build, test, commit the changes, and push to origin
apply MSG="":
    git add .
    cargo build
    cargo test
    cargo clippy
    git commit -m "{{MSG}}"
    git push origin
    # TODO: CI Stuff as well maybe here or in publish

# publishes 
publish MSG="":
    # just apply {{MSG}}
    cargo publish --dry-run
    cargo publish
    # TODO: bump to next version, before pub, if possible

apply_and_pub: apply && publish