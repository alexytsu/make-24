use std::rc::Rc;

#[derive(Clone)]
pub enum Operator {
    Add,
    Mul,
    Sub,
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Add => f.write_str("+"),
            Mul => f.write_str("*"),
            Sub => f.write_str("-"),
            Div => f.write_str("/"),
        }
    }
}

pub struct Calculation {
    from: Option<Rc<Calculation>>,
    numbers: Vec<i32>,
    operator: Operator,
    at_index: usize,
}

use Operator::*;

impl Calculation {
    pub fn new_calcs(digits: Vec<i32>, from: Option<Rc<Calculation>>) -> Vec<Calculation> {
        let mut ret = Vec::new();
        for operator in [Add, Mul, Sub, Div] {
            for index in 1..digits.len() {
                let calc = Calculation {
                    numbers: digits.clone(),
                    from: from.clone(),
                    operator: operator.clone(),
                    at_index: index,
                };
                ret.push(calc);
            }
        }

        ret
    }
    pub fn print(&self) {
        println!("calculation {}", VecDisplay(self.numbers.clone()));
    }
    pub fn explain(&self) {
        match self.from.clone() {
            Some(parent) => {
                parent.explain();
                println!(
                    "{} operator {} at {} ",
                    VecDisplay(self.numbers.clone()),
                    self.operator,
                    self.at_index
                )
            }
            None => {
                println!("========================");
                println!(
                    "{} operator {} at {} ",
                    VecDisplay(self.numbers.clone()),
                    self.operator,
                    self.at_index
                )
            }
        }
    }
}

pub struct VecDisplay(Vec<i32>);

// TODO: impl for all inside that impls Display
impl Display for VecDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<");
        for el in self.0.iter().enumerate() {
            let (index, number) = el;
            if index == self.0.len() - 1 {
                return f.write_str(format!("{}>", number).as_str());
            }
            f.write_str(format!("{}, ", number).as_str());
        }

        Err(std::fmt::Error)
    }
}

use std::fmt::Display;

pub fn collapse(parent: Calculation) -> Vec<Calculation> {
    let new_numbers = generate_new_numbers(&parent);

    let mut res = Vec::new();
    if new_numbers.len() == 1 {
        if new_numbers.get(0).unwrap().eq(&24) {
            res.push(parent);
        }
        return res;
    } else {
        let new_calcs = Calculation::new_calcs(new_numbers, Some(Rc::new(parent)));
        for calc in new_calcs {
            let mut collapsed_calcs = collapse(calc);
            res.append(collapsed_calcs.as_mut())
        }
        res
    }
}

fn generate_new_numbers(parent: &Calculation) -> Vec<i32> {
    let new_numbers = parent
        .numbers
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, el| {
            let (index, value) = el;
            if index == parent.at_index {
                let last = acc.last_mut().unwrap();
                match parent.operator {
                    Add => *last += value,
                    Mul => *last *= value,
                    Sub => *last -= value,
                    Div => {
                        if *value != 0 {
                            *last /= value;
                        }
                    }
                }
            } else {
                acc.push(*value);
            }
            acc
        });
    new_numbers
}
