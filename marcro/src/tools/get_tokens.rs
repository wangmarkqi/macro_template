use proc_macro::{TokenStream};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Data, DataStruct,
parse,Meta,DeriveInput, Fields, LitStr, Token,FieldsNamed,Type};
use std::collections::HashMap;
use quote::{quote, ToTokens};
pub fn gen_tokens(name:&Ident,field_name:&Vec<Option<Ident>>,field_type:&Vec<Type>,field_namestr:&Vec<String>,field_attrs:&Vec<String>)->TokenStream{

     let expanded = quote! {
        impl HelloWorld for #name {
            fn say(& mut self) -> () {

     #(

     // 读写特定field的value,太牛了
     if #field_namestr=="name"{
        let myv=&self.#field_name;
        dbg!(&myv);
        self.name="4444".to_string();
     };

     // 读attr,结合上面的，可以根据macro 的attri返回特定字段值，这个很有用
     let attrs=#field_attrs;
     if attrs.contains("bar"){
        dbg!(#field_namestr);
     }

     println!("{}=={}=={}={}",stringify!(#field_name),stringify!(#field_type),#field_namestr,#field_attrs);


     )*

       println!("Hello, {}!", stringify!(#name));
            }
        }
    };
    TokenStream::from(expanded)
}
