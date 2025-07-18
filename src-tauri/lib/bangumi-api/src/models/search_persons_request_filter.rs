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

/// SearchPersonsRequestFilter : 不同条件之间是 `且` 的关系
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchPersonsRequestFilter {
    /// 职业，可以多次出现。多值之间为 `且` 关系。
    #[serde(rename = "career", skip_serializing_if = "Option::is_none")]
    pub career: Option<Vec<String>>,
}

impl SearchPersonsRequestFilter {
    /// 不同条件之间是 `且` 的关系
    pub fn new() -> SearchPersonsRequestFilter {
        SearchPersonsRequestFilter {
            career: None,
        }
    }
}

