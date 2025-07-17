# SlimSubject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**r#type** | [**models::SubjectType**](SubjectType.md) |  | 
**name** | **String** |  | 
**name_cn** | **String** |  | 
**short_summary** | **String** | 截短后的条目描述。 | 
**date** | Option<**String**> | air date in `YYYY-MM-DD` format | [optional]
**images** | [**models::Images**](Images.md) |  | 
**volumes** | **i32** | 书籍条目的册数，由旧服务端从wiki中解析 | 
**eps** | **i32** | 由旧服务端从wiki中解析，对于书籍条目为`话数` | 
**collection_total** | **i32** | 收藏人数 | 
**score** | **f64** | 分数 | 
**rank** | **i32** | 排名 | 
**tags** | [**Vec<models::Tag>**](Tag.md) | 前 10 个 tag | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


