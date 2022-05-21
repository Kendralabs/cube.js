use crate::{sql::ColumnType, transport::CubeColumn};

pub trait CubeColumnPostgresExt {
    fn get_data_type(&self) -> String;
    fn get_udt_name(&self) -> String;
    fn get_is_nullable(&self) -> String;
    fn get_udt_schema(&self) -> String;
    fn get_numeric_precision(&self) -> Option<i32>;
    fn get_numeric_precision_radix(&self) -> Option<i32>;
    fn get_numeric_scale(&self) -> Option<i32>;
    fn get_datetime_precision(&self) -> Option<i32>;
    fn get_char_octet_length(&self) -> Option<i32>;
}

impl CubeColumnPostgresExt for CubeColumn {
    fn get_data_type(&self) -> String {
        match self.get_column_type() {
            ColumnType::String | ColumnType::VarStr => "text",
            ColumnType::Double => "numeric",
            ColumnType::Boolean => "boolean",
            ColumnType::Int8 | ColumnType::Int16 => "smallint",
            ColumnType::Int32 => "integer",
            ColumnType::Int64 => "bigint",
            ColumnType::Blob => "bytea",
            ColumnType::Timestamp => "timestamp without time zone",
            ColumnType::List(_) => "ARRAY",
        }
        .to_string()
    }

    fn get_udt_name(&self) -> String {
        match self.get_column_type() {
            ColumnType::String | ColumnType::VarStr => "text",
            ColumnType::Double => "float8",
            ColumnType::Boolean => "bool",
            ColumnType::Int8 | ColumnType::Int16 => "int2",
            ColumnType::Int32 => "int4",
            ColumnType::Int64 => "int8",
            ColumnType::Blob => "bytea",
            ColumnType::Timestamp => "timestamp",
            ColumnType::List(_) => "anyarray",
        }
        .to_string()
    }

    fn get_is_nullable(&self) -> String {
        if self.sql_can_be_null() {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    fn get_udt_schema(&self) -> String {
        return "pg_catalog".to_string();
    }

    fn get_numeric_precision(&self) -> Option<i32> {
        match self.get_column_type() {
            ColumnType::Double => Some(53),
            ColumnType::Int8 | ColumnType::Int16 => Some(16),
            ColumnType::Int32 => Some(32),
            ColumnType::Int64 => Some(64),
            _ => None,
        }
    }

    fn get_numeric_precision_radix(&self) -> Option<i32> {
        match self.get_column_type() {
            ColumnType::Double
            | ColumnType::Int8
            | ColumnType::Int16
            | ColumnType::Int32
            | ColumnType::Int64 => Some(2),
            _ => None,
        }
    }

    fn get_numeric_scale(&self) -> Option<i32> {
        match self.get_column_type() {
            ColumnType::Int8 | ColumnType::Int16 | ColumnType::Int32 | ColumnType::Int64 => Some(0),
            _ => None,
        }
    }

    fn get_datetime_precision(&self) -> Option<i32> {
        match self.get_column_type() {
            ColumnType::Timestamp => Some(6),
            _ => None,
        }
    }

    fn get_char_octet_length(&self) -> Option<i32> {
        match self.get_column_type() {
            ColumnType::String | ColumnType::VarStr => Some(1073741824),
            _ => None,
        }
    }
}
