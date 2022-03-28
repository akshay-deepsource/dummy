fn main() {
    let x @ _ = 2;
}

trait X {
    #[inline]
    fn f() {}
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
