use crate::arrow::datatypes::{DataType, Field};
use bitflags::bitflags;
use msql_srv::{
    ColumnFlags as MysqlColumnFlags, ColumnType as MysqlColumnType, StatusFlags as MysqlStatusFlags,
};
use pg_srv::{protocol::CommandComplete, PgTypeId};

#[derive(Clone, PartialEq, Debug)]
pub enum ColumnType {
    String,
    VarStr,
    Double,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    Blob,
    Timestamp,
    List(Box<Field>),
}

impl ColumnType {
    pub fn to_mysql(&self) -> MysqlColumnType {
        match self {
            ColumnType::String => MysqlColumnType::MYSQL_TYPE_STRING,
            ColumnType::VarStr => MysqlColumnType::MYSQL_TYPE_VAR_STRING,
            ColumnType::Double => MysqlColumnType::MYSQL_TYPE_DOUBLE,
            ColumnType::Boolean => MysqlColumnType::MYSQL_TYPE_TINY,
            ColumnType::Int8 => MysqlColumnType::MYSQL_TYPE_TINY,
            ColumnType::Int16 => MysqlColumnType::MYSQL_TYPE_SHORT,
            ColumnType::Int32 => MysqlColumnType::MYSQL_TYPE_LONG,
            ColumnType::Int64 => MysqlColumnType::MYSQL_TYPE_LONGLONG,
            _ => MysqlColumnType::MYSQL_TYPE_BLOB,
        }
    }

    pub fn to_pg_tid(&self) -> PgTypeId {
        match self {
            ColumnType::Blob => PgTypeId::BYTEA,
            ColumnType::Boolean => PgTypeId::BOOL,
            ColumnType::Int8 | ColumnType::Int16 => PgTypeId::INT2,
            ColumnType::Int32 => PgTypeId::INT4,
            ColumnType::Int64 => PgTypeId::INT8,
            ColumnType::String | ColumnType::VarStr => PgTypeId::TEXT,
            ColumnType::Timestamp => PgTypeId::TIMESTAMP,
            ColumnType::Double => PgTypeId::NUMERIC,
            ColumnType::List(field) => match field.data_type() {
                DataType::Binary => PgTypeId::ARRAYBYTEA,
                DataType::Boolean => PgTypeId::ARRAYBOOL,
                DataType::Utf8 => PgTypeId::ARRAYTEXT,
                DataType::Int16 => PgTypeId::ARRAYINT2,
                DataType::Int32 => PgTypeId::ARRAYINT4,
                DataType::Int64 => PgTypeId::ARRAYINT8,
                dt => unimplemented!("Unsupported data type for List: {}", dt),
            },
        }
    }
}

bitflags! {
    pub struct ColumnFlags: u8 {
        const NOT_NULL  = 0b00000001;
        const UNSIGNED  = 0b00000010;
    }
}

impl ColumnFlags {
    pub fn to_mysql(&self) -> MysqlColumnFlags {
        MysqlColumnFlags::empty()
    }
}

bitflags! {
    pub struct StatusFlags: u8 {
        const SERVER_STATE_CHANGED = 0b00000001;
        const AUTOCOMMIT           = 0b00000010;
    }
}

impl StatusFlags {
    pub fn to_mysql_flags(&self) -> MysqlStatusFlags {
        MysqlStatusFlags::empty()
    }
}

#[derive(Debug, Clone)]
pub enum CommandCompletion {
    Begin,
    Commit,
    Use,
    Rollback,
    Set,
    Select(u32),
    Discard(String),
}

impl CommandCompletion {
    pub fn to_pg_command(self) -> CommandComplete {
        match self {
            CommandCompletion::Begin => CommandComplete::Plain("BEGIN".to_string()),
            CommandCompletion::Commit => CommandComplete::Plain("COMMIT".to_string()),
            CommandCompletion::Rollback => CommandComplete::Plain("ROLLBACK".to_string()),
            CommandCompletion::Set => CommandComplete::Plain("SET".to_string()),
            CommandCompletion::Use => CommandComplete::Plain("USE".to_string()),
            CommandCompletion::Select(rows) => CommandComplete::Select(rows),
            CommandCompletion::Discard(tp) => CommandComplete::Plain(format!("DISCARD {}", tp)),
        }
    }
}
