fn main() {
    if std::env::var("TARGET").is_ok_and(|target| target.contains("espidf")) {
        embuild::espidf::sysenv::output();
    }
}
