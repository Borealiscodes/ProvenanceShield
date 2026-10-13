# 🛡️ ProvenanceShield

Hardware‑Attested Cognitive Boundary Framework for Human‑Rights‑Aligned Computation

ProvenanceShield is a zero‑trust, self‑defending execution framework engineered for human rights defenders (HRDs), investigative researchers, and safety‑critical open‑science teams operating under coercive, high‑risk geopolitical conditions.  
It demonstrates how software can cryptographically enforce its intended purpose directly at the silicon level, preventing repurposing, tampering, or dual‑use weaponization.

By binding ethical invariants to Confidential Computing Hardware Roots of Trust (TEEs), ProvenanceShield establishes a hardware‑anchored cognitive boundary that cannot be bypassed by hostile hosts, compromised clouds, or state‑level adversaries.

---

⚙️ Deterministic Hardware Fingerprint & Core Verification Invariants

All operational logic is cryptographically pinned to a deterministic enclave identity:

- Release Version: v3.0.0  
- Verified MRENCLAVE:  
  e0a24538626b2bf46fa5e194ae6de0e82f2b8b3fca04c0730b0cc77067a2d68e
- Enforced Purpose Context: Non-Military-Defensive-Sandbox
- Supply-Chain Safeguards: Strict --locked dependency trees, multi‑stage container hardening, and deterministic build pipelines.

These invariants ensure the framework cannot be silently refactored into offensive, kinetic, or extrajudicial computational roles.

---

🧱 Architecture Overview

ProvenanceShield integrates hardware attestation, geometric safety invariants, and tamper‑evident forensic logging into a three‑tier sandbox:

---

1. Silicon Gatekeeper (rust-enclave/)

A hardened Rust subsystem that interfaces directly with bare‑metal TEE device nodes:

- AMD SEV: /dev/sev-guest  
- Intel TDX: /dev/tdx-guest

It issues raw ioctl commands (0xc0405301, 0xc0405402) to extract unforgeable attestation quotes from the CPU security processor.  
These quotes are verified against remote attestation endpoints before any cryptographic keys are released.

Outcome:  
Execution is physically bound to a verified enclave identity and an ethical purpose context. If the enclave fingerprint or purpose invariant fails, the system refuses to decrypt or run.

---

2. Analytical Manifold Engine (python-engine/)

A memory‑encrypted analytical engine performing high‑dimensional geometric reasoning using:

- Singular Value Decomposition (SVD)  
- Soft‑thresholding  
- Structural manifold monitoring

It implements the Snowman Three‑Sphere Invariant Model, a state vector:

\[
X(t) = [M(t), O(t), H(t)]
\]

representing:

- M — Mandate sphere (legal/ethical perimeter)  
- O — Operational sphere (permitted computational space)  
- H — Harm sphere (risk of coercive or dual‑use deformation)

If an adversary attempts structural compression—e.g., forcing the system into a kinetic or surveillance role—the manifold collapses:

- \( r_M \rightarrow 0.0 \)  
- \( r_H \rightarrow 1.0 \)

This triggers an immediate geometric cascade halt, locking execution and preventing downstream payload modification.

Outcome:  
The system mathematically refuses to operate outside its ethical perimeter.

---

3. Tamper‑Evident Forensic Ledger (secure_logging.py)

A cryptographically chained, append‑only ledger that serializes telemetry and analytical outputs.

Each block:

- embeds the signature of the previous block  
- is bound to the active MRENCLAVE fingerprint  
- is timestamped via RFC 3161 Time‑Stamp Authority (TSA) DER tokens

Outcome:  
A court‑ready, tamper‑evident chain of custody suitable for hostile cross‑examination and international human rights investigations.

---

🧭 Why Hardware?

Software‑only trust boundaries fail under coercion.

Human rights defenders and investigative researchers operate in environments where:

- cloud providers may be compromised  
- host servers may be tampered with  
- state actors may compel platform access  
- digital coercion is routine  
- supply‑chain attacks are common

TEEs provide the only non‑negotiable boundary:

- memory isolation  
- hardware‑rooted attestation  
- sealed cryptographic key release  
- resistance to host OS compromise  
- verifiable execution identity

ProvenanceShield uses TEEs not for corporate confidentiality, but as ethical governors that enforce purpose at the silicon level.

---

🛡️ Threat Model

ProvenanceShield assumes adversaries with:

1. Physical Access
- Host server tampering  
- Cold‑boot memory extraction  
- Hardware probing  
- BIOS/UEFI compromise  

2. Digital Access
- Root‑level host control  
- Hypervisor compromise  
- Cloud provider insider threats  
- Supply‑chain injection  

3. Coercive Access
- State‑sponsored digital coercion  
- Forced platform access  
- Legal or extrajudicial pressure  
- Infrastructure seizure  

4. Dual‑Use Repurposing
- Refactoring humanitarian analytics into:  
  - kinetic targeting systems  
  - autonomous drone path‑planning  
  - surveillance grids  
  - extrajudicial containment networks  

Security Guarantees:

- Enclave identity cannot be spoofed  
- Purpose context cannot be bypassed  
- Execution halts under geometric deformation  
- Logs cannot be silently altered  
- Key release is physically bound to attestation  
- Host OS compromise does not grant access  

---

🌍 Geopolitical Context: The Cognitive Dual‑Use Crisis

As AI systems shift toward geometric reasoning and world‑model cognition, the risk of dual‑use weaponization increases dramatically.

1. World Models Are Inherently Dual‑Use
The same manifold embeddings used for:

- climate modeling  
- humanitarian migration analysis  
- civil infrastructure optimization  

can be repurposed for:

- targeting  
- surveillance  
- autonomous weapons  
- coercive border systems  

2. Corporate AI Sustainability Has Collapsed
Massive neural architectures require:

- thermodynamically unsustainable compute  
- expanding carbon budgets  
- centralized cloud dependence  

The future lies in localized, invariant‑bounded, resource‑efficient geometric reasoning—not brute‑force token prediction.

3. Independent Researchers Are Systemically Unsupported
Human rights groups cannot rely on:

- enterprise licensing  
- cloud platform neutrality  
- corporate ethics boards  

ProvenanceShield provides a decentralized, hardware‑attested alternative.

---

👤 Intended Users

ProvenanceShield is an Erga Omnes utility—an obligation owed to the protection of human dignity.

Designed for:

- Human Rights Defenders (HRDs)  
- Investigative Forensics Labs  
- Open‑Science Governance Teams  
- Digital Security Researchers  
- Cross‑border legal advocates  

Use cases include:

- secure processing of transnational repression logs  
- maintaining chain‑of‑custody evidence  
- climate‑risk modeling in hostile clouds  
- encrypted analytical workflows under coercion  

---

📢 Disclaimer

ProvenanceShield is a functional proof‑of‑concept, not a final production release.

Its purpose is to demonstrate a foundational thesis:

> Software can defend its own ethical perimeter directly on the physical processor silicon.

It establishes a baseline blueprint for dignity‑aligned computation.  
Future ecosystems should expand, diversify, and decentralize this architecture.

Historical documentation snapshots are preserved under:  
docs/archive/READMEv2.0archival_snapshot.md

---

