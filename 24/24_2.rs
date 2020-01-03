fn main() {
    let _input = "\
#.###
.....
#..#.
##.##
..#.#";

let input = "\
....#
#..#.
#.?##
..#..
#....";

    let mut generation = vec![to_biodev(input)];

    //for _i in 0..200 {
    for _i in 0..1 {
        let mut next_generation = Vec::new();

        for j in -1 as i32..generation.len() as i32 + 1 {
            next_generation.push(next_gen(&generation, j));
        }
        generation = next_generation;
    }
    
    let mut result = 0;
    for g in generation {
        print_field(g);
        for i in 0..32 {
            if g & 1 << i != 0 {
                result += 1;
            }
        }
    }
    println!("result: {}", result);
}

fn next_gen(current: &Vec<i32>, i: i32) -> i32 {
    let mut result = 0;
    for x in 0..5 {
        for y in 0..5 {
            if x == 2 && y == 2 { continue; }
            result += gen(&current, i, x, y);
        }
    }
    return result;
}

fn gen(current: &Vec<i32>, layer: i32, x: i32, y: i32) -> i32 {
    let flood = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut n = 0;
    for f in flood {
        let mut l = layer;

        let mut i = x + f.0;
        let mut j = y + f.1;

        let l2 = layer as usize + 1;
        if i == 2 && j == 2 && current.len() > l2 {
            // count inside neighbours
            for k in 0..5 {
                let index;
                if x == 2 {
                    index = k * 5 + 0;
                } else if x == 4 {
                    index = k * 5 + 4;
                } else if y == 2 {
                    index = 0 * 5 + k;
                } else /*if y == 4 */ {
                    index = 4 * 5 + k;
                }
                if (current[l2] & 1 << index) != 0 {
                    n += 1;
                }
            }
        } else {
            if i < 0 || i >= 5 || j < 0 || j >= 5 {
                l -= 1;
            }

            if l < 0 || l >= current.len() as i32 {
                continue;
            }
            if i < 0 {
                i = 1;
            } else if i >= 5 {
                i = 3;
            }
            if j < 0 {
                j = 1;
            } else if j >= 5 {
                j = 3;
            }

            let index = j * 5 + i;
            //println!("gen {} {} {}; {} {}", layer, x, y, i, j);
            if (current[l as usize] & 1 << index) != 0 {
                n += 1;
            }
        }
    }

    let cur_index = y * 5 + x;
    let is_occupied;
    if layer < 0 || layer >= current.len() as i32 {
        is_occupied = false;
    } else {
        is_occupied = current[layer as usize] & (1 << cur_index) != 0;
    }

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
