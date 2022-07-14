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

#[derive(Debug, Clone, Default)]
pub struct ContextBuilder(ObjectBuilder);

impl ContextBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_value<S, C>(self, name: S, value: C) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    Self(self.0.add_value(name, value))
  }

  pub fn add_values<S, C, const N: usize>(self, name: S, values: [C; N]) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    Self(self.0.add_values(name, values))
  }

  pub fn build(self) -> Context {
    Context {
      contents: self.0.contents,
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

  pub fn add_value<S, C>(mut self, name: S, value: C) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    self.contents.insert(name.to_string(), value.into());
    self
  }

  pub fn add_values<S, C, const N: usize>(self, name: S, values: [C; N]) -> Self
  where
    S: ToString,
    C: Into<Value>,
  {
    self.add_value(name, values.into_iter().map(C::into).collect::<Vec<_>>())
  }

  pub fn build(self) -> Value {
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

#[cfg(feature = "serde_json")]
impl From<Vec<serde_json::Value>> for Value {
  fn from(array: Vec<serde_json::Value>) -> Self {
    Self::List(array.into_iter().map(Self::from).collect())
  }
}

#[cfg(feature = "serde_json")]
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

#[cfg(feature = "serde_json")]
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
