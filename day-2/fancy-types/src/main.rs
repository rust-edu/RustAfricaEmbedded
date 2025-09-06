#[derive(Debug, Clone)]
struct Object {
    height: u64,
}

#[derive(Debug, Clone)]
struct TObject(u64);

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Any { r: f32, g: f32, b: f32 },
}

fn main() {
    let height = 12;
    let q = Object { height };
    let qr = &q;
    println!("{}", qr.height);
    let r = TObject(13);
    println!("{} {}", qr.height, r.0);

    let color = Color::Any { r: 0.4, g: 0.8, b: 0.2 };
    println!("{:?}", color);
    match color {
        Color::Red => {
            println!("it's red!");
        }
        Color::Any{ r, g, b } if r > g && r > b => {
            println!("it's reddish ({},{},{})", r, g, b);
        }
        Color::Any{ r, g, b } if g > r && g > b => {
            println!("it's greenish ({},{},{})", r, g, b);
        }
        _ => {}
    }

    // type DemoArray = [u32; 1024];

    let mut a: [u32; 1024] =
        core::array::from_fn(|i| i as u32 + 1);
    let mut show_array = || {
        a[0] += 1;
        println!("{} {}", a[0], a[1023]);
    };
    show_array();
    show_array();
}
