#![feature(rust_2018_preview)]
#![feature(nll)]

pub mod block;
pub mod pos;

/*

NbtRead // io::read
NbtWrite
NbtValue
NbtFlate


// create a structure for following NBT read/write
NbtVia 

// decalres `level_name` and `generator_name`
let via = nbt_via!{
    "" compound matches {    
        "Data" compound matches {
            "LevelName" into level_name
            "generatorName" into generator_name 
            "version" matches int 19133
        }
    }
};
// or declare like this:
let mut via = NbtVia::new();
via.split(".");
via.add_type_match("", TYPE_ID_COMPOUND);
via.add_type_match(".Data", TYPE_ID_COMPOUND);
via.add_value_match(".Data.version", NbtTag::Int(19133));
let mut level_name;
via.add_value_parse(".Data.LevelName", &mut level_name);
let mut generator_name;
via.add_value_parse(".Data.generatorName", &mut generator_name);

let mut cur = Cursor::new(buf);
via.parse(&mut cur)?;
// uses `level_name` and `generator_name` which are both instances of NbtTag
println!("Name: {:?}, Generaor: {:?}", level_name, generator_name);

 */
use byteorder::{BigEndian, ReadBytesExt};//, WriteBytesExt};

use std::collections::HashMap;


type NbtMeta = (u8, String);

const TYPE_ID_END: u8 = 0;
const TYPE_ID_BYTE: u8 = 1;
const TYPE_ID_SHORT: u8 = 2;
const TYPE_ID_INT: u8 = 3;
const TYPE_ID_LONG: u8 = 4;
const TYPE_ID_FLOAT: u8 = 5;
const TYPE_ID_DOUBLE: u8 = 6;
const TYPE_ID_BYTE_ARRAY: u8 = 7;
const TYPE_ID_STRING: u8 = 8;
const TYPE_ID_LIST: u8 = 9;
const TYPE_ID_COMPOUND: u8 = 10;
const TYPE_ID_INT_ARRAY: u8 = 11;
const TYPE_ID_LONG_ARRAY: u8 = 12;

#[derive(Debug, PartialEq)]
pub struct NbtData {
    root_name: String,
    root_tag: NbtTag
}

