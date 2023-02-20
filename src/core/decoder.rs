/*
    I'm trying to have a dynamic Decoder that works with serde
    and everything that supports serde, the decoder should be
    defined at startup of a node and then use that definition to decode
    the structs that support deserialize

    See encoder since it works the way I want it with the erased_serde crate
*/

use anyhow::Result;
use erased_serde::Deserializer;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::de::SliceRead;
use std::{
    fmt::Debug,
    io::Cursor,
    sync::{Arc, Mutex},
};

pub type DynDecoder = Box<dyn Decoder<'static>>;

pub trait Decoder<'a>: Send + Sync + Debug {
    fn decode(&'a mut self, data: &'a [u8]) -> Box<dyn Deserializer<'a> + 'a>;
}

// pub trait DecoderClone {
//     fn clone_box(&self) -> Box<dyn Decoder>;
// }

// impl<T> DecoderClone for T
// where
//     T: 'static + Decoder + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Decoder> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn Decoder> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

pub struct JsonDecoder<'a> {
    json_de: serde_json::Deserializer<SliceRead<'a>>,
}

impl<'a> JsonDecoder<'a> {
    pub fn new() -> Self {
        JsonDecoder {
            json_de: serde_json::Deserializer::from_slice(&[]),
        }
    }
}

impl<'a> std::fmt::Debug for JsonDecoder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonDecoder")
    }
}

impl<'a> Decoder<'a> for JsonDecoder<'a> {
    fn decode(&'a mut self, data: &'a [u8]) -> Box<dyn Deserializer<'a> + 'a> {
        self.json_de = serde_json::Deserializer::from_slice(data);
        let de = <dyn Deserializer>::erase(&mut self.json_de);
        Box::new(de)
    }
}

pub fn from_decoder<'a, T: DeserializeOwned>(
    decoder: &'a mut dyn Decoder<'a>,
    data: &'a [u8],
) -> Result<T> {
    let mut de = decoder.decode(data);
    Ok(erased_serde::deserialize(&mut de)?)
}

// #[derive(Serialize, Deserialize, Clone)]
// pub struct MyTest {}

// fn test_my_test_decode() {
//     let mut decoder = JsonDecoder {
//         json_de: serde_json::Deserializer::from_slice(&[]),
//     };

//     let data = vec![];

//     let my_test: MyTest = test_from_decoder(&mut decoder, &data);
// }

// fn test_from_decoder<'a, T: DeserializeOwned>(
//     decoder: &'a mut dyn Decoder<'a>,
//     data: &'a [u8],
// ) -> T {
//     let mut de = decoder.decode(data);
//     erased_serde::deserialize(&mut de).unwrap()
// }
