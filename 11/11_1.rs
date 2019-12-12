use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Rot {
    CW,
    CCW,
}

fn get_rot(num: i64) -> Rot {
    return if num == 0 { Rot::CCW } else { Rot::CW };
}

fn rotate(curr: Dir, rot: Rot) -> Dir {
    return if rot == Rot::CW {
        match curr {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            _ => Dir::Up,
        }
    } else {
        match curr {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            _ => Dir::Up,
        }
    };
}

fn to_vec(dir: Dir) -> (i64, i64) {
    return match dir {
        Dir::Up => (0, 1),
        Dir::Down => (0, -1),
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
    };
}

fn zeros(size: usize) -> Vec<i64> {
    let mut zero_vec: Vec<i64> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}

fn int_comp(arr: Vec<i64>) -> HashMap<(i64,i64),i64> {
    let mut a = arr;
    let mut i = 0;
    let mut output_num = 0;
    let mut painted = HashMap::new();
    let mut curr = (0, 0);
    painted.insert(curr, 1);
    let mut dir = Dir::Up;

    let mut rb: i64 = 0;
    loop {
        let op = a[i] % 100;

        if op == 99 {
            break;
        }

        let ind1 = match a[i] % 1000 / 100 {
            1 => i + 1 as usize,
            0 => a[i + 1] as usize,
            2 => (rb + a[i + 1]) as usize,
            _ => std::i64::MAX as usize,
        };

        let ind2 = match a[i] % 10000 / 1000 {
            1 => i + 2 as usize,
            0 => a[i + 2] as usize,
            2 => (rb + a[i + 2]) as usize,
            _ => std::i64::MAX as usize,
        };

        let ind3 = match a[i] % 100000 / 10000 {
            1 => i + 3 as usize,
            0 => a[i + 3] as usize,
            2 => (rb + a[i + 3]) as usize,
            _ => std::i64::MAX as usize,
        };

        if op == 3 {
            a[ind1] = if painted.contains_key(&curr) {
                painted[&curr]
            } else {
                0
            };
            i += 2;
            continue;
        }

        let a1 = a[ind1];
        let a2 = a[ind2];

        if op == 4 {
            let output = a1;
            //print!("{},", output);

            if output_num % 2 == 0 {
                painted.insert(curr, output);
            } else {
                dir = rotate(dir, get_rot(output));
                let shift = to_vec(dir);
                curr.0 += shift.0;
                curr.1 += shift.1;
            }

            i += 2;
            output_num += 1;
            continue;
        }

        if op == 9 {
            rb += a1;
            i += 2;
            continue;
        }

        if op == 1 {
            a[ind3] = a1 + a2;
            i += 4;
        }
        if op == 2 {
            a[ind3] = a1 * a2;
            i += 4;
        }
        if op == 5 {
            if a1 > 0 {
                i = a2 as usize;
            } else {
                i += 3;
            }
        }
        if op == 6 {
            if a1 == 0 {
                i = a2 as usize;
            } else {
                i += 3;
            }
        }
        if op == 7 {
            a[ind3] = if a1 < a2 { 1 } else { 0 };
            i += 4;
        }
        if op == 8 {
            a[ind3] = if a1 == a2 { 1 } else { 0 };
            i += 4;
        }
    }
    return painted;
}

fn main() {
    let a = [3,8,1005,8,326,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,29,2,1003,17,10,1006,0,22,2,106,5,10,1006,0,87,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,65,2,7,20,10,2,9,17,10,2,6,16,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,99,1006,0,69,1006,0,40,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,127,1006,0,51,2,102,17,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,155,1006,0,42,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,101,0,8,180,1,106,4,10,2,1103,0,10,1006,0,14,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,213,1,1009,0,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,239,1006,0,5,2,108,5,10,2,1104,7,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,272,2,1104,12,10,1,1109,10,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,302,1006,0,35,101,1,9,9,1007,9,1095,10,1005,10,15,99,109,648,104,0,104,1,21102,937268449940,1,1,21102,1,343,0,1105,1,447,21101,387365315480,0,1,21102,1,354,0,1105,1,447,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,29220891795,1,21102,1,401,0,1106,0,447,21101,0,248075283623,1,21102,412,1,0,1105,1,447,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,984353760012,1,21102,1,435,0,1105,1,447,21102,1,718078227200,1,21102,1,446,0,1105,1,447,99,109,2,21202,-1,1,1,21102,40,1,2,21101,0,478,3,21101,468,0,0,1106,0,511,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,473,474,489,4,0,1001,473,1,473,108,4,473,10,1006,10,505,1102,1,0,473,109,-2,2105,1,0,0,109,4,1202,-1,1,510,1207,-3,0,10,1006,10,528,21102,1,0,-3,22102,1,-3,1,22101,0,-2,2,21101,0,1,3,21102,1,547,0,1105,1,552,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,575,2207,-4,-2,10,1006,10,575,21202,-4,1,-4,1105,1,643,21202,-4,1,1,21201,-3,-1,2,21202,-2,2,3,21102,1,594,0,1106,0,552,22102,1,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,613,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,635,22101,0,-1,1,21101,0,635,0,106,0,510,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0];
    let mut arr = zeros(a.len() * 100);
    for i in 0..a.len() {
        arr[i] = a[i];
    }
    let result = int_comp(arr);
    println!("res {}", result.len());
    
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (k,_v) in result.iter() {
        min_x = std::cmp::min(min_x, k.0);
        max_x = std::cmp::max(max_x, k.0);
        min_y = std::cmp::min(min_y, k.1);
        max_y = std::cmp::max(max_y, k.1);
    }
    
    let width: usize = (max_x - min_x + 1) as usize;
    let height: usize = (max_y - min_y + 1) as usize;
    println!("{} by {}", width, height);
    
    // Base 1d array
    let mut grid_raw = vec![' '; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();
    
    for (k,v) in result.iter() {
        if *v == 0 { continue; }
        let x = (k.0 - min_x) as usize;
        let y = (k.1 - min_y) as usize;
        grid[y][x] = '*';
    }
    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[height - y -1][x]);
        }
        print!("\n");
    }
}
