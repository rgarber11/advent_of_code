// Copyright (c) 2024 Richard Garber. All Rights Reserved.
#include <cassert>
#include <charconv>
#include <fstream>
#include <print>
#include <string>
#include <utility>
bool checkIfValid(int num1, int num2, bool increasing) {
  if (!increasing) {
    std::swap(num1, num2);
  }
  return num2 - num1 <= 3 && num2 - num1 >= 1;
}
int readNumbers(const char *inp) {
  std::ifstream inp_stream{inp};
  std::string line;
  int ans = 0;
  while (std::getline(inp_stream, line)) {
    int num1;
    int num2;
    int begin = 0;
    while (begin < line.size() && line[begin] != ' ') {
      begin++;
    }
    std::from_chars(line.data(), line.data() + begin, num1);
    int end = begin + 1;
    while (end < line.size() && line[end] != ' ') {
      end++;
    }
    std::from_chars(line.data() + begin + 1, line.data() + end, num2);
    bool increasing = num2 > num1;
    if (!checkIfValid(num1, num2, increasing)) {
      continue;
    }
    bool worked = true;
    begin = end + 1;
    for (end = begin; end < line.size(); ++end) {
      if (line[end] == ' ') {
        num1 = num2;
        std::from_chars(line.data() + begin, line.data() + end, num2);
        if (!checkIfValid(num1, num2, increasing)) {
          worked = false;
          break;
        }
        begin = end + 1;
      }
    }
    if (worked) {
      num1 = num2;
      std::from_chars(line.data() + begin, line.data() + line.size(), num2);
      if (!checkIfValid(num1, num2, increasing)) {
        continue;
      }
      ans++;
    }
  }
  return ans;
}

int main(int argc, char *argv[]) {
  int ans = readNumbers("/home/rgarber11/advent_of_code/2024/day2/input");
  std::println("The answer is: {}", ans);
  return 0;
}
