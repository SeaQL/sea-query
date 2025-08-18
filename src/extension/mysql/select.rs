use super::{IndexHint, IndexHintScope, IndexHintType};
use crate::{IntoIden, IntoTableRef, SelectStatement};

macro_rules! get_or_insert_index {
    ($self:ident, $table:expr, $index:expr, $ty:expr, $scope:expr) => {
        let key = $table.into();
        match $self.index_hints.get_mut(&key) {
            Some(value) => value.push(IndexHint {
                index: $index,
                r#type: $ty,
                scope: $scope,
            }),
            None => {
                $self.index_hints.insert(
                    key,
                    vec![IndexHint {
                        index: $index,
                        r#type: $ty,
                        scope: $scope,
                    }],
                );
            }
        };
    };
}

pub trait MySqlSelectStatementExt {
    fn use_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden;

    fn use_index_on<T, I>(&mut self, table: T, index: I, scope: IndexHintScope) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIden;

    fn force_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden;

    fn force_index_on<T, I>(&mut self, table: T, index: I, scope: IndexHintScope) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIden;

    fn ignore_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden;

    fn ignore_index_on<T, I>(&mut self, table: T, index: I, scope: IndexHintScope) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIden;
}

const PANIC_MSG: &str =
    "No table in from clause, you should specify the table before using index hint";

impl MySqlSelectStatementExt for SelectStatement {
    /// Use index hint for MySQL
    ///
    /// Give the optimizer information about how to choose indexes during query processing.
    /// See [MySQL reference manual for Index Hints](https://dev.mysql.com/doc/refman/8.0/en/index-hints.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .use_index(IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` USE INDEX (`IDX_123456`)"#
    /// );
    /// ```
    fn use_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden,
    {
        let table_ref = self.from.last().expect(PANIC_MSG);

        get_or_insert_index!(
            self,
            table_ref,
            index.into_iden(),
            IndexHintType::Use,
            scope
        );

        self
    }

    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .from(Font::Table)
    ///     .use_index_on(Char::Table, IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .use_index_on(Font::Table, IndexName::new("IDX_654321"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` USE INDEX (`IDX_123456`), `font` USE INDEX (`IDX_654321`)"#
    /// );
    /// ```
    fn use_index_on<T, I>(&mut self, table: T, index: I, scope: IndexHintScope) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIden,
    {
        get_or_insert_index!(
            self,
            table.into_table_ref(),
            index.into_iden(),
            IndexHintType::Use,
            scope
        );

        self
    }

    /// Force index hint for MySQL
    ///
    /// Give the optimizer information about how to choose indexes during query processing.
    /// See [MySQL reference manual for Index Hints](https://dev.mysql.com/doc/refman/8.0/en/index-hints.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .force_index(IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` FORCE INDEX (`IDX_123456`)"#
    /// );
    /// ```
    fn force_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden,
    {
        let table_ref = self.from.last().expect(PANIC_MSG);

        get_or_insert_index!(
            self,
            table_ref,
            index.into_iden(),
            IndexHintType::Force,
            scope
        );

        self
    }

    fn force_index_on<T, I>(&mut self, table: T, index: I, scope: IndexHintScope) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIden,
    {
        get_or_insert_index!(
            self,
            table.into_table_ref(),
            index.into_iden(),
            IndexHintType::Force,
            scope
        );

        self
    }

    /// Ignore index hint for MySQL
    ///
    /// Give the optimizer information about how to choose indexes during query processing.
    /// See [MySQL reference manual for Index Hints](https://dev.mysql.com/doc/refman/8.0/en/index-hints.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .ignore_index(IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` IGNORE INDEX (`IDX_123456`)"#
    /// )
    /// ```
    fn ignore_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden,
    {
        let table_ref = self.from.last().expect(PANIC_MSG);

        get_or_insert_index!(
            self,
            table_ref,
            index.into_iden(),
            IndexHintType::Ignore,
            scope
        );

        self
    }

    fn ignore_index_on<T, I>(&mut self, table: T, index: I, scope: IndexHintScope) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIden,
    {
        get_or_insert_index!(
            self,
            table.into_table_ref(),
            index.into_iden(),
            IndexHintType::Ignore,
            scope
        );

        self
    }
}
