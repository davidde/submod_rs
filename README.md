# submod_rs
Originally a rewrite of [submod_py](https://github.com/DavidDeprost/submod_py) to learn Rust,
it has since surpassed its Python ancestor in terms of features.

## Usage
```
submod [FLAGS] [OPTIONS] <filename> <seconds>

FLAGS:
    -O, --overwrite    Overwrite input file, copying the original
                       (Only do this the first time; consecutive overwrites
                       will copy the already modified input file)
    -d, --destroy      Overwrite input file, destroying the original
        --srt          Convert to srt format
        --vtt          Convert to vtt format
    -h, --help         Prints help information
    -v, --version      Prints version information

OPTIONS:
    -o, --output <filename>    File name or path where the modified file should be stored
                               Note: The file name's extension takes precedence over
                                     any --srt or --vtt flags
    -s, --start <hh:mm:ss>     Indicates at what time the modification should start
    -S, --stop <hh:mm:ss>      Indicates at what time the modification should stop

ARGS:
    <filename>    (Path to) .srt or .vtt subtitle file to modify
    <seconds>     Seconds by which to add or subtract the time encoding

```
