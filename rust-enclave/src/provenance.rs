// provenance-shield: provenance.rs
// Provenance verification + measurement enforcement.
// Ensures execution is bound to enclave identity and non-military constraints.

use anyhow::{Result, anyhow};
use sha2::{Sha256, Digest};
use crate::policy::AttestationPolicy;

/// Verify that the enclave measurement matches the expected provenance.
/// This prevents modified binaries, repurposed builds, or hostile forks.
pub fn verify_measurement(measurement: &str, policy: &AttestationPolicy) -> Result<bool> {
    let expected = policy.expected_measurement();

    if measurement != expected {
        return Ok(false);
    }

    Ok(true)
}

/// Compute a SHA-256 hash of the enclave binary.
/// Used during CI/CD to generate the expected measurement.
pub fn compute_measurement(binary: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(binary);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Enforce provenance-locked execution.
/// This ensures the enclave identity, measurement, and environment
/// all satisfy the non-military constraints.
pub fn enforce_provenance(
    measurement_ok: bool,
    environment_ok: bool
) -> Result<()> {

    if !measurement_ok {
        return Err(anyhow!(
            "Provenance failure: enclave measurement does not match expected value"
        ));
    }

    if !environment_ok {
        return Err(anyhow!(
            "Provenance failure: environment violates non-military policy"
        ));
    }

    Ok(())
}
