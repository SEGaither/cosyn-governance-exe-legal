# Changelog

## v5.0.0-dev (2026-03-28)

Fork-by-copy from [cosyn-governance-exe v4.1.1](https://github.com/SEGaither/cosyn-governance-exe/releases/tag/v4.1.1) to create the v5-legal line for local-LLM legal professional services.

### What Changed in the Fork

- Package renamed from `cosyn` to `cosyn-legal`, version set to `5.0.0-dev`
- LLM client updated from OpenAI API to local Ollama endpoint (localhost:11434)
- Removed OPENAI_API_KEY dependency from bootstrap, CLI, and LLM client
- Documentation updated to reflect v5-legal identity and local-LLM architecture
- v4.x build reports and test results archived to `v4x-archive/` subdirectories

### Design Decisions Approved

1. Separate repo (not branch) — v4.x and v5.x do not share commit history
2. Self-hosted SearXNG for external research
3. Explicit dropdown for data classification, default Client
4. Defer internal network access (DMS/SharePoint) to v5.1
5. Legal-specific test manifest

### Planned Sub-Projects (Not Yet Implemented)

- **A** — Local LLM backend (Ollama swap + configurable endpoint)
- **A.1** — Model qualification (25-prompt suite against candidate models)
- **B** — Session isolation
- **C** — Data classification (Client / Internal / External)
- **D** — Controlled external access
- **E** — Enhanced telemetry (ABA compliance)
- **F** — 7-phase validation test

### Status

Not ready for use. Governance pipeline compiles and tests pass, but LLM client requires a running Ollama server with the target model loaded.

---

## v4.x History (Pre-Fork)

v4.1.0 and v4.1.1 history is documented in the original [cosyn-governance-exe](https://github.com/SEGaither/cosyn-governance-exe) repository. Archived build reports from v4.x are in `docs/build-reports/v4x-archive/`.
