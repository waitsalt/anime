# \DefaultApi

All URIs are relative to *https://api.bgm.tv*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_subject_to_index_by_index_id**](DefaultApi.md#add_subject_to_index_by_index_id) | **POST** /v0/indices/{index_id}/subjects | Add a subject to Index
[**collect_character_by_character_id_and_user_id**](DefaultApi.md#collect_character_by_character_id_and_user_id) | **POST** /v0/characters/{character_id}/collect | Collect character for current user
[**collect_index_by_index_id_and_user_id**](DefaultApi.md#collect_index_by_index_id_and_user_id) | **POST** /v0/indices/{index_id}/collect | Collect index for current user
[**collect_person_by_person_id_and_user_id**](DefaultApi.md#collect_person_by_person_id_and_user_id) | **POST** /v0/persons/{person_id}/collect | Collect person for current user
[**delelte_subject_from_index_by_index_id_and_subject_id**](DefaultApi.md#delelte_subject_from_index_by_index_id_and_subject_id) | **DELETE** /v0/indices/{index_id}/subjects/{subject_id} | Delete a subject from a Index
[**edit_index_by_id**](DefaultApi.md#edit_index_by_id) | **PUT** /v0/indices/{index_id} | Edit index's information
[**edit_index_subjects_by_index_id_and_subject_id**](DefaultApi.md#edit_index_subjects_by_index_id_and_subject_id) | **PUT** /v0/indices/{index_id}/subjects/{subject_id} | Edit subject information in a index
[**get_calendar**](DefaultApi.md#get_calendar) | **GET** /calendar | 每日放送
[**get_character_by_id**](DefaultApi.md#get_character_by_id) | **GET** /v0/characters/{character_id} | Get Character Detail
[**get_character_image_by_id**](DefaultApi.md#get_character_image_by_id) | **GET** /v0/characters/{character_id}/image | Get Character Image
[**get_character_revision_by_revision_id**](DefaultApi.md#get_character_revision_by_revision_id) | **GET** /v0/revisions/characters/{revision_id} | Get Character Revision
[**get_character_revisions**](DefaultApi.md#get_character_revisions) | **GET** /v0/revisions/characters | Get Character Revisions
[**get_episode_by_id**](DefaultApi.md#get_episode_by_id) | **GET** /v0/episodes/{episode_id} | Get Episode
[**get_episode_revision_by_revision_id**](DefaultApi.md#get_episode_revision_by_revision_id) | **GET** /v0/revisions/episodes/{revision_id} | Get Episode Revision
[**get_episode_revisions**](DefaultApi.md#get_episode_revisions) | **GET** /v0/revisions/episodes | Get Episode Revisions
[**get_episodes**](DefaultApi.md#get_episodes) | **GET** /v0/episodes | Get Episodes
[**get_index_by_id**](DefaultApi.md#get_index_by_id) | **GET** /v0/indices/{index_id} | Get Index By ID
[**get_index_subjects_by_index_id**](DefaultApi.md#get_index_subjects_by_index_id) | **GET** /v0/indices/{index_id}/subjects | Get Index Subjects
[**get_myself**](DefaultApi.md#get_myself) | **GET** /v0/me | Get User
[**get_person_by_id**](DefaultApi.md#get_person_by_id) | **GET** /v0/persons/{person_id} | Get Person
[**get_person_image_by_id**](DefaultApi.md#get_person_image_by_id) | **GET** /v0/persons/{person_id}/image | Get Person Image
[**get_person_revision_by_revision_id**](DefaultApi.md#get_person_revision_by_revision_id) | **GET** /v0/revisions/persons/{revision_id} | Get Person Revision
[**get_person_revisions**](DefaultApi.md#get_person_revisions) | **GET** /v0/revisions/persons | Get Person Revisions
[**get_related_characters_by_person_id**](DefaultApi.md#get_related_characters_by_person_id) | **GET** /v0/persons/{person_id}/characters | get person related characters
[**get_related_characters_by_subject_id**](DefaultApi.md#get_related_characters_by_subject_id) | **GET** /v0/subjects/{subject_id}/characters | Get Subject Characters
[**get_related_persons_by_character_id**](DefaultApi.md#get_related_persons_by_character_id) | **GET** /v0/characters/{character_id}/persons | get character related persons
[**get_related_persons_by_subject_id**](DefaultApi.md#get_related_persons_by_subject_id) | **GET** /v0/subjects/{subject_id}/persons | Get Subject Persons
[**get_related_subjects_by_character_id**](DefaultApi.md#get_related_subjects_by_character_id) | **GET** /v0/characters/{character_id}/subjects | get character related subjects
[**get_related_subjects_by_person_id**](DefaultApi.md#get_related_subjects_by_person_id) | **GET** /v0/persons/{person_id}/subjects | get person related subjects
[**get_related_subjects_by_subject_id**](DefaultApi.md#get_related_subjects_by_subject_id) | **GET** /v0/subjects/{subject_id}/subjects | Get Subject Relations
[**get_subject_by_id**](DefaultApi.md#get_subject_by_id) | **GET** /v0/subjects/{subject_id} | 获取条目
[**get_subject_image_by_id**](DefaultApi.md#get_subject_image_by_id) | **GET** /v0/subjects/{subject_id}/image | Get Subject Image
[**get_subject_revision_by_revision_id**](DefaultApi.md#get_subject_revision_by_revision_id) | **GET** /v0/revisions/subjects/{revision_id} | Get Subject Revision
[**get_subject_revisions**](DefaultApi.md#get_subject_revisions) | **GET** /v0/revisions/subjects | Get Subject Revisions
[**get_subjects**](DefaultApi.md#get_subjects) | **GET** /v0/subjects | 浏览条目
[**get_user_avatar_by_name**](DefaultApi.md#get_user_avatar_by_name) | **GET** /v0/users/{username}/avatar | Get User Avatar by name
[**get_user_by_name**](DefaultApi.md#get_user_by_name) | **GET** /v0/users/{username} | Get User by name
[**get_user_character_collection**](DefaultApi.md#get_user_character_collection) | **GET** /v0/users/{username}/collections/-/characters/{character_id} | 获取用户单个角色收藏信息
[**get_user_character_collections**](DefaultApi.md#get_user_character_collections) | **GET** /v0/users/{username}/collections/-/characters | 获取用户角色收藏列表
[**get_user_collection**](DefaultApi.md#get_user_collection) | **GET** /v0/users/{username}/collections/{subject_id} | 获取用户单个条目收藏
[**get_user_collections_by_username**](DefaultApi.md#get_user_collections_by_username) | **GET** /v0/users/{username}/collections | 获取用户收藏
[**get_user_episode_collection**](DefaultApi.md#get_user_episode_collection) | **GET** /v0/users/-/collections/-/episodes/{episode_id} | 章节收藏信息
[**get_user_person_collection**](DefaultApi.md#get_user_person_collection) | **GET** /v0/users/{username}/collections/-/persons/{person_id} | 获取用户单个人物收藏信息
[**get_user_person_collections**](DefaultApi.md#get_user_person_collections) | **GET** /v0/users/{username}/collections/-/persons | 获取用户人物收藏列表
[**get_user_subject_episode_collection**](DefaultApi.md#get_user_subject_episode_collection) | **GET** /v0/users/-/collections/{subject_id}/episodes | 章节收藏信息
[**new_index**](DefaultApi.md#new_index) | **POST** /v0/indices | Create a new index
[**patch_user_collection**](DefaultApi.md#patch_user_collection) | **PATCH** /v0/users/-/collections/{subject_id} | 修改用户单个收藏
[**patch_user_subject_episode_collection**](DefaultApi.md#patch_user_subject_episode_collection) | **PATCH** /v0/users/-/collections/{subject_id}/episodes | 章节收藏信息
[**post_user_collection**](DefaultApi.md#post_user_collection) | **POST** /v0/users/-/collections/{subject_id} | 新增或修改用户单个条目收藏
[**put_user_episode_collection**](DefaultApi.md#put_user_episode_collection) | **PUT** /v0/users/-/collections/-/episodes/{episode_id} | 更新章节收藏信息
[**search_characters**](DefaultApi.md#search_characters) | **POST** /v0/search/characters | 角色搜索
[**search_persons**](DefaultApi.md#search_persons) | **POST** /v0/search/persons | 人物搜索
[**search_subjects**](DefaultApi.md#search_subjects) | **POST** /v0/search/subjects | 条目搜索
[**uncollect_character_by_character_id_and_user_id**](DefaultApi.md#uncollect_character_by_character_id_and_user_id) | **DELETE** /v0/characters/{character_id}/collect | Uncollect character for current user
[**uncollect_index_by_index_id_and_user_id**](DefaultApi.md#uncollect_index_by_index_id_and_user_id) | **DELETE** /v0/indices/{index_id}/collect | Uncollect index for current user
[**uncollect_person_by_person_id_and_user_id**](DefaultApi.md#uncollect_person_by_person_id_and_user_id) | **DELETE** /v0/persons/{person_id}/collect | Uncollect person for current user



## add_subject_to_index_by_index_id

> add_subject_to_index_by_index_id(index_id, index_subject_add_info)
Add a subject to Index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |
**index_subject_add_info** | Option<[**IndexSubjectAddInfo**](IndexSubjectAddInfo.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collect_character_by_character_id_and_user_id

> collect_character_by_character_id_and_user_id(character_id)
Collect character for current user

为当前用户收藏角色

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collect_index_by_index_id_and_user_id

> collect_index_by_index_id_and_user_id(index_id)
Collect index for current user

为当前用户收藏一条目录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collect_person_by_person_id_and_user_id

> collect_person_by_person_id_and_user_id(person_id)
Collect person for current user

为当前用户收藏人物

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 人物 ID | [required] |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delelte_subject_from_index_by_index_id_and_subject_id

> delelte_subject_from_index_by_index_id_and_subject_id(index_id, subject_id)
Delete a subject from a Index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |
**subject_id** | **i32** | 条目 ID | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_index_by_id

> models::Index edit_index_by_id(index_id, index_basic_info)
Edit index's information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |
**index_basic_info** | Option<[**IndexBasicInfo**](IndexBasicInfo.md)> |  |  |

### Return type

[**models::Index**](Index.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_index_subjects_by_index_id_and_subject_id

> edit_index_subjects_by_index_id_and_subject_id(index_id, subject_id, index_subject_edit_info)
Edit subject information in a index

如果条目不存在于目录，会创建该条目

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |
**subject_id** | **i32** | 条目 ID | [required] |
**index_subject_edit_info** | Option<[**IndexSubjectEditInfo**](IndexSubjectEditInfo.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_calendar

> Vec<models::GetCalendar200ResponseInner> get_calendar()
每日放送

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetCalendar200ResponseInner>**](getCalendar_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_character_by_id

> models::Character get_character_by_id(character_id)
Get Character Detail

cache with 60s

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |

### Return type

[**models::Character**](Character.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_character_image_by_id

> get_character_image_by_id(character_id, r#type)
Get Character Image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |
**r#type** | **String** | 枚举值 {small|grid|large|medium} | [required] |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_character_revision_by_revision_id

> models::CharacterRevision get_character_revision_by_revision_id(revision_id)
Get Character Revision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revision_id** | **i32** | 版本 ID | [required] |

### Return type

[**models::CharacterRevision**](CharacterRevision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_character_revisions

> models::PagedRevision get_character_revisions(character_id, limit, offset)
Get Character Revisions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedRevision**](Paged_Revision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_by_id

> models::EpisodeDetail get_episode_by_id(episode_id)
Get Episode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_id** | **i32** | 章节 ID | [required] |

### Return type

[**models::EpisodeDetail**](EpisodeDetail.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_revision_by_revision_id

> models::DetailedRevision get_episode_revision_by_revision_id(revision_id)
Get Episode Revision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revision_id** | **i32** | 版本 ID | [required] |

### Return type

[**models::DetailedRevision**](DetailedRevision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_revisions

> models::PagedRevision get_episode_revisions(episode_id, limit, offset)
Get Episode Revisions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_id** | **i32** | 章节 ID | [required] |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedRevision**](Paged_Revision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episodes

> models::PagedEpisode get_episodes(subject_id, r#type, limit, offset)
Get Episodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**r#type** | Option<[**models::EpType**](.md)> | 参照章节的`type` |  |
**limit** | Option<**i32**> | 分页参数 |  |[default to 100]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedEpisode**](Paged_Episode.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_index_by_id

> models::Index get_index_by_id(index_id)
Get Index By ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |

### Return type

[**models::Index**](Index.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_index_subjects_by_index_id

> get_index_subjects_by_index_id(index_id, r#type, limit, offset)
Get Index Subjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |
**r#type** | Option<[**SubjectType**](.md)> | 条目类型 |  |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_myself

> models::GetMyself200Response get_myself()
Get User

返回当前 Access Token 对应的用户信息

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetMyself200Response**](getMyself_200_response.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_by_id

> models::PersonDetail get_person_by_id(person_id)
Get Person

cache with 60s

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 人物 ID | [required] |

### Return type

[**models::PersonDetail**](PersonDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_image_by_id

> get_person_image_by_id(person_id, r#type)
Get Person Image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 人物 ID | [required] |
**r#type** | **String** | 枚举值 {small|grid|large|medium} | [required] |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_revision_by_revision_id

> models::PersonRevision get_person_revision_by_revision_id(revision_id)
Get Person Revision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revision_id** | **i32** | 历史版本 ID | [required] |

### Return type

[**models::PersonRevision**](PersonRevision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_revisions

> models::PagedRevision get_person_revisions(person_id, limit, offset)
Get Person Revisions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 角色 ID | [required] |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedRevision**](Paged_Revision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_characters_by_person_id

> Vec<models::PersonCharacter> get_related_characters_by_person_id(person_id)
get person related characters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 人物 ID | [required] |

### Return type

[**Vec<models::PersonCharacter>**](PersonCharacter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_characters_by_subject_id

> Vec<models::RelatedCharacter> get_related_characters_by_subject_id(subject_id)
Get Subject Characters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |

### Return type

[**Vec<models::RelatedCharacter>**](RelatedCharacter.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_persons_by_character_id

> Vec<models::CharacterPerson> get_related_persons_by_character_id(character_id)
get character related persons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |

### Return type

[**Vec<models::CharacterPerson>**](CharacterPerson.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_persons_by_subject_id

> Vec<models::RelatedPerson> get_related_persons_by_subject_id(subject_id)
Get Subject Persons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |

### Return type

[**Vec<models::RelatedPerson>**](RelatedPerson.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_subjects_by_character_id

> Vec<models::V0RelatedSubject> get_related_subjects_by_character_id(character_id)
get character related subjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |

### Return type

[**Vec<models::V0RelatedSubject>**](v0_RelatedSubject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_subjects_by_person_id

> Vec<models::V0RelatedSubject> get_related_subjects_by_person_id(person_id)
get person related subjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 人物 ID | [required] |

### Return type

[**Vec<models::V0RelatedSubject>**](v0_RelatedSubject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_subjects_by_subject_id

> Vec<models::V0SubjectRelation> get_related_subjects_by_subject_id(subject_id)
Get Subject Relations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |

### Return type

[**Vec<models::V0SubjectRelation>**](v0_subject_relation.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subject_by_id

> models::Subject get_subject_by_id(subject_id)
获取条目

cache with 300s

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |

### Return type

[**models::Subject**](Subject.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subject_image_by_id

> get_subject_image_by_id(subject_id, r#type)
Get Subject Image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**r#type** | **String** | 枚举值 {small|grid|large|medium|common} | [required] |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subject_revision_by_revision_id

> models::SubjectRevision get_subject_revision_by_revision_id(revision_id)
Get Subject Revision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revision_id** | **i32** | 版本 ID | [required] |

### Return type

[**models::SubjectRevision**](SubjectRevision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subject_revisions

> models::PagedRevision get_subject_revisions(subject_id, limit, offset)
Get Subject Revisions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedRevision**](Paged_Revision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subjects

> models::PagedSubject get_subjects(r#type, cat, series, platform, sort, year, month, limit, offset)
浏览条目

第一页会 cache 24h，之后会 cache 1h

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | [**SubjectType**](.md) | 条目类型 | [required] |
**cat** | Option<[**SubjectCategory**](.md)> | 条目分类，参照 `SubjectCategory` enum |  |
**series** | Option<**bool**> | 是否系列，仅对书籍类型的条目有效 |  |
**platform** | Option<**String**> | 平台，仅对游戏类型的条目有效 |  |
**sort** | Option<**String**> | 排序，枚举值 {date|rank} |  |
**year** | Option<**i32**> | 年份 |  |
**month** | Option<**i32**> | 月份 |  |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedSubject**](Paged_Subject.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_avatar_by_name

> get_user_avatar_by_name(username, r#type)
Get User Avatar by name

获取用户头像，302 重定向至头像地址，设置了 username 之后无法使用 UID 查询。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |
**r#type** | **String** | 枚举值 {small|large|medium} | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_name

> models::User get_user_by_name(username)
Get User by name

获取用户信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_character_collection

> models::UserCharacterCollection get_user_character_collection(username, character_id)
获取用户单个角色收藏信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |
**character_id** | **i32** | 角色 ID | [required] |

### Return type

[**models::UserCharacterCollection**](UserCharacterCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_character_collections

> models::PagedUserCharacterCollection get_user_character_collections(username)
获取用户角色收藏列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |

### Return type

[**models::PagedUserCharacterCollection**](Paged_UserCharacterCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection

> models::UserSubjectCollection get_user_collection(username, subject_id)
获取用户单个条目收藏

获取对应用户的收藏，查看私有收藏需要 access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |
**subject_id** | **i32** | 条目 ID | [required] |

### Return type

[**models::UserSubjectCollection**](UserSubjectCollection.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collections_by_username

> models::PagedUserCollection get_user_collections_by_username(username, subject_type, r#type, limit, offset)
获取用户收藏

获取对应用户的收藏，查看私有收藏需要access token。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |
**subject_type** | Option<[**SubjectType**](.md)> | 条目类型，默认为全部  具体含义见 [SubjectType](#model-SubjectType) |  |
**r#type** | Option<**String**> | 收藏类型，默认为全部  具体含义见 [CollectionType](#model-CollectionType) |  |
**limit** | Option<**i32**> | 分页参数 |  |[default to 30]
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]

### Return type

[**models::PagedUserCollection**](Paged_UserCollection.md)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_episode_collection

> models::UserEpisodeCollection get_user_episode_collection(episode_id)
章节收藏信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_id** | **i32** | 章节 ID | [required] |

### Return type

[**models::UserEpisodeCollection**](UserEpisodeCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_person_collection

> models::UserPersonCollection get_user_person_collection(username, person_id)
获取用户单个人物收藏信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |
**person_id** | **i32** | 人物 ID | [required] |

### Return type

[**models::UserPersonCollection**](UserPersonCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_person_collections

> models::PagedUserPersonCollection get_user_person_collections(username)
获取用户人物收藏列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | 设置了用户名之后无法使用 UID。 | [required] |

### Return type

[**models::PagedUserPersonCollection**](Paged_UserPersonCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_subject_episode_collection

> models::GetUserSubjectEpisodeCollection200Response get_user_subject_episode_collection(subject_id, offset, limit, episode_type)
章节收藏信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**offset** | Option<**i32**> | 分页参数 |  |[default to 0]
**limit** | Option<**i32**> | 分页参数 |  |[default to 100]
**episode_type** | Option<[**EpType**](.md)> | 章节类型，不传则不按照章节进行筛选 |  |

### Return type

[**models::GetUserSubjectEpisodeCollection200Response**](getUserSubjectEpisodeCollection_200_response.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_index

> models::Index new_index()
Create a new index

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Index**](Index.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_collection

> patch_user_collection(subject_id, user_subject_collection_modify_payload)
修改用户单个收藏

修改条目收藏状态  由于直接修改剧集条目的完成度可能会引起意料之外效果，只能用于修改书籍类条目的完成度。  PATCH 方法的所有请求体字段均可选 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**user_subject_collection_modify_payload** | Option<[**UserSubjectCollectionModifyPayload**](UserSubjectCollectionModifyPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_subject_episode_collection

> patch_user_subject_episode_collection(subject_id, patch_user_subject_episode_collection_request)
章节收藏信息

同时会重新计算条目的完成度 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**patch_user_subject_episode_collection_request** | Option<[**PatchUserSubjectEpisodeCollectionRequest**](PatchUserSubjectEpisodeCollectionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_collection

> post_user_collection(subject_id, user_subject_collection_modify_payload)
新增或修改用户单个条目收藏

修改条目收藏状态, 如果不存在则创建，如果存在则修改  由于直接修改剧集条目的完成度可能会引起意料之外效果，只能用于修改书籍类条目的完成度。  方法的所有请求体字段均可选 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **i32** | 条目 ID | [required] |
**user_subject_collection_modify_payload** | Option<[**UserSubjectCollectionModifyPayload**](UserSubjectCollectionModifyPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_episode_collection

> put_user_episode_collection(episode_id, put_user_episode_collection_request)
更新章节收藏信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_id** | **i32** | 章节 ID | [required] |
**put_user_episode_collection_request** | Option<[**PutUserEpisodeCollectionRequest**](PutUserEpisodeCollectionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_characters

> models::PagedCharacter search_characters(limit, offset, search_characters_request)
角色搜索

## 实验性 API， 本 schema 和实际的 API 行为都可能随时发生改动  目前支持的筛选条件包括: - `nsfw`: 使用 `include` 包含NSFW搜索结果。默认排除搜索NSFW条目。无权限情况下忽略此选项，不会返回NSFW条目。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | 分页参数 |  |
**offset** | Option<**i32**> | 分页参数 |  |
**search_characters_request** | Option<[**SearchCharactersRequest**](SearchCharactersRequest.md)> |  |  |

### Return type

[**models::PagedCharacter**](Paged_Character.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_persons

> models::PagedPerson search_persons(limit, offset, search_persons_request)
人物搜索

## 实验性 API， 本 schema 和实际的 API 行为都可能随时发生改动  目前支持的筛选条件包括: - `career`: 职业，可以多次出现。`且` 关系。  不同筛选条件之间为 `且` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | 分页参数 |  |
**offset** | Option<**i32**> | 分页参数 |  |
**search_persons_request** | Option<[**SearchPersonsRequest**](SearchPersonsRequest.md)> |  |  |

### Return type

[**models::PagedPerson**](Paged_Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_subjects

> models::PagedSubject search_subjects(limit, offset, search_subjects_request)
条目搜索

## 实验性 API， 本 schema 和实际的 API 行为都可能随时发生改动  目前支持的筛选条件包括: - `type`: 条目类型，参照 `SubjectType` enum， `或`。 - `tag`: 标签，可以多次出现。`且` 关系。 - `air_date`: 播出日期/发售日期。`且` 关系。 - `rating`: 用于搜索指定评分的条目。`且` 关系。 - `rank`: 用于搜索指定排名的条目。`且` 关系。 - `nsfw`: 使用 `include` 包含NSFW搜索结果。默认排除搜索NSFW条目。无权限情况下忽略此选项，不会返回NSFW条目。  不同筛选条件之间为 `且` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | 分页参数 |  |
**offset** | Option<**i32**> | 分页参数 |  |
**search_subjects_request** | Option<[**SearchSubjectsRequest**](SearchSubjectsRequest.md)> |  |  |

### Return type

[**models::PagedSubject**](Paged_Subject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uncollect_character_by_character_id_and_user_id

> uncollect_character_by_character_id_and_user_id(character_id)
Uncollect character for current user

为当前用户取消收藏角色

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | 角色 ID | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uncollect_index_by_index_id_and_user_id

> uncollect_index_by_index_id_and_user_id(index_id)
Uncollect index for current user

为当前用户取消收藏一条目录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i32** | 目录 ID | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uncollect_person_by_person_id_and_user_id

> uncollect_person_by_person_id_and_user_id(person_id)
Uncollect person for current user

为当前用户取消收藏人物

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** | 人物 ID | [required] |

### Return type

 (empty response body)

### Authorization

[OptionalHTTPBearer](../README.md#OptionalHTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

