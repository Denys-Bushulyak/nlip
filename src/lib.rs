use std::fmt::{Display, Formatter};

pub use json_syntax::Value;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct Message<'a> {
    pub Format: Format,
    pub Subformat: &'a str,
    pub Content: Value,
    pub Submessages: Option<Vec<SubMessage<'a>>>,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct SubMessage<'a> {
    pub Label: Option<&'a str>,
    pub Format: Format,
    pub Subformat: &'a str,
    pub Content: Value,
}

impl Display for SubMessage<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{")?;

        if let Some(label) = self.Label {
            write!(f, "\"Label\":\"{}\",", label)?;
        }
        write!(f, "\"Format\":\"{}\",", self.Format)?;
        write!(f, "\"Subformat\":\"{}\",", self.Subformat)?;
        write!(f, "\"Content\":{}", self.Content)?;

        write!(f, "}}")
    }
}

impl Display for Message<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{")?;

        write!(f, "\"Format\":\"{}\",", self.Format)?;
        write!(f, "\"Subformat\":\"{}\",", self.Subformat)?;
        write!(f, "\"Content\":{}", self.Content)?;

        if let Some(submessages) = self.Submessages.as_ref()
            && !submessages.is_empty()
        {
            write!(f, ",")?;
            write!(
                f,
                "\"Submessages\":[{}]",
                submessages
                    .iter()
                    .map(|m| format!("{}", m))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataMessage<'subformat>(&'subformat str, Message<'subformat>);

impl<'subformat> DataMessage<'subformat> {
    pub fn new(
        message_type: &'subformat str,
        format: Format,
        subformat: &'subformat str,
        content: Value,
        sub_messages: Option<Vec<SubMessage<'subformat>>>,
    ) -> Self {
        Self(
            message_type,
            Message {
                Format: format,
                Subformat: subformat,
                Content: content,
                Submessages: sub_messages,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ControlMessage<'a>(Message<'a>);

impl<'a> ControlMessage<'a> {
    pub fn new(
        format: Format,
        subformat: &'a str,
        content: Value,
        sub_messages: Option<Vec<SubMessage<'a>>>,
    ) -> Self {
        Self(Message {
            Format: format,
            Subformat: subformat,
            Content: content,
            Submessages: sub_messages,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    Text,
    Token,
    Structured,
    Binary,
    Location,
    Generic,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Format::Text => write!(f, "text"),
            Format::Token => write!(f, "token"),
            Format::Structured => write!(f, "structured"),
            Format::Binary => write!(f, "binary"),
            Format::Location => write!(f, "location"),
            Format::Generic => write!(f, "generic"),
        }
    }
}

impl Display for ControlMessage<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let message = &self.0;

        write!(f, "{{")?;
        write!(f, "\"MessageType\":\"control\",")?;
        write!(f, "\"Format\":\"{}\",", message.Format)?;
        write!(f, "\"Subformat\":\"{}\",", message.Subformat)?;
        write!(f, "\"Content\":{}", message.Content)?;

        if let Some(submessages) = message.Submessages.as_ref()
            && !submessages.is_empty()
        {
            write!(f, ",")?;
            write!(
                f,
                "\"Submessages\":[{}]",
                submessages
                    .iter()
                    .map(|m| format!("{}", m))
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }

        write!(f, "}}")
    }
}

impl Display for DataMessage<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let DataMessage(message_type, message) = self;

        write!(f, "{{")?;
        write!(f, "\"MessageType\":\"{}\",", message_type)?;
        write!(f, "\"Format\":\"{}\",", message.Format)?;
        write!(f, "\"Subformat\":\"{}\",", message.Subformat)?;
        write!(f, "\"Content\":{}", message.Content)?;

        if let Some(submessages) = self.1.Submessages.as_ref()
            && !submessages.is_empty()
        {
            write!(f, ",")?;
            write!(
                f,
                "\"Submessages\":[{}]",
                submessages
                    .iter()
                    .map(|m| format!("{}", m))
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod test {
    use json_syntax::Value;

    use crate::{Format, Message, SubMessage};

    #[test]
    fn test_message_format() {
        let message = Message {
            Format: Format::Text,
            Subformat: "english",
            Content: Value::String("Some content here".into()),
            Submessages: None,
        };
        assert_eq!(
            r#"{"Format":"text","Subformat":"english","Content":"Some content here"}"#,
            message.to_string()
        );
    }

    #[test]
    fn test_message_submessages() {
        let message = Message {
            Format: Format::Text,
            Subformat: "english",
            Content: Value::String("Some content here".into()),
            Submessages: Some(vec![SubMessage {
                Label: None,
                Format: Format::Text,
                Subformat: "english",
                Content: Value::String("Some sub content here".into()),
            }]),
        };
        assert_eq!(
            r#"{"Format":"text","Subformat":"english","Content":"Some content here","Submessages":[{"Format":"text","Subformat":"english","Content":"Some sub content here"}]}"#,
            message.to_string()
        );
    }
}
