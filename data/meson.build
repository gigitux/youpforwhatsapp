gnome = import('gnome')

datadir = get_option('prefix') / get_option('datadir')

application_id = 'com.gigitux.youp'

gnome.compile_resources(
  application_id,
  '@0@.xml'.format(application_id),
  gresource_bundle: true,
  source_dir: '.',
  install_dir: datadir / application_id,
  install: true,
)


install_data(
    '@0@.desktop'.format(application_id),
    install_dir: datadir / 'applications'
)

install_data('resources/16x16/@0@.png'.format(application_id),
             install_dir : 'share/icons/hicolor/16x16/apps')
install_data('resources/32x32/@0@.png'.format(application_id),
             install_dir : 'share/icons/hicolor/24x24/apps')
install_data('resources/64x64/@0@.png'.format(application_id),
             install_dir : 'share/icons/hicolor/64x64/apps')
install_data('resources/128x128/@0@.png'.format(application_id),
             install_dir : 'share/icons/hicolor/128x128/apps')
install_data('resources/256x256/@0@.png'.format(application_id),
             install_dir : 'share/icons/hicolor/256x256/apps')

install_data(
    '@0@.metainfo.xml'.format(application_id),
    install_dir: datadir / 'metainfo'
)


install_data('com.gigitux.youp.gschema.xml', install_dir: join_paths(datadir, 'glib-2.0', 'schemas'))