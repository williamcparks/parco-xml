use parco_xml::{DeXml, dexml};

#[derive(Debug)]
struct Res<'a> {
    user_id: &'a str,
}

dexml! {
    ref Res;

    Envelope {
        Header {}
        Body {
            AuthenticateUserResponse {
                AuthenticateUserResponse {
                    UserID {
                        (user_id)
                    }
                }
            }
        }
    }
}

pub fn soap() {
    let res = Res::dexml(INPUT).unwrap();
    println!("{:#?}", res.user_id);
}

const INPUT: &str = "<s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\"><s:Header><ActivityId CorrelationId=\"015f5d30-a0f7-48b2-b395-b06aaee44051\" xmlns=\"http://schemas.microsoft.com/2004/09/ServiceModel/Diagnostics\">00000000-0000-0000-0000-000000000000</ActivityId><SessionTimeout xmlns=\"http://www.tylertech.com/\">20</SessionTimeout><EFMConfigurationHeader xmlns=\"urn:tyler:efm:services\"><Version>2025.4.0.11876</Version></EFMConfigurationHeader></s:Header><s:Body xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\"><AuthenticateUserResponse xmlns=\"urn:tyler:efm:services\"><AuthenticateUserResponse><Error xmlns=\"urn:tyler:efm:services:schema:BaseResponse\"><ErrorCode xmlns=\"urn:tyler:efm:services:schema:Common\">0</ErrorCode><ErrorText xmlns=\"urn:tyler:efm:services:schema:Common\">No Error</ErrorText></Error><UserID xmlns=\"urn:tyler:efm:services:schema:AuthenticateResponse\">a2517da8-f7f0-48e4-87e7-2dca30f4bfc4</UserID><Email xmlns=\"urn:tyler:efm:services:schema:AuthenticateResponse\">stage@efile.click</Email><FirstName xmlns=\"urn:tyler:efm:services:schema:AuthenticateResponse\">Stage</FirstName><LastName xmlns=\"urn:tyler:efm:services:schema:AuthenticateResponse\">EFile</LastName><PasswordHash xmlns=\"urn:tyler:efm:services:schema:AuthenticateResponse\">77970fe1-a4ce-4d02-8c42-109009b3ab5c</PasswordHash><ExpirationDateTime xmlns=\"urn:tyler:efm:services:schema:AuthenticateResponse\">6108-10-12T07:00:00</ExpirationDateTime></AuthenticateUserResponse></AuthenticateUserResponse></s:Body></s:Envelope>";
