use std::collections::VecDeque;

#[derive(PartialOrd, PartialEq, Clone, Copy)]
enum Type {
    DealInc = 0,
    Cut = 1,
    DealNew = 2,
}

#[derive(PartialOrd, PartialEq, Clone, Copy)]
struct Op {
    t: Type,
    a: i32,
}

const N_CARDS: i32 = 10007;

fn main() {/*
    let commands = [
        "cut -7812",
        "deal with increment 55",
        "cut -3909",
        "deal with increment 51",
        "deal into new stack",
        "deal with increment 4",
        "cut -77",
        "deal with increment 26",
        "deal into new stack",
        "deal with increment 36",
        "cut 5266",
        "deal with increment 20",
        "cut 8726",
        "deal with increment 22",
        "cut 4380",
        "deal into new stack",
        "cut 3342",
        "deal with increment 16",
        "cut -2237",
        "deal into new stack",
        "deal with increment 20",
        "cut 7066",
        "deal with increment 18",
        "cut 5979",
        "deal with increment 9",
        "cut 2219",
        "deal with increment 44",
        "cut 7341",
        "deal with increment 10",
        "cut -6719",
        "deal with increment 42",
        "deal into new stack",
        "cut -2135",
        "deal with increment 75",
        "cut 5967",
        "deal into new stack",
        "cut 6401",
        "deal with increment 39",
        "deal into new stack",
        "deal with increment 56",
        "cut 7735",
        "deal with increment 49",
        "cut -6350",
        "deal with increment 50",
        "deal into new stack",
        "deal with increment 72",
        "deal into new stack",
        "cut 776",
        "deal into new stack",
        "deal with increment 18",
        "cut 9619",
        "deal with increment 9",
        "deal into new stack",
        "cut 5343",
        "deal into new stack",
        "cut 9562",
        "deal with increment 65",
        "cut 4499",
        "deal with increment 58",
        "cut -4850",
        "deal into new stack",
        "cut -9417",
        "deal into new stack",
        "deal with increment 33",
        "cut 2763",
        "deal with increment 61",
        "cut 7377",
        "deal with increment 27",
        "cut 895",
        "deal into new stack",
        "deal with increment 41",
        "cut -1207",
        "deal with increment 22",
        "cut -7401",
        "deal with increment 48",
        "cut 5776",
        "deal with increment 3",
        "cut 2097",
        "deal with increment 49",
        "cut -8098",
        "deal with increment 68",
        "cut 2296",
        "deal with increment 35",
        "cut -4471",
        "deal with increment 56",
        "cut -2778",
        "deal with increment 5",
        "cut -6386",
        "deal with increment 54",
        "cut -7411",
        "deal with increment 20",
        "cut -4222",
        "deal into new stack",
        "cut -5236",
        "deal with increment 64",
        "cut -3581",
        "deal with increment 11",
        "cut 3255",
        "deal with increment 20",
        "cut -5914",
    ];

    let mut d1 = VecDeque::new();
    let mut d2 = VecDeque::new();
    for i in 0..N_CARDS {
        d1.push_back(i as u32);
        d2.push_back(i as u32);
    }
    
    let mut cmds = parse(&commands);
    for c in cmds.iter() {
        apply(c, &mut d1);
    }
    display(&cmds);
    sort(&mut cmds);
    display(&cmds);
    collapse(&mut cmds);
    display(&cmds);
    
    for c in cmds.iter() {
        apply(c, &mut d2);
    }
    
    println!("d1: {:?}", d1);
    println!("d2: {:?}", d2);
*/
    println!("Test test test");
    let mut d1 = VecDeque::new();
    let mut d2 = VecDeque::new();
    for i in 0..N_CARDS {
        d1.push_back(i as u32);
        d2.push_back(i as u32);
    }
    
    let c1 = [
        "cut -7812",
        "deal with increment 55",
        "cut -3909",
        "deal with increment 51",
        "deal into new stack",
        "deal with increment 4",
        "cut -77",
];
    let mut c2 = parse(&c1);
    for c in c2.iter() {
        apply(c, &mut d1);
    }
    display(&c2);
    sort(&mut c2);
    display(&c2);
    collapse(&mut c2);
    display(&c2);
    
    for c in c2.iter() {
        apply(c, &mut d2);
    }
    
    println!("d1: {:?}", d1);
    println!("d2: {:?}", d2);
}

fn apply(c: &Op, deck: &mut VecDeque<u32>) {
    match c.t {
        Type::DealInc => deal_inc(c.a, deck),
        Type::DealNew => deal_new(deck),
        _ => cut(c.a, deck),
    }
}

