#[derive(Debug)]
pub struct PgType<'a> {
    pub oid: i32,
    pub typname: &'a str,
    pub typnamespace: i32,
    pub typowner: i32,
    pub typlen: i16,
    pub typbyval: bool,
    pub typtype: &'a str,
    pub typcategory: &'a str,
    pub typisprefered: bool,
    pub typisdefined: bool,
    pub typrelid: i32,
    pub typsubscript: &'static str,
    pub typelem: i32,
    pub typarray: i32,
    pub typalign: &'static str,
    pub typstorage: &'static str,
    pub typbasetype: i32,
    pub typreceive: &'static str,
}

impl PgType<'_> {
    pub fn typreceive_oid(&self) -> i32 {
        // TODO: Get from pg_proc
        match self.typreceive {
            "array_recv" => 2400,
            "boolrecv" => 2436,
            "float4recv" => 2424,
            "float8recv" => 2426,
            "int2recv" => 2404,
            "int4recv" => 2406,
            "int8recv" => 2408,
            "numeric_recv" => 2460,
            "textrecv" => 2414,
            "timestamp_recv" => 2474,
            "varcharrecv" => 2432,
            _ => 0,
        }
    }
}

macro_rules! define_pg_types {
    ($($NAME:ident ($OID:expr) { $($KEY:ident: $VALUE:expr,)* },)*) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u32)]
        pub enum PgTypeId {
            UNSPECIFIED = 0,
            $($NAME = $OID,)*
        }

        impl PgTypeId {
            pub fn from_oid(oid: i32) -> Option<Self> {
                match oid {
                    0 => Some(Self::UNSPECIFIED),
                    $($OID => Some(Self::$NAME),)*
                    _ => None,
                }
            }
        }

        impl<'a> PgType<'a> {
            pub fn get_by_tid(oid: PgTypeId) -> &'static PgType<'static> {
                match oid {
                    PgTypeId::UNSPECIFIED => UNSPECIFIED,
                    $(PgTypeId::$NAME => $NAME,)*
                }
            }

            pub fn get_all() -> Vec<&'static PgType<'static>> {
                vec![
                    $($NAME,)*
                ]
            }
        }

        $(
            const $NAME: &PgType = &PgType {
                oid: PgTypeId::$NAME as i32,
                $($KEY: $VALUE,)*
            };
        )*
    }
}

const UNSPECIFIED: &PgType = &PgType {
    oid: 0,
    typname: "unspecified",
    typnamespace: 11,
    typowner: 10,
    typlen: 1,
    typbyval: true,
    typtype: "b",
    typcategory: "B",
    typisprefered: true,
    typisdefined: true,
    typrelid: 0,
    typsubscript: "-",
    typelem: 0,
    typarray: 0,
    typalign: "-",
    typstorage: "-",
    typbasetype: 0,
    typreceive: "-",
};

