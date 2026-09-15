# AegisMesh Core Architecture

A zero-trust, decentralized mesh protocol designed to make malicious AI payloads and adversarial inputs structurally incapable of producing actionable outputs.
---
## 🚨 The Imperative: Why AegisMesh is Necessary Now
We are entering an era of unprecedented asymmetric threat. Recent intelligence confirms that non-state groups, ideologically motivated networks, and state-backed asymmetric actors are actively utilizing advanced AI assistants to troubleshoot guided rocketry, conduct preliminary bioweapons research, and optimize targeting strategies against critical infrastructure.

Centralized models relying on reactive post-hoc text firewalls and rigid safety policies cannot prevent these exploits. Clever prompt injections, fine-tuning exploits, and open-weight model proliferation allow hostile actors to easily bypass cloud-hosted guardrails.

When dealing with highly motivated, adversarial human agendas, the solution is not to try to alter their intent or lecture them on ethics. **The solution is to construct a system engineering standard where their inputs are mathematically and structurally incapable of producing an actionable output.**

AegisMesh shifts the paradigm from fragile cloud-based **intent-filtering** to decentralized **architectural containment**. It is built specifically to strip the utility from malicious inputs before they can interact with execution environments or localized open-source model weights.
---
## 🏗️ The Three-Layer Security Topology
To achieve absolute resilience without enforcing heavy-handed centralized censorship, AegisMesh decouples **Identity**, **Reputation**, and **Execution** into independent, immutable protocol layers:

```
┌─────────────────────────────────────────────────────────┐
│ 1. IDENTITY LAYER (Zero-Knowledge Proof of Personhood) │ -> "I am a unique human" (Bans bots & Sybils)
└────────────────────────────┬────────────────────────────┘
▼
┌─────────────────────────────────────────────────────────┐
│ 2. REPUTATION LAYER (Decentralized Web of Trust Scoring)│ -> "I have a verified clean history"
└────────────────────────────┬────────────────────────────┘
▼
┌─────────────────────────────────────────────────────────┐
│ 3. EXECUTION LAYER (Deterministic Hardware Enclaves) │ -> Math-isolated sandbox execution
└────────────────────────────┴────────────────────────────┘
```

### 1. Cryptographic Identity Layer (`aegis-identity`)
* **Sybil Resistance:** To eliminate the threat of automated, bot-driven multi-agent swarms, every node must cryptographically prove it belongs to a unique, verified human.
* **Zero-Knowledge Privacy:** Utilizing Zero-Knowledge Proofs (ZKPs), a user can prove they are a unique, vouched human with a valid passport without revealing their real name, location, or biometric data to peer nodes.

### 2. Adaptive Reputation Layer (`aegis-reputation`)
* **Human-Vouched Web of Trust (WoT):** Interlinks within the mesh are bound by human validation. Nodes flag anomalous behavior or malicious payloads, dropping the sender's reputation score globally.
* **Collusion-Resilient Attrition:** To prevent hostile ideological factions from weaponizing the flagging system to censor innocent nodes, the protocol applies an Algorithmic Collusion Penalty that slashes the reputation of coordinated false-reporters.

### 3. Aegis Secure Mesh Proxy (`aegis-runtime`)
The core proxy is built using **Rust** for memory safety, low-overhead execution, and native compilation to WebAssembly.
* **Zero-Trust Ephemeral Sandboxing:** No matter how high a user's reputation score is, all incoming prompt strings, fine-tuning payloads, and automation code blocks run inside an isolated WebAssembly (Wasm) or MicroVM container at the receiving edge node.
* **Deterministic Input Scrubbing:** If a payload tries to trigger a semantic overflow or break memory boundaries, the sandbox container immediately self-destructs at the hardware enclave level (AMD SEV / Intel SGX), isolating the malicious actor completely from the main node operations.
---
## 🎯 Target MVP: Secure Drop-In LLM Gateway

Our immediate open-source objective is a local **Secure Mesh Gateway Proxy**. This drops directly in front of open-source edge LLM orchestrators (such as Ollama or vLLM). It forces incoming traffic to clear the `aegis-identity` ZK-check and passes all text streams through the `aegis-runtime` isolated proxy before hitting local model weights.
---
## 🤝 Call for Global Contributions

Securing decentralized infrastructure requires an open standard free from commercial gatekeeping and political alignment. We are actively inviting individuals, companies, and research institutions to contribute:

* **Systems & Cryptography Engineers:** Help us optimize the Rust-to-Wasm pipeline and integrate ZK-proof networks (such as World ID, Gitcoin Passport, or NFC government identity validations).
* **Distributed Protocol Engineers:** Help build out the p2p transport routing layer using `libp2p`.
* **Hardware Partners:** We seek collaboration with edge-node and confidential compute hardware providers to validate enclave attestation at the firmware level.

Review our active drafts in the `/specs` directory or pick up a `good-first-issue` to begin.

### 📄 License
AegisMesh is open-source infrastructure licensed under the **Apache License 2.0**.
