# Changelog

## Development version

- **Breaking change**: update [Oxipng](https://github.com/shssoichiro/oxipng) dependency to v7. Spritesheet PNGs output by Spreet are now compressed using [libdeflate](https://github.com/ebiggers/libdeflate). This produces smaller files but the PNGs won't be byte-to-byte compatible with spritesheets output by earlier versions of Spreet. This also causes Spreet's minimum Rust version to be 1.61.0

## v0.5.0 (2022-12-11)

- Rasterize SVGs in parallel
- Add tutorial and benchmarks to README
- Update clap dependency to v4
- Update oxipng dependency to v6
- Use tiny-skia and usvg as re-exported from resvg
- Move predicates to dev-dependencies
- Add CLI tests

## v0.4.0 (2022-08-16)

- Switch to [crunch-rs](https://github.com/ChevyRay/crunch-rs) rectangle-packing library
- Add `--minify-index-file` CLI flag (see [#15](https://github.com/flother/spreet/issues/15))

## v0.3.0 (2022-08-08)

- Add `--unique` argument (see [#14](https://github.com/flother/spreet/pull/14))
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
