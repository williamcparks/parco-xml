# Parco XML

**Parco XML** is a **zero-copy XML deserialization library** for Rust, designed to generate highly efficient parsing code. It allows you to extract only the fields you need without traversing unnecessary parts of the XML, making it fast and memory-efficient.

With Parco XML, you can define your data structures and mapping to XML declaratively using the `dexml!` macro. The library also supports custom parsing logic for individual fields, enabling validation and specialized deserialization when needed.

## Example

```rust,ignore
use parco_xml::{DeXml, dexml};

#[derive(Debug)]
struct Response<'a> {
    body_id: usize,
    user_token: &'a str,
    expires: &'a str,
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

let src = "
<soap:Envelope>
    <soap:Body id=\"123\" soap:mustUnderstand=\"1\">
        <Response>
            <UserToken>
                example token
            </UserToken>
            <Expires>
                2030-01-01
            </Expires>
        </Response>
    </soap:Body>
</soap:Envelope>
";

let res = Response::dexml(src).unwrap();

println!("{res:?}");
```

### Control Structures

- Render Nothing: [Empty](crate::ser::Empty)

- Render conditionally via option: [Conditional](crate::ser::Conditional)

- Render many elements: [List](crate::ser::List)
