// provenance-shield: attestation.rs
// Core attestation request + verification scaffolding.
// This module defines the enclave’s identity verification boundary.

use anyhow::{Result, anyhow};
use serde::{Deserialize};
use crate::tee_interface::{request_attestation_quote};
use crate::provenance::{verify_measurement};
use crate::policy::{AttestationPolicy};

/// Hardware-rooted attestation result returned by the TEE.
#[derive(Debug, Deserialize)]
pub struct AttestationQuote {
    pub measurement: String,
    pub platform_info: String,
    pub nonce: String,
    pub signature: String,
}

/// Verified enclave identity + provenance status.
#[derive(Debug)]
pub struct AttestationOutcome {
    pub verified: bool,
    pub reason: Option<String>,
}

/// Perform attestation and verify enclave identity.
pub fn perform_attestation(policy: &AttestationPolicy) -> Result<AttestationOutcome> {
    // 1. Request attestation quote from the underlying TEE.
    let quote = request_attestation_quote()
        .map_err(|e| anyhow!("TEE attestation request failed: {}", e))?;

    // 2. Verify measurement against expected provenance.
    let measurement_ok = verify_measurement(&quote.measurement, policy)?;

    if !measurement_ok {
        return Ok(AttestationOutcome {
            verified: false,
            reason: Some("Measurement mismatch: provenance not satisfied".into()),
        });
    }

    // 3. Enforce non-military / dual-use constraints.
    if !policy.environment_allowed(&quote.platform_info) {
        return Ok(AttestationOutcome {
            verified: false,
            reason: Some("Environment not permitted under non-military policy".into()),
        });
    }

    // 4. If all checks pass, enclave identity is verified.
    Ok(AttestationOutcome {
        verified: true,
        reason: None,
    })
}
