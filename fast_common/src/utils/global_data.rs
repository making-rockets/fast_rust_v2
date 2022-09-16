use serde::{Deserializer, Deserialize};


#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrU64 {
    String(String),
    U64(u64),
}

#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrI64 {
    String(String),
    I64(i64),
}

#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrF64 {
    String(String),
    F64(f64),
}


#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrF32 {
    String(String),
    F32(f32),
}

#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrBool {
    String(String),
    I64(i64),
    Bool(bool),
}

pub fn bool_from_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error> where D: Deserializer<'de> {


    Ok(None)
}