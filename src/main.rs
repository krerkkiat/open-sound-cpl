use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    // In PowerShell, 'Show-ControlPanelItem -Name Sound'
    // will also work.
    Command::new("powershell")
        .args(["-Command", "Show-ControlPanelItem -Name Sound"])
        .spawn()?;

    Ok(())
}
