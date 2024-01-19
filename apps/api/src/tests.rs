use rocket::local::blocking::Client;

#[test]
fn hello() {
    let client = Client::tracked(super::rocket()).expect("valid rocket instance");
    let response = client.get("/Ferris/3").dispatch();
    assert_eq!(
        response.into_string(),
        Some("Hello, 3 year old named Ferris!".into())
    );
}
