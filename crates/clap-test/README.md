# clap

<https://docs.rs/clap/latest/clap/index.html>

## Usage

```bash
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s

$ target/debug/claptest --help
Simple program to greet a person

Usage: claptest [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

