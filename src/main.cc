#include <fstream>
#include <iomanip>
#include <iostream>
#include <nlohmann/json.hpp>

int main() {
  nlohmann::json j = {{"pi", 3.141},
                      {"happy", true},
                      {"name", "Niels"},
                      {"nothing", nullptr},
                      {"answer", {{"everything", 42}}},
                      {"list", {1, 0, 2}},
                      {"object", {{"currency", "USD"}, {"value", 42.99}}}};
  std::ofstream o("pretty.json");
  o << std::setw(4) << j << std::endl;
  std::cout << "Hello World\n";
}
