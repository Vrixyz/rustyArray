#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Data<T> {
    Value(T),
    Array(Vec<Data<T>>),
}

impl<T> Data<T> {
    fn flatten(self) -> Vec<T> {
        match self {
            Data::Value(v) => vec![v],
            Data::Array(arr) => {
                let mut result = vec![];
                for elem in arr {
                    result.append(&mut elem.flatten());
                }
                result
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: Data<i32>, _: lambda::Context) -> Result<Vec<i32>, HandlerError> {
    info!("lambda called with {:#?}", e);
    Ok(e.flatten())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn json() {
        let data = serde_json::from_str::<Data<i32>>("[1, [2, 3, 4], 5, 6, [7]]").expect("Invalid json.");
        assert_eq!(data.flatten(), vec![1, 2, 3, 4, 5, 6, 7]);
    }
    #[test]
    fn complex() {
        let data = Data::Array(vec![
            Data::Value(1),
            Data::Value(2),
            Data::Array(vec![
                Data::Value(3),
                Data::Array(vec![Data::Value(4), Data::Value(5)]),
            ]),
        ]);
        assert_eq!(data.flatten(), vec![1, 2, 3, 4, 5]);
    }
}
