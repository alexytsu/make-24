use make_24::calculation::{self, Calculation};

fn main() {
    for num in 0..10000 {
        let str = format!("{:0>4}", num);
        let digits: Vec<i32> = str
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect();

        let calcs = Calculation::new_calcs(digits, None);
        let mut res = Vec::new();
        for calc in calcs {
            res.append(calculation::collapse(calc).as_mut());
        }
        if res.len() > 0 {
            println!("{} can make 24", str);
        } else {
            println!("{} cannot make 24", str);
        }
    }
}
