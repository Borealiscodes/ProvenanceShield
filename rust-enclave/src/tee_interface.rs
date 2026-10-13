// rust-enclave/src/tee_interface.rs
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::mem::MaybeUninit;

// --- NATIVE LINUX KERNEL IOCTL ARCHITECTURE CONFIGURATION ---
// Definitions derived from the Linux kernel drivers for SEV-SNP and TDX guests.
const SEV_GUEST_IOC_MAGIC: u8 = b'S';
const TDX_GUEST_IOC_MAGIC: u8 = b'T';

// System request sequence macros to trigger cryptographic firmware reports
const SNP_GET_REPORT_SEQ: u8 = 0x1;
const TDX_GET_REPORT_SEQ: u8 = 0x2;

/// Memory-aligned structure mapping the exact layout expected by the AMD SEV-SNP firmware.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SnpReportReq {
    pub user_data: [u8; 64], // Nonce/Session token to prevent replay attacks
    pub vmpl: u32,           // Virtual Machine Privilege Level (typically 0 for TEE roots)
    pub rsvd: [u8; 28],      // Strict memory padding required by hardware alignment specs
}

/// Memory-aligned structure mapping the exact layout expected by the Intel TDX firmware.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TdxReportReq {
    pub report_data: [u8; 64], // High-risk execution telemetry anchor bits
    pub rsvd: [u8; 32],       // Typo fixed: Removed extra keyboard character to restore memory layout
}

pub struct HardwareInterface;

impl HardwareInterface {
    /// Executes a low-level Linux ioctl system call to extract the AMD SEV-SNP hardware report.
    pub unsafe fn request_snp_hardware_report(file: &File, mut request: SnpReportReq) -> Result<[u8; 1128], &'static str> {
        println!("[KERNEL-IOCTL] Compiling raw memory layout blocks for AMD SEV-SNP...");
        
        let mut out_report = MaybeUninit::<[u8; 1128]>::uninit();
        
        // Stably derived Linux _IOWR ioctl tracking number for the SEV-SNP guest driver sequence.
        // Replaces brittle compile-time macro extraction blocks.
        let ioctl_num: libc::c_ulong = 0xc0405301; 

        let fd = file.as_raw_fd();
        let ret = libc::ioctl(fd, ioctl_num, &mut request, out_report.as_mut_ptr());

        if ret != 0 {
            return Err("OS Kernel rejected the SEV-SNP ioctl instruction. Hardware verification aborted.");
        }

        println!("[KERNEL-IOCTL] Successfully extracted 1128-byte hardware signature from security coprocessor.");
        Ok(out_report.assume_init())
    }

    /// Executes a low-level Linux ioctl system call to extract the Intel TDX hardware report.
    pub unsafe fn request_tdx_hardware_report(file: &File, mut request: TdxReportReq) -> Result<[u8; 1024], &'static str> {
        println!("[KERNEL-IOCTL] Compiling raw memory layout blocks for Intel TDX...");
        
        let mut out_report = MaybeUninit::<[u8; 1024]>::uninit();
        
        // Stably derived Linux _IOWR ioctl tracking number for the Intel TDX guest driver sequence.
        // Explicitly maps to the required platform unsigned long primitive layout.
        let ioctl_num: libc::c_ulong = 0xc0405402; 

        let fd = file.as_raw_fd();
        let ret = libc::ioctl(fd, ioctl_num, &mut request, out_report.as_mut_ptr());

        if ret != 0 {
            return Err("OS Kernel rejected the Intel TDX ioctl instruction. Trust domain validation failed.");
        }

        println!("[KERNEL-IOCTL] Successfully extracted 1024-byte trust domain report from silicon.");
        Ok(out_report.assume_init())
    }
}
