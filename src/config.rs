use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, convert::TryFrom};
use toml::value::Table;
use toml::Value as TomlValue;

use crate::{Error, Result, Value};

pub trait Config {
    fn get_value<K: Into<String>, T: TryFrom<Value, Error = Error>>(&self, key: K) -> Result<T>;
}

pub trait ToValue {
    fn to(&self) -> Value;
}

pub struct TomlConfig {
    inner: Table,
}

impl ToValue for TomlValue {
    fn to(&self) -> Value {
        match &self {
            TomlValue::String(val) => Value::String(val.clone()),
            TomlValue::Integer(val) => Value::Integer(*val),
            TomlValue::Float(val) => Value::Number(*val),
            TomlValue::Boolean(val) => Value::Boolean(*val),
            TomlValue::Datetime(val) => Value::String(val.to_string()),
            TomlValue::Array(val) => Value::Array(val.iter().map(|val| val.to()).collect()),
            TomlValue::Table(_val) => Value::Nil,
        }
    }
}

impl TomlConfig {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut fd = File::open(path)?;
        let mut toml = String::new();
        fd.read_to_string(&mut toml)?;
        Self::from_string(&toml)
    }

    pub fn from_string(toml: &str) -> Result<Self> {
        let config: toml::Value = toml.parse().or_else(|err| {
            return Err(Error::invalid_data(&format!(
                "config file is not valid TOML - {}",
                err
            )));
        })?;
        let config = config
            .as_table()
            .ok_or(Error::invalid_data("config file must be a table"))?;
        Ok(Self {
            inner: config.clone(),
        })
    }

    pub fn new(table: Table) -> TomlConfig {
        TomlConfig { inner: table }
    }
}

impl TomlConfig {
    pub fn get_values(&self) -> HashMap<String, toml::Value> {
        let mut values = HashMap::new();
        for (key, value) in &self.inner {
            values.insert(key.clone(), value.clone());
        }

        return values;
    }
}

impl Config for Table {
    fn get_value<K: Into<String>, T: TryFrom<Value, Error = Error>>(&self, key: K) -> Result<T> {
        let key = key.into();
        let value = get_value(&self, &key);
        let value = value
            .map(|val| T::try_from(val.clone()))
            .ok_or(Error::invalid_index(&format!("can't get config[{}]", key)))??;
        Ok(value)
    }
}

impl Config for TomlConfig {
    fn get_value<K: Into<String>, T: TryFrom<Value, Error = Error>>(&self, key: K) -> Result<T> {
        self.inner.get_value(key)
    }
}

fn get_value(table: &Table, key: &str) -> Option<Value> {
    if key.is_empty() {
        return None;
    }
    if let Some(first_index) = key.find(".") {
        let (key, next_key) = key.split_at(first_index + 1);
        let key = key.replace(".", "");
        if let Some(value) = table.get(&key) {
            if let Some(table) = value.as_table() {
                return get_value(table, next_key);
            }
        }
    }
    return table.get(key).map(|val| val.to());
}

pub fn find_path<P: AsRef<Path>>(p: P) -> Result<PathBuf> {
    // 测试模式下，使用上一个目录的路径
    if cfg!(test) {
        return Ok(std::env::current_dir()?.join("../").join(p.as_ref()));
    }
    // Debug 模式下，需要在项目根目录读取配置
    if cfg!(debug_assertions) {
        return Ok(p.as_ref().to_path_buf());
    }

    // 生产模式下，需要找到执行文件目录
    Ok(std::env::current_exe()?
        .parent()
        .unwrap_or(Path::new("./"))
        .join(p))
}

#[test]
fn test_get_value() {
    let config = TomlConfig::from_path(find_path("bee.toml").unwrap()).unwrap();
    let val = config.get_value::<&str, String>("riemann.host").is_ok();
    assert!(val)
}
