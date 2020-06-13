# gtkwhats

A simple wrapper written in rust and gtk-rs for whatsapp web with toggle for dark theme.

![](https://raw.githubusercontent.com/gigitux/gtkwhats/master/gtkwhats.gif)

## Development

For run a compiled version:

```
glib-compile-schemas ./data
GSETTINGS_SCHEMA_DIR=./data cargo run
```

For run a flatpak version:

```
flatpak-builder --install repo build-aux/com.gigitux.gtkwhats.json --force-clean --user -y
```
