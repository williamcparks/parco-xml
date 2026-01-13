use std::ops::Deref;

use parco_xml::{
    Xml,
    ser::{Conditional, List},
    xml,
};

struct ControlStructure<'a>(Option<&'a str>, Vec<&'a str>);

struct Hello<'a>(&'a str);

xml! {
    ref Hello;

    @ns {}

    Hello {
        (self.0)
    }
}

xml! {
    ref ControlStructure;

    @ns {}

    Control {
        (Conditional(self.0.map(Hello)).display())
        (List(self.1.iter().map(Deref::deref).map(Hello).collect()).display())
    }
}

pub fn control_ser() {
    let none_ex = ControlStructure(None, vec!["Other"]);
    let some_ex = ControlStructure(Some("World"), vec!["Rust", "Xml"]);

    println!("{}", none_ex.display());
    println!("{}", some_ex.display());
}
