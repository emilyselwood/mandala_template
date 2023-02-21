# Mandala Tracing Templates

A tool to create mandala tracing templates. 

![an example quarter circle mandala tracing tempalate](./example.svg)

## Setup

Until I can get a release together with packaged binaries you will need to do the following

* [install/setup rust](https://www.rust-lang.org/tools/install)
* git clone this repo

## Running

Run the following
```
cargo run -- -p A4 -l 15 -r 10 -o test.svg -c edge
```

Help should work too to explain the options.

```
cargo run -- --help
```