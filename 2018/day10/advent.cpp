
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <fstream>
#include <memory>
#include <gtest/gtest.h>
#include <gmock/gmock.h>
using namespace testing;

#ifdef WIN32
#include <Windows.h>
#define usleep(x) Sleep(x/1000)
#endif


/*
--- Day 10: The Stars Align ---
It's no use; your navigation system simply isn't capable of providing walking directions in the arctic circle, and certainly not in 1018.

The Elves suggest an alternative. In times like these, North Pole rescue operations will arrange points of light in the sky to guide missing Elves back to base. Unfortunately, the message is easy to miss: the points move slowly enough that it takes hours to align them, but have so much momentum that they only stay aligned for a second. If you blink at the wrong time, it might be hours before another message appears.

You can see these points of light floating in the distance, and record their position in the sky and their velocity, the relative change in position per second (your puzzle input). The coordinates are all given from your perspective; given enough time, those positions and velocities will move the points into a cohesive message!

Rather than wait, you decide to fast-forward the process and calculate what the points will eventually spell.

For example, suppose you note the following points:

position=< 9,  1> velocity=< 0,  2>\r\n"
"position=< 7,  0> velocity=<-1,  0>\r\n"
"position=< 3, -2> velocity=<-1,  1>\r\n"
"position=< 6, 10> velocity=<-2, -1>\r\n"
"position=< 2, -4> velocity=< 2,  2>\r\n"
"position=<-6, 10> velocity=< 2, -2>\r\n"
"position=< 1,  8> velocity=< 1, -1>\r\n"
"position=< 1,  7> velocity=< 1,  0>\r\n"
"position=<-3, 11> velocity=< 1, -2>\r\n"
"position=< 7,  6> velocity=<-1, -1>\r\n"
"position=<-2,  3> velocity=< 1,  0>\r\n"
"position=<-4,  3> velocity=< 2,  0>\r\n"
"position=<10, -3> velocity=<-1,  1>\r\n"
"position=< 5, 11> velocity=< 1, -2>\r\n"
"position=< 4,  7> velocity=< 0, -1>\r\n"
"position=< 8, -2> velocity=< 0,  1>\r\n"
"position=<15,  0> velocity=<-2,  0>\r\n"
"position=< 1,  6> velocity=< 1,  0>\r\n"
"position=< 8,  9> velocity=< 0, -1>\r\n"
"position=< 3,  3> velocity=<-1,  1>\r\n"
"position=< 0,  5> velocity=< 0, -1>\r\n"
"position=<-2,  2> velocity=< 2,  0>\r\n"
"position=< 5, -2> velocity=< 1,  2>\r\n"
"position=< 1,  4> velocity=< 2,  1>\r\n"
"position=<-2,  7> velocity=< 2, -2>\r\n"
"position=< 3,  6> velocity=<-1, -1>\r\n"
"position=< 5,  0> velocity=< 1,  0>\r\n"
"position=<-6,  0> velocity=< 2,  0>\r\n"
"position=< 5,  9> velocity=< 1, -2>\r\n"
"position=<14,  7> velocity=<-2,  0>\r\n"
"position=<-3,  6> velocity=< 2, -1>
Each line represents one point. Positions are given as <X, Y> pairs: X represents how far left (negative) or right (positive) the point appears, while Y represents how far up (negative) or down (positive) the point appears.

At 0 seconds, each point has the position given. Each second, each point's velocity is added to its position. So, a point with velocity <1, -2> is moving to the right, but is moving upward twice as quickly. If this point's initial position were <3, 9>, after 3 seconds, its position would become <6, 3>.

Over time, the points listed above would move like this:

Initially:
........#.............
................#.....
.........#.#..#.......
......................
#..........#.#.......#
...............#......
....#.................
..#.#....#............
.......#..............
......#...............
...#...#.#...#........
....#..#..#.........#.
.......#..............
...........#..#.......
#...........#.........
...#.......#..........

After 1 second:
......................
......................
..........#....#......
........#.....#.......
..#.........#......#..
......................
......#...............
....##.........#......
......#.#.............
.....##.##..#.........
........#.#...........
........#...#.....#...
..#...........#.......
....#.....#.#.........
......................
......................

After 2 seconds:
......................
......................
......................
..............#.......
....#..#...####..#....
......................
........#....#........
......#.#.............
.......#...#..........
.......#..#..#.#......
....#....#.#..........
.....#...#...##.#.....
........#.............
......................
......................
......................

After 3 seconds:
......................
......................
......................
......................
......#...#..###......
......#...#...#.......
......#...#...#.......
......#####...#.......
......#...#...#.......
......#...#...#.......
......#...#...#.......
......#...#..###......
......................
......................
......................
......................

After 4 seconds:
......................
......................
......................
............#.........
........##...#.#......
......#.....#..#......
.....#..##.##.#.......
.......##.#....#......
...........#....#.....
..............#.......
....#......#...#......
.....#.....##.........
...............#......
...............#......
......................
......................
After 3 seconds, the message appeared briefly: HI. Of course, your message will be much longer and will take many more seconds to appear.

What message will eventually appear in the sky?

To play, please identify yourself via one of these services:

*/

