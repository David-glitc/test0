# DOT0

DOT0 is a cutting-edge toolkit designed to bring seamless zero-knowledge (zk) proof functionality to Polkadot parachains. By leveraging state-of-the-art zk libraries, such as `Arkworks`, and providing a unified crate wrapper, DOT0 offers parachain developers a robust and compatible framework for zk-proof generation, verification, and compression. This project is optimized for both `WASM` and `no_std` environments, ensuring broad compatibility and efficient runtime performance.

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
    - [ZK Global API](#zk-global-api)
    - [zkVM Runtime Environment](#zkvm-runtime-environment)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

The primary goal of DOT0 is to enable Polkadot parachains to seamlessly integrate zk-proof capabilities into their infrastructure. DOT0 achieves this by providing a zk-compatible runtime environment and a global API for proof management, offering developers out-of-the-box zk primitives, proof compression techniques, and zkVM instance hosting.

## Architecture

DOT0 is structured into two core components: the **ZK Global API** and the **zkVM Runtime Environment**.

### ZK Global API

The ZK Global API is a Polkadot runtime-compatible pallet designed to ensure global compatibility for zero-knowledge proofs across parachains. It provides:

- **Out-of-the-box constraints** for existing pallets, simplifying zk-proof integration.
- **Polynomial commitments** for efficient proof constructions.
- **Finite fields and curve abstractions** for modular cryptographic operations.
- **Constraint primitives** that support custom proof requirements.
- **Proof generation** capabilities to create zero-knowledge proofs within the runtime.
- **Proof compression** mechanisms to reduce proof size, optimizing for both on-chain and off-chain storage.
- **Proof verification** for both on-chain and off-chain applications.

### zkVM Runtime Environment

The zkVM Runtime Environment hosts a zero-knowledge virtual machine (zkVM) instance and integrates with Polkadot’s contracts API. It provides:

- A **pallet to host zkVM instances** for executing zk-proof related operations.
- **Integration with the contracts API** to enable the generation and verification of zk proofs.
- **zk-primitives for smart contracts**, allowing smart contract environments within Polkadot to natively support zk functionality.

## Installation

### Prerequisites

- Rust (latest stable version) with `no_std` support
- Polkadot SDK
- `Arkworks` and other compatible zk libraries
- `risc0-zkvm` for zkVM functionality

### Steps

1. Clone the repository:
    ```bash
    git clone https://github.com/David-glitc/DOT0.git
    cd DOT0
    ```

2. Initialize the Rust workspace:
    ```bash
    cargo build --workspace
    ```

3. Configure Polkadot SDK integration:
   Update the configurations in the workspace to align with the Polkadot SDK for WASM and `no_std` compatibility. Refer to the `config.toml` for detailed setup instructions.

## Usage

### Setting Up the ZK Global API

The ZK Global API can be used to integrate zk-proof functionalities into your Polkadot-compatible pallets. It includes prebuilt constraints, proof generation, and compression utilities. Import the ZK Global API into your pallet as follows:

```rust
use dot0::zk_global_api::{ProofGeneration, ProofVerification};
```

### zkVM Runtime Environment

To host a zkVM instance, include the zkVM Runtime Environment pallet in your runtime:

```rust
use dot0::zkvm_runtime::{ZkVMHost, zk_contract_integration};
```

Once integrated, you can use the provided APIs to create proofs, verify arbitrary code, and pass zk-primitives to the smart contract environment.

## Contributing

Contributions are welcome! Please submit pull requests for any new features, bug fixes, or documentation improvements. Before contributing, review the [contribution guidelines](CONTRIBUTING.md).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.