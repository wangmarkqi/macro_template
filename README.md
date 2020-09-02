# Rust_macro_tutorial rust宏实用教程 

rust的macro可以让项目的api变得非常简洁，但是宏上手难度大，难以调试，英文资料有一些，但是看完可能仍然没有到敢于在实践中使用的程度。本作者通过查阅大量资料，通过一个例程，从实战角度描述rust procedural maocro中的derive macro如何使用。请注意：这仅仅是一个rust宏的使用套路，但是这个套路基本可以在应用层面应对大多数业务逻辑,从而放心的开始使用宏。现将这个套路几个要点简介如下，读者仍然需要研读本项目代码（好在也不多，留下的全是精华）。

## rust宏的分类：
从大类上分为decalare macro 和procedural macro。其中后者又可以分为#[proc_macro_attribute]，#[proc_macro_derive()]，#[proc_macro]。每类的入参不一样，从实战角度，derive 宏+attributes最为实用（个人观点，不解释），所以本hub后面全部说的都是derive宏+attributes。
## 什么叫带特征的derive宏：
就长这样：
```rust
#[derive(HelloWorld)]
struct Hello{
    #[foo]
    #[wq]
    name: String,
    #[bar]
    age:i64,
}
```
## 这东西有啥用：
看看serde，actix web，rocket啥的，一句话，让api简洁的超乎你的想象。

## 写宏的时候主要要解决的问题：
以上面的例子，首先，要把Hello给读出来吧，接着，要把里面的字段名称和字段类型读出来吧，结合trait，也可以把运行时字段值读出来，再有，下面那些foo，bar啥的，要把内容也读出来。读出来干啥？这个不能问我，要看你的api设计。比如你做一个字段要算库存，但是不知道调用的时候哪个字段名称是库存重量，于是可以用一个#【weight】来特征一下重量字段，然后在trait里面来个方法tell_me_weight啥的，这样调用代码一个#【weight】就啥都明白了。
## 如何写宏：
 - 需要使用syn，quote的包，可能使用proc_macro2，其中和proc_macro的tokenstream至少在quote包里面不兼容；
 - 需要在项目中继续cargo new --lib一个项目（如本项目macro子文件夹），proc-macro = true，由于这个crate无法导出宏以外的trait等，derive macro的trait只能接着再起一个crate（如本项目macro_trait)
 - 大概步骤就是2个，如本项目macro文件下的lib.rs，首先使用syn把结构体信息，以及attribute信息解析出来，然后使用quote把代码宏展开返回去。
 
 ## 其它
 - 解析的结构体,主要是field的name，type等，需要用列表装起来，在quote中，无法for in，使用#（）*方式循环，只针对列表，
 - attributes通过[parse_meta](https://docs.rs/syn/1.0.35/syn/struct.Attribute.html#method.parse_meta)后，[meta有三种形式](https://docs.rs/syn/1.0.35/syn/enum.Meta.html)，本文使用[#xxxx]作为attrs，感觉在应用层面够了，其他没有研究，这个形式很容易解析成hashmap返回。
 - quote外面根据业务逻辑处理的结果要搞成vec，好比解析结构体name，type一样，这样才可以在quote中用，quote中是无法使用外面函数的。
 - 查看宏展开有一个工具，cargo expand，不过那个看起来也吃力（你试试就知道了），我这项目结构，直接和运行时代码一块run，感觉更爽一些。


