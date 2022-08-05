use models::{FieldInfo, SeriesInfo, Tag, ValueType};
use std::fs;

use super::*;

#[tokio::test]
async fn test_index_add_del() {
    let _ = fs::remove_dir_all("/tmp/index_test/db_test");
    let mut index = index::DBIndex::from("/tmp/index_test/db_test");

    let mut info1 = SeriesInfo::new(
        vec![Tag::new(b"host".to_vec(), b"h1".to_vec())],
        vec![
            FieldInfo::new(0, b"cpu".to_vec(), ValueType::Float),
            FieldInfo::new(0, b"mem".to_vec(), ValueType::Float),
        ],
    );
    let id = index.add_series_if_not_exists(&mut info1).await.unwrap();

    let key = index.get_series_key(id).await.unwrap().unwrap();
    println!("mydebug id: {}, key: {:?}", id, key);
    assert_eq!("", key.table());
    assert_eq!(info1.tags(), key.tags());

    index.del_series_info(id).await.unwrap();
    let key = index.get_series_key(id).await.unwrap();
    println!("mydebug id: {}, key: {:?}", id, key);
    assert_eq!(key, None);

    index.close().await.unwrap();
}

#[tokio::test]
async fn test_index_id_list() {
    let _ = fs::remove_dir_all("/tmp/index_test/db_test");
    let mut index = index::DBIndex::from("/tmp/index_test/db_test");

    let mut info1 = SeriesInfo::new(
        vec![
            Tag::new(b"loc".to_vec(), b"bj".to_vec()),
            Tag::new(b"host".to_vec(), b"h1".to_vec()),
        ],
        vec![
            FieldInfo::new(0, b"cpu".to_vec(), ValueType::Float),
            FieldInfo::new(0, b"mem".to_vec(), ValueType::Float),
        ],
    );

    let mut info2 = SeriesInfo::new(
        vec![
            Tag::new(b"loc".to_vec(), b"bj".to_vec()),
            Tag::new(b"host".to_vec(), b"h2".to_vec()),
        ],
        vec![FieldInfo::new(0, b"mem".to_vec(), ValueType::Float)],
    );

    let mut info3 = SeriesInfo::new(
        vec![
            Tag::new(b"loc".to_vec(), b"bj".to_vec()),
            Tag::new(b"host".to_vec(), b"h3".to_vec()),
        ],
        vec![FieldInfo::new(0, b"mem".to_vec(), ValueType::Float)],
    );
    let id1 = index.add_series_if_not_exists(&mut info1).await.unwrap();
    let id2 = index.add_series_if_not_exists(&mut info2).await.unwrap();
    let id3 = index.add_series_if_not_exists(&mut info3).await.unwrap();
    let key1 = index.get_series_key(id1).await.unwrap().unwrap();
    let key2 = index.get_series_key(id2).await.unwrap().unwrap();

    let enc =
        utils::encode_inverted_index_key(&"tab".to_string(), &"tag".as_bytes(), &"val".as_bytes());
    assert_eq!(enc, "tab.tag=val".as_bytes());

    let tags = vec![
        Tag::new(b"loc".to_vec(), b"bj".to_vec()),
        Tag::new(b"host".to_vec(), b"h2".to_vec()),
    ];

    let list = index
        .get_series_id_list(&"".to_string(), &tags)
        .await
        .unwrap();
    assert_eq!(vec![id2], list);
    println!("mydebug ids1: {:#x?}", list);

    let list = index
        .get_series_id_list(&"".to_string(), &tags[0..0].to_vec())
        .await
        .unwrap();
    println!("mydebug ids2: {:#x?}", list);
    assert_eq!(vec![id1, id2, id3], list);

    index.close().await.unwrap();
}

#[tokio::test]
async fn test_field_type() {
    let _ = fs::remove_dir_all("/tmp/index_test/db_test");
    let mut index = index::DBIndex::from("/tmp/index_test/db_test");

    let mut info1 = SeriesInfo::new(
        vec![
            Tag::new(b"loc".to_vec(), b"bj".to_vec()),
            Tag::new(b"host".to_vec(), b"h1".to_vec()),
        ],
        vec![
            FieldInfo::new(0, b"cpu".to_vec(), ValueType::Float),
            FieldInfo::new(0, b"mem".to_vec(), ValueType::Float),
        ],
    );

    let mut info2 = SeriesInfo::new(
        vec![
            Tag::new(b"loc".to_vec(), b"bj".to_vec()),
            Tag::new(b"host".to_vec(), b"h2".to_vec()),
        ],
        vec![FieldInfo::new(0, b"mem".to_vec(), ValueType::Unsigned)],
    );
    let id1 = index.add_series_if_not_exists(&mut info1).await.unwrap();
    let id2 = index.add_series_if_not_exists(&mut info2).await;
    //assert_eq!(id2, index::errors::ForwardIndexError::FieldType);
    println!("{:?}", id2);

    let schema = index.get_table_schema(&"".to_string()).await;
    println!("{}:{}{:#?}", std::file!(), std::line!(), schema);

    index.close().await.unwrap();
}

#[tokio::test]
async fn test_copy_from_slice() {
    let mut v1: Vec<u64> = vec![1, 2, 3];
    let v2: Vec<u64> = vec![4, 5, 6];
    v1.extend_from_slice(&v2[3..]);

    println!("{:?}", v1);
}
