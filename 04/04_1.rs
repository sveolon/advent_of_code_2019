fn main() {
    let from = 206938;
    let to = 679128;
    let mut count = 0;

    for n in from..to {
        let s = n.to_string().as_bytes().to_vec();
        let mut skip = false;
        let mut num_rep = 0;
        for i in 1..6 {
            if s[i] < s[i - 1] {
                skip = true;
            }
            if s[i] == s[i - 1] {
                num_rep += 1;
            }
        }
        if !skip && num_rep >= 1 {
            count += 1;
            print!("{} ", n);
        }
    }
    println!("\n{}", count);
}
