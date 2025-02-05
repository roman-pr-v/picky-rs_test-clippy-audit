extern crate num_bigint_dig as num_bigint;

mod pki_tests;

use num_bigint::ToBigInt;
use oid::prelude::*;
use picky_asn1::bit_string::BitString;
use picky_asn1::date::{Date, GeneralizedTime, UTCTime};
use picky_asn1::restricted_string::{IA5String, PrintableString, Utf8String};
use picky_asn1::wrapper::*;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::str::FromStr;

fn check<'de, T>(buffer: &'de [u8], expected: T)
where
    T: Serialize + Deserialize<'de> + PartialEq + Debug,
{
    let parsed: T = picky_asn1_der::from_bytes(&buffer).expect("deserialization failed");
    assert_eq!(parsed, expected);

    let encoded = picky_asn1_der::to_vec(&expected).expect("serialization failed");
    assert_eq!(encoded, buffer);
}

#[test]
fn oid() {
    let oid_buffer = [0x06, 0x05, 0x2B, 0x0E, 0x03, 0x02, 0x1A];
    let oid = ObjectIdentifierAsn1::from(ObjectIdentifier::try_from("1.3.14.3.2.26").unwrap());
    check(&oid_buffer, oid);
}

#[test]
fn bit_string() {
    #[rustfmt::skip]
        let bit_string_buffer = [
        0x03, // tag
        0x81, 0x81, // length
        0x00, // unused bits
        0x47, 0xeb, 0x99, 0x5a, 0xdf, 0x9e, 0x70, 0x0d, 0xfb, 0xa7, 0x31, 0x32, 0xc1, 0x5f, 0x5c, 0x24,
        0xc2, 0xe0, 0xbf, 0xc6, 0x24, 0xaf, 0x15, 0x66, 0x0e, 0xb8, 0x6a, 0x2e, 0xab, 0x2b, 0xc4, 0x97,
        0x1f, 0xe3, 0xcb, 0xdc, 0x63, 0xa5, 0x25, 0xec, 0xc7, 0xb4, 0x28, 0x61, 0x66, 0x36, 0xa1, 0x31,
        0x1b, 0xbf, 0xdd, 0xd0, 0xfc, 0xbf, 0x17, 0x94, 0x90, 0x1d, 0xe5, 0x5e, 0xc7, 0x11, 0x5e, 0xc9,
        0x55, 0x9f, 0xeb, 0xa3, 0x3e, 0x14, 0xc7, 0x99, 0xa6, 0xcb, 0xba, 0xa1, 0x46, 0x0f, 0x39, 0xd4,
        0x44, 0xc4, 0xc8, 0x4b, 0x76, 0x0e, 0x20, 0x5d, 0x6d, 0xa9, 0x34, 0x9e, 0xd4, 0xd5, 0x87, 0x42,
        0xeb, 0x24, 0x26, 0x51, 0x14, 0x90, 0xb4, 0x0f, 0x06, 0x5e, 0x52, 0x88, 0x32, 0x7a, 0x95, 0x20,
        0xa0, 0xfd, 0xf7, 0xe5, 0x7d, 0x60, 0xdd, 0x72, 0x68, 0x9b, 0xf5, 0x7b, 0x05, 0x8f, 0x6d, 0x1e,
    ];
    let bit_string = BitStringAsn1::from(BitString::with_bytes(&bit_string_buffer[4..]));
    check(&bit_string_buffer, bit_string);
}

#[test]
fn encapsulated_types() {
    {
        let buffer = [0x03, 0x6, 0x00, 0x02, 0x03, 0x3c, 0x1c, 0x37];
        let encapsulated: BitStringAsn1Container<u64> = u64::from(3939383u64).into();
        check(&buffer, encapsulated);
    }

    {
        let buffer = [
            0x03, 0x11, 0x00, 0x0c, 0x0e, 0x55, 0x54, 0x46, 0x2d, 0x38, 0xe6, 0x96, 0x87, 0xe5, 0xad, 0x97, 0xe5, 0x88,
            0x97,
        ];
        let encapsulated: BitStringAsn1Container<String> = String::from("UTF-8文字列").into();
        check(&buffer, encapsulated);
    }
}

