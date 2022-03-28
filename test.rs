fn main() {
    let x @ _ = 2;
}

trait X {
    fn f();
}

#[must_use]
fn f() -> () {
    ()
}

type T = ();
#[must_use]
fn f() -> T {
    ()
}
