// src/parse.cpp
#include <iostream>
#include <stdint.h>

// "Gotos considered harmful" - Edsger W. Dijkstra
// ...except when they are useful for parsing a grammar. - Chris Fogelklou

// use
// cargo clean && cargo build

#define RETURN_IF_END(p)                                                       \
  if (*p == '\0') {                                                            \
    return result;                                                             \
  }

#define ADVANCE_IF_MATCH(p, c, L)                                              \
  if (*p == '\0') {                                                            \
    return result;                                                             \
  }                                                                            \
  if (*p++ == c) {                                                             \
    goto L;                                                                    \
  }

extern "C" {
/*
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
Only the four highlighted sections are real mul instructions. Adding up the
result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
*/

int64_t parse(const char *input) {
  int64_t result = 0;
  const char *p = input;
  int64_t n0, n1;
  RETURN_IF_END(p);
m:
  ADVANCE_IF_MATCH(p, 'm', u);
  goto m;
u:
  ADVANCE_IF_MATCH(p, 'u', l);
  goto m;
l:
  ADVANCE_IF_MATCH(p, 'l', lb);
  goto m;
lb:
  n0 = 0;
  n1 = 0;
  ADVANCE_IF_MATCH(p, '(', number0);
  goto m;

number0:
  if (*p >= '0' && *p <= '9') {
    n0 = n0 * 10 + (*p - '0');
    p++;
    RETURN_IF_END(p);
    goto number0;
  }
  ADVANCE_IF_MATCH(p, ',', number1);
  goto m;
number1:
  if (*p >= '0' && *p <= '9') {
    n1 = n1 * 10 + (*p - '0');
    p++;
    RETURN_IF_END(p);
    goto number1;
  }
  ADVANCE_IF_MATCH(p, ')', saveresult);
  goto m;
saveresult:
  result += n0 * n1;
  RETURN_IF_END(p);
  goto m;
}

#undef ADVANCE_IF_MATCH_DOING
#define ADVANCE_IF_MATCH_DOING(p, c, L, S)                                     \
  if (*p == '\0') {                                                            \
    return result;                                                             \
  }                                                                            \
  if (*p == '^') {                                                             \
    doing = true;                                                              \
  }                                                                            \
  if (*p == '!') {                                                             \
    doing = false;                                                             \
  }                                                                            \
  if (*p++ == c) {                                                             \
    goto L;                                                                    \
  }

// "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
int64_t parse_do_dont(const char *input) {
  int64_t result = 0;
  const char *p = input;
  int64_t n0, n1;
  bool doing = true;
m:
  RETURN_IF_END(p);
  if (*p == 'm') {
    p++;
    goto u;
  } else if (*p == 'd') {
    p++;
    goto d;
  } else {
    p++;
    goto m;
  }
u:
  ADVANCE_IF_MATCH(p, 'u', l);
  goto m;
l:
  ADVANCE_IF_MATCH(p, 'l', lb);
  goto m;
lb:
  n0 = 0;
  n1 = 0;
  ADVANCE_IF_MATCH(p, '(', number0);
  goto m;

number0:
  if (*p >= '0' && *p <= '9') {
    n0 = n0 * 10 + (*p - '0');
    p++;
    RETURN_IF_END(p);
    goto number0;
  }
  ADVANCE_IF_MATCH(p, ',', number1);
  goto m;
number1:
  if (*p >= '0' && *p <= '9') {
    n1 = n1 * 10 + (*p - '0');
    p++;
    RETURN_IF_END(p);
    goto number1;
  }
  ADVANCE_IF_MATCH(p, ')', saveresult);
  goto m;
saveresult:
  if (doing) {
    result += n0 * n1;
  }
  RETURN_IF_END(p);
  goto m;

d:
  ADVANCE_IF_MATCH(p, 'o', o);
  goto m;
o:
  if (*p == '(') {
    p++;
    goto drb;
  } else if (*p == 'n') {
    p++;
    goto apostrophe;
  } else {
    p++;
    goto m;
  }
  // do() right bracket
drb:
  RETURN_IF_END(p);
  if (*p == ')') {
    doing = true;
  }
  p++;
  goto m;
  // don't
apostrophe:
  ADVANCE_IF_MATCH(p, '\'', t);
  goto m;
t:
  ADVANCE_IF_MATCH(p, 't', dnlb);
  goto m;
dnlb:
  ADVANCE_IF_MATCH(p, '(', dnrb);
  goto m;
dnrb:
  RETURN_IF_END(p);
  if (*p == ')') {
    doing = false;
  }
  p++;
  goto m;
}
}