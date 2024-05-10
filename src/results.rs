use serde::{Deserialize, Serialize};

// search results are encapsulated in a "results" json object
#[derive(Debug, Deserialize, Serialize)]
pub struct Results<T> {
    pub results: Vec<T>,
}
