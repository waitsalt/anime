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
pub struct DetailOneOf {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// request path
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl DetailOneOf {
    pub fn new() -> DetailOneOf {
        DetailOneOf {
            error: None,
            path: None,
        }
    }
}

