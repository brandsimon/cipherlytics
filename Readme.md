Cipherlytics
------------

Cipherlytics is a collection of tools to analyze ciphertexts.
Currently supported are the classical analysis methods: frequency-analysis, kasiski examination and showing the min and max values.

# Build and run

To build and run the app:
```
cargo build --release
./target/release/cryptolytics --help
```

## Examples

```
# To get the min and max 16-bits from a file:
cryptolytics --bytes 2 min_max FILE

# Count occurences of every 4-bytes word
cryptolytics --bytes 4 frequency_analysis FILE

# Show every duplicate word with a min length of 10 bytes
cryptolytics kasiski_examination --min-length 10 FILE

# Skip the first N bytes for a analysis
cryptolytics --skip-first N ...

# Keep only every KEEP_EVERY-th BYTES-len bytes
cryptolytics --keep-every KEEP_EVERY --bytes BYTES ...
```

##  Exit codes

0. Success
1. Error during execution
2. Error parsing parameters
