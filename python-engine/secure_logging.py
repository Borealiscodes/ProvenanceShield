# ==========================================================================
# MODULE: ledger.py (Tamper-Evident SHA-256 Forensic Chain)
# PROJECT: Borealiscodes / Lean-Hallucination-Guard-Revised
# PURPOSE: Append-only, hash-chained audit ledger recording allostatic 0x0F
#          circuit-breaker events, CAUSA scores, and A7.2 triage diagnostics.
# ==========================================================================

import hashlib
import json
import os
import platform
import time
from pathlib import Path
from typing import Any


class ForensicChainError(Exception):
    """Raised when hash-chain verification detects tampering or truncation."""


class ForensicAuditLedger:
    GENESIS_HASH = "0" * 64

    def __init__(self, log_file_path: str = "lhguard_forensic_chain.jsonl"):
        self.log_path = Path(log_file_path)
        self.previous_hash = self._recover_tail_hash()

    def _recover_tail_hash(self) -> str:
        """Reads the final entry in the JSONL ledger to resume hash chaining."""
        if not self.log_path.exists() or self.log_path.stat().st_size == 0:
            return self.GENESIS_HASH

        with self.log_path.open("r", encoding="utf-8") as f:
            last_line = ""
            for line in f:
                if line.strip():
                    last_line = line.strip()

        if not last_line:
            return self.GENESIS_HASH

        last_block = json.loads(last_line)
        return str(last_block["current_block_hash"])

    def append_event(
        self,
        event_type: str,
        payload: dict[str, Any],
        opcode: int = 0x00,
    ) -> dict[str, Any]:
        """
        Appends a hash-chained event block to the ledger and flushes immediately
        to disk before any os._exit(15) process termination.
        """
        canonical_payload = json.dumps(payload, sort_keys=True, separators=(",", ":"))
        payload_hash = hashlib.sha256(canonical_payload.encode("utf-8")).hexdigest()

        block: dict[str, Any] = {
            "schema_version": "lhguard-ledger-v1",
            "unix_timestamp_ns": time.time_ns(),
            "event_type": event_type,
            "opcode": f"0x{opcode:02X}",
            "runtime_context": {
                "pid": os.getpid(),
                "platform": platform.platform(),
                "tee_hardware_attested": False,  # Honest disclosure: software enclave
            },
            "payload": payload,
            "payload_sha256": payload_hash,
            "previous_block_hash": self.previous_hash,
        }

        block_bytes = json.dumps(block, sort_keys=True, separators=(",", ":")).encode("utf-8")
        current_hash = hashlib.sha256(block_bytes).hexdigest()
        block["current_block_hash"] = current_hash

        with self.log_path.open("a", encoding="utf-8") as f:
            f.write(json.dumps(block, sort_keys=True) + "\n")
            f.flush()
            os.fsync(f.fileno())

        self.previous_hash = current_hash
        return block

    def verify_chain(self) -> tuple[bool, int]:
        """
        Re-hashes every block from genesis to tip to verify tamper-evidence.
        Returns (True, block_count) if intact; raises ForensicChainError if broken.
        """
        if not self.log_path.exists():
            return True, 0

        expected_prev = self.GENESIS_HASH
        count = 0

        with self.log_path.open("r", encoding="utf-8") as f:
            for line_num, raw_line in enumerate(f, start=1):
                line = raw_line.strip()
                if not line:
                    continue

                block = json.loads(line)
                recorded_hash = block.pop("current_block_hash", None)

                if block.get("previous_block_hash") != expected_prev:
                    raise ForensicChainError(
                        f"Chain link broken at line {line_num}: previous_block_hash mismatch."
                    )

                recomputed_bytes = json.dumps(
                    block, sort_keys=True, separators=(",", ":")
                ).encode("utf-8")
                recomputed_hash = hashlib.sha256(recomputed_bytes).hexdigest()

                if recorded_hash != recomputed_hash:
                    raise ForensicChainError(
                        f"Block tampering detected at line {line_num}: SHA-256 digest mismatch."
                    )

                expected_prev = recorded_hash
                count += 1

        return True, count

