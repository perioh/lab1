use std::fmt::Display;
use std::fs::File;
use std::io::read_to_string;
fn main() {
    let file = File::open("toys.txt").expect("File is absent");

    let mut toys = read_to_string(file)
        .expect("File reading error")
        .split('\n')
        .map(|line| Toy::try_from(line.to_owned()).expect("Error while deserializing file line"))
        .collect::<Vec<Toy>>();

    toys.sort_by(|a, b| a.price.cmp(&b.price));

    toys.iter()
        .filter(|toy| toy.name == "constructor")
        .for_each(|toy| println!("{} ({}) - {}", toy.name, toy.age, Price::from(toy.price)));
}

#[allow(unused)]
#[derive(Debug)]
struct Toy {
    name: String,
    specific: ToySpecific,
    price: usize,
    age: String,
}

#[allow(unused)]
#[derive(Debug)]
enum ToySpecific {
    Doll { height: usize },
    Ball { weight: usize },
    Bricks { amount: usize },
    Constructor { amount: usize },
}

#[allow(unused)]
#[derive(Debug)]
enum ToyIntoError {
    NotEnoughtInputFields,
    ErrorParsing { error: String, input: String },
}

impl TryFrom<String> for Toy {
    type Error = ToyIntoError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut splited_data = value.split(",");
        let (toy_name, toy_price, toy_age, toy_specific) = (
            splited_data
                .next()
                .ok_or(Self::Error::NotEnoughtInputFields)?,
            splited_data
                .next()
                .ok_or(Self::Error::NotEnoughtInputFields)?,
            splited_data
                .next()
                .ok_or(Self::Error::NotEnoughtInputFields)?,
            splited_data
                .next()
                .ok_or(Self::Error::NotEnoughtInputFields)?,
        );
        let toy_specific = match &*toy_name.to_lowercase() {
            "doll" => ToySpecific::Doll {
                height: toy_specific
                    .parse::<usize>()
                    .map_err(|e| Self::Error::ErrorParsing {
                        error: e.to_string(),
                        input: toy_specific.to_owned(),
                    })?,
            },
            "constructor" => ToySpecific::Constructor {
                amount: toy_specific
                    .parse::<usize>()
                    .map_err(|e| Self::Error::ErrorParsing {
                        error: e.to_string(),
                        input: toy_specific.to_owned(),
                    })?,
            },
            "ball" => ToySpecific::Ball {
                weight: toy_specific
                    .parse::<usize>()
                    .map_err(|e| Self::Error::ErrorParsing {
                        error: e.to_string(),
                        input: toy_specific.to_owned(),
                    })?,
            },
            "bricks" => ToySpecific::Bricks {
                amount: toy_specific
                    .parse::<usize>()
                    .map_err(|e| Self::Error::ErrorParsing {
                        error: e.to_string(),
                        input: toy_specific.to_owned(),
                    })?,
            },
            _ => {
                unimplemented!()
            }
        };
        Ok(Toy {
            name: toy_name.to_owned(),
            specific: toy_specific,
            price: toy_price
                .parse::<usize>()
                .map_err(|e| Self::Error::ErrorParsing {
                    input: toy_price.to_owned(),
                    error: e.to_string(),
                })?,
            age: toy_age.to_owned(),
        })
    }
}

struct Price {
    hrn: usize,
    cop: usize,
}

impl From<usize> for Price {
    fn from(value: usize) -> Self {
        let cop = value % 100;
        let hrn = (value - cop) / 100;
        Price { hrn, cop }
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}грн {}коп", self.hrn, self.cop)
    }
}
