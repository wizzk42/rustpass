use spectral::prelude::*;

use rustpass::Database;
use std::fs::File;
use std::io::Read;

#[test]
fn test_open_aes256_aeskdf_v31() {
    let mut key_file: File = File::open("tests/fixture/empty-aes256-aeskdf-v31.key")
        .expect(".key is present");

    let mut kdbx_file: File = File::open("tests/fixture/empty-aes256-aeskdf-v31.kdbx")
        .expect(".kdbx is present");

    let mut data: Vec<u8> = vec![];

    let kdbx_result = kdbx_file.read_to_end(&mut data);
    assert_that(&kdbx_result)
        .is_ok()
        .is_greater_than(0);

    let password = None;
    let result = Database::open(
        &mut data.as_ref() as &mut &[u8],
        password,
        Some(&mut key_file)
    );

    assert_that(&result)
        .is_ok();
}
