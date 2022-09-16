use anyhow::{anyhow, Result};
use chrono::{NaiveDate, NaiveDateTime};
use rbatis::rbatis::Rbatis;
use rbatis::rbatis_codegen::ops::AsProxy;

use rbatis::utils::into_one::IntoOne;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileInfo {
    pub user_id: Option<i64>,
    pub full_name: Option<String>,
    pub address: Option<String>,
    pub education: Option<String>,
    pub birth_day: Option<NaiveDate>,
    pub create_date: Option<NaiveDateTime>,
}

rbatis::crud!(ProfileInfo {}, "profile");

impl ProfileInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &mut Rbatis, user_id: &i64) -> Result<Option<Self>> {
        let result = ProfileInfo::select_by_column(rb, "user_id", user_id).await;
        match result {
            Ok(vec) => anyhow::Ok(vec.into_one()),
            Err(_err) => anyhow::Ok(None),
        }
    }
    #[allow(dead_code)]
    pub async fn from_user_id(rb: Rbatis, user_id: i64) -> Result<Option<Self>> {
        let result = rb
            .fetch_decode::<ProfileInfo>(
                "select * from profile where user_id =?",
                vec![rbs::Value::I64(user_id)],
            )
            .await;
        match result {
            Ok(profile) => anyhow::Ok(Some(profile)),
            Err(_err) => anyhow::Ok(None),
        }
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &mut Rbatis) -> Result<u64> {
        let result = ProfileInfo::insert(rb, self).await;
        match result {
            Ok(exe_result) => anyhow::Ok(exe_result.last_insert_id.u64()),
            Err(_err) => anyhow::Ok(0),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &mut Rbatis) -> Result<u64> {
        let result = ProfileInfo::update_by_column(rb, self, "user_id").await;
        match result {
            Ok(ret) => anyhow::Ok(ret.rows_affected),
            Err(_err) => anyhow::Ok(0)
        }
    }
    #[allow(dead_code)]
    #[html_sql("fast_common/src/sql/profile.html")]
    pub async fn remove_batch(&self,rb: &Rbatis) ->Option<ProfileInfo> {
         unimplemented!()
        // let wp = rb.new_wrapper()
        //     .r#if(self.user_id.clone().is_some(), |w| w.and().eq("user_id", self.user_id.clone().unwrap()))
        //     .r#if(self.full_name.clone().is_some(), |w| w.and().eq("full_name", self.full_name.clone().unwrap()))
        //     .r#if(self.address.clone().is_some(), |w| w.and().eq("address", self.address.clone().unwrap()))
        //     .r#if(self.education.clone().is_some(), |w| w.and().eq("education", self.education.clone().unwrap()))
        //     .r#if(self.birth_day.clone().is_some(), |w| w.and().eq("birth_day", self.birth_day.clone().unwrap()))
        //     .r#if(self.create_date.clone().is_some(), |w| w.and().eq("create_date", self.create_date.clone().unwrap()));
        // rb.remove_by_wrapper::<Self>(wp).await
    }

}
