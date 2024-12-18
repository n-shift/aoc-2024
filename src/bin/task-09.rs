const INPUT: &[u8] = include_bytes!("../../input/9.txt");

fn p1() {
    let mut map: Vec<Option<u32>> = Vec::new();
    let mut length = 0;
    INPUT.chunks(2).enumerate().for_each(|(id, c)| {
        let amount = (c[0] as char).to_digit(10).unwrap();
        let spaces = if id == INPUT.len() / 2 || (c[1] == b'\n') {
            0
        } else {
            (c[1] as char).to_digit(10).unwrap()
        };
        map.extend(vec![Some(id as u32); amount as usize]);
        map.extend(vec![None; spaces as usize]);
        length += amount;
    });
    let mut compact: Vec<u32> = Vec::with_capacity(length as usize);
    for c in map.clone() {
        if compact.len() == length as usize {
            break;
        }
        if let Some(v) = c {
            compact.push(v)
        } else {
            let mut temp = map.pop().unwrap_or_else(|| panic!("hmm"));
            while temp.is_none() {
                temp = map.pop().unwrap_or_else(|| panic!("hmm"));
            }
            compact.push(temp.unwrap());
        }
    }
    let checksum = compact.iter().enumerate().map(|(pos, id)| pos * (*id as usize)).sum::<usize>();
    println!("{checksum}");
}

fn p2() {
    let mut map: Vec<Vec<Option<u32>>> = Vec::new();
    INPUT.chunks(2).enumerate().for_each(|(id, c)| {
        let amount = (c[0] as char).to_digit(10).unwrap();
        let spaces = if id == INPUT.len() / 2 || (c[1] == b'\n') {
            0
        } else {
            (c[1] as char).to_digit(10).unwrap()
        };
        map.push(vec![Some(id as u32); amount as usize]);
        if spaces != 0 {
            map.push(vec![None; spaces as usize])
        }
    });
    let mut processed = map.clone();
    for (idx_v, v) in map.iter().enumerate().rev() {
        if v.contains(&None) {
            continue;
        }
        for (idx_b, block) in processed.iter_mut().enumerate() {
            if idx_v <= idx_b {
                break;
            }
            let accepted = v.len() <= block.iter().filter(|s| s.is_none()).count();
            if accepted {
                block
                    .iter_mut()
                    .filter(|x| x.is_none())
                    .enumerate()
                    .for_each(|(i, x)| *x = *v.get(i).unwrap_or(&None));
                processed[idx_v].iter_mut().for_each(|n| *n = None);
                break;
            }
        }
    }
    let checksum = processed
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, id)| !id.is_none())
        .map(|(pos, id)| pos * (id.unwrap() as usize))
        .sum::<usize>();
    println!("{checksum}");
}

fn main() {
    p1();
    p2();
}
