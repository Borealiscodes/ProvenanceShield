# ⚠️ ARCHIVAL RESEARCH RECORD — ProvenanceShield

> **Notice & Status:** This repository is preserved as an **archival research artifact** for studying the defensive architectures, threat models, and software prototypes built by individuals facing **transnational repression**.
>
> **Do Not Deploy in Production:** This framework has not been audited, verified, or tested for real-world operational security. Several core subsystems contain simulated stubs, unverified mathematical metaphors, and architectural mismatches documented in the **Technical Audit Disclosure** below.

---

# 🛡️ ProvenanceShield
**Archival Prototype: Hardware-Attested Cognitive Boundary Concepts for Human-Rights-Aligned Computation**

ProvenanceShield is an early conceptual and architectural prototype exploring how human rights defenders (HRDs), investigative researchers, and independent open-science teams might bind software execution to verifiable hardware roots of trust when operating under coercive geopolitical conditions.

Rather than serving as a deployable security tool, this repository is maintained as a primary-source record examining:
1. **Defensive Design Under Duress:** How practitioners conceptualize zero-trust boundaries, chain-of-custody logging, and anti-repurposing constraints while actively navigating digital and cross-border coercion.
2. **The Gap Between Conceptual Security and Formal Verification:** Where ambitious architectural goals (TEE attestation, geometric state monitoring, and RFC 3161 timestamping) outpace actual implementation rigor in rapid prototypes.

---

## 🔍 Technical Audit Disclosure (What Works vs. What Is Simulated)
To prevent false security assumptions by researchers studying this archive, the three architectural tiers below are annotated with their verified implementation status:

### 1. Silicon Gatekeeper (`rust-enclave/`) — *Conceptual / Hardware-Dependent*
* **Intended Design:** Interface directly with bare-metal Confidential Computing device nodes (`/dev/sev-guest` for AMD SEV-SNP and `/dev/tdx-guest` for Intel TDX) via raw `ioctl` calls (`0xc0405301`, `0xc0405402`) to verify enclave identity before decrypting payloads.
* **Audit Findings:**
  * Mixing `MRENCLAVE` (an Intel SGX register term) with AMD SEV/TDX device paths reflects a conceptual conflation of distinct TEE architectures.
  * Raw `ioctl` calls fail immediately on standard consumer hardware or non-confidential virtual machines where hardware TEE device nodes are absent.

### 2. Analytical Manifold Engine (`python-engine/`) — *Conceptual Metaphor*
* **Intended Design:** Monitor high-dimensional state representations using Singular Value Decomposition (SVD) and a conceptual "Snowman Three-Sphere Model" ($X(t) = [M(t), O(t), H(t)]$, tracking Mandate, Operational, and Harm perimeters) to halt execution if coerced into dual-use roles.
* **Audit Findings:**
  * While SVD spectrum tracking is valid linear algebra, mapping high-level legal or ethical concepts ($M, O, H$) directly onto tensor singular values remains an unproved conceptual metaphor rather than a mathematically enforceable invariant.
  * *For rigorous, verified spectral collapse detection and triage, see the successor repository: [`A7.2-CollapseSurfaceRepairPipeline`](https://github.com).*

### 3. Tamper-Evident Forensic Ledger (`python-engine/secure_logging.py`) — *Prototype Stub*
* **Intended Design:** Maintain an append-only, SHA-256 hash-chained log (`forensic_chain.log`) bound to enclave measurements and external RFC 3161 Time-Stamp Authority (TSA) DER tokens.
* **Audit Findings:**
  * **Simulated Network & Token Claims:** `_fetch_rfc3161_timestamp_token` returns a static placeholder string rather than querying a live TSA endpoint, and `EnclaveTokenValidator` parses mock JWT strings (`.mock_signature_bits`).
  * **Hardcoded Context:** Log entries hardcode simulated cloud VM identifiers (`AZURE-CONF-VM-0042`) and an empty-string SHA-256 digest (`e3b0c442...`) regardless of the actual host environment.
  * *For an honest, crash-safe (`os.fsync`), verifiable SHA-256 JSONL audit ledger without simulated TEE claims, see `src/lhguard/ledger.py` in [`Lean-Hallucination-Guard-Revised`](https://github.com).*

---

## 🧭 Original Threat Model & Research Context
Despite its prototype limitations, the threat model preserved in this archive documents real operational pressures faced by independent researchers and human rights defenders:

* **Infrastructure Coercion:** Host server tampering, hypervisor compromise, cloud provider compulsion, and supply-chain injection.
* **Dual-Use Repurposing Risks:** The concern that analytical or geometric modeling tools built for humanitarian or civil research can be silently refactored into surveillance or targeting workflows without cryptographic purpose binding.
* **Asymmetric Resource Constraints:** The need for verifiable chain-of-custody tooling among independent groups operating without institutional or enterprise security backing.

---

## 📚 Archival Preservation & Successor Work
* **Historical Snapshots:** Earlier documentation drafts are preserved under `docs/archive/READMEv2.0archival_snapshot.md`.
* **Successor Verification Work:** Lessons learned from auditing the gaps in this prototype directly informed the formal verification and numerical triage architectures in:
  * [`Coq-Hallucination-Guard`](https://github.com) (Machine-checked Coq invariants & C-ABI extraction)
  * [`A7.2-CollapseSurfaceRepairPipeline`](https://github.com) (Honest SVD rank-collapse detection & Lean 4 reintegration gating)
  * [`NDH-Research-Pilot`](https://github.com) (*Volume I: Geometric Operator Theory of Transformer Synthesis*)
