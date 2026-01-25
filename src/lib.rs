pub mod parser;
pub mod compiler;
pub mod vm;
pub mod constants;


pub fn version() -> (u8, u8, u8) {
    match std::env::var("CARGO_PKG_VERSION") {
        Ok(val) => {
            // ugly as fuck, but this takes our semver as a string and maps it to three u8s
            let ver_vec = val.split(".")
                                        .map(|x| match x.parse::<u8>() {
                                            Ok(y) => y,
                                            Err(_) => panic!("Error parsing version number when building header")
                                    }).collect::<Vec<u8>>();
            assert_eq!(ver_vec.len(), 3);
            (ver_vec[0], ver_vec[1], ver_vec[2])
        },
        Err(_) => {
            panic!("Error fetching version number when building header");
        }
    }
}
