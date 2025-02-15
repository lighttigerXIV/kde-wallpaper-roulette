# Kde Wallpaper Roulette

A cli tool to apply a random wallpaper in a folder in KDE. It applies the SAME wallpaper on all monitors.
I was forced to do this because KDE can't apply the same randomized wallpaper on all monitors when using the builtin slideshow.

# Install

To install the app run:

```sh
cargo install --git https://github.com/lighttigerXIV/kde-wallpaper-roulette.git
```

Alternatively you can download the binary in the [releases](https://github.com/lighttigerXIV/kde-wallpaper-roulette/releases)

# Usage

To make this work you need to send a folder and the time in minutes:

```sh
# Change wallpaper inside wallpapers folder every hour
kde-wallpaper-roulette -d ~/Pictures/Wallpapers -m 60
```

If for whatever reason you need to kill the program you can by running this commands

```sh
kill $(cat /tmp/kde-wallpaper-roulette.pid)
rm /tmp/kde-wallpaper-roulette.pid
```

# Auto Start

To autostart you need to make a desktop file in `~/.config/autostart` and run the command

```
[Desktop Entry]
Exec=sh -c "kde-wallpaper-roulette -d {your wallpapers folder} -m {the amount of minutes}"
Icon=application-x-shellscript
Name=wallpaper-roulette.sh
Type=Application
X-KDE-AutostartScript=true
```
