use phlox::iso4217::ISO4217;

#[test]
fn iso4217() {
    let iso = ISO4217::new().unwrap();
    println!("{}, {} items", iso.published, iso.table.items.len());
    for it in iso.table.items.iter() {
        println!(
            "{}({:?}): {}",
            it.name.value,
            it.code.as_ref().map(|x| &x.value),
            it.country.value
        )
    }
}
