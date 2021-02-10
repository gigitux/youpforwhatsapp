sources = [
    'main.rs',
    'controllers/ui.rs',
    'controllers/settings.rs',
    'controllers/about.rs',
    'controllers/mod.rs',
    'controllers/indicator.rs',
    'controllers/headbar.rs',
    'models/applications.rs',
    'models/constants.rs',
    'models/mod.rs',
    'views/ui.rs',
    'views/settings.rs',
    'views/mod.rs',
    'views/indicator.rs',
    'views/webview.rs',
    'views/headbar.rs'
]

cargo_script = find_program(join_paths(meson.source_root(), 'build-aux/cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.build_root(),
    meson.source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    meson.project_name(),
  ]
)
