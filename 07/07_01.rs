fn int_comp(arr: [i32;17], input1: i32, input2: i32) -> i32 {
    let mut a = arr;
    let mut i = 0;
    let mut input1_used = false;
    let mut output = std::i32::MAX;
    loop {
        let op = a[i] % 100;

        if op == 99 {
            break;
        }

        if op == 3 {
            a[a[i + 1] as usize] = if !input1_used {input1} else {input2};
            input1_used = true;
            i += 2;
            continue;
        }

        let a1 = if a[i] % 1000 / 100 == 0 {
            a[a[i + 1] as usize]
        } else {
            a[i + 1]
        };

        if op == 4 {
            output = a1;
            i += 2;
            continue;
        }

        let a2 = if a[i] % 10000 / 1000 == 0 {
            a[a[i + 2] as usize]
        } else {
            a[i + 2]
        };

        if op == 1 {
            a[a[i + 3] as usize] = a1 + a2;
            i += 4;
        }
        if op == 2 {
            a[a[i + 3] as usize] = a1 * a2;
            i += 4;
        }
        if op == 5 {
            if a1 > 0 {
                i = a2 as usize;
            } else {
                i += 3;
            }
        }
        if op == 6 {
            if a1 == 0 {
                i = a2 as usize;
            } else {
                i += 3;
            }
        }
        if op == 7 {
            a[a[i + 3] as usize] = if a1 < a2 { 1 } else { 0 };
            i += 4;
        }
        if op == 8 {
            a[a[i + 3] as usize] = if a1 == a2 { 1 } else { 0 };
            i += 4;
        }
    }
    return output;
}

fn main() {
    let a = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    //let a: [i32; 11] = [3,9,8,9,10,9,4,9,99,-1,8];
    let setting = [4,3,2,1,0];
    let mut output = 0;
    for i in setting.iter() {
        output = int_comp(a, *i, output);
    }
    println!("{}", output);
}
