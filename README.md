# Scrambler

Scrambler is a simple command line tool that allows you to scramble text while preserving special characters, numbers, and white spaces.
The tool also provides a feature to exclude certain words from scrambling using regular expressions. (Next Version)

## Installation

```bash

cargo install scrambler

```

## Usage

```bash

echo "some text" | scrambler

```

## Examples

Scramble the text in input.txt and save the output to output.txt.

```bash

$ echo 'The brown fox jumps over the lazy dog' | scrambler

Uqd vruoj auj splhn hxse cth elci csn

```

## Limitations

Currently only supports ASCII characters.

## License

This project is licensed under the MIT license.
