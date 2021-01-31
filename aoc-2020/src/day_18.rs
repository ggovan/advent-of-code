use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day18;

impl AocDay for Day18 {
    type Input = String;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        18
    }
    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2020/day_18.in")?)
    }

    /// TODO make this totally streaming, i.e. pet rid of the prog buffer.
    fn part_1(input: &Self::Input) -> Self::Result1 {
        input
            .lines()
            .map(|l| {
                let mut lexed_iter = l.chars().filter_map(lex);
                let mut prog: [Option<Symbol>; 90] = [None; 90]; // 90 should be big enough
                parser(&mut lexed_iter, &mut prog, 0);

                // print_expr(&prog);

                compute(&prog)
            })
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        input
            .lines()
            .map(|l| {
                let mut lexed_iter = l.chars().filter_map(lex);
                let mut prog: [Option<Symbol>; 90] = [None; 90]; // 90 should be big enough
                parser_part_2(&mut lexed_iter, &mut prog, 0);

                // print_expr(&prog);

                compute(&prog)
            })
            .sum()
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
        n if ('0'..='9').contains(&n) => Some(Val(n as u64 - '0' as u64)),
        _ => None,
    }
}

/// Parse an expression into reverse polish notation with no operator precedence
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

/// Parse an expression into reverse polish notation with + operator precedence
fn parser_part_2<It: Iterator<Item = Symbol>>(
    iter: &mut It,
    prog: &mut [Option<Symbol>],
    mut prog_ptr: usize,
) -> usize {
    let mut open_add = false;
    let mut open_mults = 0;
    loop {
        prog_ptr = match iter.next() {
            Some(Val(v)) => {
                prog[prog_ptr] = Some(Val(v));
                if open_add {
                    prog[prog_ptr + 1] = Some(Add);
                    open_add = false;
                    prog_ptr + 2
                } else {
                    prog_ptr + 1
                }
            }
            Some(Add) => {
                open_add = true;
                prog_ptr
            }
            Some(Mult) => {
                open_mults += 1;
                prog_ptr
            }
            Some(Open) => {
                prog_ptr = parser_part_2(iter, prog, prog_ptr);
                if open_add {
                    prog[prog_ptr] = Some(Add);
                    open_add = false;
                    prog_ptr + 1
                } else {
                    prog_ptr
                }
            }
            Some(Close) => break,
            None => break,
        };
    }

    // multiplication has the lowest priority, so just a bunch on the end
    for _ in 0..open_mults {
        prog[prog_ptr] = Some(Mult);
        prog_ptr += 1;
    }

    prog_ptr
}

#[allow(unused)]
fn print_expr(expr: &[Option<Symbol>]) {
    expr.iter().filter_map(|x| *x).for_each(|s| match s {
        Add => print!("+ "),
        Mult => print!("* "),
        Val(v) => print!("{} ", v),
        _ => unreachable!(),
    });
    println!();
}

// Run a reverse-polish notation computation
#[allow(clippy::assign_op_pattern)]
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
