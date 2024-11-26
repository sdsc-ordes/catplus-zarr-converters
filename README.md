# Cat+ Zarr Converters

## About

This repository contains all the Zarr converters for the different data types in the Cat+ project (Agilent, UV, IR, etc.)
The data types are all in different formats, their data and metadata colluded together. The goal will be to convert the metadata to [an established ontology](https://github.com/sdsc-ordes/cat-plus-ontology/tree/main), and -as much as data format allow- convert the data in [Zarr array](https://zarr.readthedocs.io/en/stable/index.html).

## Tools

- synth-converter: parses a json input into a turtle output file, that conforms to the cat+ ontology

## Installation guidelines

The repo is setup with nix.

```
git clone git@github.com:sdsc-ordes/cat-plus-zarr-converters.git
cd cat-plus-zarr-converters
cargo build
```

## How to Use

You can find an example input file at `/example/1-Synth.json`

```
cargo run example/1-Synth.json 1-Synth.ttl
```

## Contribute

To be defined.

