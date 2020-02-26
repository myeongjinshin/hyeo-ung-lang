use std::cmp::{max, min};
use std::fmt;

pub struct Num {
    pos: bool,
    val: Vec<u32>,
}

impl Num {
    pub fn new(n: isize) -> Num {
        if n >= 0 {
            Num {
                pos: true,
                val: vec![n as u32],
            }
        } else {
            Num {
                pos: true,
                val: vec![(-n) as u32],
            }
        }
    }

    pub fn from_vec(v: Vec<u32>) -> Num {
        let mut res = Num {
            pos: true,
            val: v,
        };
        res.shrink_to_fit();
        res
    }

    pub fn zero() -> Num {
        Num {
            pos: true,
            val: vec![0],
        }
    }

    pub fn one() -> Num {
        Num {
            pos: true,
            val: vec![1],
        }
    }

    pub fn clone(&self) -> Num {
        Num {
            pos: self.pos,
            val: self.val.clone(),
        }
    }

    pub fn is_pos(&self) -> bool {
        self.pos
    }

    pub fn is_zero(&self) -> bool {
        self.val == vec![0]
    }

    pub fn to_string(&self) -> String {
        self.to_string_base(10)
    }

    pub fn to_string_base(&self, _base: usize) -> String {
        // TODO: https://en.wikipedia.org/wiki/Double_dabble
        unimplemented!()
    }

    fn shrink_to_fit(&mut self) {
        while match self.val.last() {
            Some(x) => *x == 0,
            None => false,
        } {
            self.val.pop();
        }
    }

    fn add_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; max(lhs.len(), rhs.len()) + 1];

        for i in 0..min(lhs.len(), rhs.len()) {
            let mut t = (lhs[i] as u64) + (rhs[i] as u64) + (v[i] as u64);
            if t > u32::max_value() as u64 {
                v[i + 1] += 1;
                t -= u32::max_value() as u64;
            }
            v[i] = t as u32;
        }

        let t = if lhs.len() < rhs.len() {
            rhs
        } else {
            lhs
        };

        for i in min(lhs.len(), rhs.len())..t.len() {
            v[i] = t[i];
        }

        v
    }

    fn sub_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> (Vec<u32>, bool) {
        let mut v = vec![0; max(lhs.len(), rhs.len())];

        let (a, b, swapped) = if Num::less(lhs, rhs) {
            (lhs, rhs, false)
        } else {
            (rhs, lhs, true)
        };

        for i in 0..a.len() {
            let mut t = (a[i] as i64) - (b[i] as i64) - (v[i] as i64);
            if t < 0 {
                v[i + 1] += 1;
                t += u32::max_value() as i64;
            }
            v[i] = t as u32;
        }

        for i in a.len()..b.len() {
            let mut t = (b[i] as i64) - (v[i] as i64);
            if t < 0 {
                v[i + 1] += 1;
                t += u32::max_value() as i64;
            }
            v[i] = t as u32;
        }

        (v, swapped)
    }

    // TODO: https://en.wikipedia.org/wiki/Sch%C3%B6nhage%E2%80%93Strassen_algorithm
    fn mult_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; lhs.len() + rhs.len() + 1];

        for i in 0..(lhs.len() + rhs.len()) {}

        v
    }

    // TODO: https://en.wikipedia.org/wiki/Division_algorithm
    fn div_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        unimplemented!()
    }

    fn less<T: PartialOrd>(lhs: &Vec<T>, rhs: &Vec<T>) -> bool {
        if lhs.len() != rhs.len() {
            lhs.len() < rhs.len()
        } else {
            for i in (0..lhs.len()).rev() {
                if lhs[i] != rhs[i] {
                    return lhs[i] < rhs[i];
                }
            }
            false
        }
    }

    pub fn minus(&mut self) {
        self.pos = !self.pos;
    }

    pub fn add(lhs: &Num, rhs: &Num) -> Num {
        let mut need_flip = false;
        let mut res = Num::from_vec(if lhs.pos {
            if rhs.pos {
                Num::add_core(&lhs.val, &rhs.val)
            } else {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip = need_flip ^ swapped;
                tmp
            }
        } else {
            let t = if rhs.pos {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            } else {
                Num::add_core(&lhs.val, &rhs.val)
            };
            need_flip = !need_flip;
            t
        });
        if need_flip {
            res.minus();
        }
        res.shrink_to_fit();
        res
    }

    pub fn sub(lhs: &Num, rhs: &Num) -> Num {
        let mut need_flip = false;
        let mut res = Num::from_vec(if lhs.pos {
            if !rhs.pos {
                Num::add_core(&lhs.val, &rhs.val)
            } else {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip = need_flip ^ swapped;
                tmp
            }
        } else {
            let t = if !rhs.pos {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            } else {
                Num::add_core(&lhs.val, &rhs.val)
            };
            need_flip = !need_flip;
            t
        });
        if need_flip {
            res.minus();
        }
        res.shrink_to_fit();
        res
    }

    pub fn mul(lhs: &Num, rhs: &Num) -> Num {
        let mut res = Num::from_vec(Num::mult_core(&lhs.val, &rhs.val));
        if lhs.pos ^ rhs.pos {
            res.minus();
        }
        res.shrink_to_fit();
        res
    }

    pub fn div(lhs: &Num, rhs: &Num) -> Num {
        let mut res = Num::from_vec(Num::div_core(&lhs.val, &rhs.val));
        if !lhs.pos {
            res = Num::add(&res, &Num::one());
        }
        if !rhs.pos {
            res.minus()
        }
        res.shrink_to_fit();
        res
    }

    pub fn rem(lhs: &Num, rhs: &Num) -> Num {
        let q = Num::div(&lhs, &rhs);
        Num::sub(&lhs, &Num::mul(&q, &rhs))
    }

    // TODO: https://en.wikipedia.org/wiki/Lehmer%27s_GCD_algorithm
    pub fn gcd(lhs: &Num, rhs: &Num) -> Num {
        unimplemented!()
    }

    pub fn neg(v: &Num) -> Num {
        Num {
            pos: !v.pos,
            val: v.val.clone(),
        }
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            true
        } else {
            self.pos == other.pos && self.val == other.val
        }
    }
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}