define_pg_types![
    BOOL (16) {
        typname: "bool",
        typnamespace: 11,
        typowner: 10,
        typlen: 1,
        typbyval: true,
        typtype: "b",
        typcategory: "B",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "c",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "boolrecv",
    },

    BYTEA (17) {
        typname: "bytea",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "U",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "bytearecv",
    },

    NAME (19) {
        typname: "name",
        typnamespace: 11,
        typowner: 10,
        typlen: 64,
        typbyval: false,
        typtype: "b",
        typcategory: "S",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "raw_array_subscript_handler",
        typelem: 0,
        typarray: 0,
        typalign: "c",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "namerecv",
    },

    INT8 (20) {
        typname: "int8",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "int8recv",
    },

    INT2 (21) {
        typname: "int2",
        typnamespace: 11,
        typowner: 10,
        typlen: 2,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "s",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "int2recv",
    },

    INT4 (23) {
        typname: "int4",
        typnamespace: 11,
        typowner: 10,
        typlen: 4,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "int4recv",
    },

    TEXT (25) {
        typname: "text",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "S",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "textrecv",
    },

    OID (26) {
        typname: "oid",
        typnamespace: 11,
        typowner: 10,
        typlen: 4,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1028,
        typalign: "i",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "oidrecv",
    },

    TID (27) {
        typname: "tid",
        typnamespace: 11,
        typowner: 10,
        typlen: 6,
        typbyval: false,
        typtype: "b",
        typcategory: "U",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1010,
        typalign: "s",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "tidrecv",
    },

    PGCLASS (83) {
        typname: "pg_class",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "c",
        typcategory: "C",
        typisprefered: false,
        typisdefined: true,
        typrelid: 1259,
        typsubscript: "-",
        typelem: 0,
        typarray: 273,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "record_recv",
    },

    FLOAT4 (700) {
        typname: "float4",
        typnamespace: 11,
        typowner: 10,
        typlen: 4,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1021,
        typalign: "i",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "float4recv",
    },

    FLOAT8 (701) {
        typname: "float8",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1022,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "float8recv",
    },

    MONEY (790) {
        typname: "money",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "N",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 791,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "cash_recv",
    },

    INET (869) {
        typname: "inet",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "I",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1041,
        typalign: "i",
        typstorage: "m",
        typbasetype: 0,
        typreceive: "inet_recv",
    },

    ARRAYBOOL (1000) {
        typname: "_bool",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 16,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYBYTEA (1001) {
        typname: "_bytea",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 17,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYINT2 (1005) {
        typname: "_int2",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 21,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYINT4 (1007) {
        typname: "_int4",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 23,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYTEXT (1009) {
        typname: "_text",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 25,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYINT8 (1016) {
        typname: "_int8",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 20,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYFLOAT4 (1021) {
        typname: "_float4",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 700,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    ARRAYFLOAT8 (1022) {
        typname: "_float8",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "A",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "array_subscript_handler",
        typelem: 701,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "array_recv",
    },

    BPCHAR (1042) {
        typname: "bpchar",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "S",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1014,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "bpcharrecv",
    },

    VARCHAR (1043) {
        typname: "varchar",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "S",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "varcharrecv",
    },

    DATE (1082) {
        typname: "date",
        typnamespace: 11,
        typowner: 10,
        typlen: 4,
        typbyval: true,
        typtype: "b",
        typcategory: "D",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "date_recv",
    },

    TIME (1083) {
        typname: "time",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "D",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1183,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "time_recv",
    },

    TIMESTAMP (1114) {
        typname: "timestamp",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "D",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "timestamp_recv",
    },

    TIMESTAMPTZ (1184) {
        typname: "timestamptz",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "D",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "timestamptz_recv",
    },

    INTERVAL (1186) {
        typname: "interval",
        typnamespace: 11,
        typowner: 10,
        typlen: 16,
        typbyval: false,
        typtype: "b",
        typcategory: "T",
        typisprefered: true,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1187,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "interval_recv",
    },

    TIMETZ (1266) {
        typname: "timetz",
        typnamespace: 11,
        typowner: 10,
        typlen: 12,
        typbyval: false,
        typtype: "b",
        typcategory: "D",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 1270,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "timetz_recv",
    },

    NUMERIC (1700) {
        typname: "numeric",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "b",
        typcategory: "N",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "m",
        typbasetype: 0,
        typreceive: "numeric_recv",
    },

    RECORD (2249) {
        typname: "record",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "p",
        typcategory: "P",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 2287,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "record_recv",
    },

    ANYARRAY (2277) {
        typname: "anyarray",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "p",
        typcategory: "P",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "anyarray_recv",
    },

    ANYELEMENT (2283) {
        typname: "anyelement",
        typnamespace: 11,
        typowner: 10,
        typlen: 4,
        typbyval: true,
        typtype: "p",
        typcategory: "P",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "-",
    },

    INT4RANGE (3904) {
        typname: "int4range",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "range_recv",
    },

    NUMRANGE (3906) {
        typname: "numrange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "range_recv",
    },

    TSRANGE (3908) {
        typname: "tsrange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "range_recv",
    },

    PGLSN (3220) {
        typname: "pg_lsn",
        typnamespace: 11,
        typowner: 10,
        typlen: 8,
        typbyval: true,
        typtype: "b",
        typcategory: "U",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 3221,
        typalign: "d",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "pg_lsn_recv",
    },

    ANYENUM (3500) {
        typname: "anyenum",
        typnamespace: 11,
        typowner: 10,
        typlen: 4,
        typbyval: true,
        typtype: "p",
        typcategory: "P",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "p",
        typbasetype: 0,
        typreceive: "-",
    },

    ANYRANGE (3831) {
        typname: "anyrange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "p",
        typcategory: "P",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "-",
    },

    TSTZRANGE (3910) {
        typname: "tstzrange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "range_recv",
    },

    DATERANGE (3912) {
        typname: "daterange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "range_recv",
    },

    INT8RANGE (3926) {
        typname: "int8range",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "range_recv",
    },

    NUMMULTIRANGE (4532) {
        typname: "nummultirange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "m",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "multirange_recv",
    },

    TSMULTIRANGE (4533) {
        typname: "tsmultirange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "m",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "multirange_recv",
    },

    DATEMULTIRANGE (4535) {
        typname: "datemultirange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "m",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "multirange_recv",
    },

    INT8MULTIRANGE (4536) {
        typname: "int8multirange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "m",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "d",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "multirange_recv",
    },

    INT4MULTIRANGE (4451) {
        typname: "int4multirange",
        typnamespace: 11,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "r",
        typcategory: "R",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 0,
        typreceive: "multirange_recv",
    },

    CHARACTERDATA (13408) {
        typname: "character_data",
        typnamespace: 13000,
        typowner: 10,
        typlen: -1,
        typbyval: false,
        typtype: "d",
        typcategory: "S",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "i",
        typstorage: "x",
        typbasetype: 1043,
        typreceive: "domain_recv",
    },

    SQLIDENTIFIER (13410) {
        typname: "sql_identifier",
        typnamespace: 13000,
        typowner: 10,
        typlen: 64,
        typbyval: false,
        typtype: "d",
        typcategory: "S",
        typisprefered: false,
        typisdefined: true,
        typrelid: 0,
        typsubscript: "-",
        typelem: 0,
        typarray: 0,
        typalign: "c",
        typstorage: "p",
        typbasetype: 19,
        typreceive: "domain_recv",
    },
];

impl PgTypeId {
    pub fn to_type(self) -> &'static PgType<'static> {
        PgType::get_by_tid(self)
    }
}
