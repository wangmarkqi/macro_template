use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Data, DataStruct,
parse,Meta,DeriveInput, Fields, LitStr, Token};

use quote::{quote, ToTokens};
use syn::spanned::Spanned;
mod tools;
use crate::tools::mytool::*;
use crate::tools::get_tokens::*;


#[proc_macro_derive(HelloWorld,attributes(foo,bar,wq))]
pub fn example(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields=get_fieldsNamed(&input);


    let field_name=get_field_names(&fields);
    let field_type=get_field_types(&fields);
    let field_namestr =get_field_namestr(&fields);
    let field_attrs=get_field_attrs(&fields);
    // 函数调用只能在quote外面，通过列表穿进去 #（）*

    // 上面代码需要把处理结果全部整理成vec，这样进入下面quote代码循环

    // 外面的变量进去quote！需要用#,现在看只能用列表，对应顺序

    gen_tokens(name,&field_name,&field_type,&field_namestr,&field_attrs)


}

