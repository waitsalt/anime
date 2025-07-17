# UserPersonCollection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**name** | **String** |  | 
**r#type** | [**models::PersonType**](PersonType.md) | `1`, `2`, `3` 表示 `个人`, `公司`, `组合` | 
**career** | [**Vec<models::PersonCareer>**](PersonCareer.md) |  | 
**images** | Option<[**models::PersonImages**](PersonImages.md)> | object with some size of images, this object maybe `null` | [optional]
**created_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


