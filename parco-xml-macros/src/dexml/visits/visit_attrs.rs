use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};

pub struct VisitAttrs {
    pub const_attrs: Vec<(LitStr, LitStr)>,
    pub dynamic_attrs: Vec<(LitStr, Ident)>,
}

impl VisitAttrs {
    pub fn print(&self) -> TokenStream {
        let dyn_ids = self.dynamic_attrs.iter().map(|v| &v.1).collect::<Vec<_>>();

        let const_arms = self.const_attrs.iter().map(Self::const_arm);
        let dyn_arms = self.dynamic_attrs.iter().map(Self::dyn_arm);

        let dyn_if_checks = self.dynamic_attrs.iter().map(Self::dyn_if_check);

        quote! {
            let (#(#dyn_ids),*) = {
                #(
                    let mut #dyn_ids = ::core::option::Option::None;
                )*

                loop {
                    let (attr_name, attr_value) = match reader.attr_opt()? {
                        ::core::option::Option::Some(attr) => attr,
                        _ => break,
                    };

                    match attr_name {
                        #(#const_arms)*
                        #(#dyn_arms)*
                        _ => {},
                    }
                }

                #(#dyn_if_checks)*

                (
                    #(#dyn_ids),*
                )
            };
        }
    }

    fn const_arm((key, value): &(LitStr, LitStr)) -> TokenStream {
        let format_arg = format!("Expected `{}` Saw `{{attr_value}}`", value.value());
        let format_arg = LitStr::new(format_arg.as_str(), value.span());

        quote! {
            #key => {
                if attr_value == #value {
                    continue;
                }

                reader.append_path(::parco_xml::de::AppendPath::NamedAttr(#key));
                return ::core::result::Result::Err(
                    reader.err(
                        ::std::borrow::Cow::Owned(
                            ::std::format!(#format_arg)
                        )
                    )
                );
            },
        }
    }

    fn dyn_arm((key, dyn_id): &(LitStr, Ident)) -> TokenStream {
        quote! {
            #key => {
                #dyn_id = ::core::option::Option::Some(attr_value);
            },
        }
    }

    fn dyn_if_check((key, dyn_id): &(LitStr, Ident)) -> TokenStream {
        let msg = format!("Expected Attribute For Property `{dyn_id}`");
        let msg = LitStr::new(msg.as_str(), key.span());

        quote! {
            reader.append_path(::parco_xml::de::AppendPath::NamedAttr(#key));

            let #dyn_id = match #dyn_id {
                ::core::option::Option::Some(some) => {
                    let parsed = reader.dexml(some)?;
                    reader.exit_path();
                    parsed
                },
                _ => {
                    return ::core::result::Result::Err(
                        reader.err(
                            ::std::borrow::Cow::Borrowed(
                                #msg
                            )
                        )
                    )
                }
            };
        }
    }
}
