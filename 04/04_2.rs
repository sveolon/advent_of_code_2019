fn main() {
    let from = 206938;
    let to = 679128;
    let mut count = 0;

    for n in from..to+1 {
        let s = n.to_string().as_bytes().to_vec();
        let mut skip = false;
        let mut num_rep = 0;
        let mut has_rep = false;

        for i in 1..6 {
            if s[i] < s[i - 1] {
                skip = true;
            } else if s[i] == s[i - 1] {
                num_rep += 1;
            } else if num_rep == 1 {
                has_rep = true;
                num_rep = 0;
            } else {
                num_rep = 0;
            }
        }
        
        if !skip && (has_rep || num_rep == 1) {
            count += 1;
            //print!("{} ", n);
        }
    }
    println!("\n{}", count);
}
