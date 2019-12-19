use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn has_key (mask: i32, key: char) -> bool {
    let shift = (key as u8) - ('a' as u8);
    return mask & (1 << shift) != 0;
}

fn add_key (mask: &mut i32, key: char){
    let shift = (key as u8) - ('a' as u8);
    *mask |= 1 << shift;
}

fn main() {
    let _a = [
        "#################################################################################",
        "#...#p........#..k........#.K..f........#...#.........#...........#.....#.......#",
        "#.#V#.#######.#P#########.#.#########.###.#.#######.###.###.#####.#.###.#.#####Y#",
        "#.#.#.#.C...#...#....u..#...#.A.....#...#.#.........#...#...#...#.#...#...#.....#",
        "#.#.#.#.###.#########.#.#####.#####.#.#.#.#########.#.###.#####.#.###.#####.#####",
        "#.#...#...#.#.......#.#.#.#...#...#.#.#.#...#...#...#.#...#.....#.....#.#...#...#",
        "#.#####.#.#.###.###.#.#.#.#.###.#.#.#.#.###.#.#.#####.#.###.###########.#.###.###",
        "#...#.#.#.#...#.#.#...#.#.#i#...#.#.#.#.#...#.#.....#.#...#.........#.....#.....#",
        "###.#.#.#.###.#.#.#####.#.#.#.#.#.#.###F#.###.#####.#.###.#######.#.#.#####.###.#",
        "#...#..n#.#.#...#.....#...#.#.#.#.#.#a..#.....#.#...#...#.#.....#.#.#.....#...#.#",
        "#O#######.#.#####.#####.###.#.#.#.#.#.###.#####.#.#.###J#.#.###.#.#######.#####.#",
        "#...#.....#.....#.....#.#...#.#.#.#...#.#.......#.#...#.#.#.#.#.#.........#.....#",
        "###.#.#####.#.###.###.#.#.#####.#######.#.#######.#####.#.#.#.#.###.#######.###.#",
        "#.#.#.#...#.#.....#.#...#.#.....#.......#.#t....#.#.....#l#.#.#...#...#....x#.#.#",
        "#.#.#.###.#.#######.#####.###.###.###.#.###.###.#.#.#####.#.#.###.###.#.#####.#.#",
        "#...#...#.#...#.........#...#....q#...#.#...#...#.#...#.#.#...#...#.#...#...#...#",
        "#.#####.#.###.#.###########I#######.###.#.#####.#.###.#.#.###.#.###.#####.#.#.###",
        "#.......#...#.#.#......z....#...#...#...#.#...#.#.......#.#...#.#.........#.#.#.#",
        "#########.#.#.#.#.###########.#.#.###.###.#.#.#.#.#######.#.###.#.#########G#.#.#",
        "#.....#...#.#...#.#...........#.#.#.#...#.#.#.#.#...#.....#.#...#.....#...#.#...#",
        "#W#.###.###.###.#.#####.#.#####.#.#.###.#.#.#.#.###.#.#####.#.#######.#.###.###.#",
        "#.#...#...#.....#...Z.#.#...#.#...#.#...#...#.#...#.#.....#.#.#.......#.#...#...#",
        "#.###.###.#############.###.#.#####.#.#######.###.#.#####.#.###.#.#####.#.###.###",
        "#...#...#.#.....#.....#...#.#.......#.#.#...#.#.#.#.#.....#...#.#.#...#.....#...#",
        "#.#####.#.#.###.###.#.###.#.#######.#.#.#.#.#.#.#.#.#.###.###.###.#.#.#.#######.#",
        "#.#.....#...#...#...#.#...#...#...#...#.#.#.#.#.#.#.#.#.#.#.#...#.#.#...#.......#",
        "###.###.#####.###.###.#####.#.#.#.#.###.#.#.#.#.#.###.#.#.#.###.#.###.###.#######",
        "#...#.....#.#.#...#.#.#...#.#.#.#.#.....#.#...#.#.....#...#...#.#...#...#.#.....#",
        "#.#######.#.#.###.#.#.#.#.###.#.#.#####.#.#####.#######.#####.#.###.#####.#.###.#",
        "#.#.....#.#.#.....#.#.#.#...#...#...#...#...#.....#...........#...#.........#...#",
        "#.#.###.#.#.#######.#.#.###.#.#####.#.###.#.#.#.###.#########.###.#####.#######.#",
        "#r#...#.#...#.......#.#...#.#.#.....#.#.#.#.#.#.........#...#...#.....#...#...#.#",
        "#.###.#.#####.#.#####.#.#.#.###.#####.#.#.#.###########.#.#.#.#######.#####.#.###",
        "#.....#.#...#.#.#.....#.#.#.....#.....#.#.#.......#...#.#.#.#.#.....#.#.....#...#",
        "#.#####.#.#.#.#.#.#######.#######.#####.#.#######.#.#.###.#.###.###.#.#.#######.#",
        "#.#.#...#.#.#.#.#.#.....#.....#.#.#.....#.#.....#...#.....#...#...#.#...#.......#",
        "#.#.#.###.#.#.#.#.###.#.#.###.#.#.###.#.#.#.###.#############.###.#.#####.#######",
        "#.#.#...#.#...#.#.#...#.#...#.#...#...#.#.#.#...#........m..#...#.#.....#.....#.#",
        "#.#.###.#.#####.#.#.###.#####.#.###.###.#.#.#.###.#########.#.###.#####.#####.#.#",
        "#.....#.......#.....#.........#.....#.......#.....#.........#.........#.........#",
        "#######################################.@.#######################################",
        "#.#...M.........#.H...............#.........#h....#...#.....#.#..............b..#",
        "#.#.###########.#####.###########.#.###.###.#.#.#.#.#.###.#.#.#.###############.#",
        "#.#...........#.......#.........#...#...#...#.#.#...#.#...#.#.......#.D.#.....#.#",
        "#.###########.###.#####.#####.#######.###.###.#.#####.#.###.#########.#.#.###.#.#",
        "#.....#.....#...#.#...#.#...#.....#...#.#.....#.#...#.#.#...#.........#...#.#.#.#",
        "#.###.#.###.###.###.#.#.###.#####.#.###.#######.#.#.#.#.#.###.#############.#.#.#",
        "#...#.#.#.#...#.....#.#.........#...#...#.....#.#.#.#...#.#...#.............#.#.#",
        "#.###.#.#.#.#########.#########.#####.###.###.#.###.#####T#.#.###.#####.#####.#.#",
        "#.#...#...#.....#...#.#.......#...#.....#.#.....#...#...#.#.#...#.....#.......#.#",
        "#.#.###.#####.###.#.#.#.#####.###.#####.#.#######.#.#.###.#.###.#####.#########.#",
        "#.#g....#.....#...#...#...#..d..#.......#...#.#...#.#...#.#...#...#.#.......#...#",
        "#######.#.###.#.#####.#####.###########.###.#.#.#.#.###.#.#######.#.#######.###.#",
        "#.....#.#...#.#.....#.#...#.......#...#.#...#...#.#.....#.....#...#.......#.#...#",
        "#.###.#####.#######.###.#.#.###.###.#.#.#.#######.#.#########.#.#####.#####.#.#.#",
        "#.#.E.....#...#...#.....#.#...#.....#.#.#.#.......#.#.......#...#.....#.....#.#.#",
        "#.#.#####.###.#.#.#######.###.#######.#.#.#.#######.#.#.###.#####.###.#.#####.#.#",
        "#.#.#...#.....#.#.........#...#.....#.#.#.#...#.....#.#...#.#.....#w..#...#..s#.#",
        "#.###.#.#####.#.#.#########.###.###.#.#.#.###.#######.###.###.###.###.###.###.#.#",
        "#.#...#.....#.#.#.#...#.#...#...#...#...#.#...#.......#.#.......#...#...#.....#.#",
        "#.#.#######.#.#.###.#.#.#.###.###.#.#####.#.###.#######.#######.###.#.#########.#",
        "#.#.#.....#.#.#.....#.#...#.#.#...#.....#.#...............#...#...#.#.#.........#",
        "#.#.#.###.#.#.#########.###.#.#.#######.#.###############.#.#####.#.###.#########",
        "#.#.#.#...#.#...#...#...#.....#...#.....#.#...........#...#.#.....#.#...#.......#",
        "#.#.###.#.#.#####.#.#.#########.#.#######.#.#########.#####.#######.#.###.#####.#",
        "#.#.#...#.#.....L.#.#.#.......#.#.......#...#.....#.#...#...#.....#.#.#.......#.#",
        "#.#.#.###.#########.#.#.#####.#.#######.#.###.#.#.#.###.#.###.###.#.#.#######.#.#",
        "#.#.#.#.#.#.......#...#...#...#.#.......#.#.#.#.#.....#.#.#...#...#.#.....#...#.#",
        "#.#.#.#.#.#.###.#.#######.#.#####.#######.#.#.#.#######.#.###.#.###.#####.#.###.#",
        "#.#...#.#.#.#...#.#.......#.#...#....c..#.#...#.#.....#.#.....#.#.......#.#.R.#.#",
        "#.#####.#.###S###.#.#.#####.#.#.#######.#.#####.#.###.#.#######.#.###.###.#####.#",
        "#.......#.....#...#.#.#...#...#.....#...#.....#.#.#...#...#.....#...#...#...#e..#",
        "#.#####.#######B#####.#.###########.#.#######.#.#.#.###.###.#######.###.###.#.#.#",
        "#.....#.........#.....#.....#.....#.#...#...#...#.#...#...#.....#...#.....#.#.#.#",
        "#####.###########.#####.###.#.###.#.###.###.###.#.###.###.#####.#####.###.#.#.#.#",
        "#...#.....#.......#.#.....#.#...#.#...#.#.....#.#.#.......#...#...#...#...#.#.#.#",
        "###.#####.#.#######.#.#####N###.#.###.#.#.#####.#.#######.#.#.###.#.#######.#.#.#",
        "#...#.....#...#..o....#.#...#...#...#...#.#.....#.......#.#j#...#y#.......#.#.#.#",
        "#.###.#######.#.#######.#.###.###.#####.#.#.###########.###.#.###.#.#####.#.#X#.#",
        "#.............#........v#.......#.U.....#.Q...........#.....#.....#.....#.....#.#",
        "#################################################################################",
    ];

    let _a1 = ["#########", "#b.A.@.a#", "#########"];

    let _a2 = [
        "########################",
        "#f.D.E.e.C.b.A.@.a.B.c.#",
        "######################.#",
        "#d.....................#",
        "########################",
    ];

    let a = [
        "#################",
        "#i.G..c...e..H.p#",
        "########.########",
        "#j.A..b...f..D.o#",
        "########@########",
        "#k.E..a...g..B.n#",
        "########.########",
        "#l.F..d...h..C.m#",
        "#################",
    ];

    let _a4 = [
        "########################",
        "#@..............ac.GI.b#",
        "###d#e#f################",
        "###A#B#C################",
        "###g#h#i################",
        "########################",
    ];

    let max_y: usize = a.len();
    let max_x: usize = a[0].len();

    let mut passages = HashSet::new();
    let mut doors = HashMap::new();
    let mut doors_inv = HashMap::new();
    let mut keys = HashMap::new();
    let mut keys_inv = HashMap::new();
    let mut curr = (0, 0);

    for y in 0..max_y {
        for x in 0..max_x {
            let c = a[y].chars().nth(x).unwrap();

            if c == '#' {
                continue;
            }
            passages.insert((x, y));
            if c == '@' {
                curr = (x, y);
            } else if 'a' <= c && c <= 'z' {
                keys.insert(c, (x, y));
                keys_inv.insert((x, y), c);
            } else if 'A' <= c && c <= 'Z' {
                let key = (c as u8 - 'A' as u8 + 'a' as u8) as char;
                doors.insert(key, (x, y));
                doors_inv.insert((x, y), key);
            }
        }
    }

    {
        print!("\n\t");
        for x in 0..max_x {
            print!("{}", x / 10);
        }
        print!("\n\t");
        for x in 0..max_x {
            print!("{}", x % 10);
        }
        for y in 0..max_y {
            print!("\n{}\t", y);
            for x in 0..max_x {
                if doors_inv.contains_key(&(x, y)) {
                    print!("{}", doors_inv[&(x, y)]);
                } else if keys_inv.contains_key(&(x, y)) {
                    print!("{}", keys_inv[&(x, y)]);
                } else if curr.0 == x && curr.1 == y {
                    print!("@");
                } else if passages.contains(&(x, y)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
        }
        print!("\n\n");
    }

    let mut all_keys = 0;
    for k in &keys {
        add_key(&mut all_keys, *k.0);
    }
    println!("all keys: {}", all_keys);
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_front((curr, 0, 0));
    
    while queue.len() > 0 {
    
        let mut s = queue.pop_back().unwrap();
        if visited.contains(&(s.0, s.2)) {
            continue;
        }
        visited.insert((s.0, s.2));
        
        if keys_inv.contains_key(&s.0) { // it's a key
            add_key(& mut s.2, keys_inv[&s.0]);
        } else if doors_inv.contains_key(&s.0) { // it's a door
            let key = doors_inv[&s.0];
            if !has_key(s.2, key) {             // closed door
                continue;
            }
        }

        if s.2 == all_keys {
            println!("result: {}", s.1);
            return;
        }

        let flood: Vec<(i32,i32)> = vec![(0,1),(0,-1),(1,0),(-1,0)];
        
        for f in &flood {
            let x = ((s.0).0 as i32 + f.0) as usize;
            let y = ((s.0).1 as i32 + f.1) as usize;
            queue.push_front(((x,y), s.1 + 1, s.2));
        }
    }
    
    for v in visited {
        println!("v {} {} {}", (v.0).0, (v.0).1, v.1);
    }
}
