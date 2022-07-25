#![allow(dead_code)]
#[derive(Debug)]
pub struct  Point<T>{
    x:T,
    y:T,
}
#[derive(Debug)]
pub struct Point2<T,U>{
    x:T,
    y:U,

}

enum Option<T>{
    Some(T),
    None,
}

enum Result<T,E>{
    Ok(T),
    Err(E),
}

pub fn struct_t(){
    let s1 = Point{x:32,y:64};
    print!("the Point s1 is: {:#?}",s1);

    let s2 = Point{x:32.09,y:64.89};
    print!("the Point s2 is: {:#?}",s2);

    let s3 = Point2{x:32.09,y:'c'};
    print!("the Point s2 is: {:#?}",s3);
}