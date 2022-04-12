project('termautomaton', 'cpp',
  version : '0.1',
  default_options : [
    'buildtype=release',
    'cpp_std=c++2a',
    'warning_level=3',
    ]
  )

cpp_files = ['src' / 'main.cc']

nlohmann_json = subproject('nlohmann_json')
nlohmann_json_dep = nlohmann_json.get_variable('nlohmann_json_dep')
cli11 = subproject('cli11')
cli11_dep = cli11.get_variable('CLI11_dep')

curses = dependency('curses')

exe = executable('termautomaton', cpp_files,
  dependencies : [curses, nlohmann_json_dep, cli11_dep],
  install : true)

test('basic', exe)
