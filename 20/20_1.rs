use std::collections::HashMap;
use std::collections::HashSet;

fn insert_portal(
    portals: &mut HashMap<(usize, usize), String>,
    portals_rev: &mut HashMap<String, (usize, usize)>,
    portals_pairs: &mut HashMap<(usize, usize), (usize, usize)>,
    s: String,
    c: (usize, usize),
) {
    portals.insert(c, s.clone());
    if !portals_rev.contains_key(&s) {
        portals_rev.insert(s.clone(), c);
    } else {
        portals_pairs.insert(c, portals_rev[&s]);
        portals_pairs.insert(portals_rev[&s], c);
    }
}

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
        if line.len() == 0 {
            continue;
        }
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
                passages.insert((x, y));
            }
        }
    }
    let mut portals = HashMap::new();
    let mut portals_rev = HashMap::new();
    let mut portals_pairs = HashMap::new();

    for y in 0..max_y {
        for x in 0..max_x {
            if 'A' <= a[y][x] && a[y][x] <= 'Z' {
                if x + 1 < max_x && 'A' <= a[y][x + 1] && a[y][x + 1] <= 'Z' {
                    let s = format!("{}{}", a[y][x], a[y][x + 1]);

                    if x + 2 < max_x && a[y][x + 2] == '.' {
                        insert_portal(
                            &mut portals,
                            &mut portals_rev,
                            &mut portals_pairs,
                            s,
                            (x + 2, y),
                        );
                    } else if x > 0 && a[y][x - 1] == '.' {
                        insert_portal(
                            &mut portals,
                            &mut portals_rev,
                            &mut portals_pairs,
                            s,
                            (x - 1, y),
                        );
                    }
                } else if y + 1 < max_y && 'A' <= a[y + 1][x] && a[y + 1][x] <= 'Z' {
                    let s = format!("{}{}", a[y][x], a[y + 1][x]);
                    if y + 2 < max_y && a[y + 2][x] == '.' {
                        insert_portal(
                            &mut portals,
                            &mut portals_rev,
                            &mut portals_pairs,
                            s,
                            (x, y + 2),
                        );
                    } else if y > 0 && a[y - 1][x] == '.' {
                        insert_portal(
                            &mut portals,
                            &mut portals_rev,
                            &mut portals_pairs,
                            s,
                            (x, y - 1),
                        );
                    }
                }
            }
        }
    }
    let begin = portals_rev["AA"];
    let end = portals_rev["ZZ"];
}
