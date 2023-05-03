# ISO 20022 Software Developer Kit (SDK)

The `iso-20022-sdk` is a Rust library for working with the <a href="https://iso20022.org" target="_blank">ISO 20022</a> Universal financial industry message scheme.


> #
> Need ISO-20022 Integrations? [Contact us](mailto:contact@emergent.financial) for additional information about our software systems and development services.
> #

## Install the SDK Library

```toml
[dependencies]
iso-20022-sdk = { version = "0.1.0" }

```


## Features

By default, `iso-20022-sdk` includes `nvlp`, `head` and `dsig` features, which imports `iso-20022-nvlp`, `iso-20022-head` and `iso-20022-dsig` respectively.

Documents, e.g. `remt.001.001.01`, are conditionally compiled and need to be added individually, either as a business domain or message set feature, e.g.

```toml
[dependencies]
iso-20022-sdk = { version = "0.1.0", features = ["remt"] }
```

Now you can create a `Document` from the `remt.001.001.01` namespace:

```rust
use iso_20022_sdk::Document;

let doc = Document::from_namespace("remt.001.001.01")?;

```

### Business Domains

To include messages relevant only to the `payments` business domain, add the `payments` feature to your `Cargo.toml`:

```toml
[dependencies]
iso-20022-sdk = { version = "0.1.0", features = ["payments"] }

```

> *Using the payments features will include all message sets in the payments business domain.*
> 
```toml
payments = ["acmt", "auth", "acmt", "admi", "camt", "pacs", "pain", "reda", "remt"]
```
> 
> Available `business domain` features
> 
> - `payments`
> - `securities`
> - `trade`
> - `cards`
> - `fx`


### Message Sets

Each message set, e.g. `acmt`, has its own Rust library, e.g. `iso-20022-acmt`, which can be conditionally compiled using the `Cargo.toml` **features** flag corresponding to the message set. 


```toml
[dependencies]
iso-20022-sdk = { version = "0.1.0", features = ["acmt", "admi"] }

```


> Available `message set` features
>
> - `acmt`
> - `admi`
> - `auth`
> - `caaa`
> - `caad`
> - `caam`
> - `cafc`
> - `cafm`
> - `cafr`
> - `cain`
> - `camt`
> - `canm`
> - `casp`
> - `casr`
> - `catm`
> - `catp`
> - `colr`
> - `fxtr`
> - `pacs`
> - `pain`
> - `reda`
> - `remt`
> - `secl`
> - `seev`
> - `semt`
> - `sese`
> - `setr`
> - `tsin`
> - `tsmt`
> - `tsrv`

<hr/>

Copyright 2023 Emergent Financial, LLC - All Rights Reserved