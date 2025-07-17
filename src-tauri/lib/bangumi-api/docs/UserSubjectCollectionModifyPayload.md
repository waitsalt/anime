# UserSubjectCollectionModifyPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**models::SubjectCollectionType**](SubjectCollectionType.md)> | 修改条目收藏类型 | [optional]
**rate** | Option<**i32**> | 评分，`0` 表示删除评分 | [optional]
**ep_status** | Option<**i32**> | 只能用于修改书籍条目进度 | [optional]
**vol_status** | Option<**i32**> | 只能用于修改书籍条目进度 | [optional]
**comment** | Option<**String**> | 评价 | [optional]
**private** | Option<**bool**> | 仅自己可见 | [optional]
**tags** | Option<**Vec<String>**> | 不传或者 `null` 都会被忽略，传 `[]` 则会删除所有 tag。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


