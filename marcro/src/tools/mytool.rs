use proc_macro::{TokenStream, Ident};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Data, DataStruct,
parse,Meta,DeriveInput, Fields, LitStr, Token,FieldsNamed,Type};
use std::collections::HashMap;

use quote::{quote, ToTokens};
use syn::spanned::Spanned;
pub fn get_fieldsNamed(input:&DeriveInput)->FieldsNamed{
            let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(ref fields),
        ..
    }) = input.data
    {
        fields.to_owned()
    } else {
        panic!("Only support Struct with fields")
    };
    fields
}
pub fn get_field_names(fields:&FieldsNamed)->Vec<Option<proc_macro2::Ident>>{
      let mut field_name = Vec::new();

    for field in fields.named.iter() {
        // let field_str = &field.ident.as_ref().unwrap().to_string();
        let ind=&field.ident;
        let ii=ind.clone();
        field_name.push(ii);
    }
    field_name
}
pub fn get_field_types(fields:&FieldsNamed)->Vec<Type>{
    let mut field_type = Vec::new();

    for field in fields.named.iter() {
        let field_str = &field.ident.as_ref().unwrap().to_string();
        let t=&field.ty;
        let tt=t.clone();
        field_type.push(tt);
    }
    field_type
}
pub fn get_field_namestr(fields:&FieldsNamed)->Vec<String>{
      let mut field_namestr = Vec::new();

    for field in fields.named.iter() {
        let field_str = &field.ident.as_ref().unwrap().to_string();
        let s=field_str.to_owned();
        field_namestr.push(s);
    }
    field_namestr
}
pub fn get_field_attrs(fields:&FieldsNamed)->Vec<String>{
      let mut field_attrs =Vec::new();


    for field in fields.named.iter(){
        let field_str = &field.ident.as_ref().unwrap().to_string();
        let s=field_str.to_owned();
        let mut this_attrs=vec![];
        // meta有三种形式，使用最简单的可以变成字符串
        let mut attrs=&field.attrs;
        for attr in attrs.iter(){
            let meta=attr.parse_meta().unwrap();
            match meta{
                Meta::List(b)=>continue,
                Meta::NameValue(b)=>continue,
                Meta::Path(b)=>{
                    if let Some(bb)=b.get_ident(){
                        let b3=bb.to_string();
                        this_attrs.push(b3);
                    }
                },
            }
        }
        let s=this_attrs.join("&&");
        field_attrs.push(s);
    }
    field_attrs
}
