use petunia::iso4217;

#[test]
fn list_one() {
    {
        let it = iso4217::list_one::Iso4217::new().unwrap();
        println!("{:?}", it);
    }

    for it in iso4217::Currency::list_one().unwrap().iter() {
        println!("{:?}", it);
    }
}
