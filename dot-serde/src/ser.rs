use std::{borrow::Cow, io::Write};

use serde::{ser, Serialize};

use crate::{
    error::{Error, Result},
    util::Counter,
};

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer)?;
    value.serialize(&mut ser)?;

    ser.end()?;

    Ok(())
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut out = Vec::new();
    to_writer(&mut out, value)?;
    Ok(out)
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    Ok(String::from_utf8(to_vec(value)?).expect("Internal UTF8 error"))
}

type CowStr = Cow<'static, str>;

pub struct Serializer<W> {
    writer: W,
    current_node: Option<Node>,
    node_counter: Counter,
}

impl<W: Write> Serializer<W> {
    pub fn new(mut writer: W) -> Result<Self> {
        writeln!(&mut writer, "digraph {{")?;

        Ok(Self {
            writer,
            current_node: None,
            node_counter: Counter::new(),
        })
    }

    pub fn end(mut self) -> Result<W> {
        if self.current_node.is_some() {
            self.end_node(None)?;
        }

        writeln!(&mut self.writer, "}}")?;
        Ok(self.writer)
    }

    fn serialize_key(&mut self, key: CowStr) -> Result<()> {
        let node = self
            .current_node
            .as_mut()
            .unwrap_or_else(|| panic!("Missing node when attempting to serialize key {}", &key));

        if let Some(current_key) = node.current_key.as_ref() {
            panic!(
                "Missing value for key {} while trying to insert key {}",
                current_key, &key
            );
        }

        node.current_key = Some(key);

        Ok(())
    }

    fn serialize_value(&mut self, value: CowStr) -> Result<()> {
        if let Some(node) = self.current_node.as_mut() {
            let key = node.take_key();
            node.fields.push(NodeField { key, value });
        } else {
            self.current_node = Some(Node::new(value, &mut self.node_counter)?);
        }

        Ok(())
    }

    /// Returns the previous node
    fn start_node(&mut self, name: CowStr) -> Result<Option<Node>> {
        Ok(self
            .current_node
            .replace(Node::new(name, &mut self.node_counter)?))
    }

