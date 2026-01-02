use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::{Error, Result};

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let target_type = ChunkType::from_str(chunk_type)?;

        let index = self
            .chunks
            .iter()
            .position(|c| c.chunk_type() == &target_type)
            .ok_or_else(|| Error::from("Chunk type not found"))?;

        Ok(self.chunks.remove(index))
    }

    pub fn header(&self) -> &[u8; 8] {
        &Self::STANDARD_HEADER
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        match ChunkType::from_str(chunk_type) {
            Ok(chunk_type) => self.chunks.iter().find(|c| c.chunk_type() == &chunk_type),
            Err(_) => None,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&Self::STANDARD_HEADER);

        for chunk in &self.chunks {
            bytes.extend_from_slice(&chunk.as_bytes());
        }

        bytes
    }
}
