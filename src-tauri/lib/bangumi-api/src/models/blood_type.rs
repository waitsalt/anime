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

use serde_repr::{Serialize_repr,Deserialize_repr};
/// BloodType : Blood type of a person. A, B, AB, O
/// Blood type of a person. A, B, AB, O
#[repr(i64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr)]
pub enum BloodType {
    A = 1,
    B = 2,
    AB = 3,
    O = 4,

}

impl std::fmt::Display for BloodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::A => "1",
            Self::B => "2",
            Self::AB => "3",
            Self::O => "4",
        })
    }
}
impl Default for BloodType {
    fn default() -> BloodType {
        Self::A
    }
}

