pub async fn select_file() {
    let _file = rfd::AsyncFileDialog::new()
        .set_directory(dirs::home_dir().expect("failed to fetch home dir"))
        .pick_file()
        .await;
}
