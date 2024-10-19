use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    chunk_type: Vec<u8>,
}

#[allow(dead_code)]
impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        let mut result: [u8; 4] = [0; 4];
        result[0] = self.chunk_type[0];
        result[1] = self.chunk_type[1];
        result[2] = self.chunk_type[2];
        result[3] = self.chunk_type[3];
        result
    }

    fn is_valid(&self) -> bool {
        let array = self.chunk_type.iter();
        for i in array {
            if !i.is_ascii_uppercase() && !i.is_ascii_lowercase() {
                return false;
            }
        }
        if !self.is_reserved_bit_valid() {
            return false;
        }
        return true;
    }

    fn is_safe_to_copy(&self) -> bool {
        if self.chunk_type[3].is_ascii_lowercase() {
            return true;
        }
        return false;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        if self.chunk_type[2].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    fn is_public(&self) -> bool {
        if self.chunk_type[1].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    fn is_critical(&self) -> bool {
        if self.chunk_type[0].is_ascii_uppercase() {
            return true;
        }
        return false;
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let mut to_show: String;
        for c in self.chunk_type.iter() {
            let character = *c as char;
            write!(f, "{}", character)?
        }
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Vec<u8> = Vec::new();

        for character in s.chars() {
            if character.is_ascii_alphabetic() {
                let byte: u8 = character as u8;
                result.push(byte);
            } else {
                return Err("Error generating Chunk Type from string slice".into());
            }
        }
        Ok(Self { chunk_type: result })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let value_iter = value.into_iter();
        for c in value_iter {
            if !c.is_ascii() {
                return Err("Error generating Chunk Type from u8 array".into());
            }
        }
        Ok(Self {
            chunk_type: value.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
