// provenance-shield: policy.rs
// Defines attestation and key-release policies.
// Central configuration for provenance, measurement, and non-military constraints.

use serde::Deserialize;

/// Attestation policy defining expected measurement and allowed environments.
#[derive(Debug, Deserialize)]
pub struct AttestationPolicy {
    pub expected_measurement: String,
    pub allowed_platforms: Vec<String>,
    pub prohibited_platforms: Vec<String>,
}

impl AttestationPolicy {
    /// Return the expected enclave measurement.
    pub fn expected_measurement(&self) -> String {
        self.expected_measurement.clone()
    }

    /// Check whether the attested platform is allowed under non-military constraints.
    pub fn environment_allowed(&self, platform_info: &str) -> bool {
        // Explicitly deny prohibited platforms (defense, aerospace, surveillance).
        if self.prohibited_platforms.iter().any(|p| platform_info.contains(p)) {
            return false;
        }

        // Allow only explicitly permitted platforms (Azure MAA, SEV-SNP, Nitro).
        self.allowed_platforms.iter().any(|p| platform_info.contains(p))
    }
}

/// Key-release policy defining non-military constraints for key access.
#[derive(Debug, Deserialize)]
pub struct KeyReleasePolicy {
    pub allow_release: bool,
    pub prohibited_environments: Vec<String>,
}

impl KeyReleasePolicy {
    /// Check whether key release is permitted in the current environment.
    pub fn environment_allowed(&self) -> bool {
        if !self.allow_release {
            return false;
        }

        // Deny release in prohibited environments.
        // (Defense, aerospace, robotics targeting, surveillance clouds.)
        // Real implementation will inspect attestation metadata.
        true
    }
}
