<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata DCAP Attestation
[![Automata DCAP Attestation](https://img.shields.io/badge/Power%20By-Automata-orange.svg)](https://github.com/automata-network)

## Table of Contents
- [Summary](#summary)
- [Automata DCAP Attestation on EVM](#automata-dcap-attestation-on-evm)
- [Automata DCAP Attestation on Solana](#automata-dcap-attestation-on-solana)

---

## Summary

This repo serves as a code base for the Intel Data Center Attestation Primitive (DCAP) Web3-based Quote Verification program for both EVM and Solana.

Currently on the EVM, users can verify DCAP quotes with either full on-chain execution or using SNARK proofs by executing [DCAP zkVM Programs](https://github.com/automata-network/tdx-attestation-sdk/tree/main/zk) on zkVMs, such as RiscZero or Succinct SP1. We plan to add support for more zkVMs in the future.

The Solana program currently supports only SNARK proof verification.

### zkProgram Identifiers

These identifiers are required parameters for SNARK proof verifications, to show that the proofs are generated by the intended zkVM Program.

The [ImageID](https://dev.risczero.com/terminology#image-id) currently for the DCAP RiscZero Guest Program is `c2eafe1ba01610f3b71281f9dd3280b33d97370bb68d3ada2925d391be245e10`.

The [VKEY](https://docs.succinct.xyz/verification/onchain/solidity-sdk.html?#finding-your-program-vkey) currently for the DCAP SP1 Program is
`004be684aaf90b70fb2d8f586ec96c36cee5f6533850b14e8b5568f4dbf31f8e`.

---

## Automata DCAP Attestation on EVM

It consists of three smart contracts:

- PCCS Router: A central contract to read collaterals from [`automata-on-chain-pccs`](https://github.com/automata-network/automata-on-chain-pccs)

- Automata DCAP Attestation: This is the entrypoint contract for users to submit a quote to be verified. This contract parses the Quote header to identify the version, which then forwards the quote to the respective QuoteVerifier contract. 

- Quote Verifier(s): This contract provides the full implementation to verify a given quote specific to its version. This contract is intended to be called only from the Automata DCAP Attestation contract.

### On-Chain vs SNARK Attestations

Automata DCAP Attestation contract implements two attestation methods available to users. Here is a quick comparison:

|  | On-Chain | Groth16 Proof Verification with RiscZero v1.2.1 | Groth16 Proof Verification with SP1 V4 | Plonk Proof Verification with SP1 V4| 
| --- | --- | --- | --- | --- |
| Quote Verification Time | Instant | Proving takes <2 minutes, instant verification | Proving takes ~3 minutes, instant verification  | Proving takes ~5 minutes, instant verification |
| Gas Cost | ~4-5M gas (varies by collateral size) | 360k gas | 333k gas | 419k gas |
| Execution | Runs fully on-chain | Execution proven by remote prover Bonsai | Execution proven by the SP1 Network | Execution proven by the SP1 Network |

### Deployment

> ℹ️ **Note**: 
>
> The deployment addresses shown here are currently based on the latest [changes](https://github.com/automata-network/automata-dcap-attestation/pull/6) made.
>
> To view deployments on the previous version (will be deprecated soon), you may refer to this [branch](https://github.com/automata-network/automata-dcap-attestation/tree/v0).

#### Testnet

| Contract | Network | Address |
| --- | --- | --- |
| `PCCSRouter.sol` | Automata Testnet | [0x3095741175094128ae9F451fa3693B2d23719940](https://explorer-testnet.ata.network/address/0x3095741175094128ae9F451fa3693B2d23719940) |
|  | Ethereum Sepolia | [0xfFC62c8851F54723206235E24af1bf10b9ea1d47](https://sepolia.etherscan.io/address/0xfFC62c8851F54723206235E24af1bf10b9ea1d47) |
|  | Ethereum Holesky | [0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5](https://holesky.etherscan.io/address/0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5) |
|  | Base Sepolia | [0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5](https://sepolia.basescan.org/address/0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5) |
|  | OP Sepolia | [0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5](https://sepolia-optimism.etherscan.io/address/0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5) |
|  | World Sepolia | [0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5](https://worldchain-sepolia.explorer.alchemy.com/address/0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5) |
|  | Arbitrum Sepolia | [0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5](https://sepolia.arbiscan.io/address/0x729E3e7542E8A6630818E9a14A67e0Cb7008a5E5) |
| `AutomataDcapAttestationFee.sol` | Automata Testnet | [0x6D67Ae70d99A4CcE500De44628BCB4DaCfc1A145](https://explorer-testnet.ata.network/address/0x6D67Ae70d99A4CcE500De44628BCB4DaCfc1A145) |
|  | Ethereum Sepolia | [0xE28ea4E574871CA6A4331d6692bd3DD602Fb4f76](https://sepolia.etherscan.io/address/0xE28ea4E574871CA6A4331d6692bd3DD602Fb4f76) |
|  | Ethereum Holesky | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://holesky.etherscan.io/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | Base Sepolia | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://sepolia.basescan.org/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | OP Sepolia | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://sepolia-optimism.etherscan.io/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | World Sepolia | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://worldchain-sepolia.explorer.alchemy.com/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | Arbitrum Sepolia | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://sepolia.arbiscan.io/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
| `V3QuoteVerifier.sol` | Automata Testnet | [0x03F3082cC6521179b30Bccf92561ea0576931Ffc](https://explorer-testnet.ata.network/address/0x03F3082cC6521179b30Bccf92561ea0576931Ffc) |
|  | Ethereum Sepolia | [0x6E64769A13617f528a2135692484B681Ee1a7169](https://sepolia.etherscan.io/address/0x6E64769A13617f528a2135692484B681Ee1a7169) |
|  | Ethereum Holesky | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://holesky.etherscan.io/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | Base Sepolia | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://sepolia.basescan.org/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | OP Sepolia | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://sepolia-optimism.etherscan.io/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | World Sepolia | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://worldchain-sepolia.explorer.alchemy.com/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | Arbitrum Sepolia | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://sepolia.arbiscan.io/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
| `V4QuoteVerifier.sol` | Automata Testnet | [0x72221D7D8eB8949383404B1d1027E5eBd39fE53C](https://explorer-testnet.ata.network/address/0x72221D7D8eB8949383404B1d1027E5eBd39fE53C) |
|  | Ethereum Sepolia | [0x90c14Bd25744d8b1E3971951BD56BfFf24dC053A](https://sepolia.etherscan.io/address/0x90c14Bd25744d8b1E3971951BD56BfFf24dC053A) |
|  | Ethereum Holesky | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://holesky.etherscan.io/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | Base Sepolia | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://sepolia.basescan.org/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | OP Sepolia | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://sepolia-optimism.etherscan.io/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | World Sepolia | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://worldchain-sepolia.explorer.alchemy.com/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | Arbitrum Sepolia | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://sepolia.arbiscan.io/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |

#### Mainnet

| Contract | Network | Address |
| --- | --- | --- |
| `PCCSRouter.sol` | Automata Mainnet | [0x722525B96b62e182F8A095af0a79d4EA2037795C](https://explorer.ata.network/address/0x722525B96b62e182F8A095af0a79d4EA2037795C) |
|  | Ethereum Mainnet | [0x09bBC921be046726bb5b694A49888e4e2e7AA9C3](https://etherscan.io/address/0x09bBC921be046726bb5b694A49888e4e2e7AA9C3) |
|  | Base Mainnet | [0x722525B96b62e182F8A095af0a79d4EA2037795C](https://basescan.org/address/0x722525B96b62e182F8A095af0a79d4EA2037795C) |
|  | OP Mainnet | [0x722525B96b62e182F8A095af0a79d4EA2037795C](https://optimistic.etherscan.io/address/0x722525B96b62e182F8A095af0a79d4EA2037795C) |
|  | World Mainnet | [0x09bBC921be046726bb5b694A49888e4e2e7AA9C3](https://worldchain-mainnet.explorer.alchemy.com/address/0x09bBC921be046726bb5b694A49888e4e2e7AA9C3) |
|  | Arbitrum Mainnet | [0x722525B96b62e182F8A095af0a79d4EA2037795C](https://arbiscan.io/address/0x722525B96b62e182F8A095af0a79d4EA2037795C) |
| `AutomataDcapAttestationFee.sol` | Automata Mainnet | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://explorer.ata.network/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | Ethereum Mainnet | [0xE26E11B257856B0bEBc4C759aaBDdea72B64351F](https://etherscan.io/address/0xE26E11B257856B0bEBc4C759aaBDdea72B64351F) |
|  | Base Mainnet | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://basescan.org/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | OP Mainnet | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://optimistic.etherscan.io/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
|  | World Mainnet | [0xE26E11B257856B0bEBc4C759aaBDdea72B64351F](https://worldchain-mainnet.explorer.alchemy.com/address/0xE26E11B257856B0bEBc4C759aaBDdea72B64351F) |
|  | Arbitrum Mainnet | [0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246](https://arbiscan.io/address/0xaEd8bF5907fC8690b1cb70DFD459Ca5Ed1529246) |
| `V3QuoteVerifier.sol` | Automata Mainnet | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://explorer.ata.network/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | Ethereum Mainnet | [0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96](https://etherscan.io/address/0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96) |
|  | Base Mainnet | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://basescan.org/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | OP Mainnet | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://optimistic.etherscan.io/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
|  | World Mainnet | [0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96](https://worldchain-mainnet.explorer.alchemy.com/address/0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96) |
|  | Arbitrum Mainnet | [0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1](https://arbiscan.io/address/0x4613038C93aF8963dc9E5e46c9fb3cbc68724df1) |
| `V4QuoteVerifier.sol` | Automata Mainnet | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://explorer.ata.network/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | Ethereum Mainnet | [0xC86EE37Ee5030B9fF737F3E71f7611Abf5dfD9B7](https://etherscan.io/address/0xC86EE37Ee5030B9fF737F3E71f7611Abf5dfD9B7) |
|  | Base Mainnet | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://basescan.org/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | OP Mainnet | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://optimistic.etherscan.io/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |
|  | World Mainnet | [0xC86EE37Ee5030B9fF737F3E71f7611Abf5dfD9B7](https://worldchain-mainnet.explorer.alchemy.com/address/0xC86EE37Ee5030B9fF737F3E71f7611Abf5dfD9B7) |
|  | Arbitrum Mainnet | [0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2](https://arbiscan.io/address/0xdE13b52a02Bd0a48AcF4FCaefccb094b41135Ee2) |


---

## Automata DCAP Attestation on Solana

### Overview

The following diagram illustrates an overview of the execution flow of the DCAP Solana Program.

![DCAP Solana Diagram](./docs/images/DCAP%20Solana%20Diagram.jpg)

1. Invokes the `CreateDcapOutputAccount` instruction on the DCAP Program.
2. Reads the current count from DCAP Counter, which is used as seed to derive the address of the `VerifiedOutput` PDA.
3. Writes data to the `VerifiedOutput` PDA.
4. Increments the current count in DCAP Counter, which completes the `CreateDcapOutputAccount` instruction.
5. Invokes the `VerifyDcapProof` instruction on the DCAP Program.
6. Reads the output from the provided `VerifiedOutput` PDA address.
7. The output is pre-processed and converted into a Groth16 public input, then submitted along with the proofs to be verified with the corresponding zkVM verifier program.
8. Updates the data in `VerifiedOutput` PDA to indicate the status showing successful verification.
9. Downstream programs consume the data directly from the `VerifiedOutput` PDA.

### zkVM Verifier Programs

Depending on which zkVM programs that the user has chosen, the Automata DCAP Solana Program sends the processed `VerifiedOutput` along with proofs to one of the following programs:

- [RiscZero Groth16 Verifier](https://github.com/risc0/risc0-solana/blob/main/solana-verifier/programs/groth_16_verifier/src/lib.rs), this is a general-purpose Groth16 Verifier built by RiscZero that can be called by any Solana programs to perform Groth16 Verifications.

- [DCAP SP1 Solana Program](./solana/programs/dcap-sp1-solana-program/), this is a wrapper verifier program to be called only by the Automata DCAP Solana Program because it hardcodes the vkey. This program imports the [SP1 Solana Library](https://github.com/succinctlabs/sp1-solana/blob/master/example/program/src/lib.rs).

### Deployment

The DCAP Solana Program and Counter account have both been deployed to `devnet` at:

- DCAP Program: `DcapE9GZZ2KSu6udeW1pVdmqBAHP9NMBLBrxUUYdw1Qk`
- DCAP Counter: `DcapH8Bt1y6MQHE1hR2Rp1WEBeWfog2Kh9UxtG8UMaNu`
