use std::{path::Path, io::Read};

use flate2::bufread::DeflateDecoder;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use super::{diagram::{CompressDiagram, Diagram}, mxgraphmodel::MxGraphModel};

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
#[serde(rename = "mxfile")]
pub struct Mxfile {
    pub host: String,
    #[serde(rename = "diagram", default)]
    pub diagrams: Vec<Diagram>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MxfileWithCompressDiagrams {
    #[serde(rename = "diagram", default)]
    pub diagrams: Vec<CompressDiagram>,
}



pub fn read_file(path: &Path) -> Result<Mxfile> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("can read content of {}", path.display()))?;
    match content.is_empty() {
        true => Ok(Mxfile::default()),
        false => parse_compressed_content(path, content.clone())
            .or_else(|_| parse_uncompressed_content(content.as_str())),
    }
}

fn parse_compressed_content(path: &Path, content: String) -> Result<Mxfile> {
    let mxfile_with_compressed_diagrams: MxfileWithCompressDiagrams =
        fast_xml::de::from_reader(content.as_bytes())
            .with_context(|| format!("can parse xml on {}", path.display()))?;
    let mxfile = decompress(mxfile_with_compressed_diagrams)
        .with_context(|| format!("can uncompress xml on {}", path.display()))?;
    Ok(mxfile)
}

fn decompress(mxfile_with_compressed_diagrams: MxfileWithCompressDiagrams) -> Result<Mxfile> {
    let mut diagrams: Vec<Diagram> = vec![];
    for compressed_diagram in mxfile_with_compressed_diagrams.diagrams {
        let base64_raw_diagram = base64::decode(compressed_diagram.raw_diagram)?;

        let mut raw_diagram_deflate_decoder = DeflateDecoder::new(&base64_raw_diagram[..]);
        let mut urlencoded_diagram = String::new();
        raw_diagram_deflate_decoder.read_to_string(&mut urlencoded_diagram)?;

        let xml_diagram = urlencoding::decode(urlencoded_diagram.as_str())?;

        let mx_graph_model: MxGraphModel = fast_xml::de::from_reader(xml_diagram.as_bytes())?;

        diagrams.push(Diagram {
            id: compressed_diagram.id,
            name: compressed_diagram.name,
            mx_graph_model,
        })
    }

    Ok(Mxfile {host: Default::default(), diagrams })
}

pub fn parse_uncompressed_content(content: &str) -> Result<Mxfile> {
    let mxfile: Mxfile = fast_xml::de::from_reader(content.as_bytes())
        .with_context(|| format!("can parse xml on "))?;
    Ok(mxfile)
}