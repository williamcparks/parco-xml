use parco_xml::quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct Envelope<'a> {
    #[serde(rename = "Body")]
    #[serde(borrow)]
    body: Body<'a>,
}

#[derive(Deserialize)]
struct Body<'a> {
    #[serde(rename = "Response")]
    #[serde(borrow)]
    response: Response<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Response<'a> {
    field_a: &'a str,
    field_b: &'a str,
}

pub fn soap() {
    let envelope: Envelope = from_str(INPUT).unwrap();

    println!("Field A: = {}", envelope.body.response.field_a,);
    println!("Field B: = {}", envelope.body.response.field_b);
}

const INPUT: &str = r##"
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
    <soap:Header>
    </soap:Header>
    <soap:Body>
        <Response>
            <Other>Hi</Other>
            <FieldA>Hello From Serde</FieldA>
            <dummy>hi</dummy>
            <FieldB>Bye From Serde</FieldB>
        </Response>
    </soap:Body>
</soap:Envelope>
"##;
