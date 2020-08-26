use std::path::Path;
use anyhow::Context;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug)]
struct Buffer(Vec<u8>);

impl Buffer {
    fn from_json_value<P: AsRef<Path>>(json: serde_json::Value, path: P) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        struct BufferJSON {
            #[serde(rename(deserialize = "byteLength"))]
            byte_length: usize,
            uri: String
        }
        let BufferJSON {
            byte_length,
            uri
        } = serde_json::from_value(json)?;
        let data = std::fs::read(path.as_ref().join(uri))?;
        if data.len() != byte_length {
            return Err(anyhow::anyhow!("The byte length read form the json and the actual one do not match."))
        }
        Ok(Buffer(data))
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

#[derive(Debug)]
pub struct ParsedGLTF {
    buffers: Vec<Buffer>,
    buffer_views: Vec<BufferView>,
    accessors: Vec<Accessor>
}

pub fn parse_gltf<P: AsRef<Path>>(path: P) -> anyhow::Result<ParsedGLTF> {
    // get the absolute path of the file
    let canonic_path = std::fs::canonicalize(path.as_ref())?;
    // needed to find associated binary files whose paths are relative to this file
    let directory = canonic_path.parent().context("Could not compute the parent path.")?;
    let file_contents = std::fs::read_to_string(path.as_ref())?;
    let json: serde_json::Value = serde_json::from_str(&file_contents)?;
    let buffers = json.
        get("buffers").context("Missing 'buffers' field.")?.
        as_array().context("'buffers' is not array.")?.iter().
        map(|v| Buffer::from_json_value(v.clone(), &directory)).
        collect::<anyhow::Result<_>>()?;
    let buffer_views = json.
        get("bufferViews").context("Missing 'bufferViews' field.")?.
        as_array().context("'bufferViews' is not array.")?.iter().
        map(|v| serde_json::from_value(v.clone()).map_err(From::from)).
        collect::<anyhow::Result<_>>()?;
    let accessors = json.
        get("accessors").context("Missing 'accessors' field.")?.
        as_array().context("'accessors' is not array.")?.iter().
        map(|v| serde_json::from_value(v.clone()).map_err(From::from)).
        collect::<anyhow::Result<_>>()?;
        
    Ok(ParsedGLTF {
        buffers,
        buffer_views,
        accessors
    })
}