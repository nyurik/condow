use condow_fs::{Condow, FsClient};

fn create_condow_condow() -> Condow<FsClient> {
    FsClient::condow(Default::default()).unwrap()
}

fn get_test_file_path() -> String {
    format!(
        "{}/tests/test_data",
        std::env::current_dir().unwrap().display()
    )
}

#[tokio::test]
async fn download_full() {
    let condow = create_condow_condow();

    let data = condow
        .download(get_test_file_path(), ..)
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    assert_eq!(&data[..], b"abcdefghijklmnopqrstuvwxyz");
}

#[tokio::test]
async fn download_to() {
    let condow = create_condow_condow();

    let data = condow
        .download(get_test_file_path(), ..5)
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    assert_eq!(&data[..], b"abcde");
}

#[tokio::test]
async fn download_to_end() {
    let condow = create_condow_condow();

    let data = condow
        .download(get_test_file_path(), ..=26)
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    assert_eq!(&data[..], b"abcdefghijklmnopqrstuvwxyz");
}

#[tokio::test]
async fn download_from() {
    let condow = create_condow_condow();

    let data = condow
        .download(get_test_file_path(), 10..)
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    assert_eq!(&data[..], b"klmnopqrstuvwxyz");
}

#[tokio::test]
async fn download_from_to() {
    let condow = create_condow_condow();

    let data = condow
        .download(get_test_file_path(), 1..11)
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    assert_eq!(&data[..], b"bcdefghijk");
}
