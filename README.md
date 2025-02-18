# Cat+ Zarr Converters

## About

This repository contains all the Zarr converters for the different data types in the Cat+ project (Agilent, UV, IR, etc.)
The data types are all in different formats, their data and metadata colluded together. The goal will be to convert the metadata to [an established ontology](https://github.com/sdsc-ordes/catplus-ontology/tree/main), and -as much as data format allow- convert the data in [Zarr array](https://zarr.readthedocs.io/en/stable/index.html).

## Tools

### synth-converter
The Synth-converter parses a json input into an rdf graph and serializes the graph to either turtle or jsonld.
It expects the input to conform to the cat+ ontology and the struct `synth-converter/src/batch.rs`. An example input file is provided in `example/1-Synth.json`.

#### Usage

The `synth-converter` has three parameters:

- inputfile: path to input file (relative to top level of the repo or absolute)
- outputfile: path to output file (relative to top level of the repo or absolute)
- format: default is "ttl", the other option is jsonld

The `synth-converter` turns the inputfile into a rdf graph and serilizes it to either turtle or jsonld. The serialization is written to an outputfile.

```
just run example/1-Synth.json output.ttl
just run example/1-Synth.json output.json --format jsonld
```

### Architecture

The json input is read with `serde_json`: the transformation of fields is described in the struct `synth-converter/src/batch.rs`

The graph is build via `synth-converter/src/graph/graph_builder.rs` and uses `sophia_rs`. Besides `rdf` and `xsd` that have build in namespaces in `sophia_rs`, all namespaces and terms are provided in `synth-converter/src/graph/namespaces` as constants. This makes the code more readable and also ensures that the rdf iris and namespaces are controlled and spelt correctly.
Graph serializers and parsers are provided in `synth-converter/src/rdf`. The turtle serializer there is needed for the test.
The conversion is done in the public crate `synth-converter/src/convert.rs`

### Shacl Validation

The rdf graph confirms to the cat+ ontology: https://github.com/sdsc-ordes/catplus-ontology. Currently rust offeres no Shacl Validation Library, but once such a library exists, it would make sense to add a Shacl Validation.

TheShacl Validation can be done manually here: https://www.itb.ec.europa.eu/shacl/any/upload

## Installation guidelines

The repo is setup with nix.

```
git clone git@github.com:sdsc-ordes/catplus-zarr-converters.git
cd catplus-zarr-converters
cargo build
```

From here on you can work with a just file:

The rust commands can be started via a justfile:

```
just --list
Available recipes:
    build *args                      # Build the synth-converter.
    default                          # Default recipe to list all recipes.
    nix-develop *args                # Enter a Nix development shell.
    run input_file output_file *args # Run the synth-converter.
    test *args                       # Test the synth-converter.
    fmt *arg                         # Format the synth-converter.
```

### Tests

Run the tests with `just test`: only integration tests have been integrated that ensure that the serialized graph in turtle is isomorphic to an expected turtle serialization per valid substructure of the input data: this substructures are action that occur in the synthesis process.

### Contribute

The repo is a Poc under heavy development and not yet ready to take contributions.
