# Griffin

![Rust](https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white)
[![tests](https://github.com/ugsto/griffin/actions/workflows/tests.yml/badge.svg)](https://github.com/ugsto/griffin/actions/workflows/tests.yml)

---

> [!CAUTION]
> This application is **not** production ready. Use at your own risk.

## Overview

Griffin is a Rust-based domain name permutation and fuzzing tool designed to identify domain spoofing and similar phishing threats by generating and testing domain variants.

## Getting Started

Ensure Rust and Cargo are installed on your system before proceeding with the installation of Griffin.

### Installation

1. Clone the Griffin repository:

```sh
git clone https://github.com/ugsto/griffin.git
```

2. Navigate to the griffin directory and build the project:

```sh
cargo install --path griffin
```

### Usage

To use Griffin, simply run the compiled binary with your target domain and optional parameters:

```sh
griffin [OPTIONS] [DOMAIN]
```

#### Options:

- `-w, --workers <WORKERS>`: Set the number of worker threads.
- `-f, --fuzzers <FUZZERS>`: Specify fuzzing strategies
- `-h, --help`: Display help information.

#### Environment Variables:

You can also configure Griffin using environment variables:

- `GRIFFIN_WORKERS`: to set the number of worker threads.
- `GRIFFIN_FUZZERS`: to define fuzzing strategies, separated by commas.

#### Fuzzing strategies

| name               | implemented |
| ------------------ | ----------- |
| addition           | ✅          |
| azerty-replacement | ❌          |
| azerty-typo        | ❌          |
| bitsquatting       | ✅          |
| cyrillic           | ❌          |
| dictionary         | ❌          |
| dot-typo           | ✅          |
| homoglyph          | ❌          |
| hyphen-typo        | ✅          |
| omission           | ✅          |
| plural             | ✅          |
| qwerty-replacement | ❌          |
| qwerty-typo        | ❌          |
| qwertz-replacement | ❌          |
| qwertz-typo        | ❌          |
| repetition         | ✅          |
| tld                | ❌          |
| vowel-swap         | ❌          |

### Examples

> Running Griffin with default fuzzers for the domain "example.com":

```sh
griffin example.com
```

> Running Griffin with 32 workers and default fuzzers for the domain "example.com":

```sh
griffin -w 32 example.com
```

> Running Griffin with 32 workers and omission + plural fuzzers for the domain "example.com":

```sh
griffin -w 32 -f omission -f plural example.com
```

### Tests

To run the tests, execute the following command in the project directory:

```sh
cargo test
```

## Contributing

Contributions are warmly welcomed and greatly appreciated. Feel free to fork the repository, make your changes, and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the GNU Affero General Public License Version 3 (AGPLv3) - see the [LICENSE](LICENSE) file for details.

## Credits

The idea for this project was inspired by [dnstwist](https://github.com/elceef/dnstwist)
