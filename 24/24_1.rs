use std::collections::HashSet;

fn main() {
    let input = "\
#.###
.....
#..#.
##.##
..#.#";

let _input = "\
....#
#..#.
#..##
..#..
#....";

    let mut visited = HashSet::new();
    let mut current = to_biodev(input);
    visited.insert(current);
    print_field(current);
    
    //print_field(gen(current, 0, 3));

    loop {
        current = next_gen(current);
        if visited.contains(&current) {
            println!("Result: {}", current);
            return; 
        }
        visited.insert(current);
        print_field(current);
    }
}

fn print_field(f: i32) {
    print!("\n----------");
    for y in 0..5 {
        print!("\n");
        for x in 0..5 {
            let i = y * 5 + x;
            let is_occupied = (f & 1 << i) != 0;
            print!("{}", if is_occupied { "# " } else { ". " });
        }
    }
}

fn next_gen(current: i32) -> i32 {
    let mut result = 0;
    for x in 0..5 {
        for y in 0..5 {
           result += gen(current, x, y); 
        }
    }
    return result;
}

fn gen(current: i32, x: i32, y: i32) -> i32 {
    let flood = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    
    let mut n = 0;
    for f in flood {
        let i = x + f.0;
        if i < 0 || i >= 5 { continue; }
        let j = y + f.1;
        if j < 0 || j >= 5 { continue; }
        let index = j * 5 + i;
        if (current & 1 << index) != 0 {
            n += 1;
        }
    }
    
    let cur_index = y * 5 + x;
    let is_occupied = current & (1 << cur_index) != 0;
    
    if is_occupied {
        return if n == 1 { 1 << cur_index } else { 0 };
    } else {
        return if n == 1 || n == 2 { 1 << cur_index } else { 0 };
    }
}

fn to_biodev(input: &str) -> i32 {
    let mut result = 0;
    let mut index = 0;
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        if c == '#' {
            result += 1 << index;
        }
        index += 1;    
    }
    return result;
}
