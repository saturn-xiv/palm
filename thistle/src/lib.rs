#[rustler::nif]
fn add_roles_for_user(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
rustler::init!("Elixir.Thistle.NIF");
