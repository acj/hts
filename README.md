# hts

Highlight lines of program output based on the latency between them

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
