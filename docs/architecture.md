# Contract architecture

The contract repository owns durable on-chain state only. The frontend owns presentation, the backend owns read-model composition and contribution-intent orchestration, and an indexer can consume Soroban events without making the contract depend on off-chain services.

## Security boundaries

- `register` requires the declared owner to authorize the transaction.
- `set_active` requires the stored owner to authorize the update.
- `record_contribution` requires contributor authorization and rejects non-positive amounts.
- No secret key, token transfer, or off-chain trust assumption is stored by the contract.

Future changes should document storage compatibility, authorization behavior, event schema, and failure modes before implementation.
