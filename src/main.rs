use std::io;

// Generate nth fabonnaci number

fn main() {
    let mut user_input = String::new();
    println!("What position of the fabonacci sequence are you looking for? (enter number)");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_position: i32 = match user_input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter an integer");
            return;
        }
    };

    get_fabonacci_number(user_position);
}

fn get_fabonacci_number(mut position: i32) {
    let mut temp = 0;
    let mut curr_num = 1;
    let mut prev_num = 0;
    let mut count = 2;
    position = position + 1;

    loop {
        if position == 1 {
            println!("Position 1: 0");
            break
        }
        if count == position {
            println!("Position {count}: {curr_num}");
            break
        }

        temp = curr_num;

        curr_num = curr_num + prev_num;
        prev_num = temp;

        count += 1;
    }
}
    
    
