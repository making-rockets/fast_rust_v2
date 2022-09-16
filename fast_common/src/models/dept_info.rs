// use serde::Serialize;
// use serde::Deserialize;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use chrono::NaiveDateTime;
use rbatis::Rbatis;
use crate::models::MyResult;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeptInfo {
    pub dept_id: Option<i64>,
    pub pid: Option<i64>,
    pub sub_count: Option<i32>,
    pub name: Option<String>,
    pub dept_sort: Option<i32>,
    #[serde(default)]
    // #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
rbatis::crud!(DeptInfo{},"dept");

impl DeptInfo {
    pub async fn from_id(rb: &mut Rbatis, dept_id: i64) -> anyhow::Result<u64> {
        let result = DeptInfo::select_by_column(rb, "dept_id", dept_id).await;
        let t = MyResult(result);
        Ok(1)
    }
}