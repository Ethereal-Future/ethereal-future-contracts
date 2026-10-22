# Viridian Contracts

Viridian Contracts is an original Soroban workspace for recording transparent project goals and contribution commitments on Stellar. It stores public project state and emits events, while wallet UX, indexing, evidence, and payout policy remain separate concerns.

## Current implementation: `ProjectRegistry`

- Registers multiple projects under numeric identifiers.
- Requires the declared owner to authorize registration.
- Stores a project name, goal, owner, raised amount, and active flag.
- Records positive contribution amounts after contributor authorization.
- Allows the owner to pause or resume a project.
- Emits `project`, `status`, and `contribution` events for indexers.

## Contract boundary

The registry does not transfer tokens, custody funds, or declare that an off-chain payment is final. It records an auditable commitment signal. Any payout, refund, or dispute mechanism should be a separate audited change with its own threat model.

## Architecture

```text
Frontend -> Backend read model -> Soroban RPC -> ProjectRegistry
                                      |
                                      +-> events -> indexer -> read model
```

## Roadmap

- Add milestone evidence hashes and event history.
- Add controlled project metadata updates.
- Add generated clients shared with the backend repository.
- Add a separately reviewed payout adapter rather than coupling custody to registry storage.

## Contribution workflow

Open an issue before contract work, include authorization and storage reasoning, create a branch from `develop`, and submit a focused pull request into `develop`. Never commit secret keys or deployed credentials. Drips recognition is controlled by the applicable program and maintainers.

## License

MIT. See `LICENSE`.
