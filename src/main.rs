use std::env;
use crate::components::Parser;
pub mod components;
fn main(){
let a=Parser::from(env::args());
a.execute();
}
