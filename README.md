# openlibrary

Welcome to your new openlibrary project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with openlibrary, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd openlibrary/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

## In IC Main Network
```shell
Installing canisters...
Installing code for canister openlibrary_backend, with canister ID n24i7-kiaaa-aaaam-qbata-cai
Installing code for canister openlibrary_book, with canister ID n55ol-hqaaa-aaaam-qbatq-cai
Module hash 2e9dd3ed724f6363f82eceb9d42a0c7b9a8add4e2262989fd1f38de1b86d0090 is already installed.
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    openlibrary_backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/?id=n24i7-kiaaa-aaaam-qbata-cai
    openlibrary_book: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/?id=n55ol-hqaaa-aaaam-qbatq-cai
    store_backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/?id=mxsmr-faaaa-aaaam-qbauq-cai
```
1)book canister id：n55ol-hqaaa-aaaam-qbatq-cai

2)platform canister id：n24i7-kiaaa-aaaam-qbata-cai

3)store backend canister id：mxsmr-faaaa-aaaam-qbauq-cai