    fn end_node(&mut self, mut prev_node: Option<Node>) -> Result<()> {
        let node = self
            .current_node
            .take()
            .unwrap_or_else(|| panic!("Missing node when attempting to end"));

        // TODO Emit node
        write!(self.writer, "node_{} [", node.id)?;

        if node.fields.is_empty() {
            write!(self.writer, "shape=ellipse, label=\"{}\"]", &node.name)?;
        } else {
            write!(
                self.writer,
                r#"shape=none,margin=0,label=<<table><tr><td colspan="2">{}</td></tr>"#,
                htmlescape::encode_minimal(&node.name),
            )?;

            for field in node.fields.iter() {
                write!(
                    self.writer,
                    "<tr><td>{}</td><td>{}</td></tr>",
                    htmlescape::encode_minimal(&field.key),
                    htmlescape::encode_minimal(&field.value),
                )?;
            }

            writeln!(self.writer, r#"</table>>]"#)?;
        }

        if let Some(prev_node) = prev_node.as_mut() {
            let parent_key = prev_node.take_key();
            writeln!(
                self.writer,
                "node_{} -> node_{} [label=<{}>]",
                prev_node.id, node.id, parent_key,
            )?;
        }

        self.current_node = prev_node;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    id: u64,
    name: CowStr,
    fields: Vec<NodeField>,
    current_key: Option<CowStr>,
}

impl Node {
    fn new(name: Cow<'static, str>, node_counter: &mut Counter) -> Result<Self> {
        Ok(Self {
            name,
            id: node_counter.next()?,
            fields: Vec::new(),
            current_key: None,
        })
    }

    fn take_key(&mut self) -> CowStr {
        self.current_key
            .take()
            .unwrap_or_else(|| panic!("Missing key"))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NodeField {
    key: CowStr,
    value: CowStr,
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = SerializerSeq<'a, W>;
    type SerializeTuple = SerializerTuple<'a, W>;
    type SerializeTupleStruct = SerializerTupleStruct<'a, W>;
    type SerializeTupleVariant = SerializerTupleVariant<'a, W>;
    type SerializeMap = SerializerMap<'a, W>;
    type SerializeStruct = SerializerStruct<'a, W>;
    type SerializeStructVariant = SerializerStructVariant<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_value(Cow::Borrowed(if v { "true" } else { "false" }))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_value(Cow::Owned(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_value(Cow::Owned(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.serialize_value(Cow::Owned(v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_value(Cow::Owned(format!("'{}'", v)))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_value(Cow::Owned(format!("\"{}\"", v)))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.serialize_value(Cow::Owned(format!("<{} bytes>", v.len())))
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.serialize_value(Cow::Borrowed("null"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        let prev_node = self.start_node(Cow::Borrowed(name))?;
        Ok(self.end_node(prev_node)?)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_value(Cow::Owned(format!("{}::{}", name, variant)))
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let prev_node = self.start_node(Cow::Borrowed(name))?;
        value.serialize(&mut *self)?;
        self.end_node(prev_node)?;

        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let prev_node = self.start_node(Cow::Owned(format!("{}::{}", name, variant)))?;
        self.serialize_key(Cow::Borrowed(".0"))?;
        value.serialize(&mut *self)?;
        self.end_node(prev_node)?;

        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        SerializerSeq::start(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        SerializerTuple::start(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        SerializerTupleStruct::start(self, name)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        SerializerTupleVariant::start(self, name, variant)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        SerializerMap::start(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        SerializerStruct::start(self, name)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        SerializerStructVariant::start(self, name, variant)
    }
}

pub struct SerializerSeq<'a, W> {
    ser: &'a mut Serializer<W>,
    key: CowStr,
    idx: usize,
}

impl<'a, W: Write> SerializerSeq<'a, W> {
    fn start(ser: &'a mut Serializer<W>) -> Result<Self> {
        let key = if let Some(node) = ser.current_node.as_mut() {
            node.take_key()
        } else {
            ser.start_node(Cow::Borrowed("<SEQUENCE>"))?;
            Cow::Borrowed("")
        };

        Ok(Self { ser, key, idx: 0 })
    }
}

impl<'a, W: Write> ser::SerializeSeq for SerializerSeq<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.current_node.as_mut().unwrap().current_key =
            Some(Cow::Owned(format!("{}[{}]", self.key, self.idx)));
        value.serialize(&mut *self.ser)?;

        self.idx += 1;

        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.idx == 0 {
            self.ser.current_node.as_mut().unwrap().current_key = Some(self.key);
            self.ser.serialize_value(Cow::Borrowed("<EMPTY>"))?;
        }

        if let Some(node) = self.ser.current_node.as_mut() {
            node.current_key = None;
        }

        Ok(())
    }
}

pub struct SerializerTuple<'a, W> {
    ser: &'a mut Serializer<W>,
    key: CowStr,
    idx: usize,
}

impl<'a, W: Write> SerializerTuple<'a, W> {
    fn start(ser: &'a mut Serializer<W>) -> Result<Self> {
        let key = if let Some(node) = ser.current_node.as_mut() {
            node.take_key()
        } else {
            ser.start_node(Cow::Borrowed("<TUPLE>"))?;
            Cow::Borrowed("")
        };

        Ok(Self { ser, key, idx: 0 })
    }
}

impl<'a, W: Write> ser::SerializeTuple for SerializerTuple<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.current_node.as_mut().unwrap().current_key =
            Some(Cow::Owned(format!("{}.{}", self.key, self.idx)));
        value.serialize(&mut *self.ser)?;

        self.idx += 1;

        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.idx == 0 {
            self.ser.current_node.as_mut().unwrap().current_key = Some(self.key);
            self.ser.serialize_value(Cow::Borrowed("<EMPTY>"))?;
        }

        if let Some(node) = self.ser.current_node.as_mut() {
            node.current_key = None;
        }

        Ok(())
    }
}

pub struct SerializerTupleStruct<'a, W> {
    ser: &'a mut Serializer<W>,
    idx: usize,
    prev_node: Option<Node>,
}

impl<'a, W: Write> SerializerTupleStruct<'a, W> {
    fn start(ser: &'a mut Serializer<W>, name: &'static str) -> Result<Self> {
        let prev_node = ser.start_node(Cow::Borrowed(name))?;

        Ok(Self {
            prev_node,
            ser,
            idx: 0,
        })
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for SerializerTupleStruct<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.current_node.as_mut().unwrap().current_key =
            Some(Cow::Owned(format!(".{}", self.idx)));
        value.serialize(&mut *self.ser)?;

        self.idx += 1;

        Ok(())
    }

    fn end(self) -> Result<()> {
        if let Some(node) = self.ser.current_node.as_mut() {
            node.current_key = None;
        }

        self.ser.end_node(self.prev_node)?;

        Ok(())
    }
}

pub struct SerializerTupleVariant<'a, W> {
    ser: &'a mut Serializer<W>,
    idx: usize,
    prev_node: Option<Node>,
}

impl<'a, W: Write> SerializerTupleVariant<'a, W> {
    fn start(
        ser: &'a mut Serializer<W>,
        name: &'static str,
        variant: &'static str,
    ) -> Result<Self> {
        let prev_node = ser.start_node(Cow::Owned(format!("{}::{}", name, variant)))?;

        Ok(Self {
            prev_node,
            ser,
            idx: 0,
        })
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for SerializerTupleVariant<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.current_node.as_mut().unwrap().current_key =
            Some(Cow::Owned(format!(".{}", self.idx)));
        value.serialize(&mut *self.ser)?;

        self.idx += 1;

        Ok(())
    }

    fn end(self) -> Result<()> {
        if let Some(node) = self.ser.current_node.as_mut() {
            node.current_key = None;
        }

        self.ser.end_node(self.prev_node)?;

        Ok(())
    }
}

pub struct SerializerMap<'a, W> {
    ser: &'a mut Serializer<W>,
    prev_node: Option<Node>,
}

impl<'a, W: Write> SerializerMap<'a, W> {
    fn start(ser: &'a mut Serializer<W>) -> Result<Self> {
        Ok(Self {
            prev_node: ser.start_node(Cow::Borrowed("<MAP>"))?,
            ser,
        })
    }
}

impl<'a, W: Write> ser::SerializeMap for SerializerMap<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // HACK There's likely a much better way to check that the key is a string without relying on serde_json to stringify it
        // but that's hard, so this'll do for now.
        self.ser.serialize_key(Cow::Owned(
            serde_json::to_string(key).map_err(|e| <Error as ser::Error>::custom(e))?,
        ))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        self.ser.end_node(self.prev_node)
    }
}

pub struct SerializerStruct<'a, W> {
    ser: &'a mut Serializer<W>,
    prev_node: Option<Node>,
}

impl<'a, W: Write> SerializerStruct<'a, W> {
    fn start(ser: &'a mut Serializer<W>, name: &'static str) -> Result<Self> {
        Ok(Self {
            prev_node: ser.start_node(Cow::Borrowed(name))?,
            ser,
        })
    }
}

impl<'a, W: Write> ser::SerializeStruct for SerializerStruct<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.serialize_key(Cow::Borrowed(key))?;
        value.serialize(&mut *self.ser)?;

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.ser.end_node(self.prev_node)
    }
}

