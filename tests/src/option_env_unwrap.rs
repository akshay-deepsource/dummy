#[rustfmt::skip]
fn trivial() {
    let _ = option_env!("X").unwrap();
         // ^^^^^^^^^^^^^^^^^^^^^^^^^ Called `unwrap` on `option_env!` macro

}

#[rustfmt::skip]
fn qualified() {
    let _ = std::option_env!("X").unwrap();
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Called `unwrap` on `option_env!` macro
    let _ = core::option_env!("X").unwrap();
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Called `unwrap` on `option_env!` macro
    let _ = ::std::option_env!("X").unwrap();
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Called `unwrap` on `option_env!` macro
    let _ = ::core::option_env!("X").unwrap();
         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Called `unwrap` on `option_env!` macro
}

fn no_match() {
    let _ = option_env!("X").unwrap_or("abc");
}
