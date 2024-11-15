use serde::Serialize;
use url_query_string::ToQueryString;

#[derive(Serialize, ToQueryString)]
#[serde(rename_all = "camelCase")]
struct TestStruct {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub id: Option<String>,
    pub user_id: Option<String>,
}

#[test]
fn test_to_query_string() {
    let instance = TestStruct {
        page: Some(1),
        page_size: Some(20),
        id: Some("test_id".to_string()),
        user_id: Some("user_123".to_string()),
    };

    let query_string = instance.to_query_string();
    assert_eq!(
        query_string,
        "page=1&pageSize=20&id=test_id&userId=user_123"
    );
}

#[test]
fn test_try_to_query_string_ok() {
    let instance = TestStruct {
        page: Some(1),
        page_size: Some(20),
        id: Some("test_id".to_string()),
        user_id: Some("user_123".to_string()),
    };

    let result = instance.try_to_query_string();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "page=1&pageSize=20&id=test_id&userId=user_123"
    );
}
