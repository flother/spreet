# Changelog

## Development version

- Optimise spritesheet PNG using [`oxipng`](https://github.com/shssoichiro/oxipng)
- Match the way [`spritezero-cli`](https://github.com/mapbox/spritezero-cli) traverses the input directory
- Provide a Homebrew formula tap for easy MacOS/Linux installation

## v0.2.0 (2022-03-22)

- Resize the target bin as required, instead of hardcoding a square 1.4 times the size of the sprites
- Trim unused transparent pixels from the spritesheet (excluding transparent pixels within sprites)
- Ensure target bin is at least as wide/tall as the widest/tallest sprite
- Pretty-print the JSON in the sprite index file
- Strip symbols from binaries using Cargo
- Add GitHub Actions workflow to draft a new release when a new tag is created
- Use one parallel code generation unit for release
- Bump clap Rust dependency from version 3.1.5 to version 3.1.6
- Bump actions/checkout GitHub Actions dependency from version 2 to version 3

## v0.1.0 (2022-03-18)

- Initial beta release