fn deal_new(deck: &mut VecDeque<u32>) {
    let mut tmp = VecDeque::new();
    while deck.len() > 0 {
        tmp.push_front(deck.pop_front().unwrap());
    }
    while tmp.len() > 0 {
        deck.push_front(tmp.pop_back().unwrap());
    }
}
fn deal_inc(arg: i32, deck: &mut VecDeque<u32>) {
    let mut tmp = VecDeque::new();
    for _i in 0..deck.len() {
        tmp.push_front(0);
    }
    for i in 0..deck.len() {
        let idx = i * arg as usize % tmp.len();
        tmp[idx] = deck.pop_front().unwrap();
    }
    while tmp.len() > 0 {
        deck.push_front(tmp.pop_back().unwrap());
    }
}
fn cut(arg: i32, deck: &mut VecDeque<u32>) {
    let ct = if arg > 0 {
        arg
    } else {
        deck.len() as i32 + arg
    };
    for _i in 0..ct {
        let tmp = deck.pop_front().unwrap();
        deck.push_back(tmp);
    }
}

fn to_str(op: &Op) -> String {
    let t = match op.t {
        Type::Cut => "Cut",
        Type::DealNew => "DealNew",
        _ => "DealInc",
    };
    return format!("'{}, {}'", t, &op.a);    
}

fn display(cmds: &Vec<Op>) {
    println!("display(len {}): ", cmds.len());
    for i in 0..cmds.len() {
        print!("{}; " , to_str(&cmds[i]));
    }
    print!("\n");
}

fn parse(input: &[&str]) -> Vec<Op> {
    let mut res = Vec::new();
    for c in input.iter() {
        if c.starts_with("deal with increment ") {
            let arg: i32 = c.split(" ").nth(3).unwrap().parse().unwrap();
            res.push(Op {
                t: Type::DealInc,
                a: arg,
            });
        } else if *c == "deal into new stack" {
            res.push(Op {
                t: Type::DealNew,
                a: 0,
            });
        } else if c.starts_with("cut ") {
            let arg: i32 = c.split(" ").nth(1).unwrap().parse().unwrap();
            res.push(Op {
                t: Type::Cut,
                a: arg,
            });
        }
    }
    return res;
}

fn collapse(a: &mut Vec<Op>) {
    if a.len() < 1 {
        return;
    }
    
    let mut result = Vec::new();
    let mut current = a[0];
    let mut i = 1;
    while i < a.len() {
        if current.t == Type::DealNew && a[i].t == Type::DealNew {
            i += 2;
            if i >= a.len() {
                *a = result;
                return;
            } else {
                current = a[i];
                i += 1;
            }
        } else if current.t == a[i].t {
            let m = merge(&current, &a[i]);
            current = m[0];
            i += 1;
        } else {
            result.push(current);
            current = a[i];
            i += 1;
        }
    }
    result.push(current);
    *a = result;
}

fn merge(first: &Op, second: &Op) -> Vec<Op> {
    if first.t == Type::DealNew && second.t == Type::DealNew {
        return vec![];
    } else if first.t == Type::Cut && second.t == Type::Cut {
        return vec![Op{t: Type::Cut, a: (first.a+second.a) % N_CARDS}];
    } if first.t == Type::DealInc && second.t == Type::DealInc {
        return vec![Op{t: Type::DealInc, a: (first.a*second.a) % N_CARDS}];
    } else {
        return vec![*first, *second];
    }
}

fn sort(a: &mut Vec<Op>) {
    let mut swapped = true;
    while swapped {
        //println!("\nEntering swap():");
        //display(a);
        let mut input = Vec::new();
        swapped = false;
        let mut i = 0;
        while i < a.len() - 1 {
            //println!("i: {}", i);
            let (sw, next) = swap(&a[i], &a[i + 1]);
            //println!("\nswap({}, {}) returned: ", to_str(&a[i]), to_str(&a[i+1]));
            //display(&next);
            
            swapped |= sw;
            
            if sw || i + 1 >= a.len() - 1 {
                for n in next {
                    input.push(n);
                }
                i += 2;
            } else {
                input.push(next[0]);
                i += 1;
            }
        }
        *a = input;
    }
}

fn swap(first: &Op, second: &Op) -> (bool, Vec<Op>) {
    if first.t == Type::DealNew && second.t == Type::Cut {
        return (
            true,
            vec![
                Op {
                    t: Type::Cut,
                    a: N_CARDS - second.a,
                },
                *first,
            ],
        );
    } else if first.t == Type::Cut && second.t == Type::DealInc {
        return (
            true,
            vec![
                *second,
                Op {
                    t: Type::Cut,
                    a: (first.a * second.a) % N_CARDS,
                },
            ],
        );
    } else if first.t == Type::DealNew && second.t == Type::DealInc {
        return (
            true,
            vec![
                *second,
                Op {
                    t: Type::Cut,
                    a: N_CARDS + 1 - second.a,
                },
                *first,
            ],
        );
    } else {
        return (false, vec![*first, *second]);
    }
}