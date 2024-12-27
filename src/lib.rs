mod utils;

use wasm_bindgen::prelude::*;

use lz_str;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use strum_macros::Display;

#[derive(Display)]
#[wasm_bindgen]
pub enum CacherCompression {
    InvalidUtf16,
    Base64,
    ValidUtf16,
    Uri,
}

#[derive(Display)]
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

#[wasm_bindgen]
impl CacherOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            save_path: String::from(".cache"),
            compression_type: CacherCompression::Uri,
            remaining_time_unit: CacherDateRemainingUnit::Days,
            remaining_time: 5,
        };
    }
}

#[derive(Serialize)]
struct CacherInfo {
    compression_type: String,
    creation_time: String,
    remaining_time_unit: String,
    remaning_time: u32,
    data: String,
}

#[wasm_bindgen]
struct Cacher {
    options: CacherOptions,
    date_format: String,
}

use chrono::prelude::*;

#[wasm_bindgen]
impl Cacher {
    #[wasm_bindgen(constructor)]
    pub fn new(options: CacherOptions) -> Self {
        let date_format = "%Y-%m-%d %H:%M:%S";

        return Self {
            options,
            date_format: date_format.to_string(),
        };
    }

    pub fn save_file(self, data: String, bin_filepath: String) -> String {
        let now_str = Utc::now().format(&self.date_format).to_string();

        let info = CacherInfo {
            compression_type: self.options.compression_type.to_string(),
            creation_time: Utc::now().format(&self.date_format).to_string(),
            remaining_time_unit: self.options.remaining_time_unit.to_string(),
            remaning_time: self.options.remaining_time,
            data: lz_str::compress_to_encoded_uri_component(data.as_str()),
        };

        let json_data_str = to_string(&info).unwrap().to_string();

        return json_data_str;
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
