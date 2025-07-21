In order to release a new version of this package the following steps are needed:

1. Make sure the CHANGELOG is up to date
2. Change the project's version in `Cargo.toml` and `CHANGELOG.md`, then commit and push the changes
3. Tag the latest commit with `git tag vX.Y.Z`
4. Push the tag using `git push origin --tags`. This will trigger a GitHub Action workflow that creates a new release
5. Go to [Releases](https://github.com/flother/spreet/releases) and publish the draft release. This will trigger another GitHub Actions workflow that publishes the new release to [crates.io](https://crates.io/) and [GitHub Container Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
