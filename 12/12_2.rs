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
    
    let mut coords= coords_orig;

    let velos_orig = [
        [0,0,0],
        [0,0,0],
        [0,0,0],
        [0,0,0],
    ];
    
    let mut velos = velos_orig;
    
    //print_matrix("coords", &coords);
    //print_matrix("velos", &velos);
    let mut coords_counts = [0,0,0];
    let mut velos_counts = [0,0,0];
    
    let mut ind = 0;
    loop {
        if ind > 10000 { break; } 
        //if ind > 0 && velos == velos_orig && coords == coords_orig {break;}
        
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
        
        //print_matrix("coords", &coords);
        //print_matrix("velos", &velos);
        
        for l in 0..4 {
            for i in 0..3 {
                coords[l][i] += velos[l][i];
            }
        }
        
        for i in 0..3 {
            let mut num_vels = 0;
            let mut num_coords = 0;
            for l in 0..4 {
                if velos[l][i] == velos_orig[l][i] {
                    num_vels += 1;
                }
                if coords[l][i] == coords_orig[l][i] {
                    num_coords += 1;
                }
            }
            if num_vels == 4 && velos_counts[i] == 0 {
                velos_counts[i] = ind;
            }
            if num_coords == 4 && coords_counts[i] == 0 {
                coords_counts[i] = ind;
            }
        }
        
        ind += 1;
    }
    
    println!("velos_counts: {} {} {}", velos_counts[0], velos_counts[1], velos_counts[2]);
    println!("coords_counts: {} {} {}", coords_counts[0], coords_counts[1], coords_counts[2]);
    
    println!("result: {}", ind);
}
