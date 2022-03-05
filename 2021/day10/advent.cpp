#include "input.h"

#include <algorithm>
#include <fstream>
#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <iostream>
#include <memory>
#include <stack>
#include <string>
#include <vector>
using namespace testing;

// ////////////////////////////////////////////////////////////////////////////////////////////////
typedef std::stack<char> CharStack;

// ////////////////////////////////////////////////////////////////////////////////////////////////
bool isOpenBracket(const char c) {
  return (c == '<') || (c == '(') || (c == '{') || (c == '[');
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
bool isCloseBracket(const char c) {
  return (c == '>') || (c == ')') || (c == '}') || (c == ']');
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
char getMatchingCloseBracket(const char c) {
  switch (c) {
  case '<': return '>';
  case '{': return '}';
  case '(': return ')';
  case '[': return ']';
  default: return 0;
  }
  return 0;
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
int getIllegalPoints(const char c) {
  switch (c) {
  case ')': return 3;
  case ']': return 57;
  case '}': return 1197;
  case '>': return 25137;
  default: return 0;
  }
  return 0;
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
int getClosePoints(const char c) {
  switch (c) {
  case ')': return 1;
  case ']': return 2;
  case '}': return 3;
  case '>': return 4;
  default: return 0;
  }
  return 0;
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
static inline std::string& rtrim(std::string& s) {
  s.erase(
    std::find_if(s.rbegin(), s.rend(), std::not1(std::ptr_fun<int, int>(std::isspace)))
      .base(),
    s.end());
  return s;
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
// Extract points from the const char string.
void analyseCorruptLines(std::string inputString, long long& incompletePts, int& numPoints) {
  std::istringstream iss(inputString);
  std::vector<long long> incompleteScoresArr;

  numPoints = incompletePts = 0;

  // Get all lines from the input file, push them on the stack
  std::string line;
  while (std::getline(iss, line)) {
    line = rtrim(line);
    std::cout << line << "\t";
    const char* pC = line.data();
    const auto len = line.length();
    CharStack stk;
    int i        = 0;
    bool corrupt = false;

    // Fill the stack until corruption is found
    while (i < len && !corrupt) {
      const char c = pC[ i++ ];
      if (isOpenBracket(c)) {
        stk.push(c);
      } else if (isCloseBracket(c)) {
        if (stk.empty()) {
          corrupt = true;
        } else {
          const char openBracket = stk.top();
          const char closeBracket = getMatchingCloseBracket(openBracket);
          if (c == closeBracket) {
            stk.pop();
          } else {
            std::cout << "Expected " << closeBracket << " but found "
                      << c << " instead" << std::endl;
            const auto pts = getIllegalPoints(c);
            numPoints += pts;
            corrupt = true;
          }
        }
      }
    } // while

    // If no corruption is found, and the stack is not empty, then it is incomplete.
    if ((!corrupt) && (!stk.empty())) {
      long long incompleteScore = 0;
      while (!stk.empty()) {
        const char openBracket = stk.top();
        const char closeBracket = getMatchingCloseBracket(openBracket);
        const auto pts = getClosePoints(closeBracket);
        EXPECT_GE(pts, 0);
        incompleteScore = incompleteScore * 5 + pts;
        stk.pop();
      }
      EXPECT_GE(incompleteScore, 0);
      incompleteScoresArr.push_back(incompleteScore);
    }
  }

  // After processing all lines, find the midpoint
  if (incompleteScoresArr.size() > 0) {
    std::sort(
      incompleteScoresArr.begin(), incompleteScoresArr.end());
    std::cout << "Length of incomplete was "
              << incompleteScoresArr.size() << std::endl;
    const auto midPoint = (incompleteScoresArr.size() + 1) / 2 - 1;
    std::cout << "Incomplete score was "
              << incompleteScoresArr[ midPoint ] << std::endl;
    incompletePts = incompleteScoresArr[ midPoint ];
  }
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
TEST(TestRt, Example) {
  long long incompletePts = 0;
  int numPoints           = 0;
  analyseCorruptLines(exampleInput, incompletePts, numPoints);

  EXPECT_EQ(numPoints, 26397);
  EXPECT_EQ(incompletePts, 288957);
}

// ////////////////////////////////////////////////////////////////////////////////////////////////
TEST(TestRt, Day10) {
  long long incompletePts = 0;
  int numPoints           = 0;
  analyseCorruptLines(day10Input, incompletePts, numPoints);

  // Note: This is unique for my profile and the day10Input.
  EXPECT_EQ(numPoints, 394647);
  EXPECT_EQ(incompletePts, 2380061249);
}
