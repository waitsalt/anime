/*
 * Bangumi API
 *
 * 你可以在 <https://next.bgm.tv/demo/access-token> 生成一个 Access Token  ## [关于 User Agent](https://github.com/bangumi/api/blob/master/docs-raw/user%20agent.md)  如果你在使用中遇到了问题，请优先使用 GitHub issue 提交问题。在 bangumi 小组发帖可能无法得到及时反馈。 
 *
 * The version of the OpenAPI document: 2025-07-1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    #[serde(rename = "id")]
    pub id: i32,
    /// `0` 本篇，`1` SP，`2` OP，`3` ED
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "name_cn")]
    pub name_cn: String,
    /// 同类条目的排序和集数
    #[serde(rename = "sort")]
    pub sort: f64,
    /// 条目内的集数, 从`1`开始。非本篇剧集的此字段无意义
    #[serde(rename = "ep", skip_serializing_if = "Option::is_none")]
    pub ep: Option<f64>,
    #[serde(rename = "airdate")]
    pub airdate: String,
    #[serde(rename = "comment")]
    pub comment: i32,
    /// 维基人填写的原始时长
    #[serde(rename = "duration")]
    pub duration: String,
    /// 简介
    #[serde(rename = "desc")]
    pub desc: String,
    /// 音乐曲目的碟片数
    #[serde(rename = "disc")]
    pub disc: i32,
    /// 服务器解析的时长，无法解析时为 `0`
    #[serde(rename = "duration_seconds", skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
}

impl Episode {
    pub fn new(id: i32, r#type: i32, name: String, name_cn: String, sort: f64, airdate: String, comment: i32, duration: String, desc: String, disc: i32) -> Episode {
        Episode {
            id,
            r#type,
            name,
            name_cn,
            sort,
            ep: None,
            airdate,
            comment,
            duration,
            desc,
            disc,
            duration_seconds: None,
        }
    }
}

