// rust-enclave/src/policy.rs
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    PermitExecution,
    DenyExecution(&'static str),
}

pub struct PolicyEnforcementEngine {
    pub strictly_non_military: bool,
}

impl PolicyEnforcementEngine {
    /// Evaluates active environment claims and variables against non-military invariants.
    pub fn evaluate_runtime_environment(&self) -> PolicyDecision {
        println!("[POLICY] Initializing environment classification audit...");

        // Collect all environment variables into a map for checking
        let env_vars: HashMap<String, String> = env::vars().collect();

        // 1. Strict Prohibited Sector Check
        // Flags clear semantic indicators of military, weapons, or targeting stacks
        let prohibited_keywords = vec![
            "TARGET_ACQUISITION",
            "KINETIC_TRACKING",
            "WEAPONS_SYSTEM",
            "OFFENSIVE_OPERATIONS",
            "MILITARY_WORKSPACE",
        ];

        for keyword in prohibited_keywords {
            if env_vars.keys().any(|key| key.to_uppercase().contains(keyword)) {
                return PolicyDecision.DenyExecution(
                    "CRITICAL POLICY VIOLATION: Prohibited military deployment variable detected."
                );
            }
        }

        // 2. Validate Explicit Purpose Assertion
        // Ensures the environment has been structurally classified for civil/humanitarian use
        match env_vars.get("PROVENANCESHIELD_ENVIRONMENT_MODE") {
            Some(value) => {
                if value != "Non-Military-Defensive-Sandbox" {
                    return PolicyDecision.DenyExecution(
                        "POLICY DENIAL: Workspace mode claim does not match non-military constraints."
                    );
                }
            }
            None => {
                return PolicyDecision.DenyExecution(
                    "POLICY DENIAL: Mandatory environmental purpose assertion variable is missing."
                );
            }
        }

        // 3. Verify Lack of External Debugging Interferences
        // Blocks execution if the platform is configured for unsafe runtime modification
        if let Some(debug_flag) = env_vars.get("PROVENANCESHIELD_ALLOW_UNRESTRICTED_DEBUG") {
            if debug_flag.to_lowercase() == "true" && self.strictly_non_military {
                return PolicyDecision.DenyExecution(
                    "SECURITY EXCLUSION: Un-isolated debugging hooks are strictly forbidden in this mode."
                );
            }
        }

        println!("[SUCCESS] System environment cleared. Workspace meets civil protection utility bounds.");
        PolicyDecision.PermitExecution
    }
}
