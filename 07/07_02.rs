//fn int_comp(a: &mut [i32; 29], input1: i32, input2: i32, halted: &mut bool) -> i32 {
fn int_comp(a: & mut [i32; 515], input1: i32, input2: i32, halted: &mut bool, i: &mut usize) -> i32 {
    let mut output = std::i32::MAX;
    let mut inp1_used = false;
    loop {
        let op = a[*i] % 100;

        if op == 99 {
            *halted = true;
            break;
        }

        if op == 3 {
            a[a[*i + 1] as usize] = if !inp1_used {input1} else {input2};
            inp1_used = true;
            *i += 2;
            continue;
        }

        let a1 = if a[*i] % 1000 / 100 == 0 {
            a[a[*i + 1] as usize]
        } else {
            a[*i + 1]
        };

        if op == 4 {
            output = a1;
            *i += 2;
            break;
        }

        let a2 = if a[*i] % 10000 / 1000 == 0 {
            a[a[*i + 2] as usize]
        } else {
            a[*i + 2]
        };

        if op == 1 {
            a[a[*i + 3] as usize] = a1 + a2;
            *i += 4;
        }
        if op == 2 {
            a[a[*i + 3] as usize] = a1 * a2;
            *i += 4;
        }
        if op == 5 {
            if a1 > 0 {
                *i = a2 as usize;
            } else {
                *i += 3;
            }
        }
        if op == 6 {
            if a1 == 0 {
                *i = a2 as usize;
            } else {
                *i += 3;
            }
        }
        if op == 7 {
            a[a[*i + 3] as usize] = if a1 < a2 { 1 } else { 0 };
            *i += 4;
        }
        if op == 8 {
            a[a[*i + 3] as usize] = if a1 == a2 { 1 } else { 0 };
            *i += 4;
        }
    }
    return output;
}

// Generating permutation using Heap Algorithm
fn heap_permutation(a: &mut [usize; 5], size: usize, output: &mut Vec<[usize; 5]>) {
    // if size becomes 1 then prints the obtained
    // permutation
    if size == 1 {
        output.push(*a);
        return;
    }

    heap_permutation(a, size - 1, output);

    for i in 0..size - 1 {
        // if size is odd, swap first and last
        // element
        if size % 2 == 1 {
            let tmp = a[0];
            a[0] = a[size - 1];
            a[size - 1] = tmp;
        }
        // If size is even, swap ith and last
        // element
        else {
            let tmp = a[i];
            a[i] = a[size - 1];
            a[size - 1] = tmp;
        }
        heap_permutation(a, size - 1, output);
    }
}

fn main() {
    //let a = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    let a = [
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 59, 84, 93, 110, 191, 272, 353, 434, 99999, 3,
        9, 101, 2, 9, 9, 102, 3, 9, 9, 1001, 9, 5, 9, 102, 4, 9, 9, 1001, 9, 4, 9, 4, 9, 99, 3, 9,
        101, 3, 9, 9, 102, 5, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 4, 9, 1002, 9, 2, 9, 101, 2, 9, 9,
        102, 2, 9, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9,
        1001, 9, 5, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2,
        9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 99,
    ];
    //let a = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

    let mut setting1 = [5,6,7,8,9];
    let mut all_permutations = Vec::new();
    heap_permutation(&mut setting1, 5, &mut all_permutations);
    let mut result = 0;

    for setting in all_permutations {
        let mut is = [0,0,0,0,0];
        let mut amps = [a, a, a, a, a];
        let mut output = 0;
        let mut halted = false;
        
        for i in 0..5 {
            output = int_comp(& mut amps[i], setting[i] as i32, output, &mut halted, &mut is[i]);
            if halted { break; }
        }
        loop {
            for i in 0..5 {
                output = int_comp(& mut amps[i], output, output, &mut halted, &mut is[i]);
                if halted { break; }
            }
            if halted { break; }
            result = std::cmp::max(result, output);
        }
    }

    println!("{}", result);
}
