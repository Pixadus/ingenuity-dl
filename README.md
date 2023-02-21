# Ingenuity-DL
> Automatically download images from the Mars Ingenuity helicopter and compile them into a video.

<p align="center">
<img align="center" src="https://github.com/Pixadus/ingenuity-dl/raw/main/docs/sol-403.gif">
</p>
<p align="center">
<em>gif of the sol 403 flight</em>
</p>


<hr/>

**Ingenuity-DL** works with the NASA raw images API to download images from the Ingenuity Mars helicopter, and convert them into a GIF. 

## Usage

```
USAGE:
    ingenuity-dl [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --save       Save images instead of deleting them
    -V, --version    Prints version information

OPTIONS:
    -f, --fps <FPS>       FPS the gif should use. Default 3.
    -o, --output <OUT>    Folder and/or file to write the output to
    -d, --sol <SOL>       Retrieve images from a specific Sol (defaults to latest)
```
For example, to compile a gif of the sol 403 flight, with output file `sol-403.gif` and framerate of 7,

```
ingenuity-dl -d 403 -o sol-403.gif -f 7
```

If there was no flight on the specified sol, an error will be returned. A comprehensive list of all flights Ingenuity has taken is available on [this page](https://en.wikipedia.org/wiki/List_of_Ingenuity_flights).

## Installation

*Prerequisites*: Rust + Cargo must be installed. See [the Rust installation guide](https://www.rust-lang.org/tools/install) for more information. 

```
cargo install ingenuity-dl
```

Once this is done, the application will be available as `ingenuity-dl` in the command line. 
