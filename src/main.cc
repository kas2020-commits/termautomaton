#include <CLI/CLI.hpp>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <nlohmann/json.hpp>
#include <curses.h>

int main(int argc, char *argv[]) {

  CLI::App app{"App description"};
  std::string filename{"default"};
  app.add_option("-f, --file", filename, "A help string");
  try {
    app.parse(argc, argv);
  } catch (const CLI::ParseError &e) {
    return app.exit(e);
  }
  std::cout << filename << "\n";

  nlohmann::json j {{"pi", 3.141},
                      {"happy", true},
                      {"name", "Niels"},
                      {"nothing", nullptr},
                      {"answer", {{"everything", 42}}},
                      {"list", {1, 0, 2}},
                      {"object", {{"currency", "USD"}, {"value", 42.99}}}};
  std::ofstream o("pretty.json");
  o << std::setw(4) << j << std::endl;

  initscr();
  printw("Hello World !");
  refresh();
  getch();
  endwin();
}
