project('termautomaton', 'cpp',
  version : '0.1',
  default_options : [
    'buildtype=release',
    'cpp_std=c++20',
    'warning_level=3',
    ]
  )

nlohmann_json = subproject('nlohmann_json')
nlohmann_json_dep = nlohmann_json.get_variable('nlohmann_json_dep')
cli11 = subproject('cli11')
cli11_dep = cli11.get_variable('CLI11_dep')
curses = dependency('curses')

subdir('src')

test('basic', exe)
