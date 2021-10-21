use rbatis::rbatis::Rbatis;

pub async fn create() {
    crate::database::RB.link("postgres://postgres:postgres@localhost:5432/epat_webdb").await.unwrap();
}