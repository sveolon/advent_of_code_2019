use std::vec::Vec;
use std::collections::VecDeque;

fn main() {
    let inp = "59708372326282850478374632294363143285591907230244898069506559289353324363446827480040836943068215774680673708005813752468017892971245448103168634442773462686566173338029941559688604621181240586891859988614902179556407022792661948523370366667688937217081165148397649462617248164167011250975576380324668693910824497627133242485090976104918375531998433324622853428842410855024093891994449937031688743195134239353469076295752542683739823044981442437538627404276327027998857400463920633633578266795454389967583600019852126383407785643022367809199144154166725123539386550399024919155708875622641704428963905767166129198009532884347151391845112189952083025";
    let mut input_full = Vec::new();
    for _i in 0..10000 {
        for c in inp.chars() {
            let d = c as i64 - '0' as i64;
            input_full.push(d);
        }
    }
    
    let mut offset: usize = 0;
    for i in 0..7 {
        offset *= 10;
        offset += input_full[i] as usize;
    }
    
    let mut input = VecDeque::new();
    for i in offset..input_full.len() {
        input.push_back(input_full[i]);
    }
    
    println!("offset {}; full {}; cut {}", offset, input_full.len(), input.len());
    
    for _phase in 0..100 {
        let mut out = VecDeque::new();
        println!("phase {}", _phase);
        let mut r = 0;
        for i in 0..input.len() {
            let j = input.len() - i - 1;
            r += input[j];
            r = num::abs(r);
            r = r % 10;
            out.push_front(r);
        }
        input = out;
    }
    print!("\n");
    for d in 0..9 {
        print!("{}", input[d]);
    }
}
