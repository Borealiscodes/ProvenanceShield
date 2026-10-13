// provenance-shield: key_release.rs
// Attestation-gated key release mechanism.
// Ensures decryption keys only unlock inside verified, non-military enclaves.

use anyhow::{Result, anyhow};
use crate::attestation::AttestationOutcome;
use crate::policy::KeyReleasePolicy;

/// Represents a sealed key stored in the enclave or external KMS.
/// The key is encrypted and bound to the enclave measurement.
pub struct SealedKey {
    pub encrypted_key: Vec<u8>,
}

/// Represents a decrypted key ready for use inside the enclave.
pub struct ReleasedKey {
    pub key: Vec<u8>,
}

/// Attempt to release a sealed key only if attestation and provenance checks pass.
pub fn release_key(
    sealed: &SealedKey,
    attestation: &AttestationOutcome,
    policy: &KeyReleasePolicy
) -> Result<ReleasedKey> {

    // 1. Attestation must be verified.
    if !attestation.verified {
        return Err(anyhow!(
            "Key release denied: attestation not verified ({:?})",
            attestation.reason
        ));
    }

    // 2. Enforce non-military key-release constraints.
    if !policy.environment_allowed() {
        return Err(anyhow!(
            "Key release denied: environment violates non-military policy"
        ));
    }

    // 3. Decrypt the sealed key using enclave-bound sealing keys.
    let decrypted = decrypt_with_enclave_key(&sealed.encrypted_key)
        .map_err(|_| anyhow!("Key release failed: enclave decryption error"))?;

    Ok(ReleasedKey { key: decrypted })
}

/// Placeholder for enclave-bound decryption.
/// In real deployment, this uses hardware sealing keys (SEV-SNP, TDX, Nitro).
fn decrypt_with_enclave_key(data: &[u8]) -> Result<Vec<u8>> {
    // Scaffolding only — real implementation uses TEE sealing keys.
    Ok(data.to_vec())
}
