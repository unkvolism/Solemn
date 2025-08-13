use clap::Parser;
use colored::Colorize;
use std::ffi::c_void;
use std::io;
use std::mem::size_of;
use std::process::exit;
use winreg::enums::*;
use winreg::RegKey;
use windows::core::Error;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

#[derive(Parser, Debug)]
#[command(
    name = "Solemn",
    about = "Add a driver by name in HVCI Blocklist",
    version = "0.1.0",
    author = "sorahed"
)]

struct Args {
    // receive the name of the driver who is blocked
    #[arg(short, long, required = true, help = "The name of the driver who is be blocked")]
    driver_name: String,
}

const REG_PATH: &str = r"SYSTEM\CurrentControlSet\Control\CI";
const VALUE_NAME: &str = "HvciDisallowedImages";

fn is_elevated() -> Result<bool, Error> {
    unsafe {
        let mut h_token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token)?;

        let mut token_info = TOKEN_ELEVATION::default();
        let mut return_length = size_of::<TOKEN_ELEVATION>() as u32;

        GetTokenInformation(
            h_token,
            TokenElevation,
            Some(&mut token_info as *mut _ as *mut c_void),
            return_length,
            &mut return_length,
        )?;

        CloseHandle(h_token).expect("[-] Failed to close token handle");

        Ok(token_info.TokenIsElevated != 0)
    }
}

fn main() -> io::Result<()> {
    // receive args
    let args = Args::parse();
    let driver_add = args.driver_name;

    if !is_elevated().unwrap_or(false) {
        eprintln!("{}", "\nError: Could not open the required registry key.".red().bold());
        eprintln!("{}", "[-] Reason: This program must be run with administrator privileges.".red());
        eprintln!("{}", format!("[!] Path: HKLM\\{}", REG_PATH).red());
        exit(1);
    } else {
        println!("{}", "[+] Initializing HVCI Blocklist Manager...".cyan());
    }

    // open a subkey with write/read perms (needs run as administrator)
    let key = match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(REG_PATH, KEY_READ | KEY_WRITE) {
        Ok(key) => key,
        Err(e) => {
            return Err(e);
        }
    };


    println!("{}", "[!] Querying registry for the HVCI blocklist...".cyan());

    let mut current_drivers: Vec<String> = match key.get_value(VALUE_NAME) {
        // the value already exists and returns the actual list
        Ok(drivers) => {
            println!("{}", "[+] Found existing blocklist.".green());
            drivers
        }
        // the value has not found | initialize a new empty list
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("{}", format!("[!] Blocklist not found. A new one will be created.").yellow());
            Vec::new()
        }
        // another type of error occurred while reading
        Err(e) => {
            eprintln!("{}", format!("\nError: Failed to read the registry value '{}'.", VALUE_NAME).red().bold());
            eprintln!("{}", format!("[-] System Error: {}", e).red());
            return Err(e)
        }
    };


    // verify if the driver already exists in the blocklist to avoid duplicates
    if current_drivers.iter().any(|d| d.eq_ignore_ascii_case(&driver_add)) {
        println!("{}", format!("\nInfo: The driver '{}' is already on the blocklist. No changes made.", driver_add).yellow());
        return Ok(());
    }

    // add a new driver to the list
    current_drivers.push(driver_add);
    current_drivers.sort();

    println!("{}", "[!] Writing updated blocklist to the registry...".cyan());

    // write the update list (new or modified) back on the registry
    match key.set_value(VALUE_NAME, &current_drivers) {
        Ok(_) => {
            println!("{}", "\n[+] Success! The blocklist was updated.".green().bold());
            println!("{}", "\n[!] Important: A system reboot is required for changes to take effect.".yellow().bold());
            println!("{}", "\n--- Current Blocklist ---".bold());
            for driver in current_drivers {
                println!("- {}", driver);
            }
        }
        Err(e) => {
            eprintln!("{}", "\nError: Failed to write the updated list to the registry.".red().bold());
            eprintln!("{}", format!("[-] System Error: {}", e).red());
            return Err(e)
        }
    }
    Ok(())
}