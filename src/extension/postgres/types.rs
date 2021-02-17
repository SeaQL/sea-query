use std::rc::Rc;
use crate::{backend::QueryBuilder, prepare::*, types::*, value::*};

/// Helper for constructing any type statement
pub struct Type;

#[derive(Clone, Default)]
pub struct TypeCreateStatement {
    pub(crate) name: Option<Rc<dyn Iden>>,
    pub(crate) as_type: Option<TypeAs>,
    pub(crate) values: Vec<Rc<dyn Iden>>,
}

#[derive(Clone)]
pub enum TypeAs {
    // Composite,
    Enum,
    // Range,
    // Base,
    // Array,
}

pub trait TypeBuilder {
    /// Translate [`TypeCreateStatement`] into database specific SQL statement.
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));
}

impl Type {
    /// Construct type [`TypeCreateStatement`]
    pub fn create() -> TypeCreateStatement {
        TypeCreateStatement::new()
    }
}

impl TypeCreateStatement {

    pub fn new() -> Self {
        Self::default()
    }

    /// Create enum as custom type
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::Type};
    ///
    /// enum FontFamily {
    ///     Type,
    ///     Serif,
    ///     Sans,
    ///     Monospace,
    /// }
    /// 
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
    ///         write!(s, "{}", match self {
    ///             Self::Type => "font_family",
    ///             Self::Serif => "serif",
    ///             Self::Sans => "sans",
    ///             Self::Monospace => "monospace",
    ///         }).unwrap();
    ///     }
    /// }
    /// 
    /// assert_eq!(
    ///     Type::create()
    ///         .as_enum(FontFamily::Type)
    ///         .values(vec![FontFamily::Serif, FontFamily::Sans, FontFamily::Monospace])
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"CREATE TYPE "font_family" AS ENUM ('serif', 'sans', 'monospace')"#
    /// );
    /// ```
    pub fn as_enum<T: 'static>(&mut self, name: T) -> &mut Self
        where T: Iden {
        self.name = Some(Rc::new(name));
        self.as_type = Some(TypeAs::Enum);
        self
    }

    pub fn values<T: 'static>(&mut self, values: Vec<T>) -> &mut Self
        where T: Iden {
        self.values_dyn(values.into_iter().map(|c| Rc::new(c) as Rc<dyn Iden>).collect())
    }

    pub fn values_dyn(&mut self, values: Vec<Rc<dyn Iden>>) -> &mut Self {
        self.values = values;
        self
    }

    pub fn build<T: TypeBuilder>(&self, type_builder: T) -> (String, Vec<Value>) {
        self.build_ref(&type_builder)
    }

    pub fn build_ref<T: TypeBuilder>(&self, type_builder: &T) -> (String, Vec<Value>) {
        let mut params = Vec::new();
        let mut collector = |v| params.push(v);
        let sql = self.build_collect_ref(type_builder, &mut collector);
        (sql, params)
    }

    pub fn build_collect<T: TypeBuilder>(&self, type_builder: T, collector: &mut dyn FnMut(Value)) -> String {
        self.build_collect_ref(&type_builder, collector)
    }

    pub fn build_collect_ref<T: TypeBuilder>(&self, type_builder: &T, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        type_builder.prepare_type_create_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement and return SQL string
    pub fn to_string<T>(&self, type_builder: T) -> String
        where T: TypeBuilder + QueryBuilder {
        let (sql, values) = self.build_ref(&type_builder);
        inject_parameters(&sql, values, &type_builder)
    }
}
