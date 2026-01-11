use parco_xml::{Xml, xml};

struct Request<'a> {
    user_id: &'a str,
}

xml! {
    ref Request;

    /*
    soap:Envelope {
        soap:Body {
            Request {
                UserID {
                    (user_id)
                }
            }

        }
    }
    */
}

pub fn ser() {
    let request = Request {
        user_id: "hello from rust",
    };

    println!("{}", request.display());
}
