/*
<x=-3, y=10, z=-1>
<x=-12, y=-10, z=-5>
<x=-9, y=0, z=10>
<x=7, y=-5, z=-3>*/

fn print_matrix(name: &str, matrix: &[[i32; 3]; 4]) {
    println!("{}", name);
    for l in 0..4 {
        for i in 0..3 {
            print!("{},", matrix[l][i]);
        }
        print!("\n");
    }
    print!("\n");
}

fn main() {
    let coords_orig = [
        [-3, 10, -1],
        [-12, -10, -5],
        [-9, 0, 10],
        [7, -5, -3]
    ];
    //let coords_orig = [[-8, -10, 0], [5, 5, 10], [2, -7, 3], [9, -8, -3]];

    let mut coords = coords_orig;

    let mut velos = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let mut coords_counts = [0, 0, 0];

    let mut ind = 0;
    loop {
        if ind > 1000000 {
            break;
        }
        //if ind > 0 && velos == velos_orig && coords == coords_orig {break;}

        for l in 0..4 {
            for r in l..4 {
                if l == r || coords[l] == coords[r] {
                    continue;
                }
                for i in 0..3 {
                    if coords[l][i] == coords[r][i] {
                        continue;
                    }
                    if coords[l][i] < coords[r][i] {
                        velos[l][i] += 1;
                        velos[r][i] -= 1;
                    } else {
                        velos[l][i] -= 1;
                        velos[r][i] += 1;
                    }
                }
            }
        }

        for l in 0..4 {
            for i in 0..3 {
                coords[l][i] += velos[l][i];
            }
        }

        for i in 0..3 {
            let mut num_coords = 0;
            for l in 0..4 {
                if coords[l][i] == coords_orig[l][i] && velos[l][i] == 0 {
                    num_coords += 1;
                }
            }

            if num_coords == 4 && coords_counts[i] == 0 {
                coords_counts[i] = ind;
            }
        }
        ind += 1;
    }

    println!(
        "coords_counts: {} {} {}",
        coords_counts[0], coords_counts[1], coords_counts[2]
    );

    use num::integer::lcm;

    let mut result = lcm(coords_counts[0] as u64 + 1, coords_counts[1] as u64 +1);
    result = lcm(result, coords_counts[2] as u64 + 1);
    println!("result: {}", result);
}