#[derive(Debug, PartialEq)]
pub enum NbtTag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtTag>),
    Compound(HashMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

// impl NbtTag {

//     fn type_id(&self) -> u8 {
//         use self::NbtTag::*;
// 	    match self {
//             End => TYPE_ID_END,
// 	        Byte(_) => TYPE_ID_BYTE,
// 	        Short(_) => TYPE_ID_SHORT,
// 	        Int(_) => TYPE_ID_INT,
// 	        Long(_) => TYPE_ID_LONG,
// 	        Float(_) => TYPE_ID_FLOAT,
// 	        Double(_) => TYPE_ID_DOUBLE,
// 	        ByteArray(_) => TYPE_ID_BYTE_ARRAY,
// 	        String(_) => TYPE_ID_STRING,
// 	        List(_) => TYPE_ID_LIST,
// 	        Compound(_) => TYPE_ID_COMPOUND,
// 	        IntArray(_) => TYPE_ID_INT_ARRAY,
// 	        LongArray(_) => TYPE_ID_LONG_ARRAY,
// 	    }
//     }
// }

use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum NbtError {
    InvalidTypeId(u8),
    InvalidUtf8(FromUtf8Error),
    InvalidRoot,
    IoError(IoError)
}

impl From<IoError> for NbtError {
    #[inline]
    fn from(e: IoError) -> NbtError {
        NbtError::IoError(e)
    }
}

impl From<FromUtf8Error> for NbtError {
    #[inline]
    fn from(e: FromUtf8Error) -> NbtError {
        NbtError::InvalidUtf8(e)
    }
}

pub type NbtResult<T> = Result<T, NbtError>;

pub trait NbtRead {

    fn read_nbt_data(&mut self) -> NbtResult<NbtData>;
}

impl<T> NbtRead for T where T: std::io::Read {

    fn read_nbt_data(&mut self) -> NbtResult<NbtData> {
        let (root_id, root_name) = read_meta(self)?;
        if root_id != TYPE_ID_COMPOUND {
            return Err(NbtError::InvalidRoot);
        }
        let content = read_content(self, root_id)?;
        Ok(NbtData {root_name, root_tag: content})
    }
}

//pub trait NbtWrite {
//
//    fn write_nbt_data(&mut self, data: NbtData) -> NbtResult<()>;
//}








#[inline]
fn read_string<R: std::io::Read>(read: &mut R) -> NbtResult<String> {
    let len = read.read_u16::<BigEndian>()? as usize;
    if len == 0 {
        return Ok("".to_string());
    }
    let mut buf = vec![0; len];
    read.read(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

#[inline]
fn read_meta<R: std::io::Read>(read: &mut R) -> NbtResult<NbtMeta> {
    match read.read_u8()? {
        TYPE_ID_END => Ok((TYPE_ID_END, "".to_string())),
        id @ 1..=12 => Ok((id, read_string(read)?)),
        id => Err(NbtError::InvalidTypeId(id))
    }
}

macro_rules! list_read_len {
    ($read: ident, $len: ident) => {
        let $len = $read.read_i32::<BigEndian>()? as usize;
    };
}

macro_rules! read_array {
    ($func_name: ident, $read_expr: ident, $read_into: ident) => {
#[inline]
fn $func_name<R: std::io::Read>(read: &mut R) -> NbtResult<NbtTag> {
    list_read_len!(read, len);
    let mut buf = vec![0; len];
    for i in 0..len {
        buf[i] = read.$read_expr::<BigEndian>()?;
    }
    Ok(NbtTag::$read_into(buf))
}
    };
}
read_array!(read_int_array_content, read_i32, IntArray);
read_array!(read_long_array_content, read_i64, LongArray);
#[inline]
fn read_byte_array_content<R: std::io::Read>(read: &mut R) -> NbtResult<NbtTag> {
    list_read_len!(read, len);
    let mut buf = vec![0; len];
    for i in 0..len {
        buf[i] = read.read_i8()?;
    }
    Ok(NbtTag::ByteArray(buf))
}

#[inline]
fn read_content<R: std::io::Read>(read: &mut R, type_id: u8) -> NbtResult<NbtTag> {
    match type_id {
        TYPE_ID_BYTE => Ok(NbtTag::Byte(read.read_i8()?)),
        TYPE_ID_SHORT => Ok(NbtTag::Short(read.read_i16::<BigEndian>()?)),
        TYPE_ID_INT => Ok(NbtTag::Int(read.read_i32::<BigEndian>()?)),
        TYPE_ID_LONG => Ok(NbtTag::Long(read.read_i64::<BigEndian>()?)),
        TYPE_ID_FLOAT => Ok(NbtTag::Float(read.read_f32::<BigEndian>()?)),
        TYPE_ID_DOUBLE => Ok(NbtTag::Double(read.read_f64::<BigEndian>()?)),
        TYPE_ID_BYTE_ARRAY => read_byte_array_content(read),
        TYPE_ID_STRING => Ok(NbtTag::String(read_string(read)?)),
        TYPE_ID_LIST => {
            let type_id_elem = read.read_u8()?;
            list_read_len!(read, len);
            let mut buf = Vec::with_capacity(len);
            for _ in 0..len {
                buf.push(read_content(read, type_id_elem)?);
            }
            Ok(NbtTag::List(buf))
        },
        TYPE_ID_COMPOUND => {
            let mut buf = HashMap::new();
            'r: loop {
                match read_meta(read)? {
                    (TYPE_ID_END, _) => break 'r,
                    (type_id_elem, name) => buf.insert(name, read_content(read, type_id_elem)?)
                };
            }
            Ok(NbtTag::Compound(buf))
        },
        TYPE_ID_INT_ARRAY => read_int_array_content(read),
        TYPE_ID_LONG_ARRAY => read_long_array_content(read),
        invalid => Err(NbtError::InvalidTypeId(invalid))
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn read_meta() -> NbtResult<()> {
        let cond = [
            (0, "", vec![0x00]),
            (3, "0", vec![0x03, 0x00, 0x01, b'0']),
            (11, "hello world", vec![
                0x0b, 
                0x00, 0x0b, 
                0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64]),
        ];
        for (ans_id, ans_str, vec) in cond.iter() {
            let mut cur = Cursor::new(vec);
            let read = super::read_meta(&mut cur)?;
            assert_eq!(read.0, *ans_id);
            assert_eq!(read.1, *ans_str);
        }
        Ok(())
    }

    #[test]
    fn read_int_array() -> NbtResult<()> {
        let cond = [
            (vec![], vec![0, 0, 0, 0]),
            (vec![123456789], vec![0, 0, 0, 1, 7, 91, 205, 21]),
            (vec![-1, -2, 3, 4], vec![0, 0, 0, 4, 255, 255, 255, 255, 255, 255, 255, 254, 0, 0, 0, 3, 0, 0, 0, 4])
        ];
        for (input, output) in cond.iter() {
            let mut buf = output;
            let mut cur = Cursor::new(&mut buf);
            let read = read_int_array_content(&mut cur)?;
            assert_eq!(read, NbtTag::IntArray(input.to_vec()));
        }
        Ok(())
    }

    #[test]
    fn read_nbt() -> NbtResult<()> {
        let blob = vec![
0xa, // compound #1
0x0, 0xb, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, // "hello world"
    0x1, // byte
    0x0, 0x5, 0x31, 0x62, 0x79, 0x74, 0x65, // "1byte"
        0x80, // -128
    0x8, // string
    0x0, 0x7, 0x38, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, // "8string"
        0x0, 0x5, 0x68, 0x65, 0x6c, 0x6c, 0x6f, // hello
    0x7, // byte array
    0x0, 0xb, 0x37, 0x62, 0x79, 0x74, 0x65, 0x5f, 0x61, 0x72, 0x72, 0x61, 0x79, // "7byte_array"
    0x0, 0x0, 0x0, 0x4, //len = 4
        0xc, 0xde, 0x38, 0xb2, // [12, -34, 56, -78]
    0x9, // list tag
    0x0, 0x9, 0x39, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x69, 0x6e, 0x74, // "9list_int"
    0x3, // inner type: int
    0x0, 0x0, 0x0, 0x3, // len: 3 
        0x7f, 0xff, 0xff, 0xff, 
        0x6e, 0xee, 0xee, 0xee, 
        0x5d, 0xdd, 0xdd, 0xdd,
    0x5, // float
    0x0, 0x6, 0x35, 0x66, 0x6c, 0x6f, 0x61, 0x74,  // "5float"
        0x40, 0x49, 0xf, 0xdb, // float value of math constant PI
    0x4, // long
    0x0, 0x5, 0x34, 0x6c, 0x6f, 0x6e, 0x67, // "4long"
        0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // i64::min_value()
    0x6, // double
    0x0, 0x7, 0x36, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, // "6double"
        0x40, 0x5, 0xbf, 0xa, 0x8b, 0x14, 0x57, 0x69, // float value of math constant E
    0x2, // short
    0x0, 0x6, 0x32, 0x73, 0x68, 0x6f, 0x72, 0x74, // "2short"
        0x7f, 0xff, // i16::max_value()
    0x3, // int
    0x0, 0x4, 0x33, 0x69, 0x6e, 0x74, // "3int"
        0x7f, 0xff, 0xff, 0xff, // i16::max_value()
    0xa, // compund #2
    0x0, 0x9, 0x31, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, // "1compund"
        0x5, // float
        0x0, 0xb, 0x31, 0x31, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x31, 0x2e, 0x30, // "11float_1.0"
            0x3f, 0x80, 0x0, 0x0, // 1.0
        0x6, // double
        0x0, 0xd, 0x31, 0x32, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x2d, 0x31, 0x2e, 0x30, // "12double_-1.0"
            0xbf, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // -1.0
        0x0, // end of #2
    0xb, // int array
    0x0, 0xa, 0x32, 0x69, 0x6e, 0x74, 0x5f, 0x61, 0x72, 0x72, 0x61, 0x79, // "2int_array" 
    0x0, 0x0, 0x0, 0x4, // len: 4
        0x1a, 0xaa, 0xaa, 0xaa, 
        0x2b, 0xbb, 0xbb, 0xbb, 
        0x2c, 0xcc, 0xcc, 0xcc, 
        0x1d, 0xdd, 0xdd, 0xdd, 
    0x0, // end of #1
];      
        let mut inner_map = HashMap::new();
        inner_map.insert("11float_1.0".to_string(), NbtTag::Float(1.0));
        inner_map.insert("12double_-1.0".to_string(), NbtTag::Double(-1.0));
        let mut root_map = HashMap::new();    
        root_map.insert("1byte".to_string(), NbtTag::Byte(i8::min_value()));
        root_map.insert("2short".to_string(), NbtTag::Short(i16::max_value()));
        root_map.insert("3int".to_string(), NbtTag::Int(i32::max_value()));
        root_map.insert("4long".to_string(), NbtTag::Long(i64::min_value()));
        root_map.insert("5float".to_string(), NbtTag::Float(std::f32::consts::PI));
        root_map.insert("6double".to_string(), NbtTag::Double(std::f64::consts::E));
        root_map.insert("7byte_array".to_string(), NbtTag::ByteArray(vec![12, -34, 56, -78]));
        root_map.insert("8string".to_string(), NbtTag::String("hello".to_string()));   
        root_map.insert("9list_int".to_string(), NbtTag::List(vec![
            NbtTag::Int(0x7FFFFFFF),
            NbtTag::Int(0x6EEEEEEE),
            NbtTag::Int(0x5DDDDDDD),
        ]));
        root_map.insert("1compound".to_string(), NbtTag::Compound(inner_map));
        root_map.insert("2int_array".to_string(), NbtTag::IntArray(vec![
            0x1AAAAAAA, 0x2BBBBBBB, 0x2CCCCCCC, 0x1DDDDDDD
        ]));
        let correct = NbtData {
            root_name: String::from("hello world"),
            root_tag: NbtTag::Compound(root_map)
        };
        let mut cur = Cursor::new(blob);
        let data = cur.read_nbt_data()?;
        assert_eq!(data, correct);
        Ok(())
    }
}
