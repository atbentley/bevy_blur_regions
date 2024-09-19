# Releasing

This checklist walks through the typical process for creaing a release of bevy_blur_regions.

## Create release commit

- [ ] Create a release branch: `git switch -c 1.0.0`
- [ ] Update `version` field in `Cargo.toml`
- [ ] Update release title and date in `CHANGELOG.md`
- [ ] Remove unused sections in `CHANGELOG.md`
- [ ] Update compatibility matrix in `README.md`
- [ ] Verify the release works by testing all examples
- [ ] Verify the release works by pointing a local project to it: `bevy_blur_regions = {path = "../bevy_blur_regions", ... }`
- [ ] Make a release commit: `git commit -m 'Release 1.0.0'`
- [ ] Create a release PR and merge it

## Publish the release on crates.io

- [ ] `cargo publish`

## Create a release tag

- [ ] `git switch main && git pull --rebase`
- [ ] `git tag v1.0.0`
- [ ] `git push --tags`

## Create a release on GitHub
