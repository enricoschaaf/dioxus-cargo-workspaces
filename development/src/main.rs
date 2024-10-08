fn main() {
    #[cfg(feature = "web")]
    web::web();

    #[cfg(feature = "server")]
    let _ = server::server();
}
