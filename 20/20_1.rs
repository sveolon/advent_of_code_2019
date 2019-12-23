use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    let mut a = Vec::new();
    for line in input.lines() {
        if line.len() == 0 { continue; }
        let mut l = Vec::new();
        for c in line.as_bytes() {
            l.push(*c as char);
        }
        a.push(l);
    }
    
    let max_y = a.len();
    let max_x = a[0].len();
    
    for y in 0..max_y {
        print!("\n");
        for x in 0..max_x {
            print!("{}", a[y][x]);
        }
    }
    
    let mut passages = HashSet::new();
    for y in 0..max_y {
        for x in 0..max_x {
            if a[y][x] == '.' {
                passages.insert((x,y));
            }
        }
    }
}
