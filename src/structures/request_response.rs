use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct RequestResponse {
	status_code: i32,
	success: bool,
}
