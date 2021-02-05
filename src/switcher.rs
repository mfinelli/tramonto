use std::process::Command;

use detect_desktop_environment::DesktopEnvironment;

pub fn switch_theme(
    de: &DesktopEnvironment,
    theme: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    match de {
        DesktopEnvironment::Xfce => {
            let cmd = Command::new("xfconf-query")
                .arg("-c")
                .arg("xsettings")
                .arg("-p")
                .arg("/Net/ThemeName")
                .arg("-s")
                .arg(theme)
                .output();

            match cmd {
                Ok(_) => Ok(()),
                Err(_e) => Err("failed to update theme")?,
            }
        }
        _ => Err("you shouldn't be here!")?,
    }
}
