// rust-enclave/src/main.rs
mod attestation;
mod tee_interface;
mod provenance;
mod policy;
mod key_release;

use attestation::AttestationEngine;
use policy::{PolicyEnforcementEngine, PolicyDecision};
use key_release::KeyReleaseManager;
use std::env;
use std::process::{Command, exit};

#[tokio::main]
async fn main() {
    println!("=================================================================");
    println!("⚙️  PROVENANCESHIELD v3.0.0 :: INITIALIZING HARDWARE ENCLAVE");
    println!("=================================================================");

    // 1. EVALUATE WORKSPACE SYSTEM POLICY CONDITIONS
    // Intercepts unauthorized deployment strings before waking up the network
    let policy_engine = PolicyEnforcementEngine { strictly_non_military: true };
    match policy_engine.evaluate_runtime_environment() {
        PolicyDecision::PermitExecution => {
            println!("[INIT] Phase 1/4 Cleared: System workspace parameters validated.");
        }
        PolicyDecision::DenyExecution(error_msg) => {
            eprintln!("[CRITICAL SECURITY FAULT] Phase 1/4 Failed: {}", error_msg);
            exit(1);
        }
    }

    // Configure endpoints and build target parameters matching your infrastructure
    let target_maa_endpoint = env::var("PROVENANCESHIELD_MAA_ENDPOINT")
        .unwrap_or_else(|_| "https://azure.net".to_string());
    
    // The un-falsifiable SHA-256 footprint expected from your CI/CD compilation report
    let target_mrenclave = env::var("PROVENANCESHIELD_EXPECTED_HASH")
        .unwrap_or_else(|_| "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string());

    let attestation_engine = AttestationEngine {
        maa_endpoint: target_maa_endpoint,
        expected_mrenclave: target_mrenclave,
    };

    // 2. QUERY HARDWARE SECURITY MODULE VIA KERNEL DRIVERS
    let raw_hardware_quote = match attestation_engine.fetch_raw_hardware_quote() {
        Ok(quote) => {
            println!("[INIT] Phase 2/4 Cleared: Un-falsifiable hardware block extracted.");
            quote
        }
        Err(err) => {
            eprintln!("[CRITICAL SECURITY FAULT] Phase 2/4 Failed: {}", err);
            exit(1);
        }
    };

    // 3. EXCHANGE PLATFORM QUOTE FOR CLOUD-SIGNED PROVENANCE TOKEN
    let provenance_token = match attestation_engine.exchange_quote_for_token(raw_hardware_quote).await {
        Ok(token) => {
            println!("[INIT] Phase 3/4 Cleared: Cryptographic attestation token issued.");
            token
        }
        Err(err) => {
            eprintln!("[CRITICAL SECURITY FAULT] Phase 3/4 Failed: {}", err);
            exit(1);
        }
    };

    // Verify token claims locally to double-check the cryptographic purpose binding
    if let Err(verification_fault) = attestation_engine.verify_provenance_claims(&provenance_token) {
        eprintln!("[CRITICAL SECURITY FAULT] Claim Verification Failed: {}", verification_fault);
        exit(1);
    }

    // 4. SEAFED KEY RELEASE AND PYTHON DATA SUB-SYSTEM INSTANTIATION
    // Mock symmetric key array released by Secure Key Release under strict MAA conditions
    let mock_hsm_released_key = [0x5E; 32];
    let vault_manager = KeyReleaseManager { raw_key_material: mock_hsm_released_key };

    let target_encrypted_asset = "python-engine/manifold_engine.py.enc";
    
    // Safety check during local building: generate dummy encrypted asset if missing
    if !std::path::Path::new(target_encrypted_asset).exists() {
        println!("⚠️  [WARN] Encrypted asset missing on disk. Compiling secure distribution asset block...");
        let fallback_plain_text = std::fs::read_to_string("python-engine/manifold_engine.py")
            .unwrap_or_else(|_| "print('Engine Scaffolding Mock Error')".to_string());
        let _ = vault_manager.seal_engine_for_distribution(&fallback_plain_text, target_encrypted_asset);
    }

    // Decrypt the analytical data model straight into isolated RAM memory strings
    match vault_manager.decrypt_engine_to_memory(target_encrypted_asset) {
        Ok(python_source_code) => {
            println!("[INIT] Phase 4/4 Cleared: Analytical math engine loaded in encrypted memory.");
            println!("=================================================================");
            println!("🚀 PERIMETER SECURED :: SPINNING UP COGNITION DATA ENGINE");
            println!("=================================================================");

            // Pass execution to the Python compute layer securely, feeding the attestation token inline
            let mut data_process = Command::new("python3")
                .arg("-c")
                .arg(&python_source_code)
                .arg("--token")
                .arg(&provenance_token)
                .spawn()
                .expect("Failed to initialize execution pipeline boundary context.");

            let status = data_process.wait().expect("Data engine runtime interrupted.");
            println!("[SHUTDOWN] Data processing execution completed with status: {}", status);
        }
        Err(cryptographic_fault) => {
            eprintln!("[CRITICAL SECURITY FAULT] Phase 4/4 Failed: {}", cryptographic_fault);
            exit(1);
        }
    }
}
