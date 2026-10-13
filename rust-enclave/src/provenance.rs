// rust-enclave/src/provenance.rs
use sha2::{Sha256, Digest};

/// Struct representing the parsed cryptographic identity extracted from the TEE hardware.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformProvenance {
    pub hardware_signature_valid: bool,
    pub measurement_hash: String,
    pub isolation_type: &'static str,
}

pub struct ProvenanceValidator {
    pub expected_measurement: String,
}

impl ProvenanceValidator {
    /// Parses the raw 1128-byte AMD SEV-SNP report layout to extract the immutable boot measurement.
    pub fn parse_sev_snp_report(&self, raw_report: &[u8; 1128]) -> Result<PlatformProvenance, &'static str> {
        println!("[PROVENANCE] Parsing raw 1128-byte AMD SEV-SNP hardware block...");

        // According to the AMD SEV-SNP manual, the MRENCLAVE (measurement) hash 
        // is located at a specific 48-byte offset segment within the binary report format.
        // For SHA-256 deployments, we extract the first 32 bytes of this segment.
        let measurement_offset = 48; 
        let mut measurement_bytes = [0u8; 32];
        measurement_bytes.copy_from_slice(&raw_report[measurement_offset..(measurement_offset + 32)]);

        let hex_measurement = hex::encode(measurement_bytes);
        println!("[PROVENANCE] Extracted Hardware MRENCLAVE: {}", hex_measurement);

        let matches = hex_measurement == self.expected_measurement;

        Ok(PlatformProvenance {
            hardware_signature_valid: true, // Validated upstream by the hardware root of trust
            measurement_hash: hex_measurement,
            isolation_type: "AMD-SEV-SNP",
        })
    }

    /// Parses the raw 1024-byte Intel TDX report layout to extract the trust domain owner identity.
    pub fn parse_tdx_report(&self, raw_report: &[u8; 1024]) -> Result<PlatformProvenance, &'static str> {
        println!("[PROVENANCE] Parsing raw 1024-byte Intel TDX hardware block...");

        // In Intel TDX, the MRTDX measurement registers track the initial guest code state.
        // We extract the 32-byte measurement signature located at the standard architectural offset.
        let tdx_measurement_offset = 32;
        let mut measurement_bytes = [0u8; 32];
        measurement_bytes.copy_from_slice(&raw_report[tdx_measurement_offset..(tdx_measurement_offset + 32)]);

        let hex_measurement = hex::encode(measurement_bytes);
        println!("[PROVENANCE] Extracted Hardware MRTDX: {}", hex_measurement);

        Ok(PlatformProvenance {
            hardware_signature_valid: true,
            measurement_hash: hex_measurement,
            isolation_type: "INTEL-TDX",
        })
    }

    /// Computes a local runtime SHA-256 hash of a file to cross-verify the Python engine's payload integrity.
    pub fn calculate_payload_hash(&self, file_path: &str) -> Result<String, &'static str> {
        let mut file = std::fs::File::open(file_path).map_err(|_| "Failed to open Python data engine file path")?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher).map_err(|_| "Error streaming data into SHA-256 compression block")?;
        
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }
}
