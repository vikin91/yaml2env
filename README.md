# yaml2env

![CI](https://github.com/vikin91/yaml2env/workflows/CI/badge.svg)

Simple tool to convert flat yaml-file into env-file with Bash variables, so that the env-file may be sourced from a shell script.

Having variables in yaml format (e.g., extracted from Hashicorp Vault using `vault read --format=yaml secret/path`), `yaml2env` allows to convert selected variables into a form that is immediately usable from a shell script. Example:

`secret.yaml`:

```yaml
USERNAME: admin
PRIV_KEY: |
  -----BEGIN OPENSSH PRIVATE KEY-----
  b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAABFwAAAAdzc2gtcn
  NhAAAAAwEAAQAAAQEAycLCAvztPnFJEWewT49dHAEK2WphtCOpfVdodT+FnW1YOCf2bUsH
  MThWTeUYgrRdL3QlkTJW7MFQ+0VqaEI1TveVzkJzPPdhi/dISdRhE6yIxcdVTtNUqPo70l
  ...
  -----END OPENSSH PRIVATE KEY-----
```

`script.sh`

```bash
#!/usr/bin/env sh -e

# Create shell-readable env-file
yaml2env --in secret.yaml --out secret.env --filter=PRIV_KEY,USERNAME

# Source the variables
source secret.env

# Use the variables
_KEY_FILE="$(mktemp)"
echo ${PRIV_KEY} > "${_KEY_FILE}"

ssh \
    -o ConnectTimeout=1 \
    -o ConnectionAttempts=1 \
    -i "${_KEY_FILE}" \
    "${USERNAME}@example.com" uname -a
```

## Installation

The binary name for yaml2env is `yaml2env`.

Archives of precompiled binaries for yaml2env are available for macOS and Linux.
Users are advised to download one of these archives.

### Downloading release

See: https://github.com/vikin91/yaml2env/releases

## Building

yaml2env is written in Rust so you'll need to grab a Rust installation in order to compile it. yaml2env compiles with Rust 1.28.0 (stable) or newer. In general, yaml2env tracks the latest stable release of the Rust compiler.

To build yaml2env:

```shell
$ git clone https://github.com/vikin91/yaml2env
$ cd yaml2env
$ cargo build --release
$ ./target/release/yaml2env --version
yaml2env 0.1.0
```

## Running tests

yaml2env is relatively well-tested, including both unit tests and integration tests.
To run the full test suite, use:

```shell
cargo test --all
```

from the repository root.
