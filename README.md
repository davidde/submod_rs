# submod_rs
This command line tool enables you to manually correct the timing of subtitles.  
These corrections are **permanent** modifications to the subtitle files.
This means they are *not* lost when playback ends, unlike media player corrections.
This can be really useful for niche movies/series that don't have many correct subtitles.

Originally a rewrite of [submod_py](https://github.com/DavidDeprost/submod_py) to learn Rust,
it has since surpassed its Python ancestor in terms of features.

## Installation
* If you do not have the [Rust programming language](https://doc.rust-lang.org/book/foreword.html)
  installed on your system, [install Rust](https://www.rust-lang.org/tools/install) first.

* Then, it's a simple install with cargo:
    ```bash
    git clone https://github.com/DavidDeprost/submod_rs.git
    cargo install --path ./submod_rs
    ```

## Usage
```
submod 1.1.0
Modify the time encoding of .srt or .vtt subtitle files.
By default, submod generates a new output file, without overwriting the input.

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
                              Use ':' to separate hours, minutes and seconds, like so:
                              hh:mm:ss to specify hours, minutes and seconds
                                 mm:ss to only specify minutes and seconds
                                    ss to only specify seconds

ARGS:
    <file>       File name or path to the subtitle file to modify
    <seconds>    Seconds by which to add or subtract the time encoding
```

## Examples
* Delay all subtitles by 0.5 seconds:
    ```bash
    $ submod 'Humans S03E01 Episode 1.en.srt' +0.5
    Success.
     Output:   Humans S03E01 Episode 1.en__[+0.50_Sec+].srt
    ```

* To directly overwrite the input subtitle file, so you don't need to manually rename it,
  simply add the `-o` or `-O` flag:
    ```bash
    $ submod 'Humans S03E01 Episode 1.en.srt' +0.5 -o
    Success.
     The input file was overwritten.
     Output:   Humans S03E01 Episode 1.en.srt
    ```
    ```bash
    $ submod 'Humans S03E01 Episode 1.en.srt' +0.5 -O
    Success.
     The input file was renamed to `Humans S03E01 Episode 1.en__[Original].srt`.
     Output:   Humans S03E01 Episode 1.en.srt
    ```

* To display the subtitles 2 seconds earlier, starting from the 10th minute to the end:
    ```bash
    $ submod 'Humans S03E01 Episode 1.en.srt' -2 -s 10:00
    Success.
     Output:   Humans S03E01 Episode 1.en__[-2.00_Sec-].srt
    ```
  The second `-` sign in `[-2.00_Sec-]` indicates the file was only partially modified;  
  this indicates the use of `-s` or `-S` flags. It will be `+` when those flags aren't used.
