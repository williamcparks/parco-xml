# Parco XML

**Parco XML** is a **zero-copy XML serialization library** that targets Exclusive XML Canonicalization i.e: **exc-c14n** for Rust that also exports [quick_xml](https://docs.rs/quick-xml/latest/quick_xml/index.html) to handle deserialization

# Usage

```rust,ignore
use parco_xml::{Xml, xml};

struct MyXML<'a> {
    id: &'a str,
    value: &'a str,
}

// automatically impl `Xml` trait
xml! {
    // ref indicates you have a lifetime in your type
    // write `use MyXML;` if you don't have a lifetime
    ref MyXML;

    // place your namespaces here
    @ns {
        myxml = "uri:myxml",
    }

    myxml:Element my_const_attr="constant" id=(self.id) {
        (self.value)
    }
}
```

### Control Structures

- Render Nothing: [Empty](https://docs.rs/parco-xml/latest/parco_xml/ser/struct.Empty.html)

- Render conditionally via option: [Conditional](https://docs.rs/parco-xml/latest/parco_xml/ser/struct.Conditional.html)

- Render many elements: [List](https://docs.rs/parco-xml/latest/parco_xml/ser/struct.List.html)
