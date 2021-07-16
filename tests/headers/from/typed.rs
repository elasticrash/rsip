use rsip::{
    common::uri,
    headers::typed::{from::Tokenizer, From},
};
use std::convert::{TryFrom, TryInto};

validate_typed_header_trait!(From);

#[test]
fn display() -> Result<(), rsip::Error> {
    assert_eq!(
        format!(
            "{}",
            From {
                display_name: Some("Alice".into()),
                uri: uri::Uri {
                    schema: Some(uri::Schema::Sip),
                    auth: Some(uri::Auth {
                        username: "alice".into(),
                        password: None
                    }),
                    host_with_port: uri::HostWithPort::try_from("atlanta.example.com")?,
                    params: vec![],
                    headers: vec![].into()
                },
                params: vec![uri::Param::Tag(uri::param::Tag::new("9fxced76sl"))]
            }
        ),
        String::from("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl")
    );

    Ok(())
}

#[test]
fn from_tokenizer() -> Result<(), rsip::Error> {
    assert_eq!(
        Tokenizer {
            display_name: Some("Alice"),
            uri: uri::Tokenizer {
                schema: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    username: "alice".as_bytes(),
                    password: None
                }),
                host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            },
            params: vec![("tag".as_bytes(), Some("9fxced76sl".as_bytes())).into()],
        }
        .try_into(),
        Ok(From {
            display_name: Some("Alice".into()),
            uri: uri::Uri {
                schema: Some(uri::Schema::Sip),
                auth: Some(uri::Auth {
                    username: "alice".into(),
                    password: None
                }),
                host_with_port: uri::HostWithPort::try_from("atlanta.example.com")?,
                params: vec![],
                headers: vec![].into()
            },
            params: vec![uri::Param::Tag(uri::param::Tag::new("9fxced76sl"))]
        })
    );

    Ok(())
}