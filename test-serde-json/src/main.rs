#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: [u8;2],
    address: Address,
    phones: Vec<String>,
    phy_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main(){
    let data = r#" {
     "name": "John Doe", "age": [29,30],
     "address": {"street": "main", "city":"Downtown"},
     "phones":["27726550023"],
     "phy_addr":"abcd"
    } "#;
    let p: Person = serde_json::from_str(data).expect("deserialize error");
    println!("Please call {} at the number {}", p.name, p.phones[0]);
    println!("age: {},{}",p.age[0],p.age[1]);
    println!("physic address {:08X}", u32::from_str_radix(&p.phy_addr,16).unwrap());
    println!("{:#?}",p);
}
