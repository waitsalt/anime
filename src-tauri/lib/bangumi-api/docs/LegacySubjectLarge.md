# LegacySubjectLarge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 条目 ID | [optional]
**url** | Option<**String**> | 条目地址 | [optional]
**r#type** | Option<**i32**> | 条目类型 - `1` 为 书籍 - `2` 为 动画 - `3` 为 音乐 - `4` 为 游戏 - `6` 为 三次元  没有 `5` | [optional]
**name** | Option<**String**> | 条目名称 | [optional]
**name_cn** | Option<**String**> | 条目中文名称 | [optional]
**summary** | Option<**String**> | 剧情简介 | [optional]
**air_date** | Option<**String**> | 放送开始日期 | [optional]
**air_weekday** | Option<**i32**> | 放送星期 | [optional]
**images** | Option<[**models::LegacySubjectSmallImages**](Legacy_SubjectSmall_images.md)> |  | [optional]
**eps** | Option<[**Vec<models::LegacyEpisode>**](Legacy_Episode.md)> | 章节列表 | [optional]
**eps_count** | Option<**i32**> | 话数 | [optional]
**rating** | Option<[**models::LegacySubjectSmallRating**](Legacy_SubjectSmall_rating.md)> |  | [optional]
**rank** | Option<**i32**> | 排名 | [optional]
**collection** | Option<[**models::LegacySubjectSmallCollection**](Legacy_SubjectSmall_collection.md)> |  | [optional]
**crt** | Option<[**Vec<models::LegacySubjectMediumAllOfCrt>**](Legacy_SubjectMedium_allOf_crt.md)> | 角色信息 | [optional]
**staff** | Option<[**Vec<models::LegacySubjectMediumAllOfStaff>**](Legacy_SubjectMedium_allOf_staff.md)> | 制作人员信息 | [optional]
**topic** | Option<[**Vec<models::LegacyTopic>**](Legacy_Topic.md)> | 讨论版 | [optional]
**blog** | Option<[**Vec<models::LegacyBlog>**](Legacy_Blog.md)> | 评论日志 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


