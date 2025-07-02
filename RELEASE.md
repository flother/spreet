In order to release a new version of this package the following steps are needed:
1. Change the version in Cargo.toml file and push the changes.
2. Tag the latest commit: `git tag vX.Y.Z`
3. Push the tag to trigger the github action: `git push origin --tags`
4. Go to Releases and publish the draft release, this will trigger an action to publish the latest release to Crate.io and GHCR.
