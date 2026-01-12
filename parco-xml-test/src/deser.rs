use std::{borrow::Cow, fmt::Display};

use parco_xml::{
    DeXml, DeXmlError,
    de::{AppendPath, Reader},
    dexml,
};

#[derive(Debug)]
struct Response<'a> {
    body_id: usize,
    user_token: &'a str,
    expires: Expires,
}

#[derive(Debug)]
pub struct Expires {
    year: i64,
    month: u8,
    day: u8,
}

#[derive(Debug)]
enum ExpiresError {
    NoYear,
    InvalidYear,
    NoMonth,
    InvalidMonth,
    NoDay,
    InvalidDay,
}

impl Display for ExpiresError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Expires {
    fn parse(text: &str) -> Result<Self, ExpiresError> {
        let mut split = text.split('-');
        let year = split
            .next()
            .ok_or(ExpiresError::NoYear)?
            .parse()
            .map_err(|_| ExpiresError::InvalidYear)?;
        let month = split
            .next()
            .ok_or(ExpiresError::NoMonth)?
            .parse()
            .map_err(|_| ExpiresError::InvalidMonth)?;
        let day = split
            .next()
            .ok_or(ExpiresError::NoDay)?
            .parse()
            .map_err(|_| ExpiresError::InvalidDay)?;
        Ok(Self { year, month, day })
    }
}

impl<'de> DeXml<'de> for Expires {
    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError> {
        reader.append_path(AppendPath::Custom("Expires"));
        let text = reader.text()?;
        let res = match Self::parse(text) {
            Ok(ok) => ok,
            Err(err) => return Err(reader.err(Cow::Owned(err.to_string()))),
        };
        reader.exit_path();
        Ok(res)
    }
}

dexml! {
    ref Response;

    Envelope {
        Body mustUnderstand="1" id=(body_id) {
            Response {
                UserToken {
                    (user_token)
                }
                Expires {
                    (expires)
                }
            }
        }
    }
}

pub fn deser() {
    let src = "
        <soap:Envelope>
            <soap:Body id=\"123\" soap:mustUnderstand=\"1\">
                <Response>
                    <UserToken>
                        secret_token
                    </UserToken>
                    <Expires>
                        2025-01-01
                    </Expires>
                 </Response>
            </soap:Body>
        </soap:Envelope>
    ";

    let res = Response::dexml(src).unwrap();
    println!("{:?}", res.body_id);
    println!("{:?}", res.user_token);
    println!(
        "{} {} {}",
        res.expires.year, res.expires.month, res.expires.day
    );
}
