fn prod(line: String) -> u32 {
    let mut stream = line.chars();
    let mut prod = 0;
    let mut enabled = true;
    while let Some(char) = stream.next() {
        match char {
            'm' => {
                if &stream.as_str()[..3] == "ul(" && enabled {
                    let mut to_prod = [String::new(),String::new()];
                    let mut idx = 0;
                    stream.nth(2);
                    while let Some(char) = stream.next() {
                        match char {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => to_prod[idx] += &char.to_string(),
                            ',' => idx = 1,
                            ')' => {
                                prod += to_prod.map(|n| n.parse::<u32>().unwrap()).iter().product::<u32>();
                                break
                            },
                            _ => break,
                        }
                    }
                }
            },
            'd' => {
                if &stream.as_str()[..3] == "o()" {
                    stream.nth(2);
                    enabled = true;
                } else if &stream.as_str()[..6] == "on't()" {
                    stream.nth(5);
                    enabled = false;
                }
            }
            _ => (),
        }
    }
    prod
}

fn main() {
    let file = std::fs::read_to_string("input-3.txt").unwrap();
    println!("{}", prod(file));
}

