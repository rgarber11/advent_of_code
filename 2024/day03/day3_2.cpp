#include <charconv>
#include <format>
#include <fstream>
#include <iosfwd>
#include <print>
enum class PARSER_STATES {
  LETTER_M,
  LETTER_U,
  LETTER_L,
  LETTER_D,
  LETTER_O,
  LETTER_N,
  APOSTROPHE,
  LETTER_T,
  MUL_OPEN_PAREN,
  NUMBER_1_1,
  NUMBER_1_2,
  NUMBER_1_3,
  COMMA,
  NUMBER_2_1,
  NUMBER_2_2,
  NUMBER_2_3,
  MUL_CLOSE_PAREN,
  DISABLED,
  DO_OPEN_PAREN,
  DONT_OPEN_PAREN,
  DO_CLOSED_PAREN,
  DONT_CLOSED_PAREN,
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
    case PARSER_STATES::MUL_OPEN_PAREN:
      return std::format_to(ctx.out(), "{}", "MUL_OPEN_PAREN");
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
    case PARSER_STATES::MUL_CLOSE_PAREN:
      return std::format_to(ctx.out(), "{}", "MUL_CLOSE_PAREN");
    case PARSER_STATES::JUNK:
      return std::format_to(ctx.out(), "{}", "JUNK");
    case PARSER_STATES::LETTER_D:
      return std::format_to(ctx.out(), "{}", "LETTER_D");
    case PARSER_STATES::LETTER_O:
      return std::format_to(ctx.out(), "{}", "LETTER_O");
    case PARSER_STATES::LETTER_N:
      return std::format_to(ctx.out(), "{}", "LETTER_N");
    case PARSER_STATES::LETTER_T:
      return std::format_to(ctx.out(), "{}", "LETTER_T");
    case PARSER_STATES::DISABLED:
      return std::format_to(ctx.out(), "{}", "DISABLED");
    case PARSER_STATES::DO_OPEN_PAREN:
      return std::format_to(ctx.out(), "{}", "DO_OPEN_PAREN");
    case PARSER_STATES::DONT_OPEN_PAREN:
      return std::format_to(ctx.out(), "{}", "DONT_OPEN_PAREN");
    case PARSER_STATES::DO_CLOSED_PAREN:
      return std::format_to(ctx.out(), "{}", "DO_CLOSED_PAREN");
    case PARSER_STATES::DONT_CLOSED_PAREN:
      return std::format_to(ctx.out(), "{}", "DONT_CLOSED_PAREN");
    case PARSER_STATES::APOSTROPHE:
      return std::format_to(ctx.out(), "{}", "APOSTROPHE");
    }
    return std::format_to(ctx.out(), "{}", "Error");
  }
};
long long calculateMultiples(const char *filePath) {
  std::ifstream file{filePath};
  auto parserState = PARSER_STATES::JUNK;
  auto prevContext = PARSER_STATES::JUNK;
  int num1{};
  int num2{};
  std::array<char, 3> buf;
  long long ans = 0;
  while (char c = file.get()) {
    if (file.eof()) {
      break;
    }
    std::println("Previous State: {}, Current Character {}", parserState, c);
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
        parserState = PARSER_STATES::MUL_OPEN_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::MUL_OPEN_PAREN:
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
        parserState = PARSER_STATES::MUL_CLOSE_PAREN;
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
        parserState = PARSER_STATES::MUL_CLOSE_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::NUMBER_2_3:
      if (c == ')') {
        std::from_chars(buf.data(), buf.data() + 3, num2);
        parserState = PARSER_STATES::MUL_CLOSE_PAREN;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::MUL_CLOSE_PAREN:
      ans += num1 * num2;
      if (c == 'm') {
        parserState = PARSER_STATES::LETTER_M;
      } else if (c == 'd') {
        parserState = PARSER_STATES::LETTER_D;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::JUNK:
      if (c == 'm') {
        parserState = PARSER_STATES::LETTER_M;
      } else if (c == 'd') {
        parserState = PARSER_STATES::LETTER_D;
      }
      break;
    case PARSER_STATES::LETTER_D:
      if (c == 'o') {
        parserState = PARSER_STATES::LETTER_O;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::LETTER_O:
      if (c == 'n') {
        parserState = PARSER_STATES::LETTER_N;
      } else if (c == '(') {
        parserState = PARSER_STATES::DO_OPEN_PAREN;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::LETTER_N:
      if (c == '\'') {
        parserState = PARSER_STATES::APOSTROPHE;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::APOSTROPHE:
      if (c == 't') {
        parserState = PARSER_STATES::LETTER_T;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::LETTER_T:
      if (c == '(') {
        parserState = PARSER_STATES::DONT_OPEN_PAREN;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::DISABLED:
      if (c == 'd') {
        parserState = PARSER_STATES::LETTER_D;
      }
      break;
    case PARSER_STATES::DO_OPEN_PAREN:
      if (c == ')') {
        parserState = PARSER_STATES::DO_CLOSED_PAREN;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::DONT_OPEN_PAREN:
      if (c == ')') {
        parserState = PARSER_STATES::DONT_CLOSED_PAREN;
      } else {
        parserState = prevContext;
      }
      break;
    case PARSER_STATES::DO_CLOSED_PAREN:
      prevContext = PARSER_STATES::JUNK;
      if (c == 'd') {
        parserState = PARSER_STATES::LETTER_D;
      } else if (c == 'm') {
        parserState = PARSER_STATES::LETTER_M;
      } else {
        parserState = PARSER_STATES::JUNK;
      }
      break;
    case PARSER_STATES::DONT_CLOSED_PAREN:
      prevContext = PARSER_STATES::DISABLED;
      if (c == 'd') {
        parserState = PARSER_STATES::LETTER_D;
      } else {
        parserState = PARSER_STATES::DISABLED;
      }
      break;
    }
    std::println("New State: {}", parserState);
  }
  return ans;
}
int main(int argc, char *argv[]) {
  long long ans =
      calculateMultiples("/home/rgarber11/advent_of_code/2024/day3/input");
  std::println("The answer is {}.", ans);
  return 0;
}
