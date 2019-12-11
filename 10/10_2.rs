use std::collections::HashSet;
use core::f64::consts::PI;

fn main() {

    let dbg = false;
    
    let a = [
        ".###..#######..####..##...#",
        "########.#.###...###.#....#",
        "###..#...#######...#..####.",
        ".##.#.....#....##.#.#.....#",
        "###.#######.###..##......#.",
        "#..###..###.##.#.#####....#",
        "#.##..###....#####...##.##.",
        "####.##..#...#####.#..###.#",
        "#..#....####.####.###.#.###",
        "#..#..#....###...#####..#..",
        "##...####.######....#.####.",
        "####.##...###.####..##....#",
        "#.#..#.###.#.##.####..#...#",
        "..##..##....#.#..##..#.#..#",
        "##.##.#..######.#..#..####.",
        "#.....#####.##........#####",
        "###.#.#######..#.#.##..#..#",
        "###...#..#.#..##.##..#####.",
        ".##.#..#...#####.###.##.##.",
        "...#.#.######.#####.#.####.",
        "#..##..###...###.#.#..#.#.#",
        ".#..#.#......#.###...###..#",
        "#.##.#.#..#.#......#..#..##",
        ".##.##.##.#...##.##.##.#..#",
        "#.###.#.#...##..#####.###.#",
        "#.####.#..#.#.##.######.#..",
        ".#.#####.##...#...#.##...#.",
    ];
    //let a = [".#....#####...#..","##...##.#####..##","##...#...#.#####.","..#.....#...###..","..#.#.....#....##"];
    //let a = [".#..#",".....","#####","....#","...##"];
    let y_l = a.len();
    let x_l = a[0].len();

    let mut asteroids = Vec::new();
    for y in 0..y_l {
        for x in 0..x_l {
            if a[y][x..].chars().next().unwrap() == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }

    let mut res = 0;

    let mut center = asteroids[0];
    for curr in &asteroids {
        let mut hs = HashSet::new();
        for other in &asteroids {
            if curr == other {
                continue;
            }
            let x = other.0 - curr.0;
            let y = other.1 - curr.1;
            let gcd = num::integer::gcd(x, y);
            hs.insert((x / gcd, y / gcd));
        }
        if hs.len() > res {
            res = hs.len();
            center = *curr;
        }
    }
    println!("num visible:{}; center: x {},y {}", res, center.0, center.1);

    let mut bearings = Vec::new();
    for other in &asteroids {
        if center == *other {
            continue;
        }
        
        let x = other.0 - center.0;
        let y = -(other.1 - center.1);
        
        let len_2 = (x * x + y * y) as f64;
        let mut angle = (y as f64 / x as f64).atan();
        
        if x == 0 {
            angle = if y > 0 { PI / 2.0 } else { 1.5 * PI };
        } else if y == 0 {
            angle = if x > 0 { 0.0 } else { PI };
        } else if y < 0 && x < 0 {
            angle += 2.0 * PI;
        } else if x < 0 {
            angle += PI;
        }
        
        // rotate the plain
        angle -= PI / 2.0;
        if angle <= 0.0 {
            angle += 2.0 * PI;
        }
        
        //invert the direction
        angle *= -1.0;
        
        bearings.push((angle, len_2, other.0, other.1, false));
    }
    
    bearings.sort_unstable_by(|a, b| {
        if num::abs(a.0 - b.0) > 1e-10 {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });

    if dbg {
    for i in 0..bearings.len() {
        println!(
            "{}) a:{}\tl:{}\tx:{}\ty:{}\tused:{}",
            i, bearings[i].0, bearings[i].1, bearings[i].2, bearings[i].3, bearings[i].4
        );
    } }
    
    let mut curr_angle = -100.0; // impossible 
    let mut ind = 0;
    let mut count = 0;
    let sz = bearings.len();
    while count < sz {
        while num::abs(bearings[ind%sz].0 - curr_angle) < std::f64::EPSILON || bearings[ind%sz].4 {
            ind += 1;
        }
        count += 1;
        curr_angle = bearings[ind%sz].0;
        bearings[ind%sz].4 = true;
        
        println!(
            "{}] a:{} l:{} x:{} y:{} used:{}",
            count, bearings[ind%sz].0, bearings[ind%sz].1, bearings[ind%sz].2, bearings[ind%sz].3, bearings[ind%sz].4
        );
    }
    print!("\n\n\n");
    
    let i = ind%sz;
    println!(
            "{}) a:{} l:{} x:{} y:{} used:{}",
            i, bearings[i].0, bearings[i].1, bearings[i].2, bearings[i].3, bearings[i].4
        );
        
    println!("res: {}", bearings[i].2 * 100 + bearings[i].3);
}
