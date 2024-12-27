mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum CacherCompression {
    InvalidUtf16,
    Base64,
    ValidUtf16,
    Uri,
}

#[wasm_bindgen]
pub enum CacherDateRemainingUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Monts,
    Years,
}

#[wasm_bindgen]
struct CacherOptions {
    save_path: String,
    compression_type: CacherCompression,
    remaining_time_unit: CacherDateRemainingUnit,
    remaining_time: u32,
}

struct CacherInfo {
    compression_type: String,
    creation_time: String,
    remaining_time_unit: String,
    remaning_time: u32,
    data: String,
}

struct Cacher {
    options: CacherOptions,
    date_format: String,
}

use chrono::prelude::*;

impl Cacher {
    fn new(options: CacherOptions) -> Cacher {
        let date_format = "%Y-%m-%d %H:%M:%S";

        return Cacher {
            options,
            date_format: date_format.to_string(),
        };
    }

    fn save_file(self, data: String, bin_filepath: String) -> String {
        let now_str = Utc::now().format(&self.date_format).to_string();

        return now_str;
    }
}

#[wasm_bindgen]
pub fn greet() -> String {
    let cacher_options = CacherOptions {
        save_path: String::from("./static"),
        compression_type: CacherCompression::ValidUtf16,
        remaining_time_unit: CacherDateRemainingUnit::Days,
        remaining_time: 5,
    };

    let cacher = Cacher::new(cacher_options);

    return cacher.save_file(String::from(""), String::from(""));
}
