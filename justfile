cmp_now_to_head:


cmp_head_to_before:


# revert (takes a soft or hard)

revert_to_head SH:


revert_to_before SH:


# add commit push (takes a commit msg)
apply MSG:
    git add .
    git commit -m "{{MSG}}"
    git push origin

# publish to crate.io, bump to next version possible
publish: apply:
    cargo publish --dry-run
    cargo publish