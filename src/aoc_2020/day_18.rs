use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day18;

impl Aoc2020 for Day18 {
    type Input = String;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        18
    }
    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2020/day_18.in")?)
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        input
            .lines()
            .map(|l| {
                // println!("I have a line {}", l);
                let mut lexed_iter = l.chars().filter_map(lex);
                let mut prog: [Option<Symbol>; 90] = [None; 90]; // 90 should be big enough
                parser(&mut lexed_iter, &mut prog, 0);

                // print_expr(&prog);

                compute(&prog)
            })
            .sum()
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        18
    }
}

#[derive(Debug, Copy, Clone)]
enum Symbol {
    Open,
    Close,
    Mult,
    Add,
    Val(u64),
}
use Symbol::*;

fn lex(c: char) -> Option<Symbol> {
    match c {
        '(' => Some(Open),
        ')' => Some(Close),
        '*' => Some(Mult),
        '+' => Some(Add),
        n if n >= '0' && n <= '9' => Some(Val(n as u64 - '0' as u64)),
        _ => None,
    }
}

fn parser<It: Iterator<Item = Symbol>>(
    iter: &mut It,
    prog: &mut [Option<Symbol>],
    mut prog_ptr: usize,
) -> usize {
    let mut open_instuction = None;
    loop {
        prog_ptr = match iter.next() {
            Some(Val(v)) => {
                prog[prog_ptr] = Some(Val(v));
                if open_instuction.is_some() {
                    prog[prog_ptr + 1] = open_instuction;
                    open_instuction = None;
                    prog_ptr + 2
                } else {
                    prog_ptr + 1
                }
            }
            Some(Add) => {
                open_instuction = Some(Add);
                prog_ptr
            }
            Some(Mult) => {
                open_instuction = Some(Mult);
                prog_ptr
            }
            Some(Open) => {
                prog_ptr = parser(iter, prog, prog_ptr);
                if open_instuction.is_some() {
                    prog[prog_ptr] = open_instuction;
                    open_instuction = None;
                    prog_ptr + 1
                } else {
                    prog_ptr
                }
            }
            Some(Close) => break,
            None => break,
        };
    }
    prog_ptr
}

fn print_expr(expr: &[Option<Symbol>]) {
    expr.iter().filter_map(|x| *x).for_each(|s| match s {
        Add => print!("+ "),
        Mult => print!("* "),
        Val(v) => print!("{} ", v),
        _ => unreachable!(),
    });
    println!();
}

fn compute(prog: &[Option<Symbol>]) -> u64 {
    let mut stack = [0; 90];
    let mut ptr = 0;

    for s in prog.iter().filter_map(|x| *x) {
        match s {
            Val(v) => {
                stack[ptr] = v;
                ptr += 1
            }
            Mult => {
                stack[ptr - 2] = stack[ptr - 1] * stack[ptr - 2];
                ptr -= 1;
            }
            Add => {
                stack[ptr - 2] = stack[ptr - 1] + stack[ptr - 2];
                ptr -= 1;
            }
            x => unreachable!("Should be weg: {:?}", x),
        }
    }

    stack[0]
}
