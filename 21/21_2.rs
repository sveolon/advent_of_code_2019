
fn zeros(size: usize) -> Vec<i64> {
    let mut zero_vec: Vec<i64> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}


fn int_comp2(arr: Vec<i64>, input1: Vec<i64>) -> i64 {
    let mut a = arr;
    let mut i = 0;
    let mut output= std::i64::MAX;
    let mut i_num = 0;

    let mut rb: i64 = 0;
    loop {
        let op = a[i] % 100;

        if op == 99 {
            break;
        }
        
        let ind1 = match a[i] % 1000 / 100 {
            1 => i + 1 as usize,
            0 => a[i+1] as usize,
            2 => (rb + a[i+1]) as usize,
            _ => std::i64::MAX as usize,
        };
        
        let ind2 = match a[i] % 10000 / 1000 {
            1 => i + 2 as usize,
            0 => a[i+2] as usize,
            2 => (rb + a[i+2]) as usize,
            _ => std::i64::MAX  as usize,
        };
        
        let ind3 = match a[i] % 100000 / 10000 {
            1 => i + 3 as usize,
            0 => a[i+3] as usize,
            2 => (rb + a[i+3]) as usize,
            _ => std::i64::MAX as usize,
        };
        
        if op == 3 {
            a[ind1] = input1[i_num];
            i_num += 1;
            i += 2;
            continue;
        }

        let a1 = a[ind1];
        let a2 = a[ind2];
                
        if op == 4 {
            output = a1;
            print!("{}", (output as u8) as char);

            i += 2;
            continue;
        }

        if op == 9 {
            rb += a1;
            i += 2;
            continue;
        }

        if op == 1 {
            a[ind3] = a1 + a2;
            i += 4;
        }
        if op == 2 {
            a[ind3] = a1 * a2;
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
            a[ind3] = if a1 < a2 { 1 } else { 0 };
            i += 4;
        }
        if op == 8 {
            a[ind3] = if a1 == a2 { 1 } else { 0 };
            i += 4;
        }
    }
    return output;
}
fn main() {
    let a = [109,2050,21102,1,966,1,21101,0,13,0,1105,1,1378,21102,20,1,0,1105,1,1337,21101,27,0,0,1105,1,1279,1208,1,65,748,1005,748,73,1208,1,79,748,1005,748,110,1208,1,78,748,1005,748,132,1208,1,87,748,1005,748,169,1208,1,82,748,1005,748,239,21101,0,1041,1,21102,1,73,0,1105,1,1421,21101,78,0,1,21101,0,1041,2,21101,88,0,0,1106,0,1301,21101,68,0,1,21101,1041,0,2,21101,103,0,0,1106,0,1301,1102,1,1,750,1105,1,298,21102,1,82,1,21102,1,1041,2,21101,0,125,0,1106,0,1301,1101,0,2,750,1105,1,298,21101,79,0,1,21101,0,1041,2,21101,0,147,0,1105,1,1301,21102,84,1,1,21102,1,1041,2,21102,162,1,0,1106,0,1301,1102,3,1,750,1106,0,298,21102,65,1,1,21102,1,1041,2,21101,0,184,0,1105,1,1301,21101,0,76,1,21101,1041,0,2,21102,1,199,0,1106,0,1301,21102,1,75,1,21102,1,1041,2,21102,1,214,0,1106,0,1301,21102,221,1,0,1106,0,1337,21101,10,0,1,21101,0,1041,2,21101,0,236,0,1105,1,1301,1106,0,553,21101,0,85,1,21102,1,1041,2,21101,254,0,0,1106,0,1301,21102,1,78,1,21101,0,1041,2,21101,269,0,0,1105,1,1301,21102,276,1,0,1105,1,1337,21102,1,10,1,21101,0,1041,2,21101,291,0,0,1105,1,1301,1102,1,1,755,1105,1,553,21102,32,1,1,21102,1041,1,2,21102,313,1,0,1106,0,1301,21102,1,320,0,1105,1,1337,21101,327,0,0,1105,1,1279,2102,1,1,749,21101,65,0,2,21102,1,73,3,21102,1,346,0,1105,1,1889,1206,1,367,1007,749,69,748,1005,748,360,1101,0,1,756,1001,749,-64,751,1106,0,406,1008,749,74,748,1006,748,381,1101,0,-1,751,1106,0,406,1008,749,84,748,1006,748,395,1102,-2,1,751,1105,1,406,21102,1100,1,1,21102,1,406,0,1106,0,1421,21101,32,0,1,21101,0,1100,2,21101,421,0,0,1105,1,1301,21101,428,0,0,1106,0,1337,21102,435,1,0,1105,1,1279,2101,0,1,749,1008,749,74,748,1006,748,453,1101,-1,0,752,1105,1,478,1008,749,84,748,1006,748,467,1101,0,-2,752,1106,0,478,21101,1168,0,1,21101,0,478,0,1106,0,1421,21101,0,485,0,1106,0,1337,21101,10,0,1,21102,1,1168,2,21102,500,1,0,1106,0,1301,1007,920,15,748,1005,748,518,21102,1209,1,1,21102,518,1,0,1106,0,1421,1002,920,3,529,1001,529,921,529,1001,750,0,0,1001,529,1,537,1001,751,0,0,1001,537,1,545,101,0,752,0,1001,920,1,920,1105,1,13,1005,755,577,1006,756,570,21101,0,1100,1,21101,570,0,0,1105,1,1421,21102,1,987,1,1106,0,581,21102,1001,1,1,21102,1,588,0,1106,0,1378,1101,0,758,594,102,1,0,753,1006,753,654,20101,0,753,1,21102,1,610,0,1105,1,667,21102,0,1,1,21101,0,621,0,1106,0,1463,1205,1,647,21101,0,1015,1,21102,1,635,0,1106,0,1378,21101,0,1,1,21101,646,0,0,1105,1,1463,99,1001,594,1,594,1105,1,592,1006,755,664,1102,1,0,755,1106,0,647,4,754,99,109,2,1101,0,726,757,22101,0,-1,1,21101,0,9,2,21101,697,0,3,21102,692,1,0,1105,1,1913,109,-2,2106,0,0,109,2,1001,757,0,706,1202,-1,1,0,1001,757,1,757,109,-2,2105,1,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,255,63,159,95,191,223,127,0,139,100,252,50,184,34,69,163,173,162,53,121,251,250,119,236,118,190,202,109,49,153,183,42,234,216,169,220,107,221,212,249,111,102,181,136,217,246,227,60,98,231,215,94,182,174,57,152,62,241,166,93,188,126,47,199,99,205,239,38,103,138,85,116,39,61,226,56,214,76,238,70,253,235,203,117,197,43,106,204,51,245,175,154,233,244,207,167,140,186,237,55,177,200,178,206,222,232,137,113,198,229,213,168,247,243,68,218,170,172,59,79,142,108,46,58,84,110,86,201,156,124,230,78,185,196,71,242,123,171,254,248,122,179,92,157,77,219,54,35,155,114,115,125,158,228,141,189,187,101,143,87,120,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,20,73,110,112,117,116,32,105,110,115,116,114,117,99,116,105,111,110,115,58,10,13,10,87,97,108,107,105,110,103,46,46,46,10,10,13,10,82,117,110,110,105,110,103,46,46,46,10,10,25,10,68,105,100,110,39,116,32,109,97,107,101,32,105,116,32,97,99,114,111,115,115,58,10,10,58,73,110,118,97,108,105,100,32,111,112,101,114,97,116,105,111,110,59,32,101,120,112,101,99,116,101,100,32,115,111,109,101,116,104,105,110,103,32,108,105,107,101,32,65,78,68,44,32,79,82,44,32,111,114,32,78,79,84,67,73,110,118,97,108,105,100,32,102,105,114,115,116,32,97,114,103,117,109,101,110,116,59,32,101,120,112,101,99,116,101,100,32,115,111,109,101,116,104,105,110,103,32,108,105,107,101,32,65,44,32,66,44,32,67,44,32,68,44,32,74,44,32,111,114,32,84,40,73,110,118,97,108,105,100,32,115,101,99,111,110,100,32,97,114,103,117,109,101,110,116,59,32,101,120,112,101,99,116,101,100,32,74,32,111,114,32,84,52,79,117,116,32,111,102,32,109,101,109,111,114,121,59,32,97,116,32,109,111,115,116,32,49,53,32,105,110,115,116,114,117,99,116,105,111,110,115,32,99,97,110,32,98,101,32,115,116,111,114,101,100,0,109,1,1005,1262,1270,3,1262,20102,1,1262,0,109,-1,2105,1,0,109,1,21101,1288,0,0,1106,0,1263,20101,0,1262,0,1101,0,0,1262,109,-1,2106,0,0,109,5,21102,1310,1,0,1105,1,1279,21202,1,1,-2,22208,-2,-4,-1,1205,-1,1332,22102,1,-3,1,21102,1332,1,0,1106,0,1421,109,-5,2106,0,0,109,2,21102,1,1346,0,1105,1,1263,21208,1,32,-1,1205,-1,1363,21208,1,9,-1,1205,-1,1363,1106,0,1373,21102,1,1370,0,1105,1,1279,1106,0,1339,109,-2,2105,1,0,109,5,2101,0,-4,1385,21001,0,0,-2,22101,1,-4,-4,21101,0,0,-3,22208,-3,-2,-1,1205,-1,1416,2201,-4,-3,1408,4,0,21201,-3,1,-3,1105,1,1396,109,-5,2105,1,0,109,2,104,10,22102,1,-1,1,21102,1,1436,0,1105,1,1378,104,10,99,109,-2,2106,0,0,109,3,20002,594,753,-1,22202,-1,-2,-1,201,-1,754,754,109,-3,2105,1,0,109,10,21102,5,1,-5,21101,0,1,-4,21102,1,0,-3,1206,-9,1555,21101,3,0,-6,21101,5,0,-7,22208,-7,-5,-8,1206,-8,1507,22208,-6,-4,-8,1206,-8,1507,104,64,1105,1,1529,1205,-6,1527,1201,-7,716,1515,21002,0,-11,-8,21201,-8,46,-8,204,-8,1105,1,1529,104,46,21201,-7,1,-7,21207,-7,22,-8,1205,-8,1488,104,10,21201,-6,-1,-6,21207,-6,0,-8,1206,-8,1484,104,10,21207,-4,1,-8,1206,-8,1569,21102,1,0,-9,1106,0,1689,21208,-5,21,-8,1206,-8,1583,21102,1,1,-9,1106,0,1689,1201,-5,716,1589,20101,0,0,-2,21208,-4,1,-1,22202,-2,-1,-1,1205,-2,1613,21202,-5,1,1,21102,1613,1,0,1106,0,1444,1206,-1,1634,21201,-5,0,1,21102,1,1627,0,1105,1,1694,1206,1,1634,21101,2,0,-3,22107,1,-4,-8,22201,-1,-8,-8,1206,-8,1649,21201,-5,1,-5,1206,-3,1663,21201,-3,-1,-3,21201,-4,1,-4,1105,1,1667,21201,-4,-1,-4,21208,-4,0,-1,1201,-5,716,1676,22002,0,-1,-1,1206,-1,1686,21101,1,0,-4,1106,0,1477,109,-10,2106,0,0,109,11,21101,0,0,-6,21102,1,0,-8,21102,0,1,-7,20208,-6,920,-9,1205,-9,1880,21202,-6,3,-9,1201,-9,921,1724,21001,0,0,-5,1001,1724,1,1733,20102,1,0,-4,21201,-4,0,1,21102,1,1,2,21102,1,9,3,21102,1754,1,0,1106,0,1889,1206,1,1772,2201,-10,-4,1767,1001,1767,716,1767,20102,1,0,-3,1106,0,1790,21208,-4,-1,-9,1206,-9,1786,22101,0,-8,-3,1105,1,1790,21201,-7,0,-3,1001,1733,1,1796,20101,0,0,-2,21208,-2,-1,-9,1206,-9,1812,21202,-8,1,-1,1105,1,1816,21202,-7,1,-1,21208,-5,1,-9,1205,-9,1837,21208,-5,2,-9,1205,-9,1844,21208,-3,0,-1,1106,0,1855,22202,-3,-1,-1,1105,1,1855,22201,-3,-1,-1,22107,0,-1,-1,1106,0,1855,21208,-2,-1,-9,1206,-9,1869,21201,-1,0,-8,1106,0,1873,22102,1,-1,-7,21201,-6,1,-6,1105,1,1708,22101,0,-8,-10,109,-11,2106,0,0,109,7,22207,-6,-5,-3,22207,-4,-6,-2,22201,-3,-2,-1,21208,-1,0,-6,109,-7,2106,0,0,0,109,5,1202,-2,1,1912,21207,-4,0,-1,1206,-1,1930,21101,0,0,-4,22102,1,-4,1,22102,1,-3,2,21101,0,1,3,21101,0,1949,0,1106,0,1954,109,-5,2106,0,0,109,6,21207,-4,1,-1,1206,-1,1977,22207,-5,-3,-1,1206,-1,1977,21201,-5,0,-5,1105,1,2045,22102,1,-5,1,21201,-4,-1,2,21202,-3,2,3,21101,1996,0,0,1106,0,1954,21202,1,1,-5,21102,1,1,-2,22207,-5,-3,-1,1206,-1,2015,21101,0,0,-2,22202,-3,-2,-3,22107,0,-4,-1,1206,-1,2037,21202,-2,1,1,21102,2037,1,0,105,1,1912,21202,-3,-1,-3,22201,-5,-3,-5,109,-6,2105,1,0];
    
    let mut arr2 = zeros(a.len() * 100);
    for i in 0..a.len() {
        arr2[i] = a[i].clone();
    }

    let inp = "\
OR A T
AND B T
AND C T
NOT T T
AND D T
OR E J
AND I J
OR H J
AND T J
NOT A T
OR T J
RUN
";

    let mut input = Vec::new();
    for c in inp.chars() {
        input.push((c as u8) as i64);
    }
    let res = int_comp2(arr2, input);
    println!("\nresult: {}", res);
}
