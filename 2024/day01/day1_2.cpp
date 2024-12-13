// Copyright (c) 2024 Richard Garber. All Rights Reserved.
#include <algorithm>
#include <cassert>
#include <fstream>
#include <print>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>
std::pair<std::vector<int>, std::vector<int>> readNumbers(const char *inp) {
  std::ifstream inp_stream{inp};
  std::vector<int> left;
  std::vector<int> right;
  std::string line;
  while (std::getline(inp_stream, line)) {
    int leftNum;
    int rightNum;
    std::from_chars(line.c_str(), line.c_str() + 5, leftNum);
    std::from_chars(line.c_str() + 8, line.c_str() + 13, rightNum);
    left.emplace_back(leftNum);
    right.emplace_back(rightNum);
  }
  assert(left.size() == right.size());
  return {std::move(left), std::move(right)};
}

int main(int argc, char *argv[]) {
  auto [left, right] =
      readNumbers("/home/rgarber11/advent_of_code/2024/day1/input");
  std::unordered_map<int, int> rightOcurrences;
  for (int i : right) {
    rightOcurrences[i]++;
  }
  long long ans = std::ranges::fold_left(
      left, 0LL, [&rightOcurrences](long long acc, int val) {
        return acc + val * rightOcurrences[val];
      });
  std::println("The answer is: {}", ans);
  return 0;
}
