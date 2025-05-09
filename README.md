![banner](https://github.com/iotaledger/product-core/raw/HEAD/.github/banner.png)

<p align="center">
  <a href="https://discord.gg/iota-builders" style="text-decoration:none;"><img src="https://img.shields.io/badge/Discord-9cf.svg?logo=discord" alt="Discord"></a>
  <a href="https://github.com/iotaledger/identity.rs/blob/HEAD/LICENSE" style="text-decoration:none;"><img src="https://img.shields.io/github/license/iotaledger/identity.rs.svg" alt="Apache 2.0 license"></a>
  <img src="https://deps.rs/repo/github/iotaledger/identity.rs/status.svg" alt="Dependencies">
  <a href='https://coveralls.io/github/iotaledger/identity.rs?branch=main'><img src='https://coveralls.io/repos/github/iotaledger/identity.rs/badge.svg?branch=main' alt='Coverage Status' /></a>

</p>

---

## About

This repository gathers resources shared by IOTA products.

## Build

To build the Rust workspace you need to have `Rust` and `Cargo` installed.
You can find installation instructions in
the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

We recommend that you update the Rust compiler to the latest stable version first:

```shell
rustup update stable
```

The [iota_interaction_ts](bindings/wasm/iota_interaction_ts) folder contains
NodeJS and Web Javascript packages that can be build as follows:

```
> # In folder bindings/wasm/iota_interaction_ts
> npm install
> npm run build
```

## Usage

This Repository only provides libraries that can be used as dependencies in other
IOTA products:

* [iota_interaction](./iota_interaction)<br>
  Platform Agnostic Iota Interaction Interfaces
    * [iota_interaction_rust](./iota_interaction)<br>
      `iota_interaction` implementation for non-wasm targets using the IOTA
      Rust SDK
    * [iota_interaction_ts](./iota_interaction)<br>
      `iota_interaction` implementation for wasm32 targets (only NodeJS and Web) using the IOTA
      TypeScript SDK
* [product_common](./product_common)<br>
  Shared Rust code used in other IOTA product repositories  

## Issues

See the [open issues](https://github.com/iotaledger/product-core/issues) for a full list of proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the Apache License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>