pub struct SerializerStructVariant<'a, W> {
    ser: &'a mut Serializer<W>,
    prev_node: Option<Node>,
}

impl<'a, W: Write> SerializerStructVariant<'a, W> {
    fn start(
        ser: &'a mut Serializer<W>,
        name: &'static str,
        variant: &'static str,
    ) -> Result<Self> {
        Ok(Self {
            prev_node: ser.start_node(Cow::Owned(format!("{}::{}", name, variant)))?,
            ser,
        })
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for SerializerStructVariant<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.serialize_key(Cow::Borrowed(key))?;
        value.serialize(&mut *self.ser)?;

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.ser.end_node(self.prev_node)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::{fs, path::PathBuf, process::Command};

    use serde::Serialize;

    fn graph<T: Serialize>(value: &T, name: &str) {
        let dir = PathBuf::from("/tmp/dot-serde-tests");
        fs::create_dir_all(&dir).unwrap();

        println!("Graphing {}", name);

        let graph_path = dir.join(&format!("{}.dot", name));
        fs::write(&graph_path, to_string(value).unwrap()).unwrap();

        let out = Command::new("dot")
            .arg("-T")
            .arg("png")
            .arg("-O")
            .arg(&graph_path)
            .output()
            .unwrap();

        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        graph(&test, "struct");
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        graph(&E::Unit, "enum_unit");
        graph(&E::Newtype(1), "enum_newtype");
        graph(&E::Tuple(1, 2), "enum_tuple");
        graph(&E::Struct { a: 1 }, "enum_struct");
    }

    #[test]
    fn test_nested() {
        #[derive(Serialize)]
        enum E {
            One(Box<E>),
            Two(Box<E>, Box<E>),
            None,
        }

        graph(&E::None, "nested_none");
        graph(&E::One(Box::new(E::None)), "nested_one");
        graph(&E::Two(Box::new(E::None), Box::new(E::None)), "nested_two");
        graph(
            &E::Two(Box::new(E::One(Box::new(E::None))), Box::new(E::None)),
            "nested_three",
        );
        graph(
            &E::Two(
                Box::new(E::One(Box::new(E::None))),
                Box::new(E::One(Box::new(E::None))),
            ),
            "nested_four",
        );
        graph(
            &E::Two(
                Box::new(E::Two(
                    Box::new(E::None),
                    Box::new(E::One(Box::new(E::None))),
                )),
                Box::new(E::One(Box::new(E::None))),
            ),
            "nested_seven",
        );
    }
}
