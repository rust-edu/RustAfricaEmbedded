/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

/*
enum Option<T> {
    Some(T),
    None,
}
*/

fn div_raw(n: u8, d: u8) -> u8 {
    n / d
}

fn div_option(n: u8, d: u8) -> Option<u8> {
    if d != 0 {
        Some(n / d)
    } else {
        None
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum DivError {
    BadDenominator(u8),
}

fn div_result(n: u8, d: u8) -> Result<u8, DivError> {
    if d != 0 {
        Ok(n / d)
    } else {
        Err(DivError::BadDenominator(d))
    }
}

fn div_result_indirect() -> Result<u8, DivError> {
    let q = div_result(12, 0)?;
    if q == 1 {
        println!("got 1");
    }
    Ok(q)
}

fn main() {
    println!("{}", div_raw(12, 8));

    if let Some(q) = div_option(12, 0) {
        println!("{}", q);
    } else {
        println!("div_option: bad arguments");
    }

    match div_result_indirect() {
        Ok(q) => println!("{}", q),
        Err(e) => println!("{:?}", e),
    }

    println!("{}", div_raw(12, 0));
}
