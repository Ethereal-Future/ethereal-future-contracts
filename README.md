# Ethereal Future Contracts

Original Soroban contracts for registering public projects and tracking contribution commitments. The contracts are deliberately narrow: they store transparent project state and emit events, while wallets, indexing, and payout policy remain separate concerns.

## Current implementation

`ProjectRegistry` supports: project registration by an owner, project reads, contribution recording, and a monotonic total-contributed counter. Each mutation emits a typed event for indexers.

## Development

Requires Rust and the Soroban toolchain. The contract crate is under `contracts/project-registry`. Build and test commands will be added through issue-scoped work; no deployment keys belong in this repository.

## Contribution workflow

Open an issue first, create a branch from `develop`, and target `develop` in pull requests. See `CONTRIBUTING.md`.

## License

MIT. See `LICENSE`.