const char day10Input[] = 
"position=<-30052,  -9918> velocity=< 3,  1>\r\n"
"position=< 20349, -50260> velocity=<-2,  5>\r\n"
"position=< 40505, -40169> velocity=<-4,  4>\r\n"
"position=< 30444,  50599> velocity=<-3, -5>\r\n"
"position=< 40549, -50259> velocity=<-4,  5>\r\n"
"position=< 30454, -30087> velocity=<-3,  3>\r\n"
"position=< 50623,  20347> velocity=<-5, -2>\r\n"
"position=< 30439, -50262> velocity=<-3,  5>\r\n"
"position=< 50623,  40516> velocity=<-5, -4>\r\n"
"position=<-40143, -30090> velocity=< 4,  3>\r\n"
"position=< 50594,  50602> velocity=<-5, -5>\r\n"
"position=< -9884, -30087> velocity=< 1,  3>\r\n"
"position=<-19958, -40168> velocity=< 2,  4>\r\n"
"position=< 20357,  40513> velocity=<-2, -4>\r\n"
"position=<-50240,  30431> velocity=< 5, -3>\r\n"
"position=< 20351,  40516> velocity=<-2, -4>\r\n"
"position=<-50257, -20005> velocity=< 5,  2>\r\n"
"position=<-30073,  10253> velocity=< 3, -1>\r\n"
"position=<-50265,  50597> velocity=< 5, -5>\r\n"
"position=<-40154,  50601> velocity=< 4, -5>\r\n"
"position=<-19987, -40168> velocity=< 2,  4>\r\n"
"position=<-19987, -30087> velocity=< 2,  3>\r\n"
"position=<-50257,  30425> velocity=< 5, -3>\r\n"
"position=< 40558,  30425> velocity=<-4, -3>\r\n"
"position=<-19990,  10256> velocity=< 2, -1>\r\n"
"position=<-19979,  30425> velocity=< 2, -3>\r\n"
"position=<-19976, -50254> velocity=< 2,  5>\r\n"
"position=< 30446, -40172> velocity=<-3,  4>\r\n"
"position=<-40157,  40515> velocity=< 4, -4>\r\n"
"position=<-40143,  20342> velocity=< 4, -2>\r\n"
"position=< 30467, -50255> velocity=<-3,  5>\r\n"
"position=<-30040,  50606> velocity=< 3, -5>\r\n"
"position=<-30037,  50605> velocity=< 3, -5>\r\n"
"position=< 20382, -30086> velocity=<-2,  3>\r\n"
"position=< 50594,  10262> velocity=<-5, -1>\r\n"
"position=< 20342,  40515> velocity=<-2, -4>\r\n"
"position=<-19963, -50258> velocity=< 2,  5>\r\n"
"position=<-40135,  50599> velocity=< 4, -5>\r\n"
"position=< 40558,  10262> velocity=<-4, -1>\r\n"
"position=< 20366, -40177> velocity=<-2,  4>\r\n"
"position=< -9888,  -9919> velocity=< 1,  1>\r\n"
"position=<-19998,  40511> velocity=< 2, -4>\r\n"
"position=< 50608,  40516> velocity=<-5, -4>\r\n"
"position=< 10263,  40520> velocity=<-1, -4>\r\n"
"position=< 40521, -40171> velocity=<-4,  4>\r\n"
"position=< 30427, -20005> velocity=<-3,  2>\r\n"
"position=< 20333,  -9916> velocity=<-2,  1>\r\n"
"position=< -9909,  -9913> velocity=< 1,  1>\r\n"
"position=< 50628,  40511> velocity=<-5, -4>\r\n"
"position=< 50610,  30430> velocity=<-5, -3>\r\n"
"position=<-50248,  30433> velocity=< 5, -3>\r\n"
"position=< 50612,  50599> velocity=<-5, -5>\r\n"
"position=<-50253, -40170> velocity=< 5,  4>\r\n"
"position=< -9880,  40513> velocity=< 1, -4>\r\n"
"position=< 20362,  10256> velocity=<-2, -1>\r\n"
"position=< -9877, -30087> velocity=< 1,  3>\r\n"
"position=<-40182, -30082> velocity=< 4,  3>\r\n"
"position=<-50240, -20003> velocity=< 5,  2>\r\n"
"position=<-40138,  10255> velocity=< 4, -1>\r\n"
"position=<-50221,  20348> velocity=< 5, -2>\r\n"
"position=< -9896,  -9917> velocity=< 1,  1>\r\n"
"position=<-50209,  40516> velocity=< 5, -4>\r\n"
"position=< 20352,  50602> velocity=<-2, -5>\r\n"
"position=< 50607, -19998> velocity=<-5,  2>\r\n"
"position=<-19995, -30087> velocity=< 2,  3>\r\n"
"position=<-20003,  40517> velocity=< 2, -4>\r\n"
"position=<-50219,  30431> velocity=< 5, -3>\r\n"
"position=< 40521, -20001> velocity=<-4,  2>\r\n"
"position=<-30036,  20339> velocity=< 3, -2>\r\n"
"position=< 50624,  10257> velocity=<-5, -1>\r\n"
"position=<-50224,  40517> velocity=< 5, -4>\r\n"
"position=< -9907,  20344> velocity=< 1, -2>\r\n"
"position=< 40534, -40169> velocity=<-4,  4>\r\n"
"position=<-30065, -20003> velocity=< 3,  2>\r\n"
"position=< -9922,  20339> velocity=< 1, -2>\r\n"
"position=<-30052,  50602> velocity=< 3, -5>\r\n"
"position=< 10276, -50263> velocity=<-1,  5>\r\n"
"position=< 50594,  10258> velocity=<-5, -1>\r\n"
"position=< -9876,  10258> velocity=< 1, -1>\r\n"
"position=<-20001, -30091> velocity=< 2,  3>\r\n"
"position=<-50253,  50602> velocity=< 5, -5>\r\n"
"position=<-40159, -20002> velocity=< 4,  2>\r\n"
"position=<-50224, -30091> velocity=< 5,  3>\r\n"
"position=<-20001,  40511> velocity=< 2, -4>\r\n"
"position=< 20357, -40173> velocity=<-2,  4>\r\n"
"position=< 50615,  10255> velocity=<-5, -1>\r\n"
"position=< -9877, -50254> velocity=< 1,  5>\r\n"
"position=< 30440,  40516> velocity=<-3, -4>\r\n"
"position=< 30427,  50605> velocity=<-3, -5>\r\n"
"position=< 30419, -30090> velocity=<-3,  3>\r\n"
"position=< 20376, -20001> velocity=<-2,  2>\r\n"
"position=< 10255,  20344> velocity=<-1, -2>\r\n"
"position=< 30419,  20344> velocity=<-3, -2>\r\n"
"position=< 50595, -40169> velocity=<-5,  4>\r\n"
"position=<-30081,  40513> velocity=< 3, -4>\r\n"
"position=<-40143, -30090> velocity=< 4,  3>\r\n"
"position=< 40553,  20344> velocity=<-4, -2>\r\n"
"position=< 40529, -40170> velocity=<-4,  4>\r\n"
"position=< 30427, -50262> velocity=<-3,  5>\r\n"
"position=<-50240, -19996> velocity=< 5,  2>\r\n"
"position=<-50245, -30090> velocity=< 5,  3>\r\n"
"position=< 30448, -30087> velocity=<-3,  3>\r\n"
"position=< -9875, -40171> velocity=< 1,  4>\r\n"
"position=< -9900,  50599> velocity=< 1, -5>\r\n"
"position=<-19954, -40168> velocity=< 2,  4>\r\n"
"position=< 40526,  40518> velocity=<-4, -4>\r\n"
"position=<-20011,  30428> velocity=< 2, -3>\r\n"
"position=< 10283,  10257> velocity=<-1, -1>\r\n"
"position=<-40175,  40520> velocity=< 4, -4>\r\n"
"position=< 50636,  -9915> velocity=<-5,  1>\r\n"
"position=< -9904,  10255> velocity=< 1, -1>\r\n"
"position=< 30435,  40520> velocity=<-3, -4>\r\n"
"position=< 30424, -50254> velocity=<-3,  5>\r\n"
"position=< 20393,  50602> velocity=<-2, -5>\r\n"
"position=<-50261, -30083> velocity=< 5,  3>\r\n"
"position=<-30065,  10254> velocity=< 3, -1>\r\n"
"position=<-30052, -40177> velocity=< 3,  4>\r\n"
"position=< 50625, -50263> velocity=<-5,  5>\r\n"
"position=<-40142, -20001> velocity=< 4,  2>\r\n"
"position=< 20338, -19996> velocity=<-2,  2>\r\n"
"position=< 50599,  50604> velocity=<-5, -5>\r\n"
"position=< 40534, -40176> velocity=<-4,  4>\r\n"
"position=<-50235,  20339> velocity=< 5, -2>\r\n"
"position=<-30068,  -9918> velocity=< 3,  1>\r\n"
"position=<-40173,  20339> velocity=< 4, -2>\r\n"
"position=<-30054,  -9915> velocity=< 3,  1>\r\n"
"position=< 10255,  20348> velocity=<-1, -2>\r\n"
"position=<-40162,  40520> velocity=< 4, -4>\r\n"
"position=<-40158,  -9918> velocity=< 4,  1>\r\n"
"position=< 40508,  20348> velocity=<-4, -2>\r\n"
"position=< 10307, -40177> velocity=<-1,  4>\r\n"
"position=< 40545, -50261> velocity=<-4,  5>\r\n"
"position=<-30073, -40172> velocity=< 3,  4>\r\n"
"position=<-50209,  -9918> velocity=< 5,  1>\r\n"
"position=< -9913,  40515> velocity=< 1, -4>\r\n"
"position=< -9873, -50262> velocity=< 1,  5>\r\n"
"position=< -9877,  10261> velocity=< 1, -1>\r\n"
"position=<-50245, -30084> velocity=< 5,  3>\r\n"
"position=< 30440,  50606> velocity=<-3, -5>\r\n"
"position=<-40167,  40514> velocity=< 4, -4>\r\n"
"position=<-30076, -50255> velocity=< 3,  5>\r\n"
"position=< -9891, -19996> velocity=< 1,  2>\r\n"
"position=< -9877,  20340> velocity=< 1, -2>\r\n"
"position=< 20365,  -9911> velocity=<-2,  1>\r\n"
"position=<-50256,  10253> velocity=< 5, -1>\r\n"
"position=< -9909, -19996> velocity=< 1,  2>\r\n"
"position=< 30479,  -9919> velocity=<-3,  1>\r\n"
"position=<-30065,  10257> velocity=< 3, -1>\r\n"
"position=< 50652,  -9919> velocity=<-5,  1>\r\n"
"position=< 20338,  -9914> velocity=<-2,  1>\r\n"
"position=<-30062,  20339> velocity=< 3, -2>\r\n"
"position=< 20360,  10259> velocity=<-2, -1>\r\n"
"position=< 40537,  50604> velocity=<-4, -5>\r\n"
"position=<-50221,  50603> velocity=< 5, -5>\r\n"
"position=< 40505, -30090> velocity=<-4,  3>\r\n"
"position=< -9890,  20339> velocity=< 1, -2>\r\n"
"position=<-30097, -50261> velocity=< 3,  5>\r\n"
"position=<-19995, -30086> velocity=< 2,  3>\r\n"
"position=< 20341,  10255> velocity=<-2, -1>\r\n"
"position=<-19977, -20005> velocity=< 2,  2>\r\n"
"position=<-30053, -50259> velocity=< 3,  5>\r\n"
"position=<-30065, -40173> velocity=< 3,  4>\r\n"
"position=< 50617,  50601> velocity=<-5, -5>\r\n"
"position=< 50631, -40171> velocity=<-5,  4>\r\n"
"position=< -9925,  -9915> velocity=< 1,  1>\r\n"
"position=<-40150,  10262> velocity=< 4, -1>\r\n"
"position=< 40526,  40516> velocity=<-4, -4>\r\n"
"position=<-30055,  -9915> velocity=< 3,  1>\r\n"
"position=<-50264,  30432> velocity=< 5, -3>\r\n"
"position=< 10287, -20003> velocity=<-1,  2>\r\n"
"position=< 40534,  20348> velocity=<-4, -2>\r\n"
"position=<-19960, -19998> velocity=< 2,  2>\r\n"
"position=< 30464, -40173> velocity=<-3,  4>\r\n"
"position=<-50220, -50259> velocity=< 5,  5>\r\n"
"position=< 40537,  -9919> velocity=<-4,  1>\r\n"
"position=< 20357,  20346> velocity=<-2, -2>\r\n"
"position=< 40510,  50598> velocity=<-4, -5>\r\n"
"position=< 50607, -40172> velocity=<-5,  4>\r\n"
"position=< 40537, -19996> velocity=<-4,  2>\r\n"
"position=< 50636,  10261> velocity=<-5, -1>\r\n"
"position=< -9896,  30425> velocity=< 1, -3>\r\n"
"position=< -9917,  10255> velocity=< 1, -1>\r\n"
"position=<-50248,  50606> velocity=< 5, -5>\r\n"
"position=<-50237,  -9919> velocity=< 5,  1>\r\n"
"position=<-30073, -20005> velocity=< 3,  2>\r\n"
"position=< 40529,  50606> velocity=<-4, -5>\r\n"
"position=< 50650, -30082> velocity=<-5,  3>\r\n"
"position=<-30093,  10258> velocity=< 3, -1>\r\n"
"position=< 30456, -40177> velocity=<-3,  4>\r\n"
"position=< 40521, -40170> velocity=<-4,  4>\r\n"
"position=< 30468,  10257> velocity=<-3, -1>\r\n"
"position=< 10276, -50261> velocity=<-1,  5>\r\n"
"position=< 30421,  -9919> velocity=<-3,  1>\r\n"
"position=< -9877,  20346> velocity=< 1, -2>\r\n"
"position=< 10258, -40177> velocity=<-1,  4>\r\n"
"position=<-50264,  40516> velocity=< 5, -4>\r\n"
"position=< -9883, -40173> velocity=< 1,  4>\r\n"
"position=<-40127, -50256> velocity=< 4,  5>\r\n"
"position=< -9885,  20343> velocity=< 1, -2>\r\n"
"position=< 20382,  -9914> velocity=<-2,  1>\r\n"
"position=<-30076, -19998> velocity=< 3,  2>\r\n"
"position=<-30073,  50601> velocity=< 3, -5>\r\n"
"position=< -9893,  20345> velocity=< 1, -2>\r\n"
"position=< -9904,  10258> velocity=< 1, -1>\r\n"
"position=< 20393, -40171> velocity=<-2,  4>\r\n"
"position=<-19984,  50603> velocity=< 2, -5>\r\n"
"position=< -9889,  20343> velocity=< 1, -2>\r\n"
"position=<-19982,  40512> velocity=< 2, -4>\r\n"
"position=<-20009, -40168> velocity=< 2,  4>\r\n"
"position=<-30069, -19997> velocity=< 3,  2>\r\n"
"position=<-19987, -30085> velocity=< 2,  3>\r\n"
"position=< 10252,  10259> velocity=<-1, -1>\r\n"
"position=< -9917,  30428> velocity=< 1, -3>\r\n"
"position=< 10287,  20342> velocity=<-1, -2>\r\n"
"position=<-50245, -20000> velocity=< 5,  2>\r\n"
"position=<-40165, -40177> velocity=< 4,  4>\r\n"
"position=<-40135, -50262> velocity=< 4,  5>\r\n"
"position=<-19984, -30085> velocity=< 2,  3>\r\n"
"position=< 30428, -30091> velocity=<-3,  3>\r\n"
"position=<-30089,  30431> velocity=< 3, -3>\r\n"
"position=< 30467,  20344> velocity=<-3, -2>\r\n"
"position=<-19987,  40520> velocity=< 2, -4>\r\n"
"position=<-30037,  40512> velocity=< 3, -4>\r\n"
"position=< 40557, -50255> velocity=<-4,  5>\r\n"
"position=< 40550,  10254> velocity=<-4, -1>\r\n"
"position=<-40159,  10258> velocity=< 4, -1>\r\n"
"position=< 40539,  20343> velocity=<-4, -2>\r\n"
"position=< 30440,  50605> velocity=<-3, -5>\r\n"
"position=< 30453,  30434> velocity=<-3, -3>\r\n"
"position=< 40550, -30088> velocity=<-4,  3>\r\n"
"position=<-40151,  10256> velocity=< 4, -1>\r\n"
"position=< 10276,  40514> velocity=<-1, -4>\r\n"
"position=< 10280,  20348> velocity=<-1, -2>\r\n"
"position=<-20009, -30091> velocity=< 2,  3>\r\n"
"position=< 30467,  50601> velocity=<-3, -5>\r\n"
"position=< 30439, -20000> velocity=<-3,  2>\r\n"
"position=< 10276, -20005> velocity=<-1,  2>\r\n"
"position=< 40562, -30082> velocity=<-4,  3>\r\n"
"position=<-19986, -20004> velocity=< 2,  2>\r\n"
"position=< 20333,  10259> velocity=<-2, -1>\r\n"
"position=<-50224,  -9911> velocity=< 5,  1>\r\n"
"position=< -9872,  -9919> velocity=< 1,  1>\r\n"
"position=< 10295, -30085> velocity=<-1,  3>\r\n"
"position=<-19990,  30429> velocity=< 2, -3>\r\n"
"position=< 10295, -30084> velocity=<-1,  3>\r\n"
"position=< 30429,  40515> velocity=<-3, -4>\r\n"
"position=< 20365, -30086> velocity=<-2,  3>\r\n"
"position=< 50618,  10258> velocity=<-5, -1>\r\n"
"position=<-20010,  -9910> velocity=< 2,  1>\r\n"
"position=< 20373, -40169> velocity=<-2,  4>\r\n"
"position=< 10307, -50261> velocity=<-1,  5>\r\n"
"position=< 20381,  10256> velocity=<-2, -1>\r\n"
"position=< 40513,  40517> velocity=<-4, -4>\r\n"
"position=< -9865,  20345> velocity=< 1, -2>\r\n"
"position=< -9909,  20347> velocity=< 1, -2>\r\n"
"position=< 10284,  20348> velocity=<-1, -2>\r\n"
"position=<-30073,  20342> velocity=< 3, -2>\r\n"
"position=<-30078,  30425> velocity=< 3, -3>\r\n"
"position=< -9899,  20342> velocity=< 1, -2>\r\n"
"position=< 20333,  30429> velocity=<-2, -3>\r\n"
"position=< 10306,  10253> velocity=<-1, -1>\r\n"
"position=<-50261,  10260> velocity=< 5, -1>\r\n"
"position=< 30459,  -9912> velocity=<-3,  1>\r\n"
"position=< 10292, -40176> velocity=<-1,  4>\r\n"
"position=< 20389, -30083> velocity=<-2,  3>\r\n"
"position=<-40166,  20340> velocity=< 4, -2>\r\n"
"position=< -9901,  20345> velocity=< 1, -2>\r\n"
"position=< 40557, -20004> velocity=<-4,  2>\r\n"
"position=<-40158, -30090> velocity=< 4,  3>\r\n"
"position=<-50209,  10255> velocity=< 5, -1>\r\n"
"position=<-50265, -40172> velocity=< 5,  4>\r\n"
"position=<-50245,  50605> velocity=< 5, -5>\r\n"
"position=< 30443, -40177> velocity=<-3,  4>\r\n"
"position=< -9916,  20339> velocity=< 1, -2>\r\n"
"position=< 50640,  50601> velocity=<-5, -5>\r\n"
"position=<-19971, -20005> velocity=< 2,  2>\r\n"
"position=< 40522,  -9914> velocity=<-4,  1>\r\n"
"position=< 50607, -19999> velocity=<-5,  2>\r\n"
"position=<-50243,  40515> velocity=< 5, -4>\r\n"
"position=<-50240,  10260> velocity=< 5, -1>\r\n"
"position=< 40561, -19998> velocity=<-4,  2>\r\n"
"position=<-50211, -50254> velocity=< 5,  5>\r\n"
"position=<-40143,  30431> velocity=< 4, -3>\r\n"
"position=< 30430, -50259> velocity=<-3,  5>\r\n"
"position=< 30479, -20002> velocity=<-3,  2>\r\n"
"position=<-30092,  20344> velocity=< 3, -2>\r\n"
"position=<-30092, -40168> velocity=< 3,  4>\r\n"
"position=< 50601,  10257> velocity=<-5, -1>\r\n"
"position=< 20373,  40515> velocity=<-2, -4>\r\n"
"position=< 30479, -19997> velocity=<-3,  2>\r\n"
"position=<-40175,  50605> velocity=< 4, -5>\r\n"
"position=< -9880, -40173> velocity=< 1,  4>\r\n"
"position=< 20349, -50255> velocity=<-2,  5>\r\n"
"position=< 20362,  -9914> velocity=<-2,  1>\r\n"
"position=<-30089,  50601> velocity=< 3, -5>\r\n"
"position=< 30467,  10253> velocity=<-3, -1>\r\n"
"position=< 40565,  30429> velocity=<-4, -3>\r\n"
"position=<-40142,  10257> velocity=< 4, -1>\r\n"
"position=< 50609, -50263> velocity=<-5,  5>\r\n"
"position=< -9881, -40173> velocity=< 1,  4>\r\n"
"position=< 50639,  40515> velocity=<-5, -4>\r\n"
"position=<-50269, -40170> velocity=< 5,  4>\r\n"
"position=< 40553,  -9910> velocity=<-4,  1>\r\n"
"position=< 30455,  30434> velocity=<-3, -3>\r\n"
"position=< 20362,  10258> velocity=<-2, -1>\r\n"
"position=< 40505,  50603> velocity=<-4, -5>\r\n"
"position=<-19990,  10260> velocity=< 2, -1>\r\n"
"position=< 30447, -19998> velocity=<-3,  2>\r\n"
"position=<-40150, -30087> velocity=< 4,  3>\r\n"
"position=< -9907, -40172> velocity=< 1,  4>\r\n"
"position=<-30044,  20348> velocity=< 3, -2>\r\n"
"position=<-40178,  -9913> velocity=< 4,  1>\r\n"
"position=<-50261,  30428> velocity=< 5, -3>\r\n"
"position=<-40151,  20348> velocity=< 4, -2>\r\n"
"position=<-50224,  30432> velocity=< 5, -3>\r\n"
"position=< 50652, -50263> velocity=<-5,  5>\r\n"
"position=<-19952,  50606> velocity=< 2, -5>\r\n"
"position=< 30431,  40511> velocity=<-3, -4>\r\n"
"position=< 30459, -30088> velocity=<-3,  3>\r\n"
"position=< 50651,  20347> velocity=<-5, -2>\r\n"
"position=< 10292, -30082> velocity=<-1,  3>\r\n"
"position=< 20366,  10257> velocity=<-2, -1>\r\n"
"position=<-19987,  -9913> velocity=< 2,  1>\r\n"
"position=< -9865, -40170> velocity=< 1,  4>\r\n"
"position=< -9889,  50597> velocity=< 1, -5>\r\n"
"position=<-30065, -50262> velocity=< 3,  5>\r\n"
"position=< -9921,  20347> velocity=< 1, -2>\r\n"
"position=<-30084,  10253> velocity=< 3, -1>\r\n"
"position=< 10248, -20005> velocity=<-1,  2>\r\n"
"position=< 20345,  -9915> velocity=<-2,  1>\r\n"
"position=< -9882,  -9915> velocity=< 1,  1>\r\n"
"position=<-30065,  50602> velocity=< 3, -5>\r\n"
"position=< 40565,  50599> velocity=<-4, -5>\r\n"
"position=< 20349, -50261> velocity=<-2,  5>\r\n"
"position=<-19960, -19998> velocity=< 2,  2>\r\n"
"position=< 10255, -50258> velocity=<-1,  5>\r\n"
"position=< -9901, -50255> velocity=< 1,  5>\r\n"
"position=<-50219,  -9916> velocity=< 5,  1>\r\n"
"position=< 40524, -30086> velocity=<-4,  3>\r\n"
"position=< 40550,  50600> velocity=<-4, -5>\r\n"
"position=<-50233, -30091> velocity=< 5,  3>\r\n"
"position=< 30440, -50257> velocity=<-3,  5>\r\n"
"position=< -9920,  50605> velocity=< 1, -5>\r\n"
"position=< 40507,  20339> velocity=<-4, -2>\r\n"
"position=< 10264,  30426> velocity=<-1, -3>\r\n"
"position=< 40556,  20341> velocity=<-4, -2>\r\n"
"position=< -9875,  -9913> velocity=< 1,  1>\r\n"
"position=<-30068, -50254> velocity=< 3,  5>\r\n"
"position=< -9889,  30425> velocity=< 1, -3>\r\n"
"position=<-20011, -40172> velocity=< 2,  4>\r\n"
"position=<-50269,  10259> velocity=< 5, -1>\r\n"
"position=<-40132,  30432> velocity=< 4, -3>\r\n"
"position=< 40553,  -9913> velocity=<-4,  1>\r\n"
"position=<-30073,  10256> velocity=< 3, -1>\r\n"
"position=<-30092,  20340> velocity=< 3, -2>\r\n"
"position=< 40542,  50606> velocity=<-4, -5>\r\n"
"position=<-50224,  20347> velocity=< 5, -2>\r\n"
"position=<-40143,  50606> velocity=< 4, -5>\r\n"
"position=<-19971, -20000> velocity=< 2,  2>\r\n"
"position=< 40553,  20340> velocity=<-4, -2>\r\n"
"position=<-30081, -20001> velocity=< 3,  2>\r\n"
"position=<-30037, -40172> velocity=< 3,  4>\r\n"
"position=< -9865,  30428> velocity=< 1, -3>\r\n"
"position=< 30479,  20340> velocity=<-3, -2>\r\n"
"position=< 30429,  20343> velocity=<-3, -2>\r\n"
"position=<-19971, -40172> velocity=< 2,  4>\r\n"
"position=< 30431,  50601> velocity=<-3, -5>\r\n"
"position=<-19982,  10256> velocity=< 2, -1>\r\n"
"position=< 20344,  20343> velocity=<-2, -2>";


