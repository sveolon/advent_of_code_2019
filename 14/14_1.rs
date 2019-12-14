use std::collections::HashMap;

fn parse_pair(pair: &str) -> (i32, &str) {
    let split = pair.split(" ").collect::<Vec<&str>>();
    let outcount = split[0].parse::<i32>().unwrap();
    return (outcount, split[1].into());
}

fn in_and_out(all: &str) -> (&str, &str) {
    let vec = all.split(" => ").collect::<Vec<&str>>();
    return (vec[0], vec[1]);
}

fn parse_ins(inputs: &str) -> Vec<(i32, &str)> {
    let mut res = Vec::new();
    let in_split = inputs.split(", ").collect::<Vec<&str>>();
    for ins in &in_split {
        res.push(parse_pair(ins));
    }
    return res;
}

fn parse_line(line: &str) -> (&str, (i32, std::vec::Vec<(i32, &str)>)) {
    let vec = in_and_out(line);
    let outsplit = parse_pair(vec.1);
    let inputs = parse_ins(vec.0);
    return (outsplit.1, (outsplit.0, inputs));
}

fn main() {
    let ss = [
        "10 ORE => 10 A",
        "1 ORE => 1 B",
        "7 A, 1 B => 1 C",
        "7 A, 1 C => 1 D",
        "7 A, 1 D => 1 E",
        "7 A, 1 E => 1 FUEL",
    ];
    let mut recepies = HashMap::new();
    for s in &ss {
        let res = parse_line(s);
        recepies.insert(res.0, res.1);
    }
    println!("# {}", recepies.len());
}
