# Token price feed by Orcales for custome pallet OR Ink contracts 

<p align="center">
  <img src="/docs/media/sub.gif">
</p>

Substrate is a next-generation framework for blockchain innovation 🚀.

The custom pallet `Pallet-Offchain-worker` facilitates the submission of token prices by oracles through signed or unsigned transactions. This functionality is useful in decentralized exchanges (DEX) and other applications that require access to the average token price.  

The process works as follows:  

1. `Oracles submit token prices`: Multiple oracles can submit token prices through the Pallet Offchain-worker. These prices can be for various tokens in the system.  

2. `Average calculation`: The Pallet Offchain-worker calculates the average of all the submitted prices. This ensures that the resulting price reflects a consensus among the oracles.  

3. `Accessible price data`: The calculated average price is made accessible for consumption in various components of the system, such as the DEX logic or custom pallets. Smart contracts can also make use of this average price for their operations.  

By implementing the Pallet Offchain-worker in your Substrate-based blockchain, we have enabled the integration of oracles and the retrieval of reliable and consensus-based token price data. This can be essential for the proper functioning of decentralized exchanges and other applications that rely on accurate pricing information.  

## Getting Started

Head to [docs.substrate.io](https://docs.substrate.io) and follow the [installation](https://docs.substrate.io/install/) instructions.
Then try out one of the [tutorials](https://docs.substrate.io/tutorials/).

## Community & Support

Join the highly active and supportive community on the [Susbstrate Stack Exchange](https://substrate.stackexchange.com/) to ask questions about use and problems you run into using this software.
Please do report bugs and [issues here](https://github.com/paritytech/substrate/issues) for anything you suspect requires action in the source. 

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](docs/CONTRIBUTING.adoc).
In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in [`docs/SECURITY.md`](docs/SECURITY.md).

## License

- Substrate Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- Substrate Client (`/client/*` / `sc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using Substrate to create feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to Substrate's core logic (e.g. Substrate's internal consensus, crypto or database code) to be contributed back so everyone can benefit.

