use std::rc::Rc;
#[cfg(feature="with-json")]
use serde_json::Value as JsonValue;
use crate::{backend::QueryBuilder, types::*, value::*, prepare::*};

/// Insert any new rows into an existing table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
/// 
/// let query = Query::insert()
///     .into_table(Glyph::Table)
///     .columns(vec![
///         Glyph::Aspect,
///         Glyph::Image,
///     ])
///     .values_panic(vec![
///         5.15.into(),
///         "12A".into(),
///     ])
///     .values_panic(vec![
///         4.21.into(),
///         "123".into(),
///     ])
///     .to_owned();
/// 
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct InsertStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) columns: Vec<Rc<dyn Iden>>,
    pub(crate) values: Vec<Vec<Value>>,
}

impl Default for InsertStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl InsertStatement {
    /// Construct a new [`InsertStatement`]
    pub fn new() -> Self {
        Self {
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Specify which table to insert into.
    /// 
    /// # Examples
    /// 
    /// See [`InsertStatement::values`]
    #[allow(clippy::wrong_self_convention)]
    pub fn into_table<T>(&mut self, tbl_ref: T) -> &mut Self
        where T: IntoTableRef {
            self.table = Some(Box::new(tbl_ref.into_table_ref()));
            self
    }

    /// Specify what columns to insert.
    /// 
    /// # Examples
    /// 
    /// See [`InsertStatement::values`]
    pub fn columns<C, I>(&mut self, columns: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        self.columns = columns.into_iter().map(|c| c.into_iden()).collect();
        self
    }

    /// Specify a row of values to be inserted.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .values(vec![
    ///         2.1345.into(),
    ///         "24B".into(),
    ///     ])
    ///     .unwrap()
    ///     .values_panic(vec![
    ///         5.15.into(),
    ///         "12A".into(),
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// ```
    pub fn values(&mut self, values: Vec<Value>) -> Result<&mut Self, String> {
        if self.columns.len() != values.len() {
            return Err(format!("columns and values length mismatch: {} != {}", self.columns.len(), values.len()));
        }
        self.values.push(values);
        Ok(self)
    }

    /// Specify a row of values to be inserted, variation of [`InsertStatement::values`].
    pub fn values_panic(&mut self, values: Vec<Value>) -> &mut Self {
        self.values(values).unwrap()
    }

    /// Specify a row of values to be inserted, taking input of json values. A convenience method if you have multiple
    /// rows to insert at once.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .json(json!({
    ///         "aspect": 2.1345,
    ///         "image": "24B",
    ///     }))
    ///     .json(json!({
    ///         "aspect": 4.21,
    ///         "image": "123",
    ///     }))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (4.21, '123')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (4.21, '123')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (4.21, '123')"#
    /// );
    /// ```
    #[cfg(feature="with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    pub fn json(&mut self, object: JsonValue) -> &mut Self {
        match object {
            JsonValue::Object(_) => (),
            _ => panic!("object must be JsonValue::Object"),
        }
        let mut values = Vec::new();
        if self.columns.is_empty() {
            let map = object.as_object().unwrap();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys.iter() {
                self.columns.push(Rc::new(Alias::new(k)));
            }
        }
        for col in self.columns.iter() {
            values.push(
                match object.get(col.to_string()) {
                    Some(value) => json_value_to_sea_value(value),
                    None => Value::Null,
                }
            );
        }
        self.values.push(values);
        self
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .values_panic(vec![
    ///         3.1415.into(),
    ///         "041080".into(),
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (3.1415, '041080')"#
    /// );
    /// 
    /// let mut params = Vec::new();
    /// let mut collector = |v| params.push(v);
    /// 
    /// assert_eq!(
    ///     query.build_collect(MysqlQueryBuilder, &mut collector),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (?, ?)"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     vec![
    ///         Value::Double(3.1415),
    ///         Value::String(Box::new(String::from("041080"))),
    ///     ]
    /// );
    /// ```
    pub fn build_collect<T: QueryBuilder>(&self, query_builder: T, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_insert_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    pub fn build_collect_any(&self, query_builder: &dyn QueryBuilder, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_insert_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let (query, params) = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .values_panic(vec![
    ///         3.1415.into(),
    ///         "04108048005887010020060000204E0180400400".into(),
    ///     ])
    ///     .build(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (?, ?)"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     Values(vec![
    ///         Value::Double(3.1415),
    ///         Value::String(Box::new(String::from("04108048005887010020060000204E0180400400"))),
    ///     ])
    /// );
    /// ```
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
        let mut values = Vec::new();
        let mut collector = |v| values.push(v);
        let sql = self.build_collect(query_builder, &mut collector);
        (sql, Values(values))
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
        let mut values = Vec::new();
        let mut collector = |v| values.push(v);
        let sql = self.build_collect_any(query_builder, &mut collector);
        (sql, Values(values))
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .values_panic(vec![
    ///         3.1415.into(),
    ///         "041".into(),
    ///     ])
    ///     .to_string(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (3.1415, '041')"#
    /// );
    /// ```
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        let (sql, values) = self.build_any(&query_builder);
        inject_parameters(&sql, values.0, &query_builder)
    }
}
