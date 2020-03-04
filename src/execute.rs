use std::{io, process};

use crate::code;
use crate::code::Code;
use crate::io::{print_error, read_line};
use crate::number::Num;

fn push_stack_wrap<T: code::State>(state: &mut T, idx: usize, num: Num) {
    match idx {
        1 => {
            if num.is_pos() {
                print!("{}", num.floor().to_int() as u8 as char);
            } else {
                print!("{}", -&num);
            }
        }
        2 => {
            if num.is_pos() {
                eprint!("{}", num.floor().to_int() as u8 as char);
            } else {
                eprint!("{}", -&num);
            }
        }
        _ => {
            state.push_stack(idx, num);
        }
    }
}

fn pop_stack_wrap<T: code::State>(state: &mut T, idx: usize) -> Num {
    match idx {
        0 => {
            let mut s = read_line();
            for c in s.chars().rev() {
                state.push_stack(0, Num::from_num(c as isize));
            }
            state.pop_stack(0)
        }
        1 => {
            process::exit(0);
        }
        2 => {
            process::exit(1);
        }
        _ => {
            state.pop_stack(idx)
        }
    }
}

pub fn execute<T: code::State>(mut state: T, code: &T::CodeType) -> T {
    let mut cur_loc = state.push_code((*code).clone());
    let length = cur_loc + 1;

    while cur_loc < length {
        let code = (*state.get_code(cur_loc)).clone();
        let cur = state.current_stack();

        match code.get_type() {
            0 => {
                push_stack_wrap(
                    &mut state,
                    cur,
                    &Num::from_num(code.get_hangul_count() as isize)
                        * &Num::from_num(code.get_dot_count() as isize),
                );
            }
            1 => {
                let mut n = Num::zero();
                for _ in 0..code.get_hangul_count() {
                    n += &pop_stack_wrap(&mut state, cur);
                }
                push_stack_wrap(&mut state, code.get_dot_count(), n);
            }
            2 => {
                let mut n = Num::one();
                for _ in 0..code.get_hangul_count() {
                    n *= &pop_stack_wrap(&mut state, cur);
                }
                push_stack_wrap(&mut state, code.get_dot_count(), n);
            }
            3 => {
                let mut n = Num::zero();
                let mut v = Vec::with_capacity(code.get_hangul_count());

                for _ in 0..code.get_hangul_count() {
                    v.push(pop_stack_wrap(&mut state, cur));
                }

                for mut x in v {
                    x.minus();
                    n += &x;
                    push_stack_wrap(&mut state, cur, x);
                }

                push_stack_wrap(&mut state, code.get_dot_count(), n);
            }
            4 => {
                let mut n = Num::one();
                let mut v = Vec::with_capacity(code.get_hangul_count());

                for _ in 0..code.get_hangul_count() {
                    v.push(pop_stack_wrap(&mut state, cur));
                }

                for mut x in v {
                    x.flip();
                    n *= &x;
                    push_stack_wrap(&mut state, cur, x);
                }

                push_stack_wrap(&mut state, code.get_dot_count(), n);
            }
            5 => {
                let n = state.pop_stack(cur);
                for _ in 0..code.get_hangul_count() {
                    state.push_stack(code.get_dot_count(), n.clone());
                }
                state.push_stack(cur, n);
                state.set_current_stack(code.get_dot_count());
            }
            _ => unreachable!(),
        }

        cur_loc += 1;
    }

    state
}