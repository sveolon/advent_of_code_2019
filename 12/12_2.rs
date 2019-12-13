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
    
    let mut ind = 0;
    loop {
        if ind > 0 && velos == velos_orig && coords == coords_orig {break;}
        
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
        ind += 1;
    }
    
    
    println!("result: {}", ind);
}
