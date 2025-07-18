use std::io::{self, Write};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let dg_path = r"SYSTEM\CurrentControlSet\Control\DeviceGuard";
    let hvc_path = r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity";

    println!("📁 Creating/opening registry key: {}", dg_path);
    let (dg_key, _) = hklm.create_subkey(dg_path)?;

    println!("📁 Creating/opening registry key: {}", hvc_path);
    let (hvc_key, _) = hklm.create_subkey(hvc_path)?;

    dg_key.set_value("EnableVirtualizationBasedSecurity", &1u32)?;
    println!("✅ Set EnableVirtualizationBasedSecurity = 1");

    dg_key.set_value("RequirePlatformSecurityFeatures", &1u32)?;
    println!("✅ Set RequirePlatformSecurityFeatures = 1");

    dg_key.set_value("Locked", &0u32)?;
    println!("✅ Set Locked = 0");

    hvc_key.set_value("Enabled", &1u32)?;
    println!("✅ Set Enabled (HVCI) = 1");

    hvc_key.set_value("Locked", &0u32)?;
    println!("✅ Set Locked (HVCI) = 0");

    println!("\n✅ All DeviceGuard and HVCI registry values successfully updated.");

    print!("\n🔁 Do you want to reboot now to apply registry modification? [yes/no]: ");
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_lowercase();

    if answer == "yes" || answer == "y" {
        println!("🔄 Rebooting system...");
        Command::new("shutdown")
            .args(["/r", "/t", "0"])
            .spawn()?;
    } else {
        println!("⏹️ Reboot canceled. Please reboot manually later to apply changes.");
    }

    Ok(())
}
