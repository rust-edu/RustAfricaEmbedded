fn sub3(x: u64) -> u64 {
    let _y: () = ();
    x - 3
}

fn _print_stuff() {
    println!("stuff");
}

fn main() {
    let mut x = sub3(4);
    x += 1;
    let y = 5.0 + x as f32;
    // let _z = y as f64 + 12.0f64;

    if y > 6.0 {
        println!("{x} {y}");
    } else {
        println!("wha?");
    }

    for i in (0..=x).rev() {
        println!("{i}");
    }

    let z = if x < 1 {
        0
    } else {
        -1
    };
    println!("{z}");
}
