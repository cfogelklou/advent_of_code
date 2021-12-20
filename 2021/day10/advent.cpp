
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

#ifdef WIN32
#include <Windows.h>
#define usleep(x) Sleep(x / 1000)
#endif

/*
 --- Day 10: Syntax Scoring ---
 You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:

 Syntax error in navigation subsystem on line: all of them
 All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).

 The navigation subsystem syntax is made of several lines containing
 chunks. There are one or more chunks on each line, and chunks
 contain zero or more other chunks. Adjacent chunks are not
 separated by any delimiter; if one chunk stops, the next chunk
 (if any) can immediately start. Every chunk must open and close with one of four legal pairs of matching characters:

 If a chunk opens with (, it must close with ).
 If a chunk opens with [, it must close with ].
 If a chunk opens with {, it must close with }.
 If a chunk opens with <, it must close with >.
 So, () is a legal chunk that contains no other chunks, as
 is []. More complex but valid chunks include ([]), {()()()}, <([{}])>, [<>({}){}[([])<>]], and even (((((((((()))))))))).

 Some lines are incomplete, but others are corrupted. Find and discard the corrupted lines first.

 A corrupted line is one where a chunk closes with the wrong
 character - that is, where the characters it opens and closes with do not form one of the four legal pairs listed above.

 Examples of corrupted chunks include (], {()()()>, (((()))},
 and <([]){()}[{}]). Such a chunk can appear anywhere within a line, and its presence causes the whole line to be considered corrupted.

 For example, consider the following navigation subsystem:

 [({(<(())[]>[[{[]{<()<>>
 [(()[<>])]({[<{<<[]>>(
 {([(<{}[<>[]}>{[]{[(<()>
 (((({<>}<{<{<>}{[]{[]{}
 [[<[([]))<([[{}[[()]]]
 [{[{({}]{}}([{[{{{}}([]
 {<[[]]>}<{[{[{[]{()[[[]
 [<(<(<(<{}))><([]([]()
 <{([([[(<>()){}]>(<<{{
 <{([{{}}[<[[[<>{}]]]>[]]
 Some of the lines aren't corrupted, just incomplete; you can ignore these lines for now. The remaining five lines are corrupted:

 {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
 [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
 [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
 [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
 <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
 Stop at the first incorrect closing character on each corrupted line.

 Did you know that syntax checkers actually have contests to
 see who can get the high score for syntax errors in a file?
 It's true! To calculate the syntax error score for a line, take the first illegal character on the line and look it up in the following table:

 ): 3 points.
 ]: 57 points.
 }: 1197 points.
 >: 25137 points.
 In the above example, an illegal ) was found twice (2*3 = 6
 points), an illegal ] was found once (57 points), an illegal
 } was found once (1197 points), and an illegal > was found
 once (25137 points). So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!

 Find the first illegal character in each corrupted line of the navigation subsystem. What is the total syntax error score for those errors?

*/

const char exampleInput[] = "[({(<(())[]>[[{[]{<()<>>\r\n"
                            "[(()[<>])]({[<{<<[]>>(\r\n"
                            "{([(<{}[<>[]}>{[]{[(<()>\r\n"
                            "(((({<>}<{<{<>}{[]{[]{}\r\n"
                            "[[<[([]))<([[{}[[()]]]\r\n"
                            "[{[{({}]{}}([{[{{{}}([]\r\n"
                            "{<[[]]>}<{[{[{[]{()[[[]\r\n"
                            "[<(<(<(<{}))><([]([]()\r\n"
                            "<{([([[(<>()){}]>(<<{{\r\n"
                            "<{([{{}}[<[[[<>{}]]]>[]]\r\n";

