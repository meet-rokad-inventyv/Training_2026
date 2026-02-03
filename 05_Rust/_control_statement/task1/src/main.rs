fn main() {

    let start: u8 = 1;
    let end: u8 = 6;

    for i in start..=end {
        if i % 2 == 0 {
            println!("{i} is even inside for");
        } else {
            println!("{i} is odd inside for");
        }
    }

    let mut num = 3;

    while num <= 8 {
        match num {
            4 => {
                println!("4 found, skipping");
                num += 1;
                continue;
            }
            6 => {
                println!("6 found, breaking while loop");
                break;
            }
            _ => println!("{num} inside while"),
        }
        num += 1;
    }

    let mut x = 1;

    'outer_loop: loop {
        println!("x = {x} inside outer loop");

        let mut y = 1;
        while y <= 3 {
            if x == 2 && y == 2 {
                println!("Breaking outer loop when x=2 and y=2");
                break 'outer_loop;
            }
            println!("x = {x}, y = {y}");
            y += 1;
        }

        x += 1;

        if x > 4 {
            break;
        }
    }
}