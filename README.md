# Spreet: create spritesheets from SVGs

Spreet is a command-line tool that creates a [spritesheet](https://en.wikipedia.org/wiki/Spritesheet) (aka texture atlas) from a directory of SVG images. You'll need this when you create [MapLibre](https://maplibre.org/) or [Mapbox](https://docs.mapbox.com/) vector web maps, where cartographic stylesheets require that [icons be loaded from a spritesheet](https://maplibre.org/maplibre-gl-js-docs/style-spec/sprite/).

Compared to other tools for creating spritesheets from SVGs, Spreet:

- outputs smaller spritesheets (both fewer pixels and fewer bytes)
- is a self-contained ~2.2 MB binary
- is faster (multi-threaded)

_Spreet_ (also _spreit_, _spret_, _sprit_) is the [Scots](https://en.wikipedia.org/wiki/Scots_language) word for a sprite, the fairy-like creature from Western folklore.

[![CI status](https://github.com/flother/spreet/actions/workflows/ci.yml/badge.svg)](https://github.com/flother/spreet/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/flother/spreet)](https://github.com/flother/spreet/releases)

## Table of contents

- [Installation](#installation)
- [Tutorial](#tutorial)
- [Command-line usage](#command-line-usage)
- [Using Spreet as a Rust library](#using-spreet-as-a-rust-library)
- [Benchmarks](#benchmarks)

## Installation

You can install Spreet using Homebrew, `cargo install`, by downloading pre-built binaries, or by building from source.

### Homebrew

If you use [Homebrew](https://brew.sh/) on MacOS or Linux you can install Spreet from the command-line:

```
brew install flother/taps/spreet
```

(You can review [the code run by the formula](https://github.com/flother/homebrew-taps/blob/master/spreet.rb) before you install.)

### Installing from crates.io (`cargo install`)

Rust's `cargo install` command lets you install a binary crate locally. You can install the latest published version of Spreet with:

```
cargo install spreet
```

### Download pre-built binaries

Pre-built binaries are provided for MacOS, Linux, and Windows. The MacOS and Linux binaries are built for both Intel and ARM CPUs. Visit the [releases](https://github.com/flother/spreet/releases) page to download the latest version of Spreet.

### Build from source

You'll need a recent version of the Rust toolchain (try [Rustup](https://rustup.rs/) if you don't have it already). With that, you can check out this repository:

    git clone https://github.com/flother/spreet
    cd spreet

And then build a release:

    cargo build --release

Once finished, the built binary will be available as `./target/release/spreet`.

## Tutorial

When you're making your own style for a vector map, you'll have icons that you want to appear on top of the map. Symbols for roads or icons for hospitals and schools — that sort of thing. You'll have a directory of SVGs (like the [`icons` directory in the osm-bright-gl-style](https://github.com/openmaptiles/osm-bright-gl-style/tree/8af4769692d0f9219d0936711609d580b34bf365/icons)) and you'll want to convert them into a single raster image (like the [spritesheet from osm-bright-gl-style](https://github.com/openmaptiles/osm-bright-gl-style/blob/03a529f9040cfdfd3a30fb6760fc96d0ae41cf39/sprite%402x.png)).

Let's say you have a directory of SVGs named `icons` and you want to create a spritesheet named `my_style.png`. Run Spreet like this:

    spreet icons my_style

Spreet will also create an [index file](https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#index-file) named `my_style.json` that contains a description of the dimensions and location of each image contained in the spritesheet.

If you want to create a "retina" version of the spritesheet named `my_style@2x.png`, use the `--retina` option:

    spreet --retina icons my_style@2x

You might have multiple copies of the same icon — for example, you might use the same "open book" icon for both libraries (`library.svg`) and bookshops (`bookshop.svg`). If you pass the `--unique` option, Spreet will include only the icon once in the spritesheet, but reference it twice from the index file. This helps reduce the size of your spritesheet.

    spreet --retina --unique icons my_style@2x

By default the JSON index file is pretty-printed, but you can minify it with the `--minify-index-file` option:

    spreet --retina --unique --minify-index-file icons my_style@2x

When you create a spritesheet for your production environment, use `--unique --minify-index-file` for best results.

## Command-line usage

```
$ spreet --help
Create a spritesheet from a set of SVG images

Usage: spreet [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   A directory of SVGs to include in the spritesheet
  <OUTPUT>  Name of the file in which to save the spritesheet

Options:
  -r, --ratio <RATIO>      Set the output pixel ratio [default: 1]
      --retina             Set the pixel ratio to 2 (equivalent to `--ratio=2`)
      --unique             Store only unique images in the spritesheet, and map them to multiple names
      --recursive          Include images in sub-directories
  -m, --minify-index-file  Remove whitespace from the JSON index file
  -h, --help               Print help
  -V, --version            Print version
```

## Using Spreet as a Rust library

The main purpose of Spreet is to be command-line tool, but you can also use it as a library in your own Rust code. To add Spreet as a dependency, include this in your `Cargo.toml`:

```toml
spreet = { version = "0.8.0", default-features = false }
```

To learn how to build your spritesheets programatically, see the [Spreet crate docs on docs.rs](https://docs.rs/spreet) and have a [look at the spritesheet tests](https://github.com/flother/spreet/blob/master/tests/sprite.rs).

## Benchmarks

To compare the output from [spritezero](https://github.com/mapbox/spritezero-cli) and Spreet, benchmarks are run against SVG sprite sets from four diverse map styles: [osm-bright-gl-style](https://github.com/openmaptiles/osm-bright-gl-style), [openstreetmap-americana](https://github.com/ZeLonewolf/openstreetmap-americana), [mapbox-gl-styles (basic)](https://github.com/mapbox/mapbox-gl-styles), and [mapbox-gl-whaam-style](https://github.com/mapbox/mapbox-gl-whaam-style). Unique, retina spritesheets are output (`--unique --retina`), and Spreet also uses `--minify-index-file` (spritezero doesn't have that option).

### Spritesheet size (total pixels)

| Map style                | Spritezero pixels | Spreet pixels | Change |
| :----------------------- | ----------------: | ------------: | -----: |
| osm-bright-gl-style      |           208,810 |       130,048 |   -38% |
| openstreetmap-americana  |           577,548 |       389,640 |   -33% |
| mapbox-gl-styles (basic) |           271,488 |       258,064 |    -5% |
| mapbox-gl-whaam-style]   |            90,944 |        59,136 |   -35% |

### Spritesheet file size (bytes)

| Map style                | Spritezero file size | Spreet file size | Change |
| :----------------------- | -------------------: | ---------------: | -----: |
| osm-bright-gl-style      |               43,860 |           24,588 |   -44% |
| openstreetmap-americana  |              140,401 |           78,617 |   -44% |
| mapbox-gl-styles (basic) |               76,383 |           30,771 |   -60% |
| mapbox-gl-whaam-style    |               17,342 |            5,037 |   -71% |

### Index file size (bytes)

| Map style                | Spritezero file size | Spreet file size | Change |
| :----------------------- | -------------------: | ---------------: | -----: |
| osm-bright-gl-style      |               10,695 |            6,957 |   -35% |
| openstreetmap-americana  |               20,142 |           13,574 |   -33% |
| mapbox-gl-styles (basic) |               17,013 |           11,101 |   -35% |
| mapbox-gl-whaam-style    |                  553 |              372 |   -33% |
