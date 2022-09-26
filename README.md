# Substrate Node Custom

This custom example adds a pallet "TemplateModule" with two main methods: 
- setMember(club, candidate)
- removeMember(club, candidate)

Clubs are identified uniquely by u32 numbers, and candidates by T::AccountId type.

To run:
```
git clone https://github.com/labormedia/substrate-node-custom.git
git checkout autoupdate_polkadot-v0.9.28_2022-09-21_18-57-59
cargo build --release
RUST_LOG=runtime=debug ./target/release/node-template --dev -lpallet-template=debug

```

TODO: Solve the BadOrigin error when ensuring root account origin.