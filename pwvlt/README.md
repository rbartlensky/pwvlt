# pwvlt

This crate provides a way to access your local keyring and your Nitrokey
through a shared trait (`Backend`).


Currently this only works on Linux-based systems since we are using
the [`SecretService`](https://crates.io/crates/secret-service) crate to query
the local keyring storage.
