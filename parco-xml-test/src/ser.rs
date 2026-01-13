use parco_xml::{Xml, xml};

struct Request<'a> {
    user_id: &'a str,
}

xml! {
    ref Request;

    @ns {
        soap = "uri:soap",
        subns = "uri:subns",
    }

    soap:Envelope {
        soap:Body soap:Id=(self.user_id) fixed="asdf" {
            subns:Request {
                UserID {
                    (self.user_id)
                }
                Const {
                    "my constant"
                }
            }
        }
    }
}

pub fn ser() {
    let request = Request {
        user_id: "hello from rust",
    };

    println!("{}", request.display());
}
