project('gtkwhats',
        'rust',
        version: '0.1.0')
        dependency('glib-2.0')
dependency('gio-2.0')
dependency('gtk+-3.0')
subdir('data')
subdir('src')

datadir = get_option('prefix') / get_option('datadir')

application_id = 'com.gigitux.gtkwhats'

install_data(
    '@0@.desktop'.format(application_id),
    install_dir: datadir / 'applications'
)

install_data(
    '@0@.metainfo.xml'.format(application_id),
    install_dir: datadir / 'metainfo'
)

install_data(
    '@0@.svg'.format(application_id),
    install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps'
)

install_data('com.gigitux.gtkwhats.gschema.xml', install_dir: join_paths(datadir, 'glib-2.0', 'schemas'))

sources = [
    'main.rs',
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

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.source_root()
)