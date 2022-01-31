use std::env;

#[rustfmt::skip]
fn trivial() {
    let _ = ::std::env::var("RUSTFLAGS");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `::std::env::var` with a string literal
    let _ = ::std::env::set_var("RUSTFLAGS", "1");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `::std::env::set_var` with a string literal
    let _ = ::std::env::remove_var("RUSTFLAGS");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `::std::env::remove_var` with a string literal

    let _ = std::env::var("RUSTFLAGS");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `std::env::var` with a string literal
    let _ = std::env::set_var("RUSTFLAGS", "1");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `std::env::set_var` with a string literal
    let _ = std::env::remove_var("RUSTFLAGS");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `std::env::remove_var` with a string literal

    let _ = env::var("RUSTFLAGS");
         // ^^^^^^^^^^^^^^^^^^^^^ Using `env::var` with a string literal
    let _ = env::set_var("RUSTFLAGS", "1");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `env::set_var` with a string literal
    let _ = env::remove_var("RUSTFLAGS");
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `env::remove_var` with a string literal
}

#[rustfmt::skip]
fn no_match() {
    static RUSTFLAGS: &str = "RUSTFLAGS";
    let _ = std::env::var(RUSTFLAGS);
    let _ = std::env::set_var(RUSTFLAGS, "-1");
}
