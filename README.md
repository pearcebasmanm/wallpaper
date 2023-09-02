# Wallpaper

A toy wallpaper slideshow program for GNOME, Cinnamon, and Mate. (Cinnamon and Mate untested)

It will create a `com.max.Wallpaper` directory in `~/.var/app/` that contains a `config.toml`

Add images to that directory, and they will register the next time you run the program.

Configure the program in the `config.toml` file with these options:
- recursive (bool): whether or not images in sub-directories of `com.max.Wallpaper` also are registered.
- interval-seconds (positive int): how long to wait between switching wallpapers

Example
```toml
recursive = true
interval-seconds = 120
```