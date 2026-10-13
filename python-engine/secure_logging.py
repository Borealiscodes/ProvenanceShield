# python-engine/secure_logging.py
import json
import time
import hashlib
import urllib.request
from token_validation import EnclaveTokenValidator

class SecureForensicLogger:
    def __init__(self, token: str, log_file_path: str = "forensic_chain.log"):
        """
        Initializes the forensic log loop. Validates token claims to ensure
        the writer is a verified, hardware-isolated enclave node.
        """
        # Validate the token prior to opening the log structure
        TARGET_MRENCLAVE = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        validator = EnclaveTokenValidator(expected_mrenclave=TARGET_MRENCLAVE)
        self.verified_claims = validator.verify_provenance_claims(token)
        
        self.log_file_path = log_file_path
        self.enclave_hash = self.verified_claims.get("x-ms-sevsnp-mrenclave") or self.verified_claims.get("x-ms-tdx-mrownerid")
        self.previous_hash = self._calculate_tail_hash()

    def _calculate_tail_hash(self) -> str:
        """
        Reads the last line of the current log file to discover the previous block's hash.
        If the log file is empty or missing, it initializes the genesis block hash.
        """
        try:
            with open(self.log_file_path, "r") as f:
                lines = f.readlines()
                if not lines:
                    return "0" * 64  # Genesis initialization string
                last_line = lines[-1].strip()
                last_block = json.loads(last_line)
                return last_block["current_block_hash"]
        except FileNotFoundError:
            return "0" * 64

    def _fetch_rfc3161_timestamp_token(self, payload_hash: str) -> str:
        """
        Simulates an external RFC 3161 Time-Stamp Authority (TSA) DER request.
        Binds the entry to an immutable, cryptographically signed millisecond stamp.
        """
        print("[TIMESTAMP] Requesting RFC 3161 DER-encoded evidence signature block...")
        # Production environments route a binary POST query containing the nonce/hash to a public TSA.
        # Returning structural placeholder tracking tokens for validation runtime testing.
        mock_tsa_token = f"MIAGCSqGSIb3DQEHAqCAMIACAQExDzANBglghkgBZQMEAgEFAD...[TSA_HASH:{payload_hash[:8]}]"
        return mock_tsa_token

    def append_forensic_entry(self, data_payload: dict):
        """
        Constructs a new cryptographic log block N, sequentially chaining it to block N-1
        via an explicit payload compression loop.
        """
        print(f"[LOG] Initiating chain validation capture. Chaining from N-1: {self.previous_hash[:12]}...")

        # Normalize entry payload into a deterministic string format
        payload_string = json.dumps(data_payload, sort_keys=True)
        payload_hash = hashlib.sha256(payload_string.encode('utf-8')).hexdigest()

        # Request external trusted verification time proof
        rfc3161_token = self._fetch_rfc3161_timestamp_token(payload_hash)

        # Assemble the immutable architectural log block metadata
        log_block = {
            "collection_method": "Rust Native TEE Memory Isolation Capture",
            "handler_identity": "System_Enclave_Daemon_v1.0.4",
            "telemetry_payload_hash": payload_hash,
            "previous_block_hash": self.previous_hash,
            "device_context": {
                "host_id": "AZURE-CONF-VM-0042",
                "os_platform": "linux-x86_64-hardened",
                "enclave_active": True,
                "hardware_mrenclave": self.enclave_hash
            },
            "current_custodian": "Automated_Ingestion_Engine",
            "operational_purpose": "Forensic Preservation of Violation Evidence",
            "rfc3161_timestamp_token": rfc3161_token,
            "unix_time": time.time()
        }

        # Calculate the definitive block hash to finalize link binding (Block N Hash)
        block_bytes = json.dumps(log_block, sort_keys=True).encode('utf-8')
        current_block_hash = hashlib.sha256(block_bytes).hexdigest()
        log_block["current_block_hash"] = current_block_hash

        # Append the structural entry to the storage line
        with open(self.log_file_path, "a") as f:
            f.write(json.dumps(log_block) + "\n")

        print(f"[SUCCESS] Block N locked and appended. Current Block Hash: {current_block_hash[:12]}")
        
        # Advance the local structural pointer pointer tracking sequence
        self.previous_hash = current_block_hash

if __name__ == "__main__":
    # Test session token containing required hardware purpose claims
    MOCK_ENCLAVE_TOKEN = (
        "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ4LW1zLWlzb2xhdGlvbi10ZWUiOiJhbWQtc2V2LXNu"
        "cCIsIngtbXMtc2V2c25wLW1yZW5jbGF2ZSI6ImUzYjBjNDQyOThmYzFjMTQ5YWZiZjRjODk5NmZiOTI0"
        "MzdhZTQxZTQ2NDliOTM0Y2E0OTU5OTFiNzg1MmI4NTUiLCJ4LW1zLXByb3ZlbmFuY2VzaGllbGQtcHVy"
        "cG9zZSI6Ik5vbi1NaWxpdGFyeS1EZWZlbnNpdmUtU2FuZGJveCIsImV4cCI6NDg5NzAzNjAwMH0.mock"
        "_signature_bits"
    )
    
    logger = SecureForensicLogger(token=MOCK_ENCLAVE_TOKEN)
    
    # Process sequential sample blocks to verify hash propagation
    logger.append_forensic_entry({"target_id": "HRD-0921", "violation_vector": "Interpol Red Notice Abuse"})
    logger.append_forensic_entry({"target_id": "HRD-0921", "violation_vector": "Digital Surveillance Intercept"})
