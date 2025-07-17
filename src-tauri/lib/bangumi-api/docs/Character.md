# Character

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**name** | **String** |  | 
**r#type** | [**models::CharacterType**](CharacterType.md) | 角色，机体，舰船，组织... | 
**images** | Option<[**models::PersonImages**](PersonImages.md)> | object with some size of images, this object maybe `null` | [optional]
**summary** | **String** |  | 
**locked** | **bool** |  | 
**infobox** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | server parsed infobox, a map from key to string or tuple null if server infobox is not valid | [optional]
**gender** | Option<**String**> | parsed from wiki, maybe null | [optional]
**blood_type** | Option<[**models::BloodType**](BloodType.md)> | parsed from wiki, maybe null, `1, 2, 3, 4` for `A, B, AB, O` | [optional]
**birth_year** | Option<**i32**> | parsed from wiki, maybe `null` | [optional]
**birth_mon** | Option<**i32**> | parsed from wiki, maybe `null` | [optional]
**birth_day** | Option<**i32**> | parsed from wiki, maybe `null` | [optional]
**stat** | [**models::Stat**](Stat.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