#[test]
fn big_integer() {
    #[rustfmt::skip]
    let big_integer_buffer = [
        0x02, // tag
        0x81, 0x81, // length
        0x00, // + sign
        0x8f, 0xe2, 0x41, 0x2a, 0x08, 0xe8, 0x51, 0xa8, 0x8c, 0xb3, 0xe8, 0x53, 0xe7, 0xd5, 0x49, 0x50,
        0xb3, 0x27, 0x8a, 0x2b, 0xcb, 0xea, 0xb5, 0x42, 0x73, 0xea, 0x02, 0x57, 0xcc, 0x65, 0x33, 0xee,
        0x88, 0x20, 0x61, 0xa1, 0x17, 0x56, 0xc1, 0x24, 0x18, 0xe3, 0xa8, 0x08, 0xd3, 0xbe, 0xd9, 0x31,
        0xf3, 0x37, 0x0b, 0x94, 0xb8, 0xcc, 0x43, 0x08, 0x0b, 0x70, 0x24, 0xf7, 0x9c, 0xb1, 0x8d, 0x5d,
        0xd6, 0x6d, 0x82, 0xd0, 0x54, 0x09, 0x84, 0xf8, 0x9f, 0x97, 0x01, 0x75, 0x05, 0x9c, 0x89, 0xd4,
        0xd5, 0xc9, 0x1e, 0xc9, 0x13, 0xd7, 0x2a, 0x6b, 0x30, 0x91, 0x19, 0xd6, 0xd4, 0x42, 0xe0, 0xc4,
        0x9d, 0x7c, 0x92, 0x71, 0xe1, 0xb2, 0x2f, 0x5c, 0x8d, 0xee, 0xf0, 0xf1, 0x17, 0x1e, 0xd2, 0x5f,
        0x31, 0x5b, 0xb1, 0x9c, 0xbc, 0x20, 0x55, 0xbf, 0x3a, 0x37, 0x42, 0x45, 0x75, 0xdc, 0x90, 0x65,
    ];

    // from signed bytes

    let big_integer = IntegerAsn1::from_bytes_be_signed(big_integer_buffer[3..].to_vec());
    assert!(big_integer.is_positive());
    assert!(!big_integer.is_negative());
    assert_eq!(big_integer.as_unsigned_bytes_be(), &big_integer_buffer[4..]);
    check(&big_integer_buffer, big_integer);

    // check we have same result using unsigned bytes

    let big_integer = IntegerAsn1::from_bytes_be_unsigned(big_integer_buffer[4..].to_vec());
    assert!(big_integer.is_positive());
    assert!(!big_integer.is_negative());
    assert_eq!(big_integer.as_signed_bytes_be(), &big_integer_buffer[3..]);
    check(&big_integer_buffer, big_integer);
}

#[test]
fn small_integer() {
    let buffer = [0x02, 0x01, 0x03];
    let big_integer = IntegerAsn1::from(3.to_bigint().unwrap().to_signed_bytes_be());

    assert!(big_integer.is_positive());
    assert!(!big_integer.is_negative());
    assert_eq!(big_integer.as_unsigned_bytes_be(), &[0x03]);

    check(&buffer, big_integer);
}

#[test]
fn small_integer_negative() {
    let buffer = [0x02, 0x01, 0xF9];
    let big_integer = IntegerAsn1::from((-7).to_bigint().unwrap().to_signed_bytes_be());

    assert!(!big_integer.is_positive());
    assert!(big_integer.is_negative());
    assert_eq!(big_integer.as_unsigned_bytes_be(), &[0xF9]);

    check(&buffer, big_integer);
}

#[test]
fn date() {
    let buffer = [
        0x17, 0x0D, 0x31, 0x39, 0x31, 0x30, 0x31, 0x37, 0x31, 0x37, 0x34, 0x31, 0x32, 0x38, 0x5A,
    ];
    let timestamp = UTCTimeAsn1(Date::new(2019, 10, 17, 17, 41, 28).unwrap());
    check(&buffer, timestamp);
}

#[test]
fn utc_time() {
    let buffer = [
        0x17, 0x0D, 0x31, 0x39, 0x31, 0x30, 0x31, 0x37, 0x31, 0x37, 0x34, 0x31, 0x32, 0x38, 0x5A,
    ];
    let time: UTCTimeAsn1 = UTCTime::new(2019, 10, 17, 17, 41, 28).unwrap().into();
    check(&buffer, time);
}

