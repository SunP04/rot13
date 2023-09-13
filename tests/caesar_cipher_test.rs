use rot_thirteen_encoder::{caesar_cipher::CaesarCipher, Cipher};

#[test]
fn it_can_create_objects() {
    let (word, steps) = (String::from("Hello"), 13);
    let sut = CaesarCipher::new(word.clone(), steps);

    assert_eq!(word, sut.word);
    assert_eq!(steps, sut.move_amount);
}

#[test]
fn it_can_encode_strings() {
    let (word, steps) = (String::from("Hello"), 13);
    let sut = CaesarCipher::new(word.clone(), steps);

    let result = sut.encode();
    assert_eq!(String::from("Uryyb"), result);
}

#[test]
fn it_can_decode_strings() {
    let (word, steps) = (String::from("Uryyb"), 13);
    let sut = CaesarCipher::new(word.clone(), steps);

    let result = sut.decode();
    assert_eq!(String::from("Hello"), result);
}
