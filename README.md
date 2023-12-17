# Ethereum Research
This repository is the culmination of work done by [Colin Roberts](https://github.com/Autoparallel) and [Waylon Jepsen](https://github.com/0xJepsen), supported by a small grant from the Ethereum Foundation. The goal of this project is to investigate the POSEIDON hash function closely. We aim to provide a comprehensive learning resource for the Poseidon hash function with a precise mathematical intuition on its function and security assumptions. This repository also serves as a playground to attack implementations of Poseidon with small field sizes to better understand the security assumptions of the hash function.

## Repository Contents

This repository has two essential components; the first is written educational content in the form of a rust mdbook in the `journal/` directory. To run and host the journal, make sure that you have [cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html); then you can install `mdbook` with the following command:

```bash
cargo install mdbook
```

Then, you can run the journal with the following command:

```bash
mdbook serve
```

This command will host a local server on your machine that you can access at `http://localhost:3000` to view the journal. If this project is widely used and a popular resource, we will consider hosting it on a public server under some domain to make it accessible on the web. This is also the best place to start if you want to follow our learning resources to understand the Poseidon hash function and its security assumptions.

The second component of this repository is the `Poseidon/` directory, which contains a rust implementation of the Poseidon hash function. This implementation is based on the [Poseidon paper](https://eprint.iacr.org/2019/458.pdf) and the [Poseidon paper with S-Boxes](https://eprint.iacr.org/2019/458.pdf). This directory currently contains four modules:
- `attacks` serving as a playground for attacking Poseidon with small field sizes
- `home_baked_crypto` serving as one of the implementations of Poseidon inspired by the [Arnacube Implementation](https://github.com/arnaucube/poseidon-rs)
- `python_transcription` serving as a transcription of the [Poseidon python implementation](https://github.com/ingonyama-zk/poseidon-hash/tree/main/poseidon) here
- `implementations` Containing what we found to be the two most utilized rust implementations of the Poseidon hash function:
    - [neptune](https://github.com/lurk-lab/neptune)
    - [starknet_crypto](https://github.com/xJonathanLEI/starknet-rs)

These comprise a collection of at-home and production implementations, along with some examples of attacking the hash function with small field sizes. We hope this repository can serve as a learning resource for the Poseidon hash function and a place to experiment with the hash function in a safe environment.