#[test]
fn generalized_time() {
    let buffer = [
        0x18, 0x0F, 0x32, 0x30, 0x31, 0x31, 0x31, 0x30, 0x30, 0x36, 0x30, 0x38, 0x33, 0x39, 0x35, 0x36, 0x5A,
    ];
    let time: GeneralizedTimeAsn1 = GeneralizedTime::new(2011, 10, 06, 08, 39, 56).unwrap().into();
    check(&buffer, time);
}

#[test]
fn set_of() {
    #[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq)]
    struct Elem<'a> {
        #[serde(borrow)]
        first_name: Cow<'a, str>,
        #[serde(borrow)]
        last_name: Cow<'a, str>,
    }

    let set_of_elems = Asn1SetOf(vec![
        Elem {
            first_name: "名前".into(),
            last_name: "苗字".into(),
        },
        Elem {
            first_name: "和夫".into(),
            last_name: "田中".into(),
        },
    ]);

    #[rustfmt::skip]
        let buffer = [
        0x31, 0x24,
        0x30, 0x10,
        0x0C, 0x06, 0xE5, 0x90, 0x8D, 0xE5, 0x89, 0x8D,
        0x0C, 0x06, 0xE8, 0x8B, 0x97, 0xE5, 0xAD, 0x97,
        0x30, 0x10,
        0x0C, 0x06, 0xE5, 0x92, 0x8C, 0xE5, 0xA4, 0xAB,
        0x0C, 0x06, 0xE7, 0x94, 0xB0, 0xE4, 0xB8, 0xAD,
    ];

    check(&buffer, set_of_elems);
}

#[test]
fn sequence_of() {
    #[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq)]
    struct Elem<'a> {
        #[serde(borrow)]
        first_name: Cow<'a, str>,
        #[serde(borrow)]
        last_name: Cow<'a, str>,
    }

    let set_of_elems = Asn1SequenceOf(vec![
        Elem {
            first_name: "名前".into(),
            last_name: "苗字".into(),
        },
        Elem {
            first_name: "和夫".into(),
            last_name: "田中".into(),
        },
    ]);

    #[rustfmt::skip]
        let buffer = [
        0x30, 0x24,
        0x30, 0x10,
        0x0C, 0x06, 0xE5, 0x90, 0x8D, 0xE5, 0x89, 0x8D,
        0x0C, 0x06, 0xE8, 0x8B, 0x97, 0xE5, 0xAD, 0x97,
        0x30, 0x10,
        0x0C, 0x06, 0xE5, 0x92, 0x8C, 0xE5, 0xA4, 0xAB,
        0x0C, 0x06, 0xE7, 0x94, 0xB0, 0xE4, 0xB8, 0xAD,
    ];

    check(&buffer, set_of_elems);
}

#[test]
fn application_tag0() {
    let buffer = [0xA0, 0x03, 0x02, 0x01, 0xF9];
    let application_tag = ExplicitContextTag0(IntegerAsn1::from((-7).to_bigint().unwrap().to_signed_bytes_be()));
    check(&buffer, application_tag);
}

#[test]
fn restricted_strings() {
    let printable_string_buffer = b"\x13\x02\x4E\x4C";
    let printable_string = PrintableString::from_str("NL").unwrap();
    check::<PrintableStringAsn1>(printable_string_buffer, printable_string.into());

    let utf8_string_buffer = b"\x0C\x10\x50\x6F\x6C\x61\x72\x53\x53\x4C\x20\x54\x65\x73\x74\x20\x43\x41";
    let utf8_string = Utf8String::from_str("PolarSSL Test CA").unwrap();
    check::<Utf8StringAsn1>(utf8_string_buffer, utf8_string.into());

    let ia5_string_buffer = b"\x16\x10\x50\x6F\x6C\x61\x72\x53\x53\x4C\x20\x54\x65\x73\x74\x20\x43\x41";
    let ia5_string = IA5String::from_str("PolarSSL Test CA").unwrap();
    check::<IA5StringAsn1>(ia5_string_buffer, ia5_string.into());
}

#[test]
fn nested_encapsulators() {
    let buffer = [0xA1, 0x5, 0xA4, 0x3, 0x02, 0x01, 0x05];
    let expected = ExplicitContextTag1(ExplicitContextTag4(u8::from(5)));
    check(&buffer, expected);
}
