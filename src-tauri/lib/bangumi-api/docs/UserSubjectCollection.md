# UserSubjectCollection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject_id** | **i32** |  | 
**subject_type** | [**models::SubjectType**](SubjectType.md) |  | 
**rate** | **i32** |  | 
**r#type** | [**models::SubjectCollectionType**](SubjectCollectionType.md) |  | 
**comment** | Option<**String**> |  | [optional]
**tags** | **Vec<String>** |  | 
**ep_status** | **i32** |  | 
**vol_status** | **i32** |  | 
**updated_at** | **String** | 本时间并不代表条目的收藏时间。修改评分，评价，章节观看状态等收藏信息时未更新此时间是一个 bug。请不要依赖此特性 | 
**private** | **bool** |  | 
**subject** | Option<[**models::SlimSubject**](SlimSubject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


