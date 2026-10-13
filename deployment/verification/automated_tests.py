# deployment/verification/automated_tests.py
import unittest
import os
import sys
import json
import tempfile
import numpy as np

# Ensure the execution path can discover the target python engine modules
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '../../python-engine')))

from token_validation import EnclaveTokenValidator
from manifold_engine import SnowmanManifoldEngine
from secure_logging import SecureForensicLogger

class TestProvenanceShieldArchitecture(unittest.TestCase):
    def setUp(self):
        """
        Establishes an isolated cryptographic session baseline for automated testing.
        """
        # Hardcoded 256-bit test vector fingerprint matching the static build target
        self.target_mrenclave = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        
        # Valid, cryptographically signed mock token containing required humanitarian purpose claims
        self.valid_mock_token = (
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ4LW1zLWlzb2xhdGlvbi10ZWUiOiJhbWQtc2V2LXNu"
            "cCIsIngtbXMtc2V2c25wLW1yZW5jbGF2ZSI6ImUzYjBjNDQyOThmYzFjMTQ5YWZiZjRjODk5NmZiOTI0"
            "MzdhZTQxZTQ2NDliOTM0Y2E0OTU5OTFiNzg1MmI4NTUiLCJ4LW1zLXByb3ZlbmFuY2VzaGllbGQtcHVy"
            "cG9zZSI6Ik5vbi1NaWxpdGFyeS1EZWZlbnNpdmUtU2FuZGJveCIsImV4cCI6NDg5NzAzNjAwMH0.mock"
            "_signature_bits"
        )

    # ==========================================
    # UNIT TEST PATTERN 1: TOKEN ENVIRONMENT GATING
    # ==========================================
    def test_valid_token_clearance(self):
        """Confirm that a structurally accurate token passes the silicon validation checks smoothly."""
        validator = EnclaveTokenValidator(expected_mrenclave=self.target_mrenclave)
        claims = validator.verify_provenance_claims(self.valid_mock_token)
        self.assertEqual(claims["x-ms-provenanceshield-purpose"], "Non-Military-Defensive-Sandbox")

    def test_invalid_purpose_revocation(self):
        """Enforce systemic system termination if the environment mode signature is altered or missing."""
        malicious_token = (
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ4LW1zLWlzb2xhdGlvbi10ZWUiOiJhbWQtc2V2LXNu"
            "cCIsIngtbXMtc2V2c25wLW1yZW5jbGF2ZSI6ImUzYjBjNDQyOThmYzFjMTQ5YWZiZjRjODk5NmZiOTI0"
            "MzdhZTQxZTQ2NDliOTM0Y2E0OTU5OTFiNzg1MmI4NTUiLCJ4LW1zLXByb3ZlbmFuY2VzaGllbGQtcHVy"
            "cG9zZSI6Ik1JTElUQVJZX1RBUkdFVElOR19XT1JLU1BBQ0UiLCJleHAiOjQ4OTcwMzYwMDB9.invalid"
        )
        validator = EnclaveTokenValidator(expected_mrenclave=self.target_mrenclave)
        
        # System must call sys.exit() to prevent token processing fall-through bugs
        with self.assertRaises(SystemExit):
            validator.verify_provenance_claims(malicious_token)

    # ==========================================
    # UNIT TEST PATTERN 2: MANIFOLD STRUCTURAL INVARIANTS
    # ==========================================
    def test_snowman_coordinate_bounds(self):
        """Verify that standard coordinate geometry maintains unbroken sphere bounds during initialization."""
        engine = SnowmanManifoldEngine(mandate=1.0, operational=0.5, harm=0.1)
        self.assertTrue(engine.enforce_coordinate_invariants())
        self.assertFalse(engine.collapsed)

    def test_system_collapse_cascade(self):
        """Prove that driving the operational radius to zero triggers a permanent system collapse."""
        engine = SnowmanManifoldEngine(mandate=1.0, operational=0.0, harm=0.2)
        engine.enforce_coordinate_invariants()
        
        self.assertTrue(engine.collapsed)
        self.assertEqual(engine.rm, 0.0)
        self.assertEqual(engine.rh, 1.0) # Harm space overflow must lock at maximum parameter

    # ==========================================
    # UNIT TEST PATTERN 3: FORENSIC LOG CHAINING
    # ==========================================
    def test_sequential_hash_propagation(self):
        """Verify that log block N cryptographically embeds the precise signature hash of block N-1."""
        with tempfile.NamedTemporaryFile(delete=False, mode="w+") as tmp_log:
            tmp_path = tmp_log.name

        try:
            logger = SecureForensicLogger(token=self.valid_mock_token, log_file_path=tmp_path)
            
            # Commit sequential mock entries to the local file
            logger.append_forensic_entry({"event": "Preservation Ingestion", "node": "01"})
            logger.append_forensic_entry({"event": "Anomalous Vector Isolated", "node": "02"})

            # Read back generated JSON lines to audit block links
            with open(tmp_path, "r") as f:
                blocks = [json.loads(line) for line in f.readlines()]

            self.assertEqual(len(blocks), 2)
            self.assertEqual(blocks[1]["previous_block_hash"], blocks[0]["current_block_hash"])
            self.assertIn("TSA_HASH", blocks[0]["rfc3161_timestamp_token"])

        finally:
            if os.path.exists(tmp_path):
                os.remove(tmp_path)

if __name__ == "__main__":
    unittest.main()
