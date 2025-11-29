use std::io;

fn main() {
    println!("Let's convert some temperature!");

    let output_temp: f32;
    let output_unit: char;

    let input_temp = grab_num_input();
    let temp_unit = grab_char_input();

    if temp_unit == 'C' {
      output_temp = (input_temp * 1.8) + 32.0;
      output_unit = 'F';
    } else {
      output_temp = (input_temp - 32.0) / 1.8;
      output_unit = 'C';
    };

    println!("{input_temp} {temp_unit} is {output_temp} in {output_unit}");
}

fn grab_num_input() -> f32 {
    let input_temp: f32 = loop {
        println!("What is the temperature?");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("not a valid number");
                 continue
            },
        };
    };

    input_temp
}

fn grab_char_input() -> char {
    let temp_unit: char = loop {
        println!("If converting FROM Fahrenheit type F otherwise type C");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: char = match user_input.trim().parse() {
            Ok(letter) => letter,
            Err(_) => {
                println!("not a character");
                continue
            },
        };

        if user_input == 'F' || user_input == 'f' {
            break 'F';
        } else if user_input == 'C' || user_input == 'c' {
            break 'C';
        }

        println!("Please try again!");
    };

    temp_unit
}
