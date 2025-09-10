trait Double {
    type Doubled;

    fn double(&self) -> Self::Doubled;
}

impl Double for u32 {
    type Doubled = u32;

    fn double(&self) -> Self::Doubled {
        2 * self
    }
}

impl Double for str {
    type Doubled = String;

    fn double(&self) -> Self::Doubled {
        let mut s = self.to_string();
        s += self;
        s
    }
}

#[derive(Debug)]
struct S;

impl Clone for S {
    fn clone(&self) -> Self {
        todo!()
    }
}

fn main() {
    println!("{}", 17u32.double());
    println!("{}", "hello".double());
    println!("{:?}", S);
}
