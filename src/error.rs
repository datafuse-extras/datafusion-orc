use std::io;
use std::string::FromUtf8Error;

use arrow::datatypes::DataType as ArrowDataType;
use arrow::datatypes::TimeUnit;
use arrow::error::ArrowError;
use snafu::prelude::*;
use snafu::Location;

use crate::proto;
use crate::proto::r#type::Kind;
use crate::schema::DataType;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum OrcError {
    #[snafu(display("Failed to seek, source: {}", source))]
    SeekError {
        source: std::io::Error,
        location: Location,
    },

    #[snafu(display("Failed to read, source: {}", source))]
    IoError {
        source: std::io::Error,
        location: Location,
    },

    #[snafu(display("Empty file"))]
    EmptyFile { location: Location },

    #[snafu(display("Invalid input, message: {}", msg))]
    InvalidInput { msg: String, location: Location },

    #[snafu(display("Out of spec, message: {}", msg))]
    OutOfSpec { msg: String, location: Location },

    #[snafu(display("Error from map builder: {}", source))]
    MapBuilder {
        source: arrow::error::ArrowError,
        location: Location,
    },

    #[snafu(display("Failed to create new string builder: {}", source))]
    StringBuilder {
        source: arrow::error::ArrowError,
        location: Location,
    },

    #[snafu(display("Failed to decode float, source: {}", source))]
    DecodeFloat {
        location: Location,
        source: std::io::Error,
    },

    #[snafu(display(
        "Overflow while decoding timestamp (seconds={}, nanoseconds={}) to {:?}",
        seconds,
        nanoseconds,
        to_time_unit,
    ))]
    DecodeTimestamp {
        location: Location,
        seconds: i64,
        nanoseconds: u64,
        to_time_unit: TimeUnit,
    },

    #[snafu(display("Failed to decode proto, source: {}", source))]
    DecodeProto {
        location: Location,
        source: prost::DecodeError,
    },

    #[snafu(display("No types found"))]
    NoTypes { location: Location },

    #[snafu(display("unsupported type: {:?}", kind))]
    UnsupportedType { location: Location, kind: Kind },

    #[snafu(display("unsupported type variant: {}", msg))]
    UnsupportedTypeVariant {
        location: Location,
        msg: &'static str,
    },

    #[snafu(display("Field not found: {:?}", name))]
    FieldNotFound { location: Location, name: String },

    #[snafu(display("Invalid column : {:?}", name))]
    InvalidColumn { location: Location, name: String },

    #[snafu(display(
        "Cannot decode ORC type {:?} into Arrow type {:?}",
        orc_type,
        arrow_type,
    ))]
    MismatchedSchema {
        location: Location,
        orc_type: DataType,
        arrow_type: ArrowDataType,
    },

    #[snafu(display("Invalid encoding for column '{}': {:?}", name, encoding))]
    InvalidColumnEncoding {
        location: Location,
        name: String,
        encoding: proto::column_encoding::Kind,
    },

    #[snafu(display("Failed to add day to a date"))]
    AddDays { location: Location },

    #[snafu(display("Invalid utf8, source: {}", source))]
    InvalidUft8 {
        location: Location,
        source: FromUtf8Error,
    },

    #[snafu(display("Out of bound at: {}", index))]
    OutOfBound { location: Location, index: usize },

    #[snafu(display("Failed to convert to record batch: {}", source))]
    ConvertRecordBatch {
        location: Location,
        source: ArrowError,
    },

    #[snafu(display("Varint being decoded is too large"))]
    VarintTooLarge { location: Location },

    #[snafu(display("unexpected: {}", msg))]
    Unexpected { location: Location, msg: String },

    #[snafu(display("Failed to build zstd decoder: {}", source))]
    BuildZstdDecoder {
        location: Location,
        source: io::Error,
    },

    #[snafu(display("Failed to build snappy decoder: {}", source))]
    BuildSnappyDecoder {
        location: Location,
        source: snap::Error,
    },

    #[snafu(display("Failed to build lzo decoder: {}", source))]
    BuildLzoDecoder {
        location: Location,
        source: lzokay_native::Error,
    },

    #[snafu(display("Failed to build lz4 decoder: {}", source))]
    BuildLz4Decoder {
        location: Location,
        source: lz4_flex::block::DecompressError,
    },

    #[snafu(display("Arrow error: {}", source))]
    Arrow {
        source: arrow::error::ArrowError,
        location: Location,
    },
}

pub type Result<T, E = OrcError> = std::result::Result<T, E>;

impl From<OrcError> for ArrowError {
    fn from(value: OrcError) -> Self {
        ArrowError::ExternalError(Box::new(value))
    }
}
