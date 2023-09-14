use crate::client::utils::{REGISTERED_EMAIL, REGISTERED_PASSWORD, REGISTERED_USER_MUTEX};
use crate::client_api_client;
use crate::collab::workspace_id_from_client;

use collab_define::CollabType;
use shared_entity::error_code::ErrorCode;
use storage_entity::{DeleteCollabParams, InsertCollabParams, QueryCollabParams};
use uuid::Uuid;

#[tokio::test]
async fn success_insert_collab_test() {
  let _guard = REGISTERED_USER_MUTEX.lock().await;

  let mut c = client_api_client();
  c.sign_in_password(&REGISTERED_EMAIL, &REGISTERED_PASSWORD)
    .await
    .unwrap();

  let raw_data = "hello world".to_string().as_bytes().to_vec();
  let workspace_id = workspace_id_from_client(&c).await;
  let object_id = Uuid::new_v4().to_string();
  c.create_collab(InsertCollabParams::new(
    1,
    &object_id,
    CollabType::Document,
    raw_data.clone(),
    workspace_id,
  ))
  .await
  .unwrap();

  let bytes = c
    .get_collab(QueryCollabParams {
      object_id,
      collab_type: CollabType::Document,
    })
    .await
    .unwrap();

  assert_eq!(bytes, raw_data);
}

#[tokio::test]
async fn success_delete_collab_test() {
  let _guard = REGISTERED_USER_MUTEX.lock().await;

  let mut c = client_api_client();
  c.sign_in_password(&REGISTERED_EMAIL, &REGISTERED_PASSWORD)
    .await
    .unwrap();

  let raw_data = "hello world".to_string().as_bytes().to_vec();
  let workspace_id = workspace_id_from_client(&c).await;
  let object_id = Uuid::new_v4().to_string();
  c.create_collab(InsertCollabParams::new(
    1,
    object_id.clone(),
    CollabType::Document,
    raw_data.clone(),
    workspace_id,
  ))
  .await
  .unwrap();

  c.delete_collab(DeleteCollabParams {
    object_id: object_id.clone(),
  })
  .await
  .unwrap();

  let error = c
    .get_collab(QueryCollabParams {
      object_id,
      collab_type: CollabType::Document,
    })
    .await
    .unwrap_err();

  assert_eq!(error.code, ErrorCode::RecordNotFound);
}

#[tokio::test]
async fn fail_insert_collab_with_empty_payload_test() {
  let _guard = REGISTERED_USER_MUTEX.lock().await;

  let mut c = client_api_client();
  c.sign_in_password(&REGISTERED_EMAIL, &REGISTERED_PASSWORD)
    .await
    .unwrap();

  let workspace_id = workspace_id_from_client(&c).await;
  let error = c
    .create_collab(InsertCollabParams::new(
      1,
      Uuid::new_v4().to_string(),
      CollabType::Document,
      vec![],
      workspace_id,
    ))
    .await
    .unwrap_err();

  assert_eq!(error.code, ErrorCode::StorageError);
}

#[tokio::test]
async fn fail_insert_collab_with_invalid_workspace_id_test() {
  let _guard = REGISTERED_USER_MUTEX.lock().await;

  let mut c = client_api_client();
  c.sign_in_password(&REGISTERED_EMAIL, &REGISTERED_PASSWORD)
    .await
    .unwrap();

  let workspace_id = Uuid::new_v4().to_string();
  let raw_data = "hello world".to_string().as_bytes().to_vec();
  let error = c
    .create_collab(InsertCollabParams::new(
      1,
      Uuid::new_v4().to_string(),
      CollabType::Document,
      raw_data.clone(),
      workspace_id,
    ))
    .await
    .unwrap_err();

  assert_eq!(error.code, ErrorCode::StorageError);
}