mod utils;

use wasm_bindgen::prelude::*;

use bincode::{deserialize, serialize};
use chrono::prelude::*;
use lz4_flex::block::{compress_prepend_size, decompress_size_prepended};
use lz_str;
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
pub struct CacherReturnInfo {
    is_expired: bool,
    data: String,
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

#[derive(Serialize, Deserialize)]
struct CacherInfo {
    compression_type: String,
    creation_time: String,
    remaining_time_unit: String,
    remaning_time: i64,
    data: String,
}

#[wasm_bindgen]
struct Cacher {
    options: CacherOptions,
    date_format: String,
}

#[wasm_bindgen]
impl Cacher {
    #[wasm_bindgen(constructor)]
    pub fn new(options: &CacherOptions) -> Self {
        let date_format = "%Y-%m-%d %H:%M:%S";

        let new_options = options.clone();

        return Self {
            options: new_options,
            date_format: date_format.to_string(),
        };
    }

    fn str_compress(&self, data: &str) -> String {
        let compressed = match self.options.compression_type {
            CacherCompression::InvalidUtf16 => String::from_utf16(&lz_str::compress(data)).unwrap(),
            CacherCompression::ValidUtf16 => lz_str::compress_to_utf16(data),
            CacherCompression::Base64 => lz_str::compress_to_base64(data),
            CacherCompression::Uri => lz_str::compress_to_encoded_uri_component(data),
        };

        return compressed;
    }

    fn str_decompress(&self, compressed: &str, compressed_type: &str) -> String {
        let decompressed_type = CacherCompression::from_str(compressed_type).unwrap();

        let decompressed = match decompressed_type {
            CacherCompression::InvalidUtf16 => {
                String::from_utf16(&lz_str::decompress(compressed).unwrap()).unwrap()
            }
            CacherCompression::ValidUtf16 => {
                String::from_utf16(&lz_str::decompress_from_utf16(compressed).unwrap()).unwrap()
            }
            CacherCompression::Base64 => {
                String::from_utf16(&lz_str::decompress_from_base64(compressed).unwrap()).unwrap()
            }
            CacherCompression::Uri => String::from_utf16(
                &lz_str::decompress_from_encoded_uri_component(compressed).unwrap(),
            )
            .unwrap(),
        };

        return decompressed;
    }

    pub fn get_compressed_info(&self, data: &str) -> Vec<u8> {
        let info = CacherInfo {
            compression_type: self.options.compression_type.to_string(),
            creation_time: Utc::now().format(&self.date_format).to_string(),
            remaining_time_unit: self.options.remaining_time_unit.to_string(),
            remaning_time: self.options.remaining_time,
            data: self.str_compress(data),
        };

        let serialized = serialize(&info).unwrap();
        let compressed = compress_prepend_size(&serialized);

        return compressed;
    }

    fn check_if_expired(
        &self,
        creation_time: &String,
        remaining_unit: &String,
        remaining_time: i64,
    ) -> bool {
        let now = Utc::now();

        let created_time =
            NaiveDateTime::parse_from_str(&creation_time, &self.date_format).unwrap();

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

    pub fn get_data(&self, info: &[u8]) -> CacherReturnInfo {
        let decompressed = decompress_size_prepended(info).unwrap();
        let info_data: CacherInfo = deserialize(&decompressed).unwrap();

        let is_expired = self.check_if_expired(
            &info_data.creation_time,
            &info_data.remaining_time_unit,
            info_data.remaning_time,
        );

        let data = self.str_decompress(&info_data.data, &info_data.compression_type);

        return CacherReturnInfo { data, is_expired };
    }
}
