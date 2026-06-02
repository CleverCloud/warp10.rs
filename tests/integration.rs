use testcontainers::{
    GenericImage, ImageExt,
    core::{ContainerPort, IntoContainerPort, WaitFor, wait::HttpWaitStrategy},
    runners::AsyncRunner,
};
use warp10::{Client, Data, Label, Value};

async fn start_warp10() -> (testcontainers::ContainerAsync<GenericImage>, Client) {
    let container = GenericImage::new("warp10io/warp10", "3.5.0-ubuntu-ci")
        .with_exposed_port(8080.tcp())
        .with_wait_for(WaitFor::http(
            HttpWaitStrategy::new("/api/v0/check")
                .with_port(ContainerPort::Tcp(8080))
                .with_expected_status_code(200u16),
        ))
        .with_startup_timeout(std::time::Duration::from_secs(60))
        .start()
        .await
        .expect("failed to start warp10 container");

    let host = container.get_host().await.expect("failed to get host");
    let port = container
        .get_host_port_ipv4(8080.tcp())
        .await
        .expect("failed to get port");

    let url = format!("http://{}:{}", host, port);

    let client = Client::builder()
        .url(&url)
        .read_token("readTokenCI")
        .write_token("writeTokenCI")
        .build()
        .expect("failed to build client");

    (container, client)
}

#[tokio::test]
#[ignore]
async fn exec_simple_arithmetic() {
    let (_container, client) = start_warp10().await;

    let response = client.exec("1 2 +").await.unwrap();

    assert_eq!(response.body, serde_json::json!([3]));
    assert!(response.meta.elapsed.is_some());
    assert!(response.meta.ops.is_some());
}

#[tokio::test]
#[ignore]
async fn exec_error_returns_detail() {
    let (_container, client) = start_warp10().await;

    let err = client.exec("INVALID_FUNCTION_THAT_DOES_NOT_EXIST").await;

    match err {
        Err(warp10::Error::Exec(detail)) => {
            assert!(!detail.message.is_empty());
        }
        other => panic!("expected Exec error, got: {:?}", other),
    }
}

#[tokio::test]
#[ignore]
async fn write_and_exec_roundtrip() {
    let (_container, client) = start_warp10().await;
    let writer = client.get_writer();

    let data = vec![Data::new(
        time::OffsetDateTime::now_utc(),
        None,
        "test.integration.roundtrip".to_string(),
        vec![Label::new("source", "rust-test")],
        Value::Long(42),
    )];

    writer.post(data).await.unwrap();

    let script = format!(
        "[ '{}' 'test.integration.roundtrip' {{}} NOW -1 ] FETCH",
        "readTokenCI"
    );
    let response = client.exec(&script).await.unwrap();

    assert!(response.body.is_array());
    assert!(!response.body.as_array().unwrap().is_empty());
}

#[tokio::test]
#[ignore]
async fn find_series() {
    let (_container, client) = start_warp10().await;
    let writer = client.get_writer();

    let data = vec![Data::new(
        time::OffsetDateTime::now_utc(),
        None,
        "test.integration.find".to_string(),
        vec![Label::new("source", "rust-test")],
        Value::Long(1),
    )];

    writer.post(data).await.unwrap();

    let response = client
        .find("test.integration.find{source=rust-test}")
        .await
        .unwrap();

    assert!(!response.series().is_empty());
}

#[tokio::test]
#[ignore]
async fn exec_meta_headers_present() {
    let (_container, client) = start_warp10().await;

    let response = client.exec("1 2 + 3 4 +").await.unwrap();

    assert!(response.meta.elapsed.is_some());
    assert!(response.meta.ops.is_some());
}
