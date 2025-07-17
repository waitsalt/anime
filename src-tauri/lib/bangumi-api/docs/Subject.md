# Subject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**r#type** | [**models::SubjectType**](SubjectType.md) |  | 
**name** | **String** |  | 
**name_cn** | **String** |  | 
**summary** | **String** |  | 
**series** | **bool** | 是否为书籍系列的主条目 | 
**nsfw** | **bool** |  | 
**locked** | **bool** |  | 
**date** | Option<**String**> | air date in `YYYY-MM-DD` format | [optional]
**platform** | **String** | TV, Web, 欧美剧, DLC... | 
**images** | [**models::Images**](Images.md) |  | 
**infobox** | Option<[**Vec<models::Item>**](Item.md)> |  | [optional]
**volumes** | **i32** | 书籍条目的册数，由旧服务端从wiki中解析 | 
**eps** | **i32** | 由旧服务端从wiki中解析，对于书籍条目为`话数` | 
**total_episodes** | **i32** | 数据库中的章节数量 | 
**rating** | [**models::Rating**](Rating.md) |  | 
**collection** | [**models::Collection**](Collection.md) |  | 
**meta_tags** | **Vec<String>** | 由维基人维护的 tag | 
**tags** | [**Vec<models::Tag>**](Tag.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


