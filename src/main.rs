use std::{
    fs::{self},
    path::PathBuf,
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

use clap::Parser;
use daemonize::Daemonize;
use rand::{rng, Rng};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    directory: PathBuf,

    #[arg(short, long)]
    minutes: u64,
}

fn main() {
    let cli = Args::parse();
    let path = cli.directory;
    let minutes = cli.minutes;

    if !path.is_dir() {
        println!("Path is not a directory");
        exit(1);
    }

    let daemonize = Daemonize::new()
        .pid_file("/tmp/kde-wallpaper-roulette.pid")
        .working_directory("/tmp");

    match daemonize.start() {
        Ok(_) => {
            let image_formats = vec![
                "apng", "png", "avif", "gif", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "svg", "svgz",
                "apng", "webp", "bmp", "ico", "cur", "tif", "tiff",
            ];

            let mut images = Vec::<PathBuf>::new();

            for entry in fs::read_dir(path).unwrap() {
                if let Ok(entry) = entry {
                    if let Some(extension) = entry.path().extension() {
                        let extension_str: String = extension.to_string_lossy().into();

                        if image_formats.iter().any(|e| e == &extension_str) {
                            images.push(entry.path());
                        }
                    }
                }
            }

            loop {
                let random_image_index = rng().random_range(0..(images.len() - 1));
                let image = images.get(random_image_index).unwrap();
                let image_path = r#"file://{wallpaper}"#.replace(
                    "{wallpaper}",
                    &image.clone().into_os_string().into_string().unwrap(),
                );

                let _wallaper_command = Command::new("sh")
                .arg("-c")
                .arg(
                    r#"dbus-send --session --dest=org.kde.plasmashell --type=method_call /PlasmaShell org.kde.PlasmaShell.evaluateScript 'string:
var allDesktops = desktops();
for (i=0; i<allDesktops.length; i++) {
    d = allDesktops[i];
    d.wallpaperPlugin = "org.kde.image";
    d.currentConfigGroup = Array("Wallpaper", "org.kde.image", "General");
    d.writeConfig("Image", "{wallpaper}");
}'"#
                    .replace(
                        "{wallpaper}",
                        &image_path,
                    ),
                )
                .spawn();

                let _lock_screen_command = Command::new("sh").arg("-c").arg(r#"kwriteconfig6 --file ~/.config/kscreenlockerrc --group Greeter --group Wallpaper --group org.kde.image --group General --key Image "{wallpaper}""#.replace("{wallpaper}", &image_path)).spawn();

                sleep(Duration::from_secs(minutes * 60));
            }
        }
        Err(_) => {}
    }
}
