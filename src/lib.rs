mod utils;

use wasm_bindgen::prelude::*;

use bincode::{deserialize, serialize};
use chrono::prelude::*;
use lz4_flex::block::{compress_prepend_size, decompress_size_prepended};
use lz_str::{
    compress, compress_to_base64, compress_to_encoded_uri_component, compress_to_utf16, decompress,
    decompress_from_base64, decompress_from_encoded_uri_component, decompress_from_utf16,
};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use std::str::FromStr;

#[derive(Display, Copy, Clone)]
#[wasm_bindgen]
pub enum CacherCompression {
    InvalidUtf16,
    Base64,
    ValidUtf16,
    Uri,
}

impl FromStr for CacherCompression {
    type Err = ();

    fn from_str(input: &str) -> Result<CacherCompression, Self::Err> {
        match input {
            "InvalidUtf16" => Ok(CacherCompression::InvalidUtf16),
            "ValidUtf16" => Ok(CacherCompression::ValidUtf16),
            "Base64" => Ok(CacherCompression::Base64),
            "Uri" => Ok(CacherCompression::Uri),
            _ => Err(()),
        }
    }
}

#[derive(Display, Copy, Clone)]
#[wasm_bindgen]
pub enum CacherDateRemainingUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl FromStr for CacherDateRemainingUnit {
    type Err = ();

    fn from_str(input: &str) -> Result<CacherDateRemainingUnit, Self::Err> {
        match input {
            "Milliseconds" => Ok(CacherDateRemainingUnit::Milliseconds),
            "Seconds" => Ok(CacherDateRemainingUnit::Seconds),
            "Minutes" => Ok(CacherDateRemainingUnit::Hours),
            "Hours" => Ok(CacherDateRemainingUnit::Hours),
            "Days" => Ok(CacherDateRemainingUnit::Days),
            "Weeks" => Ok(CacherDateRemainingUnit::Weeks),
            "Months" => Ok(CacherDateRemainingUnit::Months),
            "Years" => Ok(CacherDateRemainingUnit::Years),
            _ => Err(()),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CacherOptions {
    save_path: String,
    compression_type: CacherCompression,
    remaining_time_unit: CacherDateRemainingUnit,
    remaining_time: i64,
}

#[wasm_bindgen]
impl CacherOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            save_path: String::from(".cache"),
            compression_type: CacherCompression::ValidUtf16,
            remaining_time_unit: CacherDateRemainingUnit::Days,
            remaining_time: 5,
        };
    }

    #[wasm_bindgen(getter)]
    pub fn save_path(&self) -> String {
        self.save_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_save_path(&mut self, path: &str) {
        self.save_path = String::from_str(path).unwrap();
    }

    #[wasm_bindgen(getter)]
    pub fn compression_type(&self) -> CacherCompression {
        self.compression_type
    }

    #[wasm_bindgen(setter)]
    pub fn set_compression_type(&mut self, compress_type: CacherCompression) {
        self.compression_type = compress_type;
    }

    #[wasm_bindgen(getter)]
    pub fn remaining_time_unit(&self) -> CacherDateRemainingUnit {
        self.remaining_time_unit
    }

    #[wasm_bindgen(setter)]
    pub fn set_remaining_time_unit(&mut self, time_unit: CacherDateRemainingUnit) {
        self.remaining_time_unit = time_unit;
    }

    #[wasm_bindgen(getter)]
    pub fn remaining_time(&self) -> i64 {
        self.remaining_time
    }

    #[wasm_bindgen(setter)]
    pub fn set_remaining_time(&mut self, time: i64) {
        self.remaining_time = time;
    }
}

#[wasm_bindgen]
pub struct CacherReturnInfo {
    is_expired: bool,
    data: String,
}

#[wasm_bindgen]
impl CacherReturnInfo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            is_expired: false,
            data: String::new(),
        };
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> String {
        self.data.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn is_expired(&self) -> bool {
        self.is_expired
    }
}

#[derive(Serialize, Deserialize)]
struct CacherInfo {
    compression_type: String,
    creation_time: String,
    remaining_time_unit: String,
    remaning_time: i64,
    data: String,
}

fn str_compress(compression_type: CacherCompression, data: &str) -> String {
    let compressed = match compression_type {
        CacherCompression::InvalidUtf16 => String::from_utf16(&compress(data)).unwrap(),
        CacherCompression::ValidUtf16 => compress_to_utf16(data),
        CacherCompression::Base64 => compress_to_base64(data),
        CacherCompression::Uri => compress_to_encoded_uri_component(data),
    };

    return compressed;
}

fn str_decompress(compressed: &str, compressed_type: &str) -> String {
    let decompressed_type = CacherCompression::from_str(compressed_type).unwrap();

    let decompressed = match decompressed_type {
        CacherCompression::InvalidUtf16 => {
            String::from_utf16(&decompress(compressed).unwrap()).unwrap()
        }
        CacherCompression::ValidUtf16 => {
            String::from_utf16(&decompress_from_utf16(compressed).unwrap()).unwrap()
        }
        CacherCompression::Base64 => {
            String::from_utf16(&decompress_from_base64(compressed).unwrap()).unwrap()
        }
        CacherCompression::Uri => {
            String::from_utf16(&decompress_from_encoded_uri_component(compressed).unwrap()).unwrap()
        }
    };

    return decompressed;
}

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[wasm_bindgen]
pub fn get_compressed_cacher_info(options: &CacherOptions, data: &str) -> Vec<u8> {
    let info = CacherInfo {
        compression_type: options.compression_type.to_string(),
        creation_time: Utc::now().format(DATE_FORMAT).to_string(),
        remaining_time_unit: options.remaining_time_unit.to_string(),
        remaning_time: options.remaining_time,
        data: str_compress(options.compression_type, data),
    };

    let serialized = serialize(&info).unwrap();
    let compressed = compress_prepend_size(&serialized);

    return compressed;
}

fn check_if_expired(creation_time: &String, remaining_unit: &String, remaining_time: i64) -> bool {
    let now = Utc::now();

    let created_time = NaiveDateTime::parse_from_str(&creation_time, DATE_FORMAT).unwrap();

    let diff_time = created_time.signed_duration_since(now.naive_utc());

    let unit = CacherDateRemainingUnit::from_str(remaining_unit).unwrap();

    let time = match unit {
        CacherDateRemainingUnit::Milliseconds => diff_time.num_milliseconds(),
        CacherDateRemainingUnit::Seconds => diff_time.num_seconds(),
        CacherDateRemainingUnit::Minutes => diff_time.num_minutes(),
        CacherDateRemainingUnit::Hours => diff_time.num_hours(),
        CacherDateRemainingUnit::Days => diff_time.num_days(),
        CacherDateRemainingUnit::Weeks => diff_time.num_weeks(),
        CacherDateRemainingUnit::Months => diff_time.num_weeks() / 4,
        CacherDateRemainingUnit::Years => diff_time.num_weeks() / 12,
    };

    time >= remaining_time
}

#[wasm_bindgen]
pub fn get_decompressed_data(info: &[u8]) -> CacherReturnInfo {
    let decompressed = decompress_size_prepended(info).unwrap();
    let info_data: CacherInfo = deserialize(&decompressed).unwrap();

    let is_expired = check_if_expired(
        &info_data.creation_time,
        &info_data.remaining_time_unit,
        info_data.remaning_time,
    );

    let data = str_decompress(&info_data.data, &info_data.compression_type);

    return CacherReturnInfo { data, is_expired };
}
