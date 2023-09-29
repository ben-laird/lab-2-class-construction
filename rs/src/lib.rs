use utils::{cin, cout};

pub mod utils;

pub struct Dimensions {
    pub length: i32,
    pub width: i32,
    pub height: i32,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            length: 0,
            width: 0,
            height: 0,
        }
    }
}

impl Dimensions {
    pub fn calc_volume(&self) -> f64 {
        calc_volume(self)
    }
}

pub fn input_dimension(message: &str, err_message: Option<&str>) -> i32 {
    cout(message);

    let input = cin().parse::<i32>();

    match input {
        Ok(val) => val,
        Err(e) => {
            cout(&e.to_string());
            cout(err_message.unwrap_or_default());

            input_dimension(message, err_message)
        }
    }
}

fn calc_volume_raw(length: &i32, width: &i32, height: &i32) -> f64 {
    ((length + width) * height) as f64 / 200.0
}
fn calc_volume(d: &Dimensions) -> f64 {
    let Dimensions {
        length,
        width,
        height,
    } = d;

    calc_volume_raw(length, width, height)
}
