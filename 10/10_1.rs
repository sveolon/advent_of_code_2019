fn main() {
    let a = [
    ".#..#",
    ".....",
    "#####",
    "....#",
    "...##"];
    
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
    
    for curr in &asteroids {
        for other in &asteroids {
            if curr == other { continue; }
            let x = other.0 - curr.0;
            let y = other.1 - curr.1;
        }
    }
}
