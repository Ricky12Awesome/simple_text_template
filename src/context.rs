use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Context {
  contents: HashMap<String, Value>,
}

impl Context {
  pub fn get_value<S>(&self, value: S) -> Option<&Value>
  where
    S: ToString,
  {
    self.contents.get(&value.to_string())
  }

  pub fn get_bool<S>(&self, value: S) -> Option<bool>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::Bool(boolean) => Some(*boolean),
      _ => None,
    }
  }

  pub fn get_string<S>(&self, value: S) -> Option<&String>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::String(string) => Some(string),
      _ => None,
    }
  }

  pub fn get_list<S>(&self, value: S) -> Option<&Vec<Value>>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::List(list) => Some(list),
      _ => None,
    }
  }

  pub fn get_object<S>(&self, value: S) -> Option<&HashMap<String, Value>>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::Object(object) => Some(object),
      _ => None,
    }
  }
}

pub trait Builder<T>: Sized {
  fn contents(&self) -> &HashMap<String, Value>;
  fn contents_mut(&mut self) -> &mut HashMap<String, Value>;

  fn add_bool<S>(mut self, name: S, boolean: bool) -> Self
  where
    S: ToString,
  {
    self
      .contents_mut()
      .insert(name.to_string(), Value::Bool(boolean));
    self
  }

  fn add_string<S>(mut self, name: S, string: S) -> Self
  where
    S: ToString,
  {
    self
      .contents_mut()
      .insert(name.to_string(), Value::String(string.to_string()));
    self
  }

  fn add_value<S, C>(mut self, name: S, value: C) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    self.contents_mut().insert(name.to_string(), value.into());
    self
  }

  fn add_values<S, C, const N: usize>(self, name: S, values: [C; N]) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    self.add_value(name, values.into_iter().map(C::into).collect::<Vec<_>>())
  }

  fn build(self) -> T;
}

#[derive(Debug, Clone, Default)]
pub struct ContextBuilder {
  contents: HashMap<String, Value>,
}

impl ContextBuilder {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Builder<Context> for ContextBuilder {
  fn contents(&self) -> &HashMap<String, Value> {
    &self.contents
  }

  fn contents_mut(&mut self) -> &mut HashMap<String, Value> {
    &mut self.contents
  }

  fn build(self) -> Context {
    Context {
      contents: self.contents,
    }
  }
}

#[derive(Debug, Clone)]
pub enum Value {
  Bool(bool),
  String(String),
  List(Vec<Value>),
  Object(HashMap<String, Value>),
  None,
}

#[derive(Debug, Clone, Default)]
pub struct ObjectBuilder {
  contents: HashMap<String, Value>,
}

impl ObjectBuilder {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Builder<Value> for ObjectBuilder {
  fn contents(&self) -> &HashMap<String, Value> {
    &self.contents
  }

  fn contents_mut(&mut self) -> &mut HashMap<String, Value> {
    &mut self.contents
  }

  fn build(self) -> Value {
    Value::Object(self.contents)
  }
}

impl From<ObjectBuilder> for Value {
  fn from(builder: ObjectBuilder) -> Self {
    builder.build()
  }
}

impl From<bool> for Value {
  fn from(boolean: bool) -> Self {
    Value::Bool(boolean)
  }
}

impl From<String> for Value {
  fn from(string: String) -> Self {
    Self::String(string)
  }
}

impl From<&str> for Value {
  fn from(string: &str) -> Self {
    Self::String(string.to_string())
  }
}

impl From<Vec<Value>> for Value {
  fn from(list: Vec<Value>) -> Self {
    Self::List(list)
  }
}

impl From<HashMap<String, Value>> for Value {
  fn from(object: HashMap<String, Value>) -> Self {
    Self::Object(object)
  }
}

impl From<HashMap<&str, Value>> for Value {
  fn from(object: HashMap<&str, Value>) -> Self {
    Self::Object(
      object
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect(),
    )
  }
}

#[cfg(feature = "serde")]
#[allow(unused_parens, unused_variables, dead_code)]
pub mod serde {
  use crate::context::{ContextBuilder, Value};
  use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
  };
  use serde::{Serialize, Serializer};
  use std::fmt::Display;
  use thiserror::Error;

