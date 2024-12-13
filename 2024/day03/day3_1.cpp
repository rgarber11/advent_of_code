#include <charconv>
#include <format>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <iostream>
#include <optional>
#include <print>
enum class PARSER_STATES {
  LETTER_M,
  LETTER_U,
  LETTER_L,
  OPEN_PAREN,
  NUMBER_1_1,
  NUMBER_1_2,
  NUMBER_1_3,
  COMMA,
  NUMBER_2_1,
  NUMBER_2_2,
  NUMBER_2_3,
  CLOSE_PAREN,
  JUNK,
};
template <> struct std::formatter<PARSER_STATES> {
  bool val_{false};
  constexpr auto parse(std::format_parse_context &ctx) {
    auto pos = ctx.begin();
    while (pos != ctx.end() && *pos != '}') {
      if (*pos == 'v') {
        val_ = true;
      }
      pos++;
    }
    return pos;
  }
  auto format(const PARSER_STATES &state, std::format_context &ctx) const {
    if (val_) {
      return std::format_to(ctx.out(), "{}", static_cast<int>(state));
    }
    switch (state) {
    case PARSER_STATES::LETTER_M:
      return std::format_to(ctx.out(), "{}", "LETTER_M");
    case PARSER_STATES::LETTER_U:
      return std::format_to(ctx.out(), "{}", "LETTER_U");
    case PARSER_STATES::LETTER_L:
      return std::format_to(ctx.out(), "{}", "LETTER_L");
    case PARSER_STATES::OPEN_PAREN:
      return std::format_to(ctx.out(), "{}", "OPEN_PAREN");
    case PARSER_STATES::NUMBER_1_1:
      return std::format_to(ctx.out(), "{}", "NUMBER_1_1");
    case PARSER_STATES::NUMBER_1_2:
      return std::format_to(ctx.out(), "{}", "NUMBER_1_2");
    case PARSER_STATES::NUMBER_1_3:
      return std::format_to(ctx.out(), "{}", "NUMBER_1_3");
    case PARSER_STATES::COMMA:
      return std::format_to(ctx.out(), "{}", "COMMA");
    case PARSER_STATES::NUMBER_2_1:
      return std::format_to(ctx.out(), "{}", "NUMBER_2_1");
    case PARSER_STATES::NUMBER_2_2:
      return std::format_to(ctx.out(), "{}", "NUMBER_2_2");
    case PARSER_STATES::NUMBER_2_3:
      return std::format_to(ctx.out(), "{}", "NUMBER_2_3");
    case PARSER_STATES::CLOSE_PAREN:
      return std::format_to(ctx.out(), "{}", "CLOSE_PAREN");
    case PARSER_STATES::JUNK:
      return std::format_to(ctx.out(), "{}", "JUNK");
      break;
    }
    return std::format_to(ctx.out(), "{}", "Error");
  }
};
long long calculateMultiples(const char *filePath) {
  std::ifstream file{filePath};
  auto parserState = PARSER_STATES::JUNK;
  int num1{};
  int num2{};
  std::array<char, 3> buf;
  long long ans = 0;
  while (char c = file.get()) {
    if (file.eof()) {
      break;
    }
    switch (parserState) {
    case PARSER_STATES::LETTER_M:
      if (c == 'u') {
        parserState = PARSER_STATES::LETTER_U;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::LETTER_U:
      if (c == 'l') {
        parserState = PARSER_STATES::LETTER_L;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::LETTER_L:
      if (c == '(') {
        parserState = PARSER_STATES::OPEN_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::OPEN_PAREN:
      if (c >= '0' && c <= '9') {
        buf[0] = c;
        parserState = PARSER_STATES::NUMBER_1_1;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_1_1:
      if (c >= '0' && c <= '9') {
        buf[1] = c;
        parserState = PARSER_STATES::NUMBER_1_2;
      } else if (c == ',') {
        std::from_chars(buf.data(), buf.data() + 1, num1);
        parserState = PARSER_STATES::COMMA;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_1_2:
      if (c >= '0' && c <= '9') {
        buf[2] = c;
        parserState = PARSER_STATES::NUMBER_1_3;
      } else if (c == ',') {
        std::from_chars(buf.data(), buf.data() + 2, num1);
        parserState = PARSER_STATES::COMMA;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_1_3:
      if (c == ',') {
        std::from_chars(buf.data(), buf.data() + 3, num1);
        parserState = PARSER_STATES::COMMA;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::COMMA:
      if (c >= '0' && c <= '9') {
        buf[0] = c;
        parserState = PARSER_STATES::NUMBER_2_1;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_2_1:
      if (c >= '0' && c <= '9') {
        buf[1] = c;
        parserState = PARSER_STATES::NUMBER_2_2;
      } else if (c == ')') {
        std::from_chars(buf.data(), buf.data() + 1, num2);
        parserState = PARSER_STATES::CLOSE_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_2_2:
      if (c >= '0' && c <= '9') {
        buf[2] = c;
        parserState = PARSER_STATES::NUMBER_2_3;
      } else if (c == ')') {
        std::from_chars(buf.data(), buf.data() + 2, num2);
        parserState = PARSER_STATES::CLOSE_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_2_3:
      if (c == ')') {
        std::from_chars(buf.data(), buf.data() + 3, num2);
        parserState = PARSER_STATES::CLOSE_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::CLOSE_PAREN:
      ans += num1 * num2;
      if (c == 'm') {
        parserState = PARSER_STATES::LETTER_M;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
    case PARSER_STATES::JUNK:
      if (c == 'm') {
        parserState = PARSER_STATES::LETTER_M;
      }
      break;
    }
  }
  return ans;
}
int main(int argc, char *argv[]) {
  long long ans =
      calculateMultiples("/home/rgarber11/advent_of_code/2024/day3/input");
  std::println("The answer is {}.", ans);
  return 0;
}
