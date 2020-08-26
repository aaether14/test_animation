use std::path::Path;
use anyhow::Context;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
struct Buffer {
    #[serde(rename(deserialize = "byteLength"))]
    byte_length: usize,
    uri: String
}

impl Buffer {
    fn get_data(&self) -> anyhow::Result<Vec<u8>> {
        let data = std::fs::read(&self.uri)?;
        if data.len() != self.byte_length {
            return Err(anyhow::anyhow!("The byte length read form the json and the actual one do not match."))
        }
        Ok(data)
    }
}

#[derive(Debug, Deserialize)]
struct BufferView {
    buffer: usize,
    #[serde(rename(deserialize = "byteOffset"))]
    byte_offset: usize,
    #[serde(rename(deserialize = "byteLength"))]
    byte_length: usize,
    #[serde(rename(deserialize = "byteStride"), default)]
    byte_stride: usize
}

#[allow(non_camel_case_types)] 
#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
enum ComponentType {
    BYTE = 5120,
    UNSIGNED_BYTE = 5121,
    SHORT = 5122,
    UNSIGNED_SHORT = 5123,
    UNSIGNED_INT = 5125,
    FLOAT = 5126
}

#[derive(Debug, Deserialize)]
enum AccessorType {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    MAT2,
    MAT3,
    MAT4
}

#[derive(Debug, Deserialize)]
struct Accessor {
    #[serde(rename(deserialize = "bufferView"))]
    buffer_view: usize,
    #[serde(rename(deserialize = "byteOffset"), default)]
    byte_offset: usize,
    #[serde(rename(deserialize = "type"))]
    data_type: AccessorType,
    #[serde(rename(deserialize = "componentType"))]
    component_type: ComponentType,
    count: usize
}

#[derive(Debug, Deserialize)]
pub struct ParsedGLTF {
    buffers: Vec<Buffer>,
    #[serde(rename(deserialize = "bufferViews"))]
    buffer_views: Vec<BufferView>,
    accessors: Vec<Accessor>
}

pub fn parse_gltf<P: AsRef<Path>>(path: P) -> anyhow::Result<ParsedGLTF> {
    // get the absolute path of the file
    let canonic_path = std::fs::canonicalize(path.as_ref())?;
    // needed to find associated binary files whose paths are relative to this file
    let directory = canonic_path.parent().context("Could not compute the parent path.")?;
    let file = std::fs::File::open(path)?;
    let mut result: ParsedGLTF = serde_json::from_reader(file)?;
    for b in &mut result.buffers {
        // update uri to contain the absolute path to the resource
        b.uri = directory.join(&b.uri).to_str().context("Could not set uri.")?.to_string();
    }
    Ok(result)
}