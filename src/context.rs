use std::collections::HashMap;

#[cfg(feature = "serde_json")]
pub use crate::context::serde_json::*;

#[derive(Debug, Clone, Default)]
pub struct Context {
  contents: HashMap<String, Value>,
}

impl Context {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn to_value(self) -> Value {
    Value::Object(self.contents)
  }
}

pub trait GetContents {
  fn contents(&self) -> &HashMap<String, Value>;
}

pub trait GetContentsMut {
  fn contents_mut(&mut self) -> &mut HashMap<String, Value>;
}

pub trait GetValue: GetContents {
  fn get_value<S>(&self, path: S) -> Option<&Value>
  where
    S: ToString,
  {
    let path = path.to_string();
    let mut path = path.split('.');
    let mut result = self.contents().get(path.next().unwrap());

    for name in path {
      result = match &result {
        Some(Value::Object(contents)) => contents.get(name),
        Some(_) => None,
        value => *value,
      };
    }

    result
  }

  fn get_bool<S>(&self, value: S) -> bool
  where
    S: ToString,
  {
    match self.get_value(value) {
      Some(Value::Bool(boolean)) => *boolean,
      _ => false,
    }
  }

  fn get_string<S>(&self, value: S) -> Option<&String>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::String(string) => Some(string),
      _ => None,
    }
  }

  fn get_list<S>(&self, value: S) -> Option<&Vec<Value>>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::List(list) => Some(list),
      _ => None,
    }
  }

  fn get_object<S>(&self, value: S) -> Option<&HashMap<String, Value>>
  where
    S: ToString,
  {
    match self.get_value(value)? {
      Value::Object(object) => Some(object),
      _ => None,
    }
  }
}

pub trait SetValue: Sized + GetContentsMut {
  fn set_bool<S>(&mut self, name: S, boolean: bool) -> Option<Value>
  where
    S: ToString,
  {
    self
      .contents_mut()
      .insert(name.to_string(), Value::Bool(boolean))
  }

  fn set_string<S>(&mut self, name: S, string: S) -> Option<Value>
  where
    S: ToString,
  {
    self
      .contents_mut()
      .insert(name.to_string(), Value::String(string.to_string()))
  }

  fn set_value<S, C>(&mut self, name: S, value: C) -> Option<Value>
  where
    S: ToString,
    C: Into<Value>,
  {
    self.contents_mut().insert(name.to_string(), value.into())
  }

  fn set_list<S, C, const N: usize>(&mut self, name: S, values: [C; N]) -> Option<Value>
  where
    S: ToString,
    C: Into<Value>,
  {
    self.set_value(name, values.into_iter().map(C::into).collect::<Vec<_>>())
  }

  fn remove<S>(&mut self, name: S) -> Option<Value>
  where
    S: ToString,
  {
    self.contents_mut().remove(&name.to_string())
  }
}

impl GetContents for HashMap<String, Value> {
  fn contents(&self) -> &HashMap<String, Value> {
    self
  }
}

impl GetContentsMut for HashMap<String, Value> {
  fn contents_mut(&mut self) -> &mut HashMap<String, Value> {
    self
  }
}

impl GetContents for Context {
  fn contents(&self) -> &HashMap<String, Value> {
    &self.contents
  }
}

impl GetContentsMut for Context {
  fn contents_mut(&mut self) -> &mut HashMap<String, Value> {
    &mut self.contents
  }
}

impl SetValue for Context {}
impl GetValue for Context {}

impl SetValue for HashMap<String, Value> {}
impl GetValue for HashMap<String, Value> {}

#[derive(Debug, Clone, Default)]
pub struct ContextBuilder {
  context: Context,
}

impl ContextBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_bool<S>(mut self, name: S, boolean: bool) -> Self
  where
    S: ToString,
  {
    self.context.set_bool(name, boolean);
    self
  }

  pub fn set_string<S>(mut self, name: S, string: S) -> Self
  where
    S: ToString,
  {
    self.context.set_string(name, string);
    self
  }

  pub fn set_list<S, C, const N: usize>(mut self, name: S, values: [C; N]) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    self.context.set_list(name, values);
    self
  }

  pub fn set_value<S, C>(mut self, name: S, value: C) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    self.context.set_value(name, value);
    self
  }

  pub fn build(self) -> Context {
    self.context
  }

  pub fn build_to_value(self) -> Value {
    self.build().to_value()
  }
}

impl From<Context> for ContextBuilder {
  fn from(context: Context) -> Self {
    Self { context }
  }
}

#[derive(Debug, Clone)]
pub enum Value {
  Bool(bool),
  String(String),
  List(Vec<Value>),
  Object(HashMap<String, Value>),
}

impl From<Context> for Value {
  fn from(context: Context) -> Self {
    Value::Object(context.contents)
  }
}

