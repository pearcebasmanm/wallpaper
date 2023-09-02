use std::{
    fs::{self, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

use detect_desktop_environment::DesktopEnvironment;
use itertools::Itertools;
use toml::Table;

fn main() {
    let project_directory = dirs::home_dir()
        .unwrap()
        .join(".var/app/com.max.Wallpapers/");
    fs::create_dir_all(&project_directory).unwrap();

    let config: Table = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(project_directory.join("config.toml"))
        .ok()
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents).ok()?;
            contents.parse().ok()
        })
        .unwrap_or_default();

    let recursive = config
        .get("recursive")
        .is_some_and(|value| value.as_bool() == Some(true));

    let interval: u64 = config
        .get("interval-seconds")
        .and_then(|interval| interval.as_integer()?.try_into().ok())
        .unwrap_or(5);

    let images = find_images(&project_directory, recursive);

    for image in images.iter().cycle() {
        change_wallpaper(image);
        thread::sleep(Duration::from_secs(interval));
    }
}

const IMAGE_EXTENSIONS: [&str; 4] = ["png", "jpg", "jpeg", "svg"];

fn find_images(path: &Path, recursive: bool) -> Vec<PathBuf> {
    path.read_dir()
        .unwrap()
        .flatten()
        .flat_map(|entry| {
            let file_type = entry.file_type().ok()?;
            if recursive && file_type.is_dir() {
                Some(find_images(&entry.path(), true))
            } else if IMAGE_EXTENSIONS.contains(&entry.path().extension()?.to_str()?) {
                Some(vec![entry.path()])
            } else {
                None
            }
        })
        .concat()
}

fn change_wallpaper(image: &Path) {
    let desktop_environment = match DesktopEnvironment::detect() {
        Some(DesktopEnvironment::Gnome) => "gnome",
        Some(DesktopEnvironment::Cinnamon) => "cinnamon",
        Some(DesktopEnvironment::Mate) => "mate",
        _ => panic!("Unsupported desktop environment")
    };
    for theme in ["picture-uri", "picture-uri-dark"] {
        Command::new("bash")
            .arg("-c")
            .arg(&format!(
                "gsettings set org.{desktop_environment}.desktop.background {theme} file://{}",
                image.display()
            ))
            .output()
            .unwrap();
    }
}