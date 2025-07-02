pub fn to_celcius(x: f64) {
    let result = (x - 32.0) * (5.0 / 9.0);

    println!("{x} Fahrenheit is equal to {result} deg Celcius");
}

pub fn to_kelvin(x: f64) {
    const MINIMUM: f64 = -459.67;

    if x < MINIMUM {
        println!(
            "The value you have entered evaluates into a negative value. Kevin does not have a negative scale.Please enter a value >= {MINIMUM}"
        );
    } else {
        let result = (x + 459.67) * (5.0 / 9.0);

        println!("{x} Fahrenheit is equal to {result} Kelvin");
    }
}
