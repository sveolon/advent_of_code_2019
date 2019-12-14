use std::collections::HashMap;

fn parse_pair(pair: &str) -> (u32, &str) {
    let split = pair.split(" ").collect::<Vec<&str>>();
    let outcount = split[0].parse::<u32>().unwrap();
    return (outcount, split[1].into());
}

fn in_and_out(all: &str) -> (&str, &str) {
    let vec = all.split(" => ").collect::<Vec<&str>>();
    return (vec[0], vec[1]);
}

fn parse_ins(inputs: &str) -> Vec<(u32, &str)> {
    let mut res = Vec::new();
    let in_split = inputs.split(", ").collect::<Vec<&str>>();
    for ins in &in_split {
        res.push(parse_pair(ins));
    }
    return res;
}

fn parse_line(line: &str) -> (&str, (u32, std::vec::Vec<(u32, &str)>)) {
    let vec = in_and_out(line);
    let outsplit = parse_pair(vec.1);
    let inputs = parse_ins(vec.0);
    return (outsplit.1, (outsplit.0, inputs));
}

fn calculate(
    material: &str,
    count: u32,
    recepies: &HashMap<&str, (u32, std::vec::Vec<(u32, &str)>)>,
) -> u32 {

    println!("calculate {} {}", count, material);
    
    if material == "ORE" {
        return count;
    }
    
    let mut result = 0;
    for comp in &recepies[material].1 {
        result += calculate(comp.1, comp.0, recepies);
    }

    let result = (count as f64 /recepies[material].0 as f64).ceil() as u32 * result;
    println!("calculate {} {} => {}", count, material, result);
    return result;
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
    /*let ss = [
        "10 ORE => 10 A",
        "10 ORE => 3 B",
        "7 A, 1 B => 1 C",
        "7 A, 1 C => 1 D",
        "7 A, 1 D => 1 E",
        "3 B => 1 FUEL",
    ];*/
    let mut recepies = HashMap::new();
    for s in &ss {
        let res = parse_line(s);
        recepies.insert(res.0, res.1);
    }
    
    let result = calculate("FUEL", 1, &recepies);
    println!("result {}", result);
}
