#[macro_use] extern crate marcro;
use marcro_trait::{HelloWorld};

#[derive(HelloWorld)]
struct Hello{
    #[foo]
    #[wq]
    name: String,
    #[bar]
    age:i64,
}


fn main() {

    let mut res=Hello{
        name:"111".to_string(),
        age:8
    };
    res.say();
    dbg!(res.name);
}
