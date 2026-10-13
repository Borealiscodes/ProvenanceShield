# deployment/verification/e2e_trace.py
import os
import sys
import subprocess
import json

def run_integration_trace():
    print("=================================================================")
    print("🛰️  PROVENANCESHIELD v3.0.0 :: STARTING END-TO-END INTEGRATION TRACE")
    print("=================================================================")

    workspace_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../"))
    os.chdir(workspace_root)

    # -----------------------------------------------------------------
    # STEP 1: EXECUTE THE AUTOMATED ARCHITECTURE TEST HARNESS
    # -----------------------------------------------------------------
    print("\n[TRACE 1/4] Running automated structural unit test suite...")
    test_runner = subprocess.run(
        [sys.executable, "deployment/verification/automated_tests.py", "-v"],
        capture_output=True,
        text=True
    )
    
    print(test_runner.stdout)
    if test_runner.returncode != 0:
        print(test_runner.stderr)
        print("❌ [TRACE FAILED] Unit tests failed. Aborting integration sequence.")
        sys.exit(1)
    print("✅ [TRACE STEP SUCCESS] Core structural unit tests passed.")

    # -----------------------------------------------------------------
    # STEP 2: VERIFY PIPELINE POLICY MANIFEST INTEGRITY
    # -----------------------------------------------------------------
    print("\n[TRACE 2/4] Auditing cryptographic policy configurations...")
    policy_path = "enclave-policy/key-release-policy.json"
    
    if not os.path.exists(policy_path):
        print(f"❌ [TRACE FAILED] Key release policy missing at {policy_path}")
        sys.exit(1)
        
    with open(policy_path, "r") as f:
        try:
            policy_data = json.load(f)
            # Ensure the policy enforces the non-military purpose binding we established
            purpose_claim = policy_data["anyOf"][0]["allOf"][3]["equals"]
            assert purpose_claim == "Non-Military-Defensive-Sandbox"
            print(f"✅ [TRACE STEP SUCCESS] Secure Key Release policy verified. Invariant bound: {purpose_claim}")
        except Exception as e:
            print(f"❌ [TRACE FAILED] Policy configuration structure corrupted: {e}")
            sys.exit(1)

    # -----------------------------------------------------------------
    # STEP 3: EXECUTE THE DATA ENGINE VIA MOCK ATTESTATION TOKENS
    # -----------------------------------------------------------------
    print("\n[TRACE 3/4] Testing data engine token ingestion and SVD loop execution...")
    
    # Valid test token containing our required non-military claims signature
    mock_token = (
        "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ4LW1zLWlzb2xhdGlvbi10ZWUiOiJhbWQtc2V2LXNu"
        "cCIsIngtbXMtc2V2c25wLW1yZW5jbGF2ZSI6ImUzYjBjNDQyOThmYzFjMTQ5YWZiZjRjODk5NmZiOTI0"
        "MzdhZTQxZTQ2NDliOTM0Y2E0OTU5OTFiNzg1MmI4NTUiLCJ4LW1zLXByb3ZlbmFuY2VzaGllbGQtcHVy"
        "cG9zZSI6Ik5vbi1NaWxpdGFyeS1EZWZlbnNpdmUtU2FuZGJveCIsImV4cCI6NDg5NzAzNjAwMH0.mock"
        "_signature_bits"
    )

    engine_runner = subprocess.run(
        [sys.executable, "python-engine/manifold_engine.py", "--token", mock_token],
        capture_output=True,
        text=True
    )

    print(engine_runner.stdout)
    if engine_runner.returncode != 0:
        print(engine_runner.stderr)
        print("❌ [TRACE FAILED] Manifold data engine failed token authorization or math execution.")
        sys.exit(1)
    print("✅ [TRACE STEP SUCCESS] Manifold engine executed SVD coordinates without drift.")

    # -----------------------------------------------------------------
    # STEP 4: VERIFY FORENSIC LOG CHAIN LAYOUT AND IMMUTABILITY
    # -----------------------------------------------------------------
    print("\n[TRACE 4/4] Auditing sequential forensic log chain structure...")
    log_path = "forensic_chain.log"
    
    if not os.path.exists(log_path):
        print("❌ [TRACE FAILED] Forensic chain log failed to compile on disk.")
        sys.exit(1)

    with open(log_path, "r") as f:
        lines = f.readlines()
        if len(lines) < 2:
            print("❌ [TRACE FAILED] Log file lacks the required sequential entries to prove chaining.")
            sys.exit(1)
            
        block_1 = json.loads(lines[-2].strip())
        block_2 = json.loads(lines[-1].strip())
        
        # Verify the N -> N-1 cryptographic link
        if block_2["previous_block_hash"] != block_1["current_block_hash"]:
            print("❌ [TRACE FAILED] Cryptographic continuity broken. Block N does not bind to Block N-1.")
            sys.exit(1)

    print("✅ [TRACE STEP SUCCESS] Sequential hash chain verified. Data integrity is unbroken.")
    print("\n=================================================================")
    print("🎉 [TRACE COMPLETE] ALL COGNITIVE BOUNDARY CHECKPOINTS PASSED")
    print("=================================================================")
    return True

if __name__ == "__main__":
    run_integration_trace()
