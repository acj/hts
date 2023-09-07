# hts

Highlight lines of program output based on the latency between them

<img width="601" alt="screenshot with hts showing latency for a demo program" src="https://github.com/acj/hts/assets/27923/472e7563-94a3-4d09-bedd-abdc2447fa46">

## Getting started

### Homebrew (recommended)

```
brew install acj/taps/hts
```

### From source

You'll need a [Rust toolchain](https://rustup.rs). Clone this repository, and then:

```
cargo build
```

## Usage

```
Usage: hts [OPTIONS]

Options:
  -d, --debug                        Turn debugging information on
  -e, --echo                         Echo lines of input as they arrive. Highlighted output is shown at the end
  -m, --min-latency <MIN_LATENCY>    Don't highlight lines with latency below this threshold [default: 1ms]
  -l, --latency-unit <LATENCY_UNIT>  Show the latency between lines in the given unit. Valid units are ns, us, ms, s, m, h, d [default: ms]
  -h, --help                         Print help
  -V, --version                      Print version
```
