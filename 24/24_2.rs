fn main() {
    let input = "\
#.###
.....
#..#.
##.##
..#.#";

    let mut generation = vec![to_biodev(input)];

    for _i in 0..200 {
        let mut next_generation = Vec::new();

        for j in -1 as i32..generation.len() as i32 + 1 {
            next_generation.push(next_gen(&generation, j));
        }
        generation = next_generation;
    }
}

fn next_gen(current: &Vec<i32>, i: i32) -> i32 {
    let mut result = 0;
    for x in 0..5 {
        for y in 0..5 {
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
