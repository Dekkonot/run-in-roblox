use std::{
    io::BufWriter,
    path::Path,
    process::{Child, Command, Stdio},
    sync::LazyLock,
};

use fs_err::File;
use rbx_dom_weak::{InstanceBuilder, WeakDom};
use roblox_install::RobloxStudio;

use crate::{Error, Result};

static ROBLOX_INSTALL: LazyLock<RobloxStudio> =
    LazyLock::new(|| RobloxStudio::locate().expect("roblox studio should be installed"));

const PLUGIN_NAME: &str = "run-in-roblox.rbxm";

pub fn install_plugin(plugin_src: &[u8]) -> Result<()> {
    let plugin_folder = ROBLOX_INSTALL.plugins_path();
    if !plugin_folder.exists() {
        return Err(Error::PluginFolderNoExists);
    }

    fs_err::write(plugin_folder.join(PLUGIN_NAME), plugin_src)?;
    Ok(())
}

pub fn uninstall_plugin() -> Result<()> {
    let plugin_folder = ROBLOX_INSTALL.plugins_path();
    if plugin_folder.try_exists()? {
        fs_err::remove_file(plugin_folder.join(PLUGIN_NAME))?;
    }

    Ok(())
}

pub fn launch_studio_edit(place_file: &Path) -> Result<KillOnDrop> {
    let studio_location = ROBLOX_INSTALL.application_path();
    let child = Command::new(studio_location)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(place_file)
        .spawn()?;

    Ok(KillOnDrop(child))
}

pub fn launch_studio_auto_run(place_file: &Path, client_count: u32) -> Result<KillOnDrop> {
    let studio_location = ROBLOX_INSTALL.application_path();
    let roblox_folder = if cfg!(windows) {
        studio_location.parent().unwrap().parent().unwrap()
    } else {
        panic!("roblox folder not located. please open an issue with your platform and the location of roblox's install folder.")
    };
    fs_err::copy(place_file, roblox_folder.join("server.rbxl"))?;

    let child = Command::new(studio_location)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args([
            "-task",
            "StartServer",
            "-placeId",
            "0",
            "-universeId",
            "0",
            "-creatorId",
            "0",
            "-creatorType",
            "0",
            "-numTestServerPlayersUponStartup",
        ])
        .arg(client_count.to_string())
        .spawn()?;

    Ok(KillOnDrop(child))
}

pub fn create_empty_place(path: &Path) -> Result<()> {
    let dom = WeakDom::new(InstanceBuilder::new("DataModel"));
    let file = BufWriter::new(File::create(path)?);

    rbx_binary::to_writer(file, &dom, &[dom.root_ref()])?;

    Ok(())
}

pub struct KillOnDrop(Child);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        match self.0.kill() {
            Ok(_) => {}
            Err(e) => {
                if !std::thread::panicking() {
                    panic!("failed to kill child process: {}", e)
                } else {
                    eprintln!("failed to kill child process: {e}")
                }
            }
        }
    }
}
