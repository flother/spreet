# Changelog

## Development version

- Update [clap](https://crates.io/crates/clap) dependency to v4.5
- Update [multimap](https://crates.io/crates/multimap) dependency to v0.10

The minimum supported version of Rust is now 1.74.0 (released November 2023).

## v0.11.0 (2023-12-05)

- Add support for SDF icons (aka [re-colourable images](https://docs.mapbox.com/help/troubleshooting/using-recolorable-images-in-mapbox-maps/)). See [#58](https://github.com/flother/spreet/issues/58)
- **Breaking change**: due to the addition of SDF icons, both the `SpriteDescription` and `SpritesheetBuilder` structs have a new boolean field named `sdf`, while `SpriteDescription::new()` also takes a new `sdf` argument. Set these to `false` if you want to match the existing behaviour (i.e. no SDF icons). To create a spritesheet of SDF icons, call `SpritesheetBuilder::make_sdf()`.
- Add a new constructor, `Sprite::new_sdf()`. This rasterises an SVG to a bitmap as usual, but generates a signed distance field for the image and stores that data in the bitmap's alpha channel

## v0.10.0 (2023-11-29)

- **Breaking change:** move all public identifiers to the root of the crate, e.g. `spreet::error::SpreetError` -> `spreet::SpreetError`, with the exception of `resvg`.
- Update [oxipng](https://crates.io/crates/oxipng) dependency to [v9.0.0](https://github.com/shssoichiro/oxipng/blob/master/CHANGELOG.md#version-900). This improves compression of PNG spritesheets without visual changes, but the PNGs won't be byte-to-byte compatible with spritesheets output by earlier versions of Spreet
- Update [resvg](https://crates.io/crates/resvg) dependency to [v0.36.0](https://github.com/RazrFalcon/resvg/blob/master/CHANGELOG.md#user-content-0360---2023-10-01)
- Remove the deprecated function `spreet::sprite::generate_pixmap_from_svg()`
- The `spreet::sprite_name` function (previously available as `spreet::sprite::sprite_name`) now returns `Result<String, Error>` instead of `String`, and will no longer panic
- The `spreet::get_svg_input_paths` function (previously available as `spreet::fs::get_svg_input_paths`) now returns `Result<Vec<PathBuf>, Error>` instead of `Vec<PathBuf>`, and will no longer panic

## v0.9.0 (2023-10-08)

- Support stretchable icons (see [#53](https://github.com/flother/spreet/issues/53))
- Make the CLI an optional (but default) feature ([#62](https://github.com/flother/spreet/pull/62)). This speeds up the build when using Spreet as a Rust library (see [README](README.md#using-spreet-as-a-rust-library))
- Fix bug that meant URLs in SVG `<image>` elements were resolved relative to the current working directory, not to the SVG itself (see [#60](https://github.com/flother/spreet/issues/60))
- Update [resvg](https://crates.io/crates/resvg) dependency to v0.35
- Update [clap](https://crates.io/crates/clap) dependency to v4.4
- Remove [Rayon](https://crates.io/crates/rayon) dependency. This means the Spreet CLI no longer parses SVGs in parallel, but that was a fun-but-unnecessary optimisation in the first place that generally saved only a handful of milliseconds
- **Deprecated**: `spreet::sprite::generate_pixmap_from_svg()` has been deprecated and will be removed in a future version. Use `spreet::sprite::Spreet::pixmap()` instead

The minimum supported version of Rust is now 1.70.0 (released June 2023).

## v0.8.0 (2023-06-15)

- Improvements to using Spreet as a Rust library (#57 and #59)
- Optimise Oxipng usage to reduce dev dependencies (#61)
- Optimise the `main` function (#56)
- Update [crunch](https://crates.io/crates/crunch) dependency to v0.5.3
- Update [resvg](https://crates.io/crates/resvg) dependency to v0.34
- Update [clap](https://crates.io/crates/clap) dependency to v4.3
- Update [multimap](https://crates.io/crates/multimap) dependency to v0.9.0
- Update [Rayon](https://crates.io/crates/rayon) dependency to v1.7
- Update [assert_fs](https://crates.io/crates/assert_fs) dependency to v1.0.13

Note: the update to [resvg](https://crates.io/crates/resvg) brings a new image rendering algorithm. This produces smaller images and improves performance, but the PNGs won't be byte-to-byte compatible with spritesheets output by earlier versions of Spreet. There should be no visual change though.

## v0.7.0 (2023-03-26)

- Replace unmaintained [actions-rs/toolchain](https://github.com/actions-rs/toolchain) with [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) ([#44](https://github.com/flother/spreet/pull/44) and [#45](https://github.com/flother/spreet/pull/45))
- Publish to crates.io when new version is released ([#46](https://github.com/flother/spreet/pull/46))
- Update clap dependency to v4.1

## v0.6.0 (2023-02-13)

- Add `--recursive` argument, to include images in sub-directories (see [#43](https://github.com/flother/spreet/pull/43))
- **Breaking change**: update [Oxipng](https://github.com/shssoichiro/oxipng) dependency to v8. Spritesheet PNGs output by Spreet are now compressed using [libdeflate](https://github.com/ebiggers/libdeflate). This produces smaller files but the PNGs won't be byte-to-byte compatible with spritesheets output by earlier versions of Spreet. This also causes Spreet's minimum Rust version to be 1.61.0

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
