# Spreet: create spritesheets from SVGs

Spreet is a command-line interface (CLI) that can create [spritesheets](https://en.wikipedia.org/wiki/Spritesheet) from a directory of SVG images. This is useful when you're creating [MapLibre](https://maplibre.org/) or [Mapbox](https://docs.mapbox.com/) vector web maps, where cartographic stylesheets require that [icons be loaded from a spritesheet](https://maplibre.org/maplibre-gl-js-docs/style-spec/sprite/).

_Spreet_ (also _spreit_, _spret_, _sprit_) is the [Scots](https://en.wikipedia.org/wiki/Scots_language) word for a sprite, the fairy-like creature from  Western folklore.

## Installation

If you use [Homebrew](https://brew.sh/) on MacOS or Linux you can install Spreet from the command-line:

```
brew install flother/taps/spreet
```

(You can review [the code run by the formula](https://github.com/flother/homebrew-taps/blob/master/spreet.rb) before you install.)

Otherwise, pre-built binaries are provided for MacOS, Linux, and Windows. Visit the [releases](https://github.com/flother/spreet/releases) page to download the latest version.

## Usage

```
USAGE:
    spreet [OPTIONS] <INPUT> <OUTPUT>

ARGS:
    <INPUT>     A directory of SVGs to include in the spritesheet
    <OUTPUT>    Name of the file in which to save the spritesheet

OPTIONS:
    -h, --help             Print help information
    -r, --ratio <RATIO>    Set the output pixel ratio [default: 1]
        --retina           Set the pixel ratio to 2 (equivalent to `--ratio=2`)
    -V, --version          Print version information
```
