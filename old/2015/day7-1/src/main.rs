use std::collections::HashMap;

use anyhow::Context;
use neure::{ctx::re_policy, prelude::*};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
pub enum Op<'a> {
    Signal(u16),

    Wire(&'a str),
}

#[derive(Debug)]
pub enum Inst<'a> {
    And((Op<'a>, Op<'a>)),

    Or((Op<'a>, Op<'a>)),

    LShift((Op<'a>, Op<'a>)),

    RShift((Op<'a>, Op<'a>)),

    Not(Op<'a>),

    Store(Op<'a>),
}

type Memory<'a> = HashMap<&'a str, u16>;
type InstMap<'a> = HashMap<&'a str, Inst<'a>>;

fn main() -> anyhow::Result<()> {
    let signal = neu::digit(10)
        .repeat_one_more()
        .map(map::from_str::<u16>())
        .map(|v| Ok(Op::Signal(v)));
    let wire = char::is_ascii_lowercase.repeat_one_more();
    let op = signal.or(wire.map(|v| Ok(Op::Wire(v))));
    let and = op.sep_once("AND", op).map(|v| Ok(Inst::And(v)));
    let or = op.sep_once("OR", op).map(|v| Ok(Inst::Or(v)));
    let lshift = op.sep_once("LSHIFT", op).map(|v| Ok(Inst::LShift(v)));
    let rshift = op.sep_once("RSHIFT", op).map(|v| Ok(Inst::RShift(v)));
    let not = "NOT".then(op)._1().map(|v| Ok(Inst::Not(v)));
    let store = op.map(|v| Ok(Inst::Store(v)));
    let inst = and
        .or(or.or(lshift.or(rshift.or(not.or(store)))))
        .into_box()
        .sep_once("->", wire);

    let inst_pairs: Vec<(Inst, &str)> = CharsCtx::new(INPUT)
        .with_policy(re_policy(neu::whitespace().repeat_full()))
        .ctor(&inst.collect())?;

    let mut memory: Memory = HashMap::default();
    let insts: InstMap = inst_pairs.into_iter().map(|v| (v.1, v.0)).collect();

    dbg!(run("a", &insts, &mut memory)?);

    Ok(())
}

pub fn run<'a>(name: &'a str, insts: &InstMap<'a>, memory: &mut Memory<'a>) -> anyhow::Result<u16> {
    let get_or_run = |insts: &InstMap<'a>, memory: &mut Memory<'a>, op: &Op<'a>| match op {
        Op::Signal(digit) => Ok(*digit),
        Op::Wire(wire) => run(wire, insts, memory),
    };
    let value = if let Some(value) = memory.get(name) {
        *value
    } else {
        let inst = insts
            .get(name)
            .with_context(|| anyhow::anyhow!("Can not get value of wire `{}`", name))?;

        let value = match inst {
            Inst::And((l, r)) => get_or_run(insts, memory, l)? & get_or_run(insts, memory, r)?,
            Inst::Or((l, r)) => get_or_run(insts, memory, l)? | get_or_run(insts, memory, r)?,
            Inst::LShift((l, r)) => get_or_run(insts, memory, l)? << get_or_run(insts, memory, r)?,
            Inst::RShift((l, r)) => get_or_run(insts, memory, l)? >> get_or_run(insts, memory, r)?,
            Inst::Not(op) => !get_or_run(insts, memory, op)?,
            Inst::Store(op) => get_or_run(insts, memory, op)?,
        };

        memory.insert(name, value);
        value
    };

    Ok(value)
}
