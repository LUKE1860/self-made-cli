use std::env::Args;
pub struct Parser{
pub reader:Args,
}
impl Parser{
pub fn from(a:Args)->Parser{
Parser{reader:a}
}
#[inline (always)]
pub fn execute(self){
let message=["List of commands","+ add 2 numbers","- substract 2 numbers","* multiply 2 numbers","/ divide 2 numbers"];
let mut a:Vec<String>=self.reader.collect();
a.remove(0);
if a.contains(&"-h".to_string()){
println!("------------------------------");
for i in message{
println!("{i}");
} 
println!("------------------------------");
}
if a.contains(&"+".to_string()){
let num_1=a.get(1).unwrap();
let num_2=a.get(2).unwrap();
let parsed1=num_1.trim().parse::<f64>().unwrap();
let parsed2=num_2.trim().parse::<f64>().unwrap();
println!("{}",parsed1+parsed2);
}
else if a.contains(&"-".to_string()){
let b=a.clone();
let num_1=b.get(1).unwrap();
let num_2=b.get(2).unwrap();
let parsed1=num_1.trim().parse::<f64>().unwrap();
let parsed2=num_2.trim().parse::<f64>().unwrap();
println!("{}",parsed1-parsed2);
}
else if a.contains(&"*".to_string()){
let b=a.clone();
let num_1=b.get(1).unwrap();
let num_2=b.get(2).unwrap();
let parsed1=num_1.trim().parse::<f64>().unwrap();
let parsed2=num_2.trim().parse::<f64>().unwrap();
println!("{}",parsed1*parsed2);
}
else if a.contains(&"/".to_string()){
let b=a.clone();
let num_1=b.get(1).unwrap();
let num_2=b.get(2).unwrap();
let parsed1=num_1.trim().parse::<f64>().unwrap();
let parsed2=num_2.trim().parse::<f64>().unwrap();
println!("{}",parsed1/parsed2);
}
}
}