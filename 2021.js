// Day 5

input = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`
    .split("\n")
    .map((r) =>
        r.split(" -> ").map((c) => c.split(",").map((n) => Number.parseInt(n)))
    );

points = {};

input.map((wire) => {
    if (
        wire[0][0] == wire[1][0] ||
        wire[0][1] == wire[1][1] ||
        Math.abs(wire[0][0] - wire[1][0]) == Math.abs(wire[0][1] - wire[1][1])
    ) {
        let xf = wire[1][0] > wire[0][0] ? 1 : wire[1][0] < wire[0][0] ? -1 : 0;
        let yf = wire[1][1] > wire[0][1] ? 1 : wire[1][1] < wire[0][1] ? -1 : 0;

        for (
            x = wire[0][0], y = wire[0][1];
            x != wire[1][0] || y != wire[1][1];
            x += xf, y += yf
        ) {
            p = `${x},${y}`;
            let point = points[p];
            points[p] = 1 + (point ?? 0);
        }
        // for good luck
        let point = points[`${x},${y}`];
        points[`${x},${y}`] = 1 + (point ?? 0);

        //     console.log(wire[0], wire[1], xf, yf, [x, y])
    }
});

// console.log(Object.values(points))

console.log(Object.values(points).filter((x) => x > 1).length);

// Day 8

input =
    `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`
        .split("\n")
        .map((l) => l.split(" | ")[1].split(" "));

input
    .map(
        (l) =>
            l.filter(
                (w) =>
                    w.length == 2 ||
                    w.length == 3 ||
                    w.length == 4 ||
                    w.length == 7
            ).length
    )
    .reduce((a, b) => a + b, 0);

input =
    `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`
        .split("\n")
        .map((l) =>
            l
                .split(" | ")
                .map((side) =>
                    side.split(" ").map((w) => w.split("").sort().join(""))
                )
        );

input
    .map(
        (l) =>
            l[1].filter(
                (w) =>
                    w.length == 2 ||
                    w.length == 3 ||
                    w.length == 4 ||
                    w.length == 7
            ).length
    )
    .reduce((a, b) => a + b, 0);

input
    .map((line) => {
        line = line.map((side) =>
            side.map((w) => [
                w,
                w
                    .split("")
                    .reduce(
                        (acc, c) =>
                            acc + 2 ** ("g".charCodeAt(0) - c.charCodeAt(0)),
                        0
                    ),
            ])
        );
        all = [...line[0], ...line[1]];

        let _1 = all.find(([w, b]) => w.length == 2)?.[1];
        let _7 = all.find(([w, b]) => w.length == 3)?.[1];
        let _4 = all.find(([w, b]) => w.length == 4)?.[1];
        let _8 = all.find(([w, b]) => w.length == 7)?.[1];
        let _3 = all.find(
            ([w, b]) => w.length == 5 && ((b & _7) == _7 || (b & _1) == _1)
        )?.[1];

        let _9 = all.find(
            ([w, b]) => w.length == 6 && ((b & _4) == _4 || (b & _3) == _3)
        )?.[1];
        let _6 = all.find(
            ([w, b]) => w.length == 6 && b != _9 && (b & (_3 - _1)) == _3 - _1
        )?.[1];

        let _5 = all.find(
            ([w, b]) =>
                w.length == 5 && b != _3 && ((b & _9) == b || (b & _6) == b)
        )?.[1];
        let _2 = all.find(([w, b]) => w.length == 5 && b != _3 && b != _5)?.[1];

        let _0 = all.find(([w, b]) => w.length == 6 && b != _6 && b != _9)?.[1];

        let solved = {
            [_1]: 1,
            [_7]: 7,
            [_4]: 4,
            [_8]: 8,
            [_3]: 3,
            [_5]: 5,
            [_9]: 9,
            [_6]: 6,
            [_2]: 2,
            [_0]: 0,
        };

        return line[1]
            .map(([w, b], i) => 10 ** (3 - i) * (solved[b] ?? 10000))
            .reduce((a, b) => a + b, 0);
    })
    .reduce((a, b) => a + b, 0);


// Day 8

input=`2199943210
3987894921
9856789892
8767896789
9899965678`.split("\n").map(line => line.split("").map(n => Number.parseInt(n)));

basinPoints = input.flatMap((line,y) => 
  line.flatMap((v,x) => (//console.log(x,y),
    (y == 0 || v < input[y-1][x])
  		&& (x == 0 || v < input[y][x-1])
  		&& (y == input.length - 1 || v < input[y+1][x])
  		&& (x == input[0].length - 1 || v < input[y][x+1])
			&& [[v,x,y]] || []
    ))
)

// console.log(basinPoints)

console.log("Part 1:", basinPoints.reduce((a,[v,x,y])=> a+v+1,0))

basinPoints.map(([v,x,y]) => {
//   console.log(`${x},${y}`)
  visited = {};
  queue = [[x,y]];
  
  while(queue.length) {
    let [x,y] = queue.shift();
    if (visited[`${x},${y}`]) continue;
    visited[`${x},${y}`] = true;
//     console.log(`  ${x},${y}`)
    
    if  (y != 0 && 9 != input[y-1][x])
      queue.push([x,y-1])
  	if (x != 0 && 9 != input[y][x-1])
      queue.push([x-1,y])
    if (y != input.length - 1 && 9 != input[y+1][x])
      queue.push([x,y+1])
  	if (x != input[0].length - 1 && 9 != input[y][x+1])
      queue.push([x+1,y])
  }
  
  return Object.keys(visited).length
}).sort((a,b) => a<=b).splice(0,3).reduce((a,b)=>a*b,1)


// Day 10
input=`[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`.split("\n").map(line => line.split(""));

p1 = input.flatMap(line => {
  let stack = [];
  
  for (let c of line) {
    if (c == '<')
      stack.push('>')
    else if (c == '{')
      stack.push('}')
    else if (c == '(')
      stack.push(')')
    else if (c == '[')
      stack.push(']')
    else if (c != stack.pop()) {
      return [({
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137,
      })[c]]
    }
  }
  
  return []
  
}).reduce((a,b) => a+b,0)

console.log("Part 1:", p1)

p2s = input.flatMap(line => {
  let stack = [];
  
  for (let c of line) {
    if (c == '<')
      stack.push('>')
    else if (c == '{')
      stack.push('}')
    else if (c == '(')
      stack.push(')')
    else if (c == '[')
      stack.push(']')
    else if (c != stack.pop()) {
      return []
    }
  }
  
  return stack.reverse().reduce((a,c) => {
		return 5*a + ({
        ')': 1,
        ']': 2,
        '}': 3,
        '>': 4,
    })[c]    
  },0)
  
})
p2s.sort((a,b)=>a<=b)
p2s[(p2s.length-1)/2]


// Day 11

input=`5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`.split('\n').map(line => line.split('').map(c => Number.parseInt(c)));

function phase1(input) {
  
  return input.map(line => line.map(c => c+1))
}
  
function neighbours(arr, x, y) {
  return [
    [-1,-1],[0,-1],[1,-1]
    ,[-1,0],[1,0],
    [-1,1],[0,1],[1,1]
  ].map(([xd,yd])=>arr[yd+y]?.[xd+x]).filter(v => v != undefined)
}

function phase2(input) {
  while (input.some(line => line.some(v => v == 10))) {
    input = input.map((line, y) => line.map((c, x) => {
      if (c == 11) {
        return c;
      } else if (c == 10) {
        return 11;
      } else {
        let next = (neighbours(input,x,y)).filter(v => v == 10).length;
        next += c;
         return next > 10 ? 10 : next
      }
    }))
  }
  return input;
  
}

flashes = 0

function phase3(input) {
  return input.map(line => line.map(v => v == 11 ? (flashes++,0) : v)); 
}


function dbg(i) {
  console.log(i)
  return i;
}

for (i = 0; i < 100; i++)
	input = phase3((phase2((phase1(input)))));

console.log("part1", flashes)

i = 100;
while(!input.every(line => line.every(v => v == 0))){
  input = phase3((phase2((phase1(input)))))
  i++
}

console.log(i)