impl From<ContextBuilder> for Value {
  fn from(context: ContextBuilder) -> Self {
    context.build().into()
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
#[allow(unused_variables)]
pub mod serde {
  use std::collections::HashMap;
  use std::fmt::Display;

  use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
  };
  use serde::Serialize;
  use thiserror::Error;

  use crate::context::Context;
  use crate::context::Value;

  #[derive(Debug)]
  struct Serializer;

  #[derive(Error, Debug)]
  #[error("{}")]
  pub enum Error {
    #[error("unsupported: {0}")]
    Unsupported(&'static str),
    #[error("key must be a string")]
    KeyNotString,
    #[error("no value was provided for key: {0}")]
    NoValueForKey(String),
    #[error("no key was provided for value")]
    NoKeyForValue,
    #[error("{0}")]
    Custom(String),
  }

  impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
      T: Display,
    {
      Self::Custom(msg.to_string())
    }
  }

  impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = ListSerializer;
    type SerializeTuple = ListSerializer;
    type SerializeTupleStruct = ListSerializer;
    type SerializeTupleVariant = Unsupported;
    type SerializeMap = ObjectSerializer;
    type SerializeStruct = ObjectSerializer;
    type SerializeStructVariant = Unsupported;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
      Ok(Value::Bool(v))
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
      Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
      Err(Error::Unsupported("bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
      Ok(Value::Bool(false))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: Serialize,
    {
      value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      Err(Error::Unsupported("unit"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
      Err(Error::Unsupported("unit struct"))
    }

    fn serialize_unit_variant(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
      Err(Error::Unsupported("unit variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
      self,
      name: &'static str,
      value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
      T: Serialize,
    {
      Err(Error::Unsupported("newtype struct"))
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
      Err(Error::Unsupported("newtype variant"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      Ok(ListSerializer {
        elements: Vec::with_capacity(len.unwrap_or(0)),
      })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
      self,
      name: &'static str,
      len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
      Err(Error::Unsupported("tuple struct"))
    }

    fn serialize_tuple_variant(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
      len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
      Err(Error::Unsupported("tuple variant"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      Ok(ObjectSerializer {
        contents: HashMap::with_capacity(len.unwrap_or(0)),
        next_key: None,
      })
    }

    fn serialize_struct(
      self,
      _name: &'static str,
      len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
      self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
      self,
      name: &'static str,
      variant_index: u32,
      variant: &'static str,
      len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
      Err(Error::Unsupported("struct variant"))
    }
  }

  struct Unsupported;

  impl SerializeTupleVariant for Unsupported {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Error::Unsupported("enum tuple variant"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Error::Unsupported("enum tuple variant"))
    }
  }

  impl SerializeStructVariant for Unsupported {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
      &mut self,
      key: &'static str,
      value: &T,
    ) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      Err(Error::Unsupported("enum struct variant"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Err(Error::Unsupported("enum struct variant"))
    }
  }

  struct ListSerializer {
    elements: Vec<Value>,
  }

  impl SerializeSeq for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      let element = value.serialize(Serializer)?;
      self.elements.push(element);

      Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(Value::List(self.elements))
    }
  }

  impl SerializeTuple for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      SerializeSeq::end(self)
    }
  }

  impl SerializeTupleStruct for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      SerializeTuple::end(self)
    }
  }

  struct ObjectSerializer {
    contents: HashMap<String, Value>,
    next_key: Option<String>,
  }

  impl SerializeMap for ObjectSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      if let Some(key) = &self.next_key {
        return Err(Error::NoValueForKey(key.clone()));
      }

      match key.serialize(Serializer)? {
        Value::String(key) => {
          self.next_key = Some(key);
          Ok(())
        }
        _ => Err(Error::KeyNotString),
      }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      match &self.next_key {
        Some(key) => {
          let value = value.serialize(Serializer)?;

          self.contents.insert(key.clone(), value);
          self.next_key = None;

          Ok(())
        }
        None => Err(Error::NoKeyForValue),
      }
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
      &mut self,
      key: &K,
      value: &V,
    ) -> Result<(), Self::Error>
    where
      K: Serialize,
      V: Serialize,
    {
      let key = key.serialize(Serializer)?;
      let value = value.serialize(Serializer)?;

      match key {
        Value::String(key) => {
          self.contents.insert(key, value);

          Ok(())
        }
        _ => Err(Error::KeyNotString),
      }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(Value::Object(self.contents))
    }
  }

  impl SerializeStruct for ObjectSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
      &mut self,
      key: &'static str,
      value: &T,
    ) -> Result<(), Self::Error>
    where
      T: Serialize,
    {
      SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      SerializeMap::end(self)
    }
  }

  pub trait ToContext {
    fn to_context(&self) -> Result<Context, Error>
    where
      Self: Serialize,
    {
      Context::from_serialize(&self)
    }
  }

  impl Context {
    pub fn from_serialize<S>(value: &S) -> Result<Context, Error>
    where
      S: Serialize,
    {
      match value.serialize(Serializer)? {
        Value::Object(contents) => Ok(Context { contents }),
        _ => Err(Error::Unsupported("context must be an object")),
      }
    }
  }
}

#[cfg(feature = "serde_json")]
pub mod serde_json {
  use crate::context::Value;

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
        serde_json::Value::Null => Self::from(false),
        serde_json::Value::Bool(boolean) => Self::from(boolean),
        serde_json::Value::Number(number) => Self::from(number.to_string()),
        serde_json::Value::String(string) => Self::from(string),
        serde_json::Value::Array(array) => Self::from(array),
        serde_json::Value::Object(object) => Self::from(object),
      }
    }
  }
}
