use rim::message::message_model::MsgEvent;


#[test]
fn test_json() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
    {
        "event": "Login",
        "body": {
            "kind": "Text",
            "content": "hello",
            "uid": 1
        }
    }"#;

    // Parse the string of data into serde_json::Value.
    let v: MsgEvent = serde_json::from_str(data).expect("error");
    assert_eq!("hello", v.body.content);
}
