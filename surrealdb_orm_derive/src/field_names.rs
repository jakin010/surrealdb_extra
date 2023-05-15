use syn::{Data, DeriveInput, Error, Fields};
use syn::__private::Span;

pub(crate) fn get_field_names(input: &DeriveInput) -> Result<String, Error> {

    let data = &input.data;

    let struct_fields = match data {
        Data::Struct(ds) => &ds.fields,
        _ => { return Err(Error::new(Span::call_site(), "Type is not a struct"));  }
    };

    let fields_named = match struct_fields {
        Fields::Named(fields_named) => &fields_named.named,
        _ => { return Err(Error::new(Span::call_site(), "Type is not a struct"));  }
    };

    let mut fields: String = String::new();
    for field in fields_named {
        fields.push_str(&field.ident.clone().unwrap().to_string());
        fields.push(',');
    }

    if fields.chars().last().unwrap() == ',' {
        fields.pop();
    }

    Ok(fields)
}
