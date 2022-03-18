# yaml2env

![CI](https://github.com/vikin91/yaml2env/workflows/CI/badge.svg)

Simple tool to convert flat yaml file into an env file with shell variables, so that the env file may be sourced from a shell script.
Can be used when secrets from HashiCorp Vault (e.g., fetched using `vault read --format=yaml secret/path`) must be provided to a bash script.
This is the author's Rust learning project :crab:

## Example

Example input (`secret.yaml`):

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

Command:

```sh
yaml2env --in secret.yaml --out secret.env
```

Example output (`secret.env`):

```sh
#!/usr/bin/env sh

USERNAME=$(cat << '_EOF'
admin
_EOF
)

PRIV_KEY=$(cat << '_EOF'
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAABFwAAAAdzc2gtcn
NhAAAAAwEAAQAAAQEAycLCAvztPnFJEWevT49dHAEK2WphtCOpfVdodT+FnW1YOCf2bUsH
MThWTeUYgrRdL3QlkTJW7MFQ+0VqaEI1TveVzkJzPPdhi/dISdRhE6yIxcdVTtNUqPo70l
...
-----END OPENSSH PRIVATE KEY-----

_EOF
)

```

Example application scenario:

```bash
#!/usr/bin/env sh -e

IN="${1:-secret.yaml}"
# or
# vault read --format=yaml secret/path > secret.yaml

# Create shell-readable env-file
yaml2env --in "$IN" --out secret.env --filter=PRIV_KEY,USERNAME

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

Archives of pre-compiled binaries for yaml2env are available for macOS and Linux.
Users are advised to download one of these archives.

### Downloading released binaries

- [Latest release](https://github.com/vikin91/yaml2env/releases/latest)
- [All releases](https://github.com/vikin91/yaml2env/releases)

```sh
VERSION="v0.1.1"
ARCH="x86_64-apple-darwin"

# Download
curl -OSsL "https://github.com/vikin91/yaml2env/releases/download/${VERSION}/yaml2env-${ARCH}"
curl -OSsL "https://github.com/vikin91/yaml2env/releases/download/${VERSION}/yaml2env-${ARCH}.sha256.txt"
mv "yaml2env-${ARCH}" yaml2env

# Verify and Install
shasum -c "yaml2env-${ARCH}.sha256.txt" && mv yaml2env $HOME/bin/yaml2env
```

## Building

yaml2env is written in Rust so you'll need to grab a Rust installation in order to compile it. yaml2env compiles with Rust 1.28.0 (stable) or newer. In general, yaml2env tracks the latest stable release of the Rust compiler.

To build yaml2env:

```shell
$ git clone https://github.com/vikin91/yaml2env
$ cd yaml2env
$ cargo build --release
$ ./target/release/yaml2env --version
yaml2env 0.1.1
```

## Running tests

yaml2env is relatively well-tested, including both unit tests and integration tests.
To run the full test suite, use:

```shell
cargo test --all
```

from the repository root.
