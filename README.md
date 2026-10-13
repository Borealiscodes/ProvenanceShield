# README.md — ProvenanceShield

🛡️ Hardware‑Attested, Provenance‑Locked Execution for Human‑Rights Telemetry

ProvenanceShield is a defensive execution framework designed for human‑rights defenders, investigative researchers, and safety‑critical teams operating in hostile or dual‑use environments. It ensures that sensitive telemetry, evidence, and analytical models run only inside verified Trusted Execution Environments (TEEs), preventing tampering, unauthorized compilation, or weaponization.

ProvenanceShield verifies enclave identity, enforces cryptographic attestation, and decrypts protected model weights only after confirming the hardware‑rooted provenance of the runtime. If the environment is untrusted, modified, or repurposed for military or surveillance use, execution is refused and all sensitive components remain encrypted.

---

✨ Core Guarantees

🔐 Hardware‑Rooted Identity Verification
Validates AMD SEV‑SNP, Intel TDX, or AWS Nitro Enclave attestation quotes before any computation begins. Only verified enclaves can execute protected workloads.

📜 Cryptographic Provenance Enforcement
Execution is bound to hardware‑signed measurements of the binary, preventing tampering, unauthorized forks, or derivative misuse.

🧩 Secure Model Decryption
Protected model weights decrypt only inside verified enclaves. If the environment is altered, keys never release.

🚫 Dual‑Use Safeguarding
Strict non‑military licensing boundaries. If deployed in defense, aerospace, surveillance, or weapons‑adjacent stacks, attestation fails and execution halts.

📚 Chain‑of‑Custody Integrity
Telemetry processed inside the enclave is logged using sequential hash‑chaining and optional RFC 3161 trusted timestamping, producing tamper‑proof forensic records.

---

🏛️ Architecture Overview

ProvenanceShield consists of two coordinated layers:

🦾 Rust Enclave Layer (TEE Sandbox)
- Requests hardware attestation quotes  
- Verifies enclave identity and code measurement  
- Enforces provenance and licensing constraints  
- Releases decryption keys only after validation  
- Launches the protected Python engine inside encrypted memory  

🐍 Python Telemetry Engine (Encrypted Data Layer)
- Validates attestation tokens  
- Processes human‑rights telemetry inside the enclave  
- Applies safe manifold embeddings and anomaly logic  
- Writes hash‑chained forensic logs  
- Produces tamper‑proof outputs for downstream analysis  

---

🌍 Why ProvenanceShield Exists

Cloud environments cannot distinguish between:

- human‑rights telemetry  
- climate‑risk analysis  
- robotics control  
- aerospace targeting  
- military trajectory planning  

Compilers treat all math as neutral infrastructure, creating severe dual‑use risk for small teams building humanitarian systems.

ProvenanceShield provides the missing boundary:  
a hardware‑verified, cryptographically enforced execution environment that refuses to run in weaponized or hostile contexts.

---

🤝 Who This Is For

- Human rights defenders  
- Investigative journalists  
- UN‑aligned documentation teams  
- Digital forensics labs  
- Climate‑risk researchers  
- Secure AI governance teams  
- Academics studying dual‑use mitigation  
- Any group needing tamper‑proof, non‑military computation  

---

⛔ Who This Is Not For

ProvenanceShield will not execute in:

- defense or aerospace stacks  
- weapons research environments  
- robotics targeting systems  
- sovereign surveillance platforms  
- unverified cloud VMs  
- modified or repurposed binaries  

Execution is cryptographically refused.

---

📄 License

ProvenanceShield uses a non‑military, provenance‑locked license.  
Unauthorized derivative use, repurposing, or compilation into weapon systems constitutes a breach of contractual and copyright restrictions.

---

📁 Repository Structure

`
provenance-shield/
README.md
LICENSE
.gitignore

rust-enclave/
    Cargo.toml
    src/
        main.rs
        attestation.rs
        tee_interface.rs
        provenance.rs
        key_release.rs
        logging.rs
    tests/
        attestation_tests.rs

python-engine/
    requirements.txt
    manifold_engine.py
    token_validation.py
    telemetry_processing.py
    secure_logging.py

enclave-policy/
    azure-attestation-policy.json
    sev-snp-policy.toml
    nitro-enclave-policy.json
    key-release-policy.json

provenance-chain/
    block_format.md
    hash_chain.rs
    timestamping.rs
    rfc3161_client.rs

deployment/
    docker/
        Dockerfile.enclave
        Dockerfile.controller
        compose.yaml
    ci-cd/
        pipeline.yaml
        hash-measurement.sh
    cloud/
        azure_setup.md
        awsnitrosetup.md
        gcpconfidentialvm_setup.md

docs/
    architecture-overview.md
    threat-model.md
    dual-use-mitigation.md
    provenance-spec.md
    attestation-flow.md
    api-reference.md
`

---