const char day10Input[] =
  "<{[[({([{<[[([()<>]<<><>>)<[[]()]{<>()}>][<(()[])<()<>>>"
  "]]<(<(<><>)(())>)[<([]()){<><>}>{{()<>}<{}>}]>>[({\r\n"
  "<<({<{{{[(<{([{}{}]{{}[]}){({}())}}<<[[][]](()<>)>[{<><>"
  "}<{}<>>]>>(<[[()]([]{})]((()<>)[()()\r\n"
  "({<<({[<[[<[([()])]<<<{}<>><()[]>>[[()<>]<[]()>]>>([((()"
  "[])[()])]{[[()()]<<><>>][{<>[]}(()<>)]})\r\n"
  "(([[<<[{(({{{(()<>)[()()]}[({}<>)[[][]]]}([{{}<>}<[]()}]"
  "{[[][]][()<>]})})(([({[]{}}{[][]})\r\n"
  "({{<[[<<{{[{[<{}{}>{[]{}}]<([]{}){()[]}>}]<{([<><>]([]<>"
  ")){({}())({}<>)}}[[<{}<>>([]{})][[()[]]]]>}}>>]]\r\n"
  "[(<<<{([[[<{(<()<>>{<><>})(<[]<>>([][]))}[<[{}[]]>{<[]>("
  "<><>)}]>([{{[]()}{[]()}}([<>[]][{}<>\r\n"
  "<[{[{{([<(<{{[{}<>]({}{})}{{<>[]}(()())}}(<<<>{}>(<><>)>"
  "(<<>{}>[[]<>]))>)>[{[<[[()<>]{{}<>\r\n"
  "{(<<[([<{<<<({()()}(<><>)}{((){})([]())}>>[<{<[]<>>{[]{}"
  "}}([()()])>]>}>]{{([{<{{[]<>}(<>)}<({}(\r\n"
  "((<[{(({([<{(<[][]>)[<<>{}><()<>>]}<{<{}{}>}[{()[]}<{}[]"
  ">]>>])})<[{[{([{[]()}((){})][<<>()><()()>])>]}][[({{["
  "\r\n"
  "[[{([[<<[({[<<[]{}>{[][]}>(<()<>>[{}()])][([{}[]}[<><>])"
  "{[{}()]}]}[<<<{}<>>{()<>}>(<{}()>([]()))>{[[<><>][\r\n"
  "{{{<[([[{<[<{([]{}>{<>()}}{<(){}><()()>}>[{[[][]](<>())}"
  "[{[]<>}[<>[]]]]]{<{{{}[]}{<><>}}[[()()](<\r\n"
  "(([{(({[[[[{(<<>[]>((){})){<()<>><[]{}>}}{[{{}()}(()[])]"
  "((()){()<>})}]]]<[<<{(()[])<{}[]>}{\r\n"
  "{<<([(<([{{(<(()<>)[[][]]>{[[]{}]{(){}}})[{<(){}>[[]<>]}"
  "]}}])[[[([[{()()}{<>[]}]{[()]{[][]}}]<[((\r\n"
  "[{<<{<([(<[<{<{}[]>{{}[]}}(([]()))>{([(){}]<()[]>)(<[][]"
  ">{[]<>})}]>[{<<{<>()}[{}[]]>{(<><>)}>{[<[]<>>{()[]}]<"
  "\r\n"
  "<{{<{[[{{([({<()>{(){}}}((<>[])(<><>)))[(<[]<>>[()[]])<<"
  "()()>{<>{}}>]])<[{[{[]<>}{()()}][[{}{}]<[]>]}<{{{\r\n"
  "[(<[{([{({<<[<<>()>[[]()]]{<<>{}><{}<>>}><{<()()>{{}{}}}"
  "(({}<>)<()<>>)>>})({(([<{}<>>{[][]\r\n"
  "{(([{[<{{[({{[{}<>][()]}<{{}<>}>}(({<>[]})))]{((<[<>{}]["
  "<>[])>[[<>()]{[][]}]))<<<({}<>){{}<>}>(\r\n"
  "{[<{({((([[{[(<>{}){<>()}][([]{}}]}]{[<(()()){[]()}>((<>"
  "[])<(){}>)]}]){<(<<{{}<>}>>)([[{<><>}<\r\n"
  "{<{[(<{<[({<<[<><>]{()()}>[{()<>}<{}>]>[[<[]>][{()[]}{()"
  "}]]}(<(({}()){<><>})<(()())[<>()]>>{\r\n"
  "<[({[[{<([[{<{{}}>(<<><>>{[]()}}}[{{<>{}}<()<>>}{[[][]]{"
  "[]<>}}]]<((([][])<<>{}>)<((){})[{}()\r\n"
  "{{[{<[{<<{<<(<{}[]><(){}>)<{<>[]}[{}()]>>[{{[]<>}<()<>>}"
  "[{{}<>}(<>())]]><[(({}<>)({}<>))<<{}<>><<\r\n"
  "{<<({(<[(<[[{{[][]}{{}}}<[[]()]<<>())>]](<{{{}<>}{()<>}}"
  ">)>)]<(({{[[[][]]]}<[[(){}]<[]{}>]>}))>\r\n"
  "({{([<<{[(<<{{<>()}<[]()>}<{(){}}{{}{}}>><(<{}[]>{<>{}})"
  "<([]<>)([]())>>>)]}>>[[{<{[<<(<>{})\r\n"
  "<{({{({<<[{<<{<>}{{}[]}>>[[{[]<>}{()()}]<<[][]>(()())>]}"
  "]{<<{[(){}><<><>>}{{<>[]}(()<>)}>[<<(){}>[[]\r\n"
  "<({<([[[([({<{(){}}([][])>}<[{[][]}{<>()}]<[(){}][{}<>]>"
  ">){<({{}()}{[]{}})>{[{[]{})[()[]]]<[\r\n"
  "{([({[(([<{((({}()){{}<>})(<()[]>{()()})){[({}{})<<>[]>]"
  "([{}][()])}}>]{<<[<[[][]]{[][]}>({[]{}}<{}()>)\r\n"
  "<<{[<({[(<[({<{}{}><()()>})]{<<[<><>]>>}>){{[((<<><>><{}"
  "[]>)){<{[]()}(<>())>}]{{{({}<>){[]<>}}{{\r\n"
  "<<({<[{(([{({({}())(<>{})}{((){})})[<[[][]]([][])>]}<<[["
  "<>{}][[]<>]]<([]<>)[()()]>><<<{}[]>(()())>>]])[[[\r\n"
  "[{<({<(({([([{[]}]<{[]()}[()<>]>]{{(()())<{}()>}{{[]()}<"
  "()<>>}}][{(<[]<>>[<>{}])[<[]<>><<><>>]}<(<<\r\n"
  "<{{((<<(<{({<{<>{}}[[]{}]>})[[{{[][]}{{}<>}}]]}[(([(()[]"
  ")<()<>>]((<>[])<<>{}>))<[[<>()][<>\r\n"
  "{[([<<<({[{<<[<>{}]<[]<>>>>({<<>{}>((){})}<{[][]}{[]{}}>"
  ")}<{(([]{})<<>>><{{}()}(<><>)>}<{<[]\r\n"
  "<({[[{{{([(({[{}<>]{[]<>}}[{()<>}<(){}>])<[<{}()>[[]<>]]"
  ">)]{{[<{<>()}[{}()]><<<><>>{[][]}>]([[[][]](<>())\r\n"
  "[<<{{<[[{[<(<{()[]}[(){}]><[()[]]<<>{}>>)>[[[[[]{}]{[][]"
  "}]{<[]{}><(){}>}]({{<>()}<<>{}>}({()<>\r\n"
  "[<[[(<[[[[<<<{()<>}[[]()]>>{[[<>[]]{[]{}}]{[<>[]][[]<>]}"
  "}>]]({[({[<><>]<{}<>>}<<(){}>[{}()]>)[[<<>\r\n"
  "<{{({<([{({({{{}()}((){})})([([][])[[]<>]](([])[{}<>]))}"
  "<(([()<>][()[]])[<[]><{}{}>])<{({}<>)<<>()>}\r\n"
  "{<[[[([{[<<(({{}<>})(<{}()](<>{}))){[[<>[]]({}<>)]}><[(("
  "<>()){[][]})[({}())<{}<>>]]<({<>[]}{{}<>\r\n"
  "{<[<[({<({(<(({}[])<{}>)((<>[])({}<>))><(([][])(()()))>)"
  "})({{({{<>())}<{<>()}([]{})>)[[(()<>)<<>[]>](<<\r\n"
  "{(<(<<{{({{<{(<>[])[[]]}{(<>{})>>{<(()())[{}{}]>{{[]{}}{"
  "{}<>}}}}(<{[{}[]]<(){}>}[[{}{}]{()<>}]>([{<>{}}[<>]]\r\n"
  "[{[{{[[[{([<(({}{})<[][]>)<([]{})<<>{}>>>[{[[]{}]([]<>)}"
  "{(()())[<>[]]}}]((<([]<>)[()<>]>(<{}()>)){<[<>[]](\r\n"
  "{<<<<{(({<<({(()())(<>{})}<([][])[[]()]>)[{{[]()}(<>)}]>"
  "<{[<<>()>]{<[]()>[[]<>]}}>>}>[(<<<[{()<>}{{}<>\r\n"
  "[{[(<[<{<[({[{[]<>}<[][]>]{{<>()}}}[({<>[]}{{}{}}){[[]<>"
  "](<><>)}])({({{}<>}<{}<>>)}<[((){})[{\r\n"
  "{<[[{[(((<({([[]<>](()<>))}[{{()<>}({}())}(<[]{}>{[]{}})"
  "])<<(({})[{}{}])[(<>{})<()>]>>>))<[{{<[{{}<>}[[][]\r\n"
  "[[<<([{<{((([<{}{}>(<><>)][({}[])])[[(<>{}){{}<>}](<()[]"
  ">[[]()])])[{({()}{{}{}})(({}())([]{}))}({[{}[]]{\r\n"
  "<<((<[<(([<{[<{}<>>][<{}{}>{{}[]}]}[{<<>()>[[]{}]}{[()[]"
  "]([]{})}]><{({()()}{[]})<{[]()}[{}{}]>}{((<>(\r\n"
  "({<<{[(({{<<{[[]()]{{}()}}{[<>{}]<[][]>}>{(([]())[<>[]])"
  "{<[]{}>{<><>}}}>{{{{<>[]}{{}()]}{{<>{}}[{}{}\r\n"
  "{((<<{<<<((<(({}{})(()))[[()()]{{}<>}]>{(<()[]][{}[]])[["
  "<>[]]<<>>]}))>([[{({()[]}{()[]})(<[][]><{}{}>)}[[[[]["
  "\r\n"
  "[(<<{<[(<([<(<[]()><()()>)>]{({[<><>]{[][]}})}}<[([(()()"
  ")[{}]]<<(){}>[<><>]>)][<<((){})>>[[{<><>}[{}()]][([]\r\n"
  "[<{[<[[(({{{[<{}[]><<>{}}][[{}<>]<<>()>]}{[{[]}[{}[]]]{["
  "()[]]<{}<>>}}}[({<(){}>{{}{}}}<(()[])(\r\n"
  "<{([{(<[<<<<[[{}{}]([])](<{}{}>[[][]])>{{(()[]){<>[]}}<["
  "<><>][(){}]>}>(([[{}{}]{{}<>}](<()()>))<<(<>{})<<\r\n"
  "[(<[[[[(({[<<<<>{}><<><>>>>]<{[{{}<>}<()>]{{[]{}}[<>[]]}"
  "}<<<()[]>([]<>)><(<>{}){[]()}>>>}))[[([{<\r\n"
  "[([(({<{([<[({{}{}}<{}{}>)[[[]{}]({}<>)]]<[([]<>)({}[])]"
  "{(()[]){[]<>}}>>{[<{[]<>}>{[()[]]<()<>>}][<([\r\n"
  "{<[<[[<[{({{<[[]()][{}()]>({()[]}(()[]))}([[<>()]])})}{{"
  "<{{[<>]<[]()>}{<{}[]]}}<{{()[]}<[]<>>}([()()]<{}()>)>"
  "\r\n"
  "[{{(<(([([((<{{}{}}({}())>{{<>[]}<[]{}>})([{<>()}{{}<>}]"
  "{<()<>>[{}{}]})>]{{[<(<>){[]{}}>{<<>[]>[()[]]}]\r\n"
  "[<<(({[[({({[{[]{}}<<>[]]]{[()()]<<>()>}}{[{<>[]}<()>]})"
  "{{<<()[]>{[]{}}><<<>>[[]()]>}}})<[{{[<\r\n"
  "<([{{{(<{{{<<({}()){(){})>{<{}{}>{<>()}}>{(<()[]>({}()))"
  "<({}())<()()>>}}[(({[]{}}[<>{}])([{}<>])){{{{}}(<\r\n"
  "<<{[{[[<([[<(<{}<>><()[]>)<{<>()>[<>()]>>]][{{{{()<>}<{}"
  "[]>}[<[]()>[()[]]]}<{[{}](<>{})}>}[<[[<><>]<\r\n"
  "[{[([{<<[([[{<<>[]>(<>())}((()())({}{}))>{[({}())<<>()>]"
  "<(<><>){()<>}>}]({([(){}]<[]<>>){<[\r\n"
  "[{<<[[<((({[<[[]()]>(<<>>(<>{}))]<<([]<>)[<>{}]>{<[][]>("
  "<>())}>}(([{{}<>}{{}[]}]{[()[]]{{}{}}})<\r\n"
  "([<<{[([[<{<{({}())}{(()<>)}>[<[{}<>][[]()]><[<>{}]({}[]"
  ")>]}<[([<>()]){[()<>]([]{})}][[[{}[]]][\r\n"
  "<{[[{({<{(<[[(()[])<<>[]>]<<<>()>{[]<>}>]>{[<(<>()){<>[]"
  "}>(([][]))]([<[]<>><{}{}>]([<>{}](()()\r\n"
  "{[([{{(([[<{<[{}<>]([]{})>[((){})<()[]>]}[[{{}<>}<[][]>]"
  "<{[]<>}([]())>]>{<{{{}<>}{(){}}]{{[]\r\n"
  "[{{<[([<{[[({[{}[]][(){}]}<{<>()}<[]{}>>)]<<({()<>}[[]()"
  "]){[<>[]]<()()>}>>]{[<<(<>[])<[]<>>\r\n"
  "<[{<[<((<({[{[{}[])(<>{})}(({}[])[<>])]})>[[([{((){})<<>"
  ">}])[<((()<>)[{}{}])[[(){}]]>{<<{}()>{\r\n"
  "({[[[[[[[<((<[<><>][()[]]><(<><>)>))>{{<(<<>[]>{{}})[[[]"
  "<>]({}())]><{{()<>>[[]<>]}[<{}{}>{<>{}}]>}[{<(<\r\n"
  "([([(<[{<[[{<{<>[]}>{{{}()]<{}()>}}<([{}()]{{}<>})[([][]"
  "){{}<>}]>]{<{{[]{}}{{}<>}}<[[]()]>>[[{()[]}{(){}}]{[\r\n"
  "{<{([([{<<<<{[<>()]<[]>}[[()<>]<()>]>>(([[<>[]][{}{}]])("
  "(([][]){<><>})([[]{}]<()<>})))><{<<[()()]<[]()>\r\n"
  "[{({[{[(([([{{{}<>}{()<>}}(([]<>){{}{}})][<{()}{{}{}}>(<"
  "<>()>[<>()])])]))<<<({((<>()){[]{}})[{\r\n"
  "([([(<(({<(<<<{}[]>){{<><>}[[]{}]}>[{{[][]}<<><>>}])<{<<"
  "[]>(()[])>[[{}()]<[]<>>]}{({[]()}[<><>]){{{}[]}[{\r\n"
  "{{[{<{((<((<<<(){}>>{([]{})[[]()]}>([({}{})]{<()>{[]{}}}"
  "))[[(<<><>>)]<<<{}{}><(){}>>[({}[])({}\r\n"
  "<{[({{<<((([[[(){}][{}<>]]<(<>[])<[]()>>]<[[{}()]({}())]"
  ">)({<[(){}]>{(<><>){[]{}}}})){{{<<[]()>[()<>]>[<\r\n"
  "(<[[(<((<[[<{[[]()][()<>]}<[[][]]<[][]>>>((({}())((){}))"
  "(<{}<>]{()()}))]]>[[{<{[{}<>]}>([[{}<>](()\r\n"
  "<<<{<(<[<{[[([<>()][()<>])(<<>()>[{}()])][<{<>()}>]]}<<<"
  "({<>[]}([]{}))<<(){}>>>[[({}())(())]]>{({({}[\r\n"
  "([<<<{<{({<[{(()<>)<()()>}]<[({}<>)]>>{[[{<>{}}(()[])](<"
  "<>>[[]<>])]}}{<<{<<>()>}><{{<>()}<<><>>}{<(){}>[(){\r\n"
  "{{(<<[{{({({({[]()}<{}()>)(([][])[()<>])}<{(<>())[()<>]}"
  ">)[[(([]<>)[[]<>])]]}[({[({}()){{}>]})\r\n"
  "<((((<{{{{{[{[(){}]([][])}[{<>{}}([]<>)]]([((){})]<[[][]"
  "][<><>]>)}}<(<{(()<>)([]{})}><([()<>]{<>()\r\n"
  "<{<{<(<(<<<(<[[][]]<<>()>><[<>[]][[]]>)[<[<>{}][<>()]>]>"
  "{<{(()){{}()}}[<{}<>><<><>>]>[{<{}[]><<><>>}]}>{(\r\n"
  "{[{<<(<([({{<<[]<>>[<>{}]>{<{}<>>([]<>)}}([<<>{}>((){})}"
  "<{<>[]}>)}<<({{}()})<(<>[])[[]]>>[[<<>[]>{()[]}][[[]\r\n"
  "<<<<[<{<[{{(<[<>]<[]{}>><(<>{}){{}()}>)[[{<>{}}{{}[]}](<"
  "[]>[<>{}])]}{{[{{}[]}(()<>)]<([]())(\r\n"
  "[([({[[{<{{<<{()()}<{}{}>>({{}{}}[{}{}]>>[{(<>{})<()()>}"
  "[[<>[]]<{}()>]]}[<<{<><>}(()[])>(<{}[]>\r\n"
  "[<<[[(({<[[([(<>[]){()[]}][(<><>)[<>[]]])]<({<{}{}>(()[]"
  ")}[<{}<>>({}())])((({}{})<{}[]>)[{{}<>}{{}{}}])\r\n"
  "(({({[<<[<[([<<><>){[]{}}][<()<>><[]<>>])({[{}<>]<[]<>>}"
  "<({}{})[{}[]]>)][{((<><>)(()[]))}[<<{}()>{<>\r\n"
  "<((<[(<(<[[([{(){}}<<>()>]({(){}}{(){}}))]][{{[{[][]]{<>"
  "{}}]{[{}<>]([]{})}}}[<{[[]{}][<>()]}[(()[])<\r\n"
  "{((<<{({<[([(([]())([]()))(<[][]>{()})])<<[<<>[]>[<>())]"
  "(<()<>><()()>)>>]>{{[{<[<>{}]<[]<>>>}<{{<><>}<\r\n"
  "{<<<<[((<{({<{<>[]}{<>()}><{<><>){<>()}>}([[[]()][(){}]]"
  "<<()<>>{<><>}>)){[{{<>[]}{[]()}}{(<>())\r\n"
  "({<<<<[{<{<{(<{}{}><()[]>)[{<>{}}]}>(<[([]{}){<><>}]{<<>"
  "<>>[()[]]}}<[{()[]}{()[]}][{{}[]}{[][]}]>)}>}]\r\n"
  "({({{<{{{[<{{(<><>)}({[]()}{<><>})}{<[{}[]]<[]<>>>[(<>[]"
  "){()()}]}><[[<<>()><[][]>]]>]<<[{[[]()](\r\n"
  "<{[<([<[({([{(<>{}){()}}<([]<>)[{}{}]>)[[<()[]><{}<>>][("
  "<>{})[()()]]]){<{({}){{}{}}}<{<>()}[[\r\n"
  "{((({[{<{[<({[{}()]({}{})}<(<>)(()())>)({[[]{}](<><>)}[<"
  "<>[]>[<>[]]])><<[{[]<>}][([][])<[]()>]>(\r\n"
  "({[((([<[(<{[(<>[])(()[])]}<[([][])([]<>)]<(()())>>>(((["
  "(){}])<(<>())<<><>>>)<[[()<>>{{}{}\r\n"
  "[{{((<{[(<{[{{[][]}{<><>}}[[()[]]<[][]>]]<<({}[])((){})>"
  ">}>[(<([[]()]<{}[]>)([{}<>])>({<<>[]>(<>[])}[(\r\n"
  "((<((<[<(<[{(<[]()>[()[]])((())(()[]))}{[[{}<>]([]{})]<["
  "[]()]>}]>)><([<[<{[]<>}<{}[]>>[[(){}]]]({[[][]]{\r\n"
  "[(([([(([<<(<{<>}{[][]}>)[{[()<>]{{}{}}}({{}{}}<[]()>)]>"
  ">{([({<><>}<[]>){<{}{}>[()<>]}]<<[[]{}]{{\r\n"
  "[(<{<[[{[<<{<[[]<>]<[]()>>{{[]<>}}}><[[{{}()}([]<>)]][({"
  "{}<>}{{}{}})<[[][]](()())>]>><<({([]{})<<>{}>}){\r\n"
  "<{[<[(<<[[[[{<{}[]><[]<>>](<{}[]>{()[]})](<[<>()][[][]]>"
  "([()()]<<>()>))]]({([<()[]>[{}()]])<{[[]()]}{<[][\r\n"
  "<{{{([[([{{{<({}[])>}}<<({<>{}}{{}()))<({}{}){[]{}}>>[[("
  "<>())[{}[]]]<<[]{}>{<>[]}>]>}([{[<<\r\n"
  "[{<(({{<[{[[([<>{}]{<>{}})(((){})[{}[]])][{<{}<>>}[([]{}"
  ")<{}>]]]}{<<([<>{}][()[]})[{{}}<{}()>]>(\r\n"
  "[((<[{<<[<[{[{{}()}[{}[]]]<[<>[]][{}<>]>}]>]{[([([[]<>]{"
  "{}[]})[<(){}>[()[]]]]<<({}[])<[]()>>[(\r\n"
  "[{([<{([(({(<[[]<>]>{({}()){<><>}})([([]())[[]<>]])}(<[("
  "{}())(<>())]<{()<>}[[]<>]>>)){{([[<><>][()[]>](<[][]>"
  "\r\n"
  "<<(<<<{{{<<(<<[]<>>{<>()}>[<[]>])<{<{}<>>}{[{}{}]<<>[]>}"
  ">><{<<<>{}>({}<>)>}{([()[]]<<>()>)(<<><>><[]{}>)}>><\r\n"
  "{[<<[{{(<<[((({}{})[{}{}])(({}<>)[{}()]))]>{([[[(){}]<{}"
  "<>>][[{}[]]{()[]}]](<{[]{}}(<>[]>>))}>)}[(<\r\n"
  "<{{[[<[[[(((({<><>}{{}{}})<[<>()]([]())>)[<{{}<>}>{((){}"
  ")[[][]]}]))<{<{[[]()]<[][]>}{<[]<>>(<>\r\n"
  "{<[<<([[[[{{<({}<>)<{}()>>[(<>())({}[])]}<{([][])({}<>)}"
  ">}(({[<>{}]{[]{}}}<(<>{})>)[<{[][]}{[][]}>{({}()){\r\n"
  "[[({{[(((({<{{<><>]}{(<><>){<><>}}>(<[()<>]<{}{}>>((<><>"
  "){<>[]}))}{<[[{}[]]([][])][{{}[]}[\r\n"
  "[{{<([((<<{(((<><>)<(){}>))((({}[]){<>()})(<{}()}({}{}))"
  ")}><[<<{<>[]}[()<>]>{[<>{}]{[]()}}>{{\r\n"
  "[<{<{{({{<<<{[<><>]<[][]>}<<(){}>(<>)>>>([<{[]()}{{}[]}>"
  "{<{}()><[]<>>}]([([]())[[]]]([[]()]<<>{}>)))>\r\n"
  "<[{<[(<{{([[<[{}{}]><([]<>)(()())>][<((){})[[]()]>[([]<>"
  "){<><>}]]]{<{<[]<>><<>>}{<{}<>><[]{}\r\n"
  "{[((((((({([([(){}]([]<>)){[{}](<>[])}]{[<<>()>([]<>)]<<"
  "[][]>{[]<>}>})<<[{{}[]}(<><>)]<[()<>]\r\n"
  "([({(([[[([[((<>{}){{}{}}]{[()<>]((){})}]((({})(<>[])){["
  "{}{}]<<>()>})]{({(())[<>{}]}((<>{})\r\n"
  "{<[<{(<[{<[{(<{}()>)[<<>[]>(()<>)]}([{()[]}<{}<>>])}(({<"
  "[]{}><{}{}>}([[]<>]{{}()})){[[{}<>]\r\n"
  "<<({[[<<[([(<[<><>]{<>{}}>{<(){}>(()())})<{[<>{}]<()[]>}"
  ">]<{[{<>{}}<<>()>]<{{}()}[()<>]>}<[[{}<>][{\r\n"
  "\r\n";
