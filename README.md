# `tomato`

[![build](https://img.shields.io/github/workflow/status/kironono/tomato/Continuous%20integration)](https://github.com/kironono/tomato/actions/workflows/ci.yml)
[![version](https://img.shields.io/crates/v/tomato)](https://crates.io/crates/tomato)
[![license](https://img.shields.io/crates/l/tomato)](https://crates.io/crates/tomato)

Command line pomodoro timer.

## Installation

cargo install:

```
cargo install tomato
```

## Usage

```
USAGE:
    tomato [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --long-break-time <long_break_time>      Duration of long break time (minutes) [default: 15]
    -s, --short-break-time <short_break_time>    Duration of short break time (minutes) [default: 5]
    -w, --work-time <work_time>                  Duration of work time (minutes) [default: 25]
```

## License

`tomato` is distributed under the terms of the MIT license.

See the [LICENSE](LICENSE) files in this repository for more information.
