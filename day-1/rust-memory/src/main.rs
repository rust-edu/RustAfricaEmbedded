fn main() {
    let mut x = 3;
    let xp1: &u32 = &x;
    let xp2: &u32 = &x;
    let xp3: &u32 = xp1;
    println!("{} {:p} {} {} {}", x, xp1, xp1, xp2, xp3);

    x = 4;
    println!("{}", x);

    let xp: &mut u32 = &mut x;
    *xp = 5;
    println!("{}", xp);
}