  pub struct ContextSerializer {
    builder: ContextBuilder,
  }

  #[derive(Error, Debug)]
  #[error("{}")]
  pub enum ContextSerializerError {
    #[error("Unsupported: {0}")]
    Unsupported(&'static str),
    #[error("{0}")]
    Custom(String),
  }

  impl serde::ser::Error for ContextSerializerError {
    fn custom<T>(msg: T) -> Self
    where
      T: Display,
    {
      Self::Custom(msg.to_string())
    }
  }

  impl SerializeSeq for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("seq trait"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("seq trait end"))
    }
  }

  impl SerializeTuple for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("tuple trait"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("tuple trait end"))
    }
  }

  impl SerializeTupleStruct for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("tuple struct trait"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("tuple struct trait end"))
    }
  }

  impl SerializeTupleVariant for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("tuple variant trait"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("tuple variant trait end"))
    }
  }

  impl SerializeMap for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("map trait"))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("map trait"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("map trait end"))
    }
  }

  impl SerializeStruct for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_field<T: ?Sized>(
      &mut self,
      key: &'static str,
      value: &T,
    ) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("struct trait"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("struct trait end"))
    }
  }

  impl SerializeStructVariant for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;

    fn serialize_field<T: ?Sized>(
      &mut self,
      key: &'static str,
      value: &T,
    ) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("struct variant"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("struct variant end"))
    }
  }

  impl Serializer for ContextSerializer {
    type Ok = Value;
    type Error = ContextSerializerError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
      Ok(Self::Ok::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
      self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
      Ok(Self::Ok::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
      Ok(Self::Ok::None)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: Serialize,
    {
      value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      Ok(Self::Ok::None)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("unit struct"))
    }

    fn serialize_unit_variant(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
      Err(Self::Error::Unsupported("unit variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
      self,
      name: &'static str,
      value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("newtype struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
      value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
      T: Serialize,
    {
      Err(Self::Error::Unsupported("newtype variant"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      Err(Self::Error::Unsupported("seq"))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      Err(Self::Error::Unsupported("tuple"))
    }

    fn serialize_tuple_struct(
      self,
      name: &'static str,
      len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
      Err(Self::Error::Unsupported("tuple struct"))
    }

    fn serialize_tuple_variant(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
      len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
      Err(Self::Error::Unsupported("tuple variant"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      Err(Self::Error::Unsupported("map"))
    }

    fn serialize_struct(
      self,
      name: &'static str,
      len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
      Err(Self::Error::Unsupported("struct"))
    }

    fn serialize_struct_variant(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
      len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
      Err(Self::Error::Unsupported("struct variant"))
    }
  }
}

#[cfg(feature = "serde_json")]
pub mod serde_json {
  impl From<Vec<serde_json::Value>> for Value {
    fn from(array: Vec<serde_json::Value>) -> Self {
      Self::List(array.into_iter().map(Self::from).collect())
    }
  }

  impl From<serde_json::Map<String, serde_json::Value>> for Value {
    fn from(object: serde_json::Map<String, serde_json::Value>) -> Self {
      Self::Object(
        object
          .into_iter()
          .map(|(k, v)| (k, Self::from(v)))
          .collect(),
      )
    }
  }

  impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
      match value {
        serde_json::Value::Null => Self::None,
        serde_json::Value::Bool(boolean) => Self::from(boolean),
        serde_json::Value::Number(number) => Self::from(number.to_string()),
        serde_json::Value::String(string) => Self::from(string),
        serde_json::Value::Array(array) => Self::from(array),
        serde_json::Value::Object(object) => Self::from(object),
      }
    }
  }
}
