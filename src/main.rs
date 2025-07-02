use std::io;
mod conv {
    pub mod celcius;
    pub mod fahrenheit;
    pub mod kelvin;
}

fn main() {
    println!(
        "Options : \n 
        1: Celcius to Kelvin \n 
        2: Celcius to Fahrenheit \n 
        3: Kelvin to Celcius  \n 
        4: Kelvin to Fahrenheit \n 
        5: Fahrenheit to Celcius \n 
        6: Fahrenheit to Kelvin \n

        Choose an option : "
    );

    let mut choice = String::new();
    let mut x = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Please enter a valid character");

    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    println!("Please enter the value to be converted: ");

    io::stdin()
        .read_line(&mut x)
        .expect("Please enter a valid number");

    let x: f64 = x.trim().parse().expect("Please enter a valid number");

    if choice == 1 {
        conv::celcius::to_kelvin(x)
    } else if choice == 2 {
        conv::celcius::to_fahrenheit(x);
    } else if choice == 3 {
        conv::kelvin::to_celcius(x);
    } else if choice == 4 {
        conv::kelvin::to_fahrenheit(x);
    } else if choice == 5 {
        conv::fahrenheit::to_celcius(x);
    } else if choice == 6 {
        conv::fahrenheit::to_kelvin(x);
    } else {
        println!("Choose a valid option !");
    }
}
