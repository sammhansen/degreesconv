pub fn to_celcius(x: f64) {
    const AT_ZERO_K: f64 = -273.15;
    const MINIMUM: f64 = 0.0;

    if x < MINIMUM {
        println!("Kelvin does not have a negative scale. Please enter a value >= {MINIMUM}");
    } else {
        let result = x - AT_ZERO_K;
        println!("{x} Kelvin is equal to {result} deg Celcius");
    }
}

pub fn to_fahrenheit(x: f64) {
    const MINIMUM: f64 = 0.0;

    if x < MINIMUM {
        println!("Kelvin does not have a negative scale. Please enter a value >= {MINIMUM}");
    } else {
        let result = (x * 9.0 / 5.0) - 469.67;

        println!("{x} Kelvin is equal to {result} Fahrenheit");
    }
}
