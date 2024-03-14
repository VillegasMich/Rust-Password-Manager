extern crate dirs;
pub fn path() -> String {
    let home: String = dirs::home_dir()
        .expect("Home dir not found")
        .to_str()
        .expect("can not convert to str")
        .to_string();
    home + "/rustPM/passwords.json"
}
