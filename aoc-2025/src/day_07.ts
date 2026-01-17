let inputTest = `.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............`

let lines = inputTest.split("\n").map(l => l.split(""))
let linesClone = lines.map(l => l.map(v=>v))
let splits = 0

console.log(lines.map(l => l.join("")).join("\n"))

for(let i = 1; i < lines.length; i++){
let line = lines[i]
for(let j = 0; j < line.length; j++) {
    if(line[j]=='.'){
    if(lines[i-1][j]=='|' || lines[i-1][j] == 'S')
        line[j]='|'
    } else if (line[j] == '^') {
    if (lines[i-1][j]=='|') {
        if (j > 0) line[j-1] = '|'
        if (j < line.length-1) line[j+1] = '|'
        splits++
    }
    }
}
}

lines = linesClone.map(l => l.map(v => v=='.'?0:v))

for(let i = 1; i < lines.length; i++){
let line = lines[i]
for(let j = 0; j < line.length; j++) {
    if(line[j]!='^'){
    if(lines[i-1][j] == 'S') line[j]+=1
    else if(lines[i-1][j]!='^') line[j]+=lines[i-1][j]
    } else if (line[j] == '^') {
        if (j > 0) line[j-1]+= lines[i-1][j]
        if (j < line.length-1) line[j+1] += lines[i-1][j]
    }
}
}

console.log(lines.map(l => l.join("")).join("\n"))
console.log("part1", splits)
console.log("part2", lines[lines.length-1].reduce((acc,v)=> acc+v,0))
