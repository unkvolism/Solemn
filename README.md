# Solemn

<p align="center">
  <img src="https://static.wikia.nocookie.net/yugioh/images/5/5a/SolemnJudgment-TF04-JP-VG.png" width="300" alt="Solemn"/>
</p>

A command-line tool to manually add drivers to the HVCI (Memory Integrity) custom blocklist on Windows, based on research by Yarden Shafir.

## 📖 Background

### What is HVCI?

**Hypervisor-Protected Code Integrity (HVCI)**, often marketed as **Memory Integrity** in Windows Security settings, is a critical virtualization-based security (VBS) feature. It uses the Microsoft Hyper-V hypervisor to create a secure, isolated environment that protects the Windows kernel's Code Integrity (CI) process.

In simple terms, HVCI ensures that all code running in the Windows kernel — especially drivers — is securely signed and has not been tampered with. This provides strong protection against many advanced forms of malware that attempt to inject malicious code into the kernel.

### The `HvciDisallowedImages` Registry Value

While HVCI relies on a primary, Microsoft-managed blocklist for vulnerable drivers, there is a lesser-known, powerful feature for manual intervention: the `HvciDisallowedImages` registry value.

This feature, explored in detail by researcher Yarden Shafir, allows a system administrator to create a custom, on-box blocklist. By creating a `REG_MULTI_SZ` value at:

```
HKLM\SYSTEM\CurrentControlSet\Control\CI
```

An administrator can list the filenames of any drivers they wish to block. When Windows boots, the Code Integrity subsystem reads this list and prevents any driver in it from loading, regardless of its signature.

**Solemn** provides a safe and convenient command-line interface for managing this list.

## ✨ Features

- Adds a specified driver to the `HvciDisallowedImages` blocklist.
- Checks for administrator privileges before attempting any modifications.
- Automatically creates the registry value if it doesn't already exist.
- Prevents duplicate driver entries from being added to the list.
- Provides clean, colored command-line output for status, success, and error messages.

## 🛠️ Building

To build Solemn, you will need the Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).

```bash
# 1. Clone the repository
git clone https://github.com/your-username/solemn.git
cd solemn

# 2. Build the project in release mode
cargo build --release
```

## 🚀 Usage

Solemn must be run from a terminal (Command Prompt, PowerShell, etc.) with Administrator privileges.

The command accepts the driver's filename using the `-d` or `--driver-name` flag.

```bash
# Basic usage
.\solemn.exe --driver-name CursedDriver.sys

# Using the short flag
.\solemn.exe -d AnotherDriver.dll
```

## ⚠️ Warning

This tool modifies a critical part of the Windows Registry related to system boot and security. Blocking an essential system driver (e.g., disk drivers like `storahci.sys`, `nvme.sys`, core kernel files, or critical chipset drivers) will render your system unbootable.

- Use this tool at your own risk.  
- Only block drivers that you know are non-essential or are part of a specific security test.  
- Always have a system recovery plan, such as a Windows installation USB, to access recovery tools and edit the registry offline if needed.


## 🧪 Tested Environment

Solemn was tested on:

```
Microsoft Windows [Version 10.0.26100.4946] (Windows 11)
```

> **Important:**  
> - This tool will only work if **HVCI (Memory Integrity)** is enabled on the machine.  
> - The blocklist feature operates **only by matching the driver filename on disk** — it does not perform hash or signature checks.


## 🙏 Credits and Acknowledgments

This tool and its underlying concept are based on the incredible research presented by **Yarden Shafir**.

The technique of using `HvciDisallowedImages` was demonstrated in her talk *"Your Mitigations Are My Opportunities"* at OffensiveCon 2023. Solemn is a practical Rust implementation of the method she showcased.

- [Yarden Shafir on X (Twitter)](https://x.com/yarden_shafir)  
- [OffensiveCon 2023 Talk — *Your Mitigations Are My Opportunities*](https://www.youtube.com/watch?v=YnxGW8Fvqvk)
