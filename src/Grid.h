#pragma once

#include <array>
#include <cstddef>

using stateView = std::array<bool, 9>;
using updateFunction = bool (*)(stateView);

// A class which will store the automata state.
template <std::size_t Rows, std::size_t Cols> class Grid {
private:
  std::array<bool, Rows * Cols> State;

  constexpr std::size_t index(std::size_t row, std::size_t col) const noexcept {
    return row * Cols + col;
  }

  // Grab a copy of the neighbors at some index. The convention for ordering
  // of the neighbors is as follows:
  // top-left, top-mid, top-right
  // mid-left, center, mid-right
  // bot-left, bot-mid, bot-right
  constexpr stateView neighbors(std::size_t row, std::size_t col) {
    const bool dead = false;
    stateView x{};
    // top row
    x[0] = (row > 0 && col > 0) ? State[index(row - 1, col - 1)] : dead;
    x[1] = (row > 0) ? State[index(row - 1, col)] : dead;
    x[2] = (row > 0 && col < Cols) ? State[index(row - 1, col + 1)] : dead;
    // center row
    x[3] = (col > 0) ? State[index(row, col - 1)] : dead;
    x[4] = State[index(row, col)];
    x[5] = (col < Cols) ? State[index(row, col + 1)] : dead;
    // bottom row
    x[6] = (row < Rows && col > 0) ? State[index(row + 1, col - 1)] : dead;
    x[7] = (row < Rows) ? State[index(row + 1, col)] : dead;
    x[8] = (row < Rows && col < Cols) ? State[index(row + 1, col + 1)] : dead;
    return x;
  }

public:
  // Grab the rows and columns of the game state
  constexpr auto rows() const noexcept { return Rows; }
  constexpr auto cols() const noexcept { return Cols; }
  constexpr auto size() const noexcept { return Rows * Cols; }
  constexpr auto state() { return State; };

  constexpr void update(updateFunction foo) {
    for (std::size_t idx = 0; idx < size(); idx++) {
      auto col = idx % Cols;
      auto row = (idx - col) / Cols;
      State[idx] = foo(neighbors(row, col));
    }
  }
};
