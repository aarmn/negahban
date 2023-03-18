#!/usr/bin/env just --justfile

alias cmp_now := compare_head_to_now
alias cmp_head := compare_before_to_head
alias rev_last := revert_to_head
alias rev_semi := revert_to_semihead

# Im a git n00b
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


# add commit push (takes a commit msg)
# doc comment
apply MSG="":
    git add .
    git commit -m "{{MSG}}"
    git push origin

# publish to crate.io, //TODO: bump to next version possible
# publish: apply:
#     cargo build
#     cargo test
#     cargo publish --dry-run
#     cargo publish