use nlip::{ControlMessage, DataMessage, Format, SubMessage, Value};

#[test]
fn create_control_message() {
    let message = ControlMessage::new(Format::Text, "text", Value::Null, None);
    assert_eq!(
        message,
        ControlMessage::new(Format::Text, "text", Value::Null, None)
    );
    assert_eq!(
        r#"{"MessageType":"control","Format":"text","Subformat":"text","Content":null}"#,
        format!("{}", message)
    );
}

#[test]
fn create_control_message_with_submessages() {
    let message = ControlMessage::new(
        Format::Text,
        "text",
        Value::Null,
        Some(vec![SubMessage {
            Label: "some label here".into(),
            Format: Format::Text,
            Subformat: "text",
            Content: Value::String("Some content here".into()),
        }]),
    );
    assert_eq!(
        message,
        ControlMessage::new(
            Format::Text,
            "text",
            Value::Null,
            Some(vec![SubMessage {
                Label: "some label here".into(),
                Format: Format::Text,
                Subformat: "text",
                Content: Value::String("Some content here".into()),
            }]),
        )
    );
    assert_eq!(
        r#"{"MessageType":"control","Format":"text","Subformat":"text","Content":null,"Submessages":[{"Label":"some label here","Format":"text","Subformat":"text","Content":"Some content here"}]}"#,
        format!("{}", message)
    );
}

#[test]
fn create_data_message() {
    let message = DataMessage::new("confirmation", Format::Text, "text", Value::Null, None);
    assert_eq!(
        message,
        DataMessage::new("confirmation", Format::Text, "text", Value::Null, None)
    );
    assert_eq!(
        r#"{"MessageType":"confirmation","Format":"text","Subformat":"text","Content":null}"#,
        format!("{}", message)
    );
}

#[test]
fn create_data_message_with_empty_submessages() {
    let message = DataMessage::new(
        "confirmation",
        Format::Text,
        "text",
        Value::Null,
        Some(vec![]),
    );
    assert_eq!(
        r#"{"MessageType":"confirmation","Format":"text","Subformat":"text","Content":null}"#,
        format!("{}", message)
    );
}

#[test]
fn create_data_message_with_submessages() {
    let sub_message = vec![
        SubMessage {
            Label: "some label here".into(),
            Format: Format::Text,
            Subformat: "text",
            Content: Value::String("Some content here".into()),
        },
        SubMessage {
            Label: "some label here again".into(),
            Format: Format::Text,
            Subformat: "text",
            Content: Value::String("Some content here again".into()),
        },
    ];
    let message = DataMessage::new(
        "confirmation",
        Format::Text,
        "text",
        Value::Null,
        Some(sub_message),
    );

    assert_eq!(
        r#"{"MessageType":"confirmation","Format":"text","Subformat":"text","Content":null,"Submessages":[{"Label":"some label here","Format":"text","Subformat":"text","Content":"Some content here"},{"Label":"some label here again","Format":"text","Subformat":"text","Content":"Some content here again"}]}"#,
        format!("{}", message)
    );
}

#[test]
fn test_message_format_field() {
    let message = ControlMessage::new(Format::Binary, "no matter", Value::Null, None);
    assert_eq!(
        r#"{"MessageType":"control","Format":"binary","Subformat":"no matter","Content":null}"#,
        format!("{}", message)
    );

    let message = ControlMessage::new(Format::Generic, "no matter", Value::Null, None);
    assert_eq!(
        r#"{"MessageType":"control","Format":"generic","Subformat":"no matter","Content":null}"#,
        format!("{}", message)
    );

    let message = ControlMessage::new(Format::Location, "no matter", Value::Null, None);
    assert_eq!(
        r#"{"MessageType":"control","Format":"location","Subformat":"no matter","Content":null}"#,
        format!("{}", message)
    );

    let message = ControlMessage::new(Format::Structured, "no matter", Value::Null, None);
    assert_eq!(
        r#"{"MessageType":"control","Format":"structured","Subformat":"no matter","Content":null}"#,
        format!("{}", message)
    );

    let message = ControlMessage::new(Format::Text, "no matter", Value::Null, None);
    assert_eq!(
        r#"{"MessageType":"control","Format":"text","Subformat":"no matter","Content":null}"#,
        format!("{}", message)
    );

    let message = ControlMessage::new(Format::Token, "no matter", Value::Null, None);
    assert_eq!(
        r#"{"MessageType":"control","Format":"token","Subformat":"no matter","Content":null}"#,
        format!("{}", message)
    );
}
