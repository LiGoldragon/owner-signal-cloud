# meta-signal-cloud Architecture

`meta-signal-cloud` is the meta policy Signal contract for the `cloud`
component. It controls provider account registration, credential-handle
rotation, policy changes, plan preparation, and live provider plan
application.

## Authority shape

`meta-signal-cloud` is the meta authority contract for the `cloud` component. It exists because live provider mutation changes external accounts, paid resources, and public domain identity — those mutation-class verbs must be separated from the ordinary `signal-cloud` read surface by contract and socket boundary. This is a workspace generalization: a component whose state surface reflects an external resource exposes reads on the ordinary contract and mutations on the meta contract.

`PreparePlan` lives here because it mutates daemon-internal plan-store state even though it does not mutate external provider state directly. Provider-specific plan application stays outside this contract: the domain registry decides what should exist; `cloud` decides how a provider applies it.

## Boundary

The ordinary `signal-cloud` contract can observe and validate
provider-neutral desired state. This meta contract prepares, authorizes, and
applies plans because prepared plans are daemon-owned mutation intent and live
provider mutation changes external accounts, paid resources, and public domain
identity.

## Bootstrap stage

`schema/authority.ethos` is the canonical Protos Interface for the vocabulary
owned by this authority boundary. Its identities and canonical order are
sealed by `src/bootstrap_manifest.rs`; the build accepts the source only
through the strict, authority-verified bootstrap transaction and checks the
Rust projection into `src/schema/authority/generated.rs`.

The Interface is deliberately role-free. At this stage the verified producer
projects Types only: durable credential handles, policy directives, host
intent, rejection reasons, and provider-choice names. The `signal_channel!`
operations and replies in `src/lib.rs` remain the truthful handwritten wire
contract. They do not pretend to be generated Input, Output, or Refusal roles.
As Protos gains those projections, the role slots can become authoritative
without preserving this Rust implementation as a substrate.

## Public Operations

- `RegisterAccount(Registration)` binds a provider account to a credential
  handle.
- `RotateCredential(Rotation)` changes the credential handle for an existing
  provider account.
- `SetPolicy(Policy)` replaces the daemon's provider-authority policy.
- `PreparePlan(PlanPreparation)` writes a provider plan into daemon plan state.
- `PrepareHostPlan(HostPlanPreparation)` prepares host creation intent.
- `PrepareHostDestruction(HostDestruction)` prepares host destruction intent.
- `ApprovePlan(Approval)` marks a prepared plan as approved for later
  application.
- `ApplyPlan(Application)` applies a prepared plan.
- `RetireAccount(Retirement)` removes an account binding.

## Ordinary vs meta split

Per Spirit records 311 and 325 (Maximum certainty, 2026-05-23), the cloud
surface splits Mutate-class verbs onto this meta contract (privileged) and
Query-class verbs onto `signal-cloud` (public). `PreparePlan` lives here
because it mutates daemon-internal plan store state, even though it does not
mutate external provider state directly. Cloudflare and other provider states
are treated as external state the cloud daemon reflects.

This is a workspace generalization: a component whose state surface is a
reflected external resource exposes its read surface on the ordinary contract
and its mutation surface on the meta contract.

## Owns

- Secret-handle references, not secret bytes.
- Provider account policy.
- Zone allowlists.
- Capability directives.
- Meta plan approval and application records.

## Does Not Own

- Ordinary provider observation.
- Provider-neutral desired state.
- The runtime daemon's actor tree or database.
- The Criome domain registry.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Reuse public provider/domain/plan types from `signal-cloud`.
- Do not expose raw provider credential bytes.