// ////////////////////////////////////////////////////////////////////////////////////////////////

typedef std::stack<const char> CharStack;


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

  // Get all lines from the input file.
  std::string line;
  while (std::getline(iss, line)) {
    line = rtrim(line);
    std::cout << line << "\t";
    const char* pC = line.data();
    const auto len = line.length();
    CharStack stk;
    int i        = 0;
    bool corrupt = false;

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
            std::cout << "Expected " << closeBracket
                      << " but found " << c << " instead";
            const auto pts = getIllegalPoints(c);
            numPoints += pts;
            corrupt = true;
          }
        }
      }
    } // while

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
      std::cout << "Line incomplete with score "
                << incompleteScore << std::endl;
    }
  }

  // After processing all lines, handle all incomplete lines
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
  incompleteScoresArr.clear();
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
  
  EXPECT_EQ(numPoints, 394647);
  EXPECT_EQ(incompletePts, 2380061249);

}

// ////////////////////////////////////////////////////////////////////////////////////////////////
// Get message dimensions, height and width.


int main(int argc, char** argv) {
  // The following line must be executed to initialize
  // Google Mock (and Google Test) before running the tests.
  ::testing::InitGoogleMock(&argc, argv);
  const int gtest_rval = RUN_ALL_TESTS();

  return gtest_rval;
}
