# hts

Highlight lines of program output based on the latency between them

<img width="521" alt="screenshot with hts showing latency for a demo program" src="https://github.com/acj/hts/assets/27923/ba91c337-6c94-4ef1-a964-b7b7ab8f6f59">

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
  -n, --no-echo                      Do not echo lines of input as they arrive; show highlighted output after the command finishes
  -m, --min-latency <MIN_LATENCY>    Don't highlight lines with latency below this threshold [default: 1ms]
  -l, --latency-unit <LATENCY_UNIT>  Show the latency between lines in the given unit. Valid units are ns, us, ms, s, m, h, d [default: ms]
  -h, --help                         Print help
  -V, --version                      Print version
```

Try piping a command into `hts`:

```
ping google.com -c 4 | hts
```

If you built hts from source, try it again with highlighting:

```
cargo clean && cargo build 2>&1 | hts
```
