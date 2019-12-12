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
    let mut coords = [
        [-3, 10, -1],
        [-12, -10, -5],
        [-9, 0, 10],
        [7, -5, -3]
    ];
    
    /*let mut coords = [
        [-1, 0, 2],
        [2, -10, -7],
        [4, -8, 8],
        [3, 5, -1]
    ];*/

    let mut velos = [
        [0,0,0],
        [0,0,0],
        [0,0,0],
        [0,0,0],
    ];
    
    print_matrix("coords", &coords);
    print_matrix("velos", &velos);
    
    for _i in 0..1000 {
        for l in 0..4 {
            for r in l..4 {
                if l == r || coords[l] == coords[r] { continue; }
                for i in 0..3 {
                    if coords[l][i] == coords[r][i] { continue; }
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
        
        print_matrix("coords", &coords);
        print_matrix("velos", &velos);
        
        for l in 0..4 {
            for i in 0..3 {
                coords[l][i] += velos[l][i];
            }
        }
    }
    
    print_matrix("coords", &coords);
    print_matrix("velos", &velos);
    
    let mut result = 0;
    for l in 0..4 {
        let mut pot = 0;
        let mut kin = 0;
        for i in 0..3 {
            pot += num::abs(coords[l][i]);
            kin += num::abs(velos[l][i]);
        }
        result += pot * kin;
    }
    
    println!("result: {}", result);
}
