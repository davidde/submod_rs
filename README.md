# submod_rs
Originally a rewrite of [submod_py](https://github.com/DavidDeprost/submod_py) to learn Rust,
it has since surpassed its Python ancestor in terms of features.

## Usage
```
submod 1.1.0
Modify the time encoding of .srt or .vtt subtitle files.
By default, submod generates a new output file, not overwriting the input.

USAGE:
    submod [FLAGS] [OPTIONS] <file> <seconds>

FLAGS:
    -o, --overwrite    Overwrite input file, destroying the original
    -O, --overname     Overwrite input file, renaming the original
                       (Only necessary on first call; consecutive `overnames` on same input
                       will NOT rename the input since this would overwrite the 'original' input)
        --srt          Convert to srt format
        --vtt          Convert to vtt format
    -h, --help         Prints help information
    -v, --version      Prints version information

OPTIONS:
        --out <filename>      Specify file name or path to store the output file
    -s, --start <hh:mm:ss>    Specify at what time the modification should start
    -S, --stop <hh:mm:ss>     Specify at what time the modification should stop

ARGS:
    <file>       File name or path to the subtitle file to modify
    <seconds>    Seconds by which to add or subtract the time encoding
```
