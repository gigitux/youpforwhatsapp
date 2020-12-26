project('youp',
        'rust',
        version: '2.0.0')
        dependency('glib-2.0')
dependency('gio-2.0')
dependency('gtk+-3.0')

subdir('data')
subdir('src')

glib_compile_resources = find_program('glib-compile-resources', required: true)
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
meson.add_install_script('build-aux/meson_post_install.py')
