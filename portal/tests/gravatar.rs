use portal::gravatar;

#[test]
fn gravatar() {
    for email in vec![
        " MyEmailAddress@example.com",
        "MyEmailAddress@example.com ",
        " MyEmailAddress@example.com ",
    ] {
        let it = gravatar::hash(email);
        assert_eq!(
            it,
            "84059b07d4be67b806386c0aad8070a23f18836bbaae342275dc0a83414c32ee"
        );
        println!("{}: {}", email, gravatar::image(email));
    }
}
