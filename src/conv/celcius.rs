pub fn to_kelvin(x: f64) {
    const AT_ZERO_C: f64 = 273.15;
    //since Kelvin doesnt have a negative scale, the input has to be greater than -273.15 Celcius
    const MINIMUM: f64 = -273.15;

    if x < MINIMUM {
        println!(
            "The value you have entered evaluates into a negative value.Kelvin does not have a negative scale.Please enter a value >= {MINIMUM}"
        );
    } else {
        let result = x + AT_ZERO_C;
        println!("{x} degree celcius is equal to {result} Kelvin");
    }
}

pub fn to_fahrenheit(x: f64) {
    let result = (x * 9.0 / 5.0) + 32.0;

    println!("{x} degree celcius is equal to {result} Fahrenheit");
}