class Point {
public:
  Point()
    :startX(0), startY(0), x(0),y(0),vx(0),vy(0) {
  }
  Point(std::string& line)
    :startX(0), startY(0), x(0), y(0), vx(0), vy(0) {
    GetPointFromLine(line);
  }
  ~Point() {

  }

  void GetPointFromLine(std::string& line)
  {
    std::string xs = line.substr(10, 6);
    startX = x = std::stoi(xs);

    std::string ys = line.substr(18, 6);
    startY = y = std::stoi(ys);

    std::string vxs = line.substr(36, 2);
    vx = std::stoi(vxs);

    std::string vys = line.substr(40, 2);
    vy = std::stoi(vys);
  }

  double GetDistanceFromPoint(const Point& rhs) {
    double disty = rhs.y - y;
    double distx = rhs.x - x;
    return sqrt(disty * disty + distx * distx);
  }

  void Advance(const int seconds = 1) {
    x = x + vx * seconds;
    y = y + vy * seconds;
  }

  void Reset() {
    x = startX;
    y = startY;
  }

  void Print() {
    std::cout << "startX:" << startX << ", " << "startY:" << startY << std::endl;
  }

  
  int startX;
  int startY;
  int x;
  int y;
  int vx;
  int vy;
};



TEST(TestRt, Day10){
  typedef std::vector<std::shared_ptr<Point>> PointsAry;

  std::istringstream iss(day10Input);
  PointsAry points;

  // Get all lines from the input file.
  std::string line;
  while (std::getline(iss, line))
  {
    points.push_back(std::make_unique<Point>(Point(line)));
  }

  std::cout << "Got " << points.size() << " points." << std::endl;

  auto printPoint = [](std::shared_ptr<Point>p) {
    p->Print();
  };
  std::for_each(points.begin(), points.end(), printPoint);

  int minSeconds = -1;
  const int secondsIncrement = 8;
  {
    double minMaximumDistanceBetweenPoints = -1;
    bool decreasing = true;
    int seconds = 0;

    // For each second...
    while (decreasing) {

      // For each point p1, get the other point that is furthest away    
      PointsAry::iterator p1 = points.begin();
      PointsAry::iterator end1 = points.end();
      double maximumDistanceFromP1 = -1;
      double* pMax = &maximumDistanceFromP1;
      while (p1 != end1) {

        auto doCompare = [p1, pMax](std::shared_ptr<Point>p2) {
          Point& point2 = *p2;
          auto d = p1.operator*()->GetDistanceFromPoint(point2);
          if (d > * pMax) {
            *pMax = d;
          }
        };

        std::for_each(points.begin(), points.end(), doCompare);
        p1++;
      } // while (p1 != end1)

      if ((minMaximumDistanceBetweenPoints < 0) || (maximumDistanceFromP1 <= minMaximumDistanceBetweenPoints)) {
        minMaximumDistanceBetweenPoints = maximumDistanceFromP1;
      }
      else {
        minSeconds = seconds - secondsIncrement;
        std::cout << "Found minimum at " << minSeconds << " seconds, max distance between points is " << minMaximumDistanceBetweenPoints << std::endl;
        decreasing = false;
      }

      auto doAdvance = [secondsIncrement](std::shared_ptr<Point>p) {
        p->Advance(secondsIncrement);
      };

      std::for_each(points.begin(), points.end(), doAdvance);
      seconds += secondsIncrement;

      if ((seconds & 0xff) == 0) {
        std::cout << "At " << seconds << " seconds, max distance between points is " << minMaximumDistanceBetweenPoints << std::endl;
      }
    }
    std::cout << minMaximumDistanceBetweenPoints << " distance at time = " << minSeconds << " seconds.";
  }

  // Now output the message.

  std::cout << std::endl;

  std::fstream fileout("advent_2018_day_10.txt", std::ios_base::out);

  for (int seconds = minSeconds - secondsIncrement; seconds < minSeconds + secondsIncrement; seconds++) {

    fileout << "For seconds:" << seconds << std::endl;

    auto doResetAndAdvance = [seconds](std::shared_ptr<Point>p) {
      p->Reset();
      p->Advance(seconds);
    };
    std::for_each(points.begin(), points.end(), doResetAndAdvance);

    int minX = 1000000;
    int maxX = -1;
    int minY = 1000000;
    int maxY = -1;

    auto getMaxDim = [&minX, &maxX, &minY, &maxY](std::shared_ptr<Point>p1) {
      if (p1->x > maxX) {
        maxX = p1->x;
      }
      if (p1->y > maxY) {
        maxY = p1->y;
      }
      if (p1->x < minX) {
        minX = p1->x;
      }
      if (p1->y < minY) {
        minY = p1->y;
      }
    };

    std::for_each(points.begin(), points.end(), getMaxDim);


    const int widthX = maxX - minX + 1;
    const int widthY = maxY - minY + 1;
    std::vector<bool *> tbl(widthY);
    assert(tbl.size() == widthY);
    for (int y = 0; y < widthY; y++) {
      auto px = new bool[widthX];
      memset(px, 0, sizeof(bool) * widthX);

      assert(px[0] == false);
      tbl[y] = px;
      
      assert(tbl[y][0] == false);
    }

    auto printline = [&fileout, widthX](bool* xline) {
      for (int x = 0; x < widthX; x++) {
        const auto c = (xline[x]) ? "#" : ".";
        //std::cout << c;
        fileout << c;
      }
      //std::cout << std::endl;
      fileout << std::endl;
    };
    std::for_each(tbl.begin(), tbl.end(), printline);

    auto prickOff = [&tbl, minX, minY, widthX, widthY](std::shared_ptr<Point>p1) {
      const int x = p1->x - minX;
      const int y = p1->y - minY;
      if (y >= widthY) {
        std::cout << "now";
      }
      assert(y < widthY);
      assert(y < tbl.size());
      bool* xline = tbl[y];
      assert(x < widthX);
      xline[x] = true;
    };

    std::for_each(points.begin(), points.end(), prickOff);

    std::for_each(tbl.begin(), tbl.end(), printline);

    for (int y = 0; y < widthY; y++) {
      delete[] tbl[y];
    }


    std::cout << std::endl;
    fileout << std::endl;
  }
  fileout.close();

  std::cout << std::endl;



}




int main(int argc, char** argv){
  
  // The following line must be executed to initialize Google Mock
  // (and Google Test) before running the tests.
  ::testing::InitGoogleMock(&argc, argv);
  const int gtest_rval = RUN_ALL_TESTS();
  
  return gtest_rval;
}
