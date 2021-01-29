# Youp - a whatsapp wrapper written with Rust and GTK3

![Youp for whatsapp](https://raw.githubusercontent.com/gigitux/youpforwhatsapp/master/youp.gif)

Whatsapp wrapper written with Rust and GTK3

## About

Youp makes it makes the use of whatsapp more convenient thanks to the possibility of:

- Make whatsapp truly fullscreen
- Conveniently switch to dark mode
- Add icon in systray with different icon if there is new messages
- Desktop Notification

### Development

- Rust 2018 version
- Gtk 3.36

## Installation

### Flatpak

Youp can be installed on all distributions supporting [Flatpak](http://flatpak.org/) from [Flathub](https://flathub.org/apps/details/com.gigitux.youp).

## Frequently Asked Questions

## Contribute

If you want to help make Youp better the easiest thing you can do is to
[report issues and feature requests](https://github.com/gigitux/youpforwhatsapp/issues).
Or you can help in development and translation.

### Development

You are welcome to contribute code and provide pull requests for Youp.

For run a compiled version:

```
glib-compile-schemas ./data
GSETTINGS_SCHEMA_DIR=./data cargo run
```

For run a flatpak version:

```
flatpak-builder --install repo build-aux/com.gigitux.gtkwhats.json --force-clean --user -y
```

