use std::{fmt, ops::Deref};

use crate::*;

const QUOTE: Quote = Quote(b'"', b'"');

pub trait QueryBuilder:
    ValueEncoder
    + QuotedBuilder
    + EscapeBuilder
    + TableRefBuilder
    + OperLeftAssocDecider
    + PrecedenceDecider
    + Sized
{
    /// The type of placeholder the builder uses for values, and whether it is numbered.
    fn placeholder(&self) -> (&'static str, bool) {
        ("?", false)
    }

    /// Prefix for tuples in VALUES list (e.g. ROW for MySQL)
    fn values_list_tuple_prefix(&self) -> &str {
        ""
    }

    /// Translate [`InsertStatement`] into SQL statement.
    fn prepare_insert_statement(&self, insert: &InsertStatement, sql: &mut impl SqlWriter) {
        if let Some(with) = &insert.with {
            self.prepare_with_clause(with, sql);
        }

        self.prepare_insert(insert.replace, sql);

        if let Some(table) = &insert.table {
            sql.write_str(" INTO ").unwrap();

            self.prepare_table_ref(table, sql);
        }

        if insert.default_values.unwrap_or_default() != 0
            && insert.columns.is_empty()
            && insert.source.is_none()
        {
            self.prepare_output(&insert.returning, sql);
            sql.write_str(" ").unwrap();
            let num_rows = insert.default_values.unwrap();
            self.insert_default_values(num_rows, sql);
        } else {
            sql.write_str(" (").unwrap();
            let mut cols = insert.columns.iter();
            join_io!(
                cols,
                col,
                join {
                    sql.write_str(", ").unwrap();
                },
                do {
                    self.prepare_iden(col, sql);
                }
            );

            sql.write_str(")").unwrap();

            self.prepare_output(&insert.returning, sql);

            if let Some(source) = &insert.source {
                sql.write_str(" ").unwrap();
                match source {
                    InsertValueSource::Values(values) => {
                        sql.write_str("VALUES ").unwrap();
                        let mut vals = values.iter();
                        join_io!(
                            vals,
                            row,
                            join {
                                sql.write_str(", ").unwrap();
                            },
                            do {
                                sql.write_str("(").unwrap();
                                let mut cols = row.iter();
                                join_io!(
                                    cols,
                                    col,
                                    join {
                                        sql.write_str(", ").unwrap();
                                    },
                                    do {
                                        self.prepare_expr(col, sql);
                                    }
                                );

                                sql.write_str(")").unwrap();
                            }
                        );
                    }
                    InsertValueSource::Select(select_query) => {
                        self.prepare_select_statement(select_query.deref(), sql);
                    }
                }
            }
        }

        self.prepare_on_conflict(&insert.on_conflict, sql);

        self.prepare_returning(&insert.returning, sql);
    }

    fn prepare_union_statement(
        &self,
        union_type: UnionType,
        select_statement: &SelectStatement,
        sql: &mut impl SqlWriter,
    ) {
        match union_type {
            UnionType::Intersect => sql.write_str(" INTERSECT (").unwrap(),
            UnionType::Distinct => sql.write_str(" UNION (").unwrap(),
            UnionType::Except => sql.write_str(" EXCEPT (").unwrap(),
            UnionType::All => sql.write_str(" UNION ALL (").unwrap(),
        }
        self.prepare_select_statement(select_statement, sql);
        sql.write_str(")").unwrap();
    }

    /// Translate [`SelectStatement`] into SQL statement.
    fn prepare_select_statement(&self, select: &SelectStatement, sql: &mut impl SqlWriter) {
        if let Some(with) = &select.with {
            self.prepare_with_clause(with, sql);
        }

        sql.write_str("SELECT ").unwrap();

        if let Some(distinct) = &select.distinct {
            self.prepare_select_distinct(distinct, sql);
            sql.write_str(" ").unwrap();
        }

        let mut selects = select.selects.iter();
        join_io!(
            selects,
            expr,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_select_expr(expr, sql);
            }
        );

        let mut from_tables = select.from.iter();
        join_io!(
            from_tables,
            table_ref,
            first {
                sql.write_str(" FROM ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_table_ref(table_ref, sql);
                self.prepare_index_hints(table_ref,select, sql);
            },
            last {
                self.prepare_table_sample(select, sql);
            }
        );

        for expr in &select.join {
            sql.write_str(" ").unwrap();
            self.prepare_join_expr(expr, sql);
        }

        self.prepare_condition(&select.r#where, "WHERE", sql);

        let mut groups = select.groups.iter();
        join_io!(
            groups,
            expr,
            first {
                sql.write_str(" GROUP BY ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_expr(expr, sql);
            }
        );

        self.prepare_condition(&select.having, "HAVING", sql);

        if !select.unions.is_empty() {
            select.unions.iter().for_each(|(union_type, query)| {
                self.prepare_union_statement(*union_type, query, sql);
            });
        }

        let mut orders = select.orders.iter();
        join_io!(
            orders,
            expr,
            first {
                sql.write_str(" ORDER BY ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_order_expr(expr, sql);
            }
        );

        self.prepare_select_limit_offset(select, sql);

        if let Some(lock) = &select.lock {
            sql.write_str(" ").unwrap();
            self.prepare_select_lock(lock, sql);
        }

        if let Some((name, query)) = &select.window {
            sql.write_str(" WINDOW ").unwrap();
            self.prepare_iden(name, sql);
            sql.write_str(" AS (").unwrap();
            self.prepare_window_statement(query, sql);
            sql.write_str(")").unwrap();
        }
    }

    // Translate the LIMIT and OFFSET expression in [`SelectStatement`]
    fn prepare_select_limit_offset(&self, select: &SelectStatement, sql: &mut impl SqlWriter) {
        if let Some(limit) = &select.limit {
            sql.write_str(" LIMIT ").unwrap();
            self.prepare_value(limit.clone(), sql);
        }

        if let Some(offset) = &select.offset {
            sql.write_str(" OFFSET ").unwrap();
            self.prepare_value(offset.clone(), sql);
        }
    }

    /// Translate [`UpdateStatement`] into SQL statement.
    fn prepare_update_statement(&self, update: &UpdateStatement, sql: &mut impl SqlWriter) {
        if let Some(with) = &update.with {
            self.prepare_with_clause(with, sql);
        }

        sql.write_str("UPDATE ").unwrap();

        if let Some(table) = &update.table {
            self.prepare_table_ref(table, sql);
        }

        self.prepare_update_join(&update.from, &update.r#where, sql);

        sql.write_str(" SET ").unwrap();

        let mut values = update.values.iter();
        join_io!(
            values,
            row,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                let (col, v) = row;
                self.prepare_update_column(&update.table, &update.from, col, sql);
                sql.write_str(" = ").unwrap();
                self.prepare_expr(v, sql);
            }
        );

        self.prepare_update_from(&update.from, sql);

        self.prepare_output(&update.returning, sql);

        self.prepare_update_condition(&update.from, &update.r#where, sql);

        self.prepare_update_order_by(update, sql);

        self.prepare_update_limit(update, sql);

        self.prepare_returning(&update.returning, sql);
    }

    fn prepare_update_join(&self, _: &[TableRef], _: &ConditionHolder, _: &mut impl SqlWriter) {
        // MySQL specific
    }

    fn prepare_update_from(&self, from: &[TableRef], sql: &mut impl SqlWriter) {
        let mut from_iter = from.iter();
        join_io!(
            from_iter,
            table_ref,
            first {
                sql.write_str(" FROM ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_table_ref(table_ref, sql);
            }
        );
    }

    fn prepare_update_column(
        &self,
        _: &Option<Box<TableRef>>,
        _: &[TableRef],
        column: &DynIden,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_iden(column, sql);
    }

    fn prepare_update_condition(
        &self,
        _: &[TableRef],
        condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_condition(condition, "WHERE", sql);
    }

    /// Translate ORDER BY expression in [`UpdateStatement`].
    fn prepare_update_order_by(&self, update: &UpdateStatement, sql: &mut impl SqlWriter) {
        let mut orders = update.orders.iter();
        join_io!(
            orders,
            expr,
            first {
                sql.write_str(" ORDER BY ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_order_expr(expr, sql);
            }
        );
    }

    /// Translate LIMIT expression in [`UpdateStatement`].
    fn prepare_update_limit(&self, update: &UpdateStatement, sql: &mut impl SqlWriter) {
        if let Some(limit) = &update.limit {
            sql.write_str(" LIMIT ").unwrap();
            self.prepare_value(limit.clone(), sql);
        }
    }

    /// Translate [`DeleteStatement`] into SQL statement.
    fn prepare_delete_statement(&self, delete: &DeleteStatement, sql: &mut impl SqlWriter) {
        if let Some(with) = &delete.with {
            self.prepare_with_clause(with, sql);
        }

        sql.write_str("DELETE ").unwrap();

        if let Some(table) = &delete.table {
            sql.write_str("FROM ").unwrap();
            self.prepare_table_ref(table, sql);
        }

        self.prepare_output(&delete.returning, sql);

        self.prepare_condition(&delete.r#where, "WHERE", sql);

        self.prepare_delete_order_by(delete, sql);

        self.prepare_delete_limit(delete, sql);

        self.prepare_returning(&delete.returning, sql);
    }

    /// Translate ORDER BY expression in [`DeleteStatement`].
    fn prepare_delete_order_by(&self, delete: &DeleteStatement, sql: &mut impl SqlWriter) {
        let mut orders = delete.orders.iter();
        join_io!(
            orders,
            expr,
            first {
                sql.write_str(" ORDER BY ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_order_expr(expr, sql);
            }
        );
    }

    /// Translate LIMIT expression in [`DeleteStatement`].
    fn prepare_delete_limit(&self, delete: &DeleteStatement, sql: &mut impl SqlWriter) {
        if let Some(limit) = &delete.limit {
            sql.write_str(" LIMIT ").unwrap();
            self.prepare_value(limit.clone(), sql);
        }
    }

    /// Translate [`Expr`] into SQL statement.
    fn prepare_expr(&self, simple_expr: &Expr, sql: &mut impl SqlWriter) {
        self.prepare_expr_common(simple_expr, sql);
    }

    fn prepare_expr_common(&self, simple_expr: &Expr, sql: &mut impl SqlWriter) {
        match simple_expr {
            Expr::Column(column_ref) => {
                self.prepare_column_ref(column_ref, sql);
            }
            Expr::Tuple(exprs) => {
                self.prepare_tuple(exprs, sql);
            }
            Expr::Unary(op, expr) => {
                self.prepare_un_oper(op, sql);
                sql.write_str(" ").unwrap();
                let drop_expr_paren =
                    self.inner_expr_well_known_greater_precedence(expr, &(*op).into());
                if !drop_expr_paren {
                    sql.write_str("(").unwrap();
                }
                self.prepare_expr(expr, sql);
                if !drop_expr_paren {
                    sql.write_str(")").unwrap();
                }
            }
            Expr::FunctionCall(func) => {
                self.prepare_function_name(&func.func, sql);
                self.prepare_function_arguments(func, sql);
            }
            Expr::Binary(left, op, right) => match (op, right.as_ref()) {
                (BinOper::In, Expr::Tuple(t)) if t.is_empty() => {
                    self.binary_expr(&1i32.into(), &BinOper::Equal, &2i32.into(), sql)
                }
                (BinOper::NotIn, Expr::Tuple(t)) if t.is_empty() => {
                    self.binary_expr(&1i32.into(), &BinOper::Equal, &1i32.into(), sql)
                }
                _ => self.binary_expr(left, op, right, sql),
            },
            Expr::SubQuery(oper, sel) => {
                if let Some(oper) = oper {
                    self.prepare_sub_query_oper(oper, sql);
                }
                sql.write_str("(").unwrap();
                self.prepare_query_statement(sel.deref(), sql);
                sql.write_str(")").unwrap();
            }
            Expr::Value(val) => {
                self.prepare_value(val.clone(), sql);
            }
            Expr::Values(list) => {
                sql.write_str("(").unwrap();
                let mut iter = list.iter();
                join_io!(
                    iter,
                    val,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        self.prepare_value(val.clone(), sql);
                    }
                );
                sql.write_str(")").unwrap();
            }
            Expr::Custom(s) => {
                sql.write_str(s).unwrap();
            }
            Expr::CustomWithExpr(expr, values) => {
                let (placeholder, numbered) = self.placeholder();
                let mut tokenizer = Tokenizer::new(expr).iter().peekable();
                let mut count = 0;
                while let Some(token) = tokenizer.next() {
                    match token {
                        Token::Punctuation(mark) if mark == placeholder => match tokenizer.peek() {
                            Some(Token::Punctuation(next_mark)) if next_mark == &placeholder => {
                                sql.write_str(next_mark).unwrap();
                                tokenizer.next();
                            }
                            Some(Token::Unquoted(tok)) if numbered => {
                                if let Ok(num) = tok.parse::<usize>() {
                                    self.prepare_expr(&values[num - 1], sql);
                                }
                                tokenizer.next();
                            }
                            _ => {
                                self.prepare_expr(&values[count], sql);
                                count += 1;
                            }
                        },
                        _ => sql.write_str(token.as_str()).unwrap(),
                    };
                }
            }
            Expr::Keyword(keyword) => {
                self.prepare_keyword(keyword, sql);
            }
            Expr::AsEnum(_, expr) => {
                self.prepare_expr(expr, sql);
            }
            Expr::Case(case_stmt) => {
                self.prepare_case_statement(case_stmt, sql);
            }
            Expr::Constant(val) => {
                self.prepare_constant(val, sql);
            }
            Expr::TypeName(type_name) => {
                self.prepare_type_ref(type_name, sql);
            }
        }
    }

    /// Translate [`CaseStatement`] into SQL statement.
    fn prepare_case_statement(&self, stmts: &CaseStatement, sql: &mut impl SqlWriter) {
        sql.write_str("(CASE").unwrap();

        let CaseStatement { when, r#else } = stmts;

        for case in when.iter() {
            sql.write_str(" WHEN (").unwrap();
            self.prepare_condition_where(&case.condition, sql);
            sql.write_str(") THEN ").unwrap();

            self.prepare_expr(&case.result, sql);
        }
        if let Some(r#else) = r#else {
            sql.write_str(" ELSE ").unwrap();
            self.prepare_expr(r#else, sql);
        }

        sql.write_str(" END)").unwrap();
    }

    /// Translate [`SelectDistinct`] into SQL statement.
    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut impl SqlWriter) {
        match select_distinct {
            SelectDistinct::All => sql.write_str("ALL").unwrap(),
            SelectDistinct::Distinct => sql.write_str("DISTINCT").unwrap(),
            _ => {}
        }
    }

    /// Translate [`IndexHint`][crate::extension::mysql::IndexHint] into SQL statement.
    fn prepare_index_hints(
        &self,
        _table_ref: &TableRef,
        _select: &SelectStatement,
        _sql: &mut impl SqlWriter,
    ) {
    }

    /// Translate [`TableSample`][crate::extension::postgres::TableSample] into SQL statement.
    fn prepare_table_sample(&self, _select: &SelectStatement, _sql: &mut impl SqlWriter) {}

    /// Translate [`LockType`] into SQL statement.
    fn prepare_select_lock(&self, lock: &LockClause, sql: &mut impl SqlWriter) {
        sql.write_str(self.lock_phrase(lock.r#type)).unwrap();
        let mut tables = lock.tables.iter();
        join_io!(
            tables,
            table_ref,
            first {
                sql.write_str(" OF ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_table_ref(table_ref, sql);
            }
        );

        if let Some(behavior) = lock.behavior {
            match behavior {
                LockBehavior::Nowait => sql.write_str(" NOWAIT").unwrap(),
                LockBehavior::SkipLocked => sql.write_str(" SKIP LOCKED").unwrap(),
            }
        }
    }

    /// Translate [`SelectExpr`] into SQL statement.
    fn prepare_select_expr(&self, select_expr: &SelectExpr, sql: &mut impl SqlWriter) {
        self.prepare_expr(&select_expr.expr, sql);
        match &select_expr.window {
            Some(WindowSelectType::Name(name)) => {
                sql.write_str(" OVER ").unwrap();
                self.prepare_iden(name, sql);
            }
            Some(WindowSelectType::Query(window)) => {
                sql.write_str(" OVER ").unwrap();
                sql.write_str("( ").unwrap();
                self.prepare_window_statement(window, sql);
                sql.write_str(" )").unwrap();
            }
            None => {}
        };

        if let Some(alias) = &select_expr.alias {
            sql.write_str(" AS ").unwrap();
            self.prepare_iden(alias, sql);
        };
    }

    /// Translate [`JoinExpr`] into SQL statement.
    fn prepare_join_expr(&self, join_expr: &JoinExpr, sql: &mut impl SqlWriter) {
        self.prepare_join_type(&join_expr.join, sql);
        sql.write_str(" ").unwrap();
        self.prepare_join_table_ref(join_expr, sql);
        if let Some(on) = &join_expr.on {
            self.prepare_join_on(on, sql);
        }
    }

    fn prepare_join_table_ref(&self, join_expr: &JoinExpr, sql: &mut impl SqlWriter) {
        if join_expr.lateral {
            sql.write_str("LATERAL ").unwrap();
        }
        self.prepare_table_ref(&join_expr.table, sql);
    }

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref(&self, table_ref: &TableRef, sql: &mut impl SqlWriter) {
        match table_ref {
            TableRef::SubQuery(query, alias) => {
                sql.write_str("(").unwrap();
                self.prepare_select_statement(query, sql);
                sql.write_str(")").unwrap();
                sql.write_str(" AS ").unwrap();
                self.prepare_iden(alias, sql);
            }
            TableRef::ValuesList(values, alias) => {
                sql.write_str("(").unwrap();
                self.prepare_values_list(values, sql);
                sql.write_str(")").unwrap();
                sql.write_str(" AS ").unwrap();
                self.prepare_iden(alias, sql);
            }
            TableRef::FunctionCall(func, alias) => {
                self.prepare_function_name(&func.func, sql);
                self.prepare_function_arguments(func, sql);
                sql.write_str(" AS ").unwrap();
                self.prepare_iden(alias, sql);
            }
            _ => self.prepare_table_ref_iden(table_ref, sql),
        }
    }

    fn prepare_column_ref(&self, column_ref: &ColumnRef, sql: &mut impl SqlWriter) {
        match column_ref {
            ColumnRef::Column(ColumnName(table_name, column)) => {
                if let Some(table_name) = table_name {
                    self.prepare_table_name(table_name, sql);
                    sql.write_str(".").unwrap();
                }
                self.prepare_iden(column, sql);
            }
            ColumnRef::Asterisk(table_name) => {
                if let Some(table_name) = table_name {
                    self.prepare_table_name(table_name, sql);
                    sql.write_str(".").unwrap();
                }
                sql.write_str("*").unwrap();
            }
        }
    }

    /// Translate [`UnOper`] into SQL statement.
    fn prepare_un_oper(&self, un_oper: &UnOper, sql: &mut impl SqlWriter) {
        sql.write_str(match un_oper {
            UnOper::Not => "NOT",
        })
        .unwrap();
    }

    fn prepare_bin_oper_common(&self, bin_oper: &BinOper, sql: &mut impl SqlWriter) {
        sql.write_str(match bin_oper {
            BinOper::And => "AND",
            BinOper::Or => "OR",
            BinOper::Like => "LIKE",
            BinOper::NotLike => "NOT LIKE",
            BinOper::Is => "IS",
            BinOper::IsNot => "IS NOT",
            BinOper::In => "IN",
            BinOper::NotIn => "NOT IN",
            BinOper::Between => "BETWEEN",
            BinOper::NotBetween => "NOT BETWEEN",
            BinOper::Equal => "=",
            BinOper::NotEqual => "<>",
            BinOper::SmallerThan => "<",
            BinOper::GreaterThan => ">",
            BinOper::SmallerThanOrEqual => "<=",
            BinOper::GreaterThanOrEqual => ">=",
            BinOper::Add => "+",
            BinOper::Sub => "-",
            BinOper::Mul => "*",
            BinOper::Div => "/",
            BinOper::Mod => "%",
            BinOper::LShift => "<<",
            BinOper::RShift => ">>",
            BinOper::As => "AS",
            BinOper::Escape => "ESCAPE",
            BinOper::Custom(raw) => raw,
            BinOper::BitAnd => "&",
            BinOper::BitOr => "|",
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        })
        .unwrap();
    }

    /// Translate [`BinOper`] into SQL statement.
    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut impl SqlWriter) {
        self.prepare_bin_oper_common(bin_oper, sql);
    }

    /// Translate [`SubQueryOper`] into SQL statement.
    fn prepare_sub_query_oper(&self, oper: &SubQueryOper, sql: &mut impl SqlWriter) {
        sql.write_str(match oper {
            SubQueryOper::Exists => "EXISTS",
            SubQueryOper::Any => "ANY",
            SubQueryOper::Some => "SOME",
            SubQueryOper::All => "ALL",
        })
        .unwrap();
    }

    /// Translate [`LogicalChainOper`] into SQL statement.
    fn prepare_logical_chain_oper(
        &self,
        log_chain_oper: &LogicalChainOper,
        i: usize,
        length: usize,
        sql: &mut impl SqlWriter,
    ) {
        let (simple_expr, oper) = match log_chain_oper {
            LogicalChainOper::And(simple_expr) => (simple_expr, "AND"),
            LogicalChainOper::Or(simple_expr) => (simple_expr, "OR"),
        };
        if i > 0 {
            sql.write_str(" ").unwrap();
            sql.write_str(oper).unwrap();
            sql.write_str(" ").unwrap();
        }
        let both_binary = match simple_expr {
            Expr::Binary(_, _, right) => {
                matches!(right.as_ref(), Expr::Binary(_, _, _))
            }
            _ => false,
        };
        let need_parentheses = length > 1 && both_binary;
        if need_parentheses {
            sql.write_str("(").unwrap();
        }
        self.prepare_expr(simple_expr, sql);
        if need_parentheses {
            sql.write_str(")").unwrap();
        }
    }

    /// Translate [`Function`] into SQL statement.
    fn prepare_function_name_common(&self, function: &Func, sql: &mut impl SqlWriter) {
        if let Func::Custom(iden) = function {
            sql.write_str(&iden.0)
        } else {
            sql.write_str(match function {
                Func::Max => "MAX",
                Func::Min => "MIN",
                Func::Sum => "SUM",
                Func::Avg => "AVG",
                Func::Abs => "ABS",
                Func::Coalesce => "COALESCE",
                Func::Count => "COUNT",
                Func::IfNull => self.if_null_function(),
                Func::Greatest => self.greatest_function(),
                Func::Least => self.least_function(),
                Func::CharLength => self.char_length_function(),
                Func::Cast => "CAST",
                Func::Lower => "LOWER",
                Func::Upper => "UPPER",
                Func::BitAnd => "BIT_AND",
                Func::BitOr => "BIT_OR",
                Func::Custom(_) => "",
                Func::Random => self.random_function(),
                Func::Round => "ROUND",
                Func::Md5 => "MD5",
                #[cfg(feature = "backend-postgres")]
                Func::PgFunction(_) => unimplemented!(),
            })
        }
        .unwrap();
    }

    fn prepare_function_arguments(&self, func: &FunctionCall, sql: &mut impl SqlWriter) {
        sql.write_str("(").unwrap();
        let mut args = func.args.iter().zip(func.mods.iter());

        if let Some((arg, modifier)) = args.next() {
            if modifier.distinct {
                sql.write_str("DISTINCT ").unwrap();
            }
            self.prepare_expr(arg, sql);
        }

        for (arg, modifier) in args {
            sql.write_str(", ").unwrap();
            if modifier.distinct {
                sql.write_str("DISTINCT ").unwrap();
            }
            self.prepare_expr(arg, sql);
        }

        sql.write_str(")").unwrap();
    }

    /// Translate [`QueryStatement`] into SQL statement.
    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut impl SqlWriter);

    fn prepare_with_query(&self, query: &WithQuery, sql: &mut impl SqlWriter) {
        self.prepare_with_clause(&query.with_clause, sql);
        self.prepare_query_statement(query.query.as_ref().unwrap().deref(), sql);
    }

    fn prepare_with_clause(&self, with_clause: &WithClause, sql: &mut impl SqlWriter) {
        self.prepare_with_clause_start(with_clause, sql);
        self.prepare_with_clause_common_tables(with_clause, sql);
        if with_clause.recursive {
            self.prepare_with_clause_recursive_options(with_clause, sql);
        }
    }

    fn prepare_with_clause_recursive_options(
        &self,
        with_clause: &WithClause,
        sql: &mut impl SqlWriter,
    ) {
        if with_clause.recursive {
            if let Some(search) = &with_clause.search {
                sql.write_str("SEARCH ").unwrap();
                sql.write_str(match &search.order.as_ref().unwrap() {
                    SearchOrder::BREADTH => "BREADTH",
                    SearchOrder::DEPTH => "DEPTH",
                })
                .unwrap();
                sql.write_str(" FIRST BY ").unwrap();

                self.prepare_expr(&search.expr.as_ref().unwrap().expr, sql);

                sql.write_str(" SET ").unwrap();

                self.prepare_iden(search.expr.as_ref().unwrap().alias.as_ref().unwrap(), sql);
                sql.write_str(" ").unwrap();
            }
            if let Some(cycle) = &with_clause.cycle {
                sql.write_str("CYCLE ").unwrap();

                self.prepare_expr(cycle.expr.as_ref().unwrap(), sql);

                sql.write_str(" SET ").unwrap();

                self.prepare_iden(cycle.set_as.as_ref().unwrap(), sql);
                sql.write_str(" USING ").unwrap();
                self.prepare_iden(cycle.using.as_ref().unwrap(), sql);
                sql.write_str(" ").unwrap();
            }
        }
    }

    fn prepare_with_clause_common_tables(
        &self,
        with_clause: &WithClause,
        sql: &mut impl SqlWriter,
    ) {
        let mut cte_first = true;
        assert_ne!(
            with_clause.cte_expressions.len(),
            0,
            "Cannot build a with query that has no common table expression!"
        );

        for cte in &with_clause.cte_expressions {
            if !cte_first {
                sql.write_str(", ").unwrap();
            }
            cte_first = false;

            self.prepare_with_query_clause_common_table(cte, sql);
        }
    }

    fn prepare_with_query_clause_common_table(
        &self,
        cte: &CommonTableExpression,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_iden(cte.table_name.as_ref().unwrap(), sql);

        if cte.cols.is_empty() {
            sql.write_str(" ").unwrap();
        } else {
            sql.write_str(" (").unwrap();

            let mut col_first = true;
            for col in &cte.cols {
                if !col_first {
                    sql.write_str(", ").unwrap();
                }
                col_first = false;
                self.prepare_iden(col, sql);
            }

            sql.write_str(") ").unwrap();
        }

        sql.write_str("AS ").unwrap();

        self.prepare_with_query_clause_materialization(cte, sql);

        sql.write_str("(").unwrap();

        match &cte.query {
            CteQuery::SubQuery(sub_query) => self.prepare_query_statement(sub_query, sql),
            CteQuery::Values(items) => self.prepare_values_rows(items, sql),
        }

        sql.write_str(") ").unwrap();
    }

    fn prepare_with_query_clause_materialization(
        &self,
        cte: &CommonTableExpression,
        sql: &mut impl SqlWriter,
    ) {
        if let Some(materialized) = cte.materialized {
            if !materialized {
                sql.write_str("NOT MATERIALIZED ")
            } else {
                sql.write_str(" MATERIALIZED ")
            }
            .unwrap()
        }
    }

    fn prepare_with_clause_start(&self, with_clause: &WithClause, sql: &mut impl SqlWriter) {
        sql.write_str("WITH ").unwrap();

        if with_clause.recursive {
            sql.write_str("RECURSIVE ").unwrap();
        }
    }

    fn prepare_insert(&self, replace: bool, sql: &mut impl SqlWriter) {
        if replace {
            sql.write_str("REPLACE").unwrap();
        } else {
            sql.write_str("INSERT").unwrap();
        }
    }

    fn prepare_function_name(&self, function: &Func, sql: &mut impl SqlWriter) {
        self.prepare_function_name_common(function, sql)
    }

    /// Translate [`TypeRef`] into an SQL statement.
    fn prepare_type_ref(&self, type_name: &TypeRef, sql: &mut impl SqlWriter) {
        let TypeRef(schema_name, r#type) = type_name;
        if let Some(schema_name) = schema_name {
            self.prepare_schema_name(schema_name, sql);
            write!(sql, ".").unwrap();
        }
        self.prepare_iden(r#type, sql);
    }

    /// Translate [`JoinType`] into SQL statement.
    fn prepare_join_type(&self, join_type: &JoinType, sql: &mut impl SqlWriter) {
        self.prepare_join_type_common(join_type, sql)
    }

    fn prepare_join_type_common(&self, join_type: &JoinType, sql: &mut impl SqlWriter) {
        sql.write_str(match join_type {
            JoinType::Join => "JOIN",
            JoinType::CrossJoin => "CROSS JOIN",
            JoinType::InnerJoin => "INNER JOIN",
            JoinType::LeftJoin => "LEFT JOIN",
            JoinType::RightJoin => "RIGHT JOIN",
            JoinType::FullOuterJoin => "FULL OUTER JOIN",
            JoinType::StraightJoin => "STRAIGHT_JOIN",
        })
        .unwrap()
    }

    /// Translate [`OrderExpr`] into SQL statement.
    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut impl SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
    }

    /// Translate [`JoinOn`] into SQL statement.
    fn prepare_join_on(&self, join_on: &JoinOn, sql: &mut impl SqlWriter) {
        match join_on {
            JoinOn::Condition(c) => self.prepare_condition(c, "ON", sql),
            JoinOn::Columns(_c) => unimplemented!(),
        }
    }

    /// Translate [`Order`] into SQL statement.
    fn prepare_order(&self, order_expr: &OrderExpr, sql: &mut impl SqlWriter) {
        match &order_expr.order {
            Order::Asc => sql.write_str(" ASC").unwrap(),
            Order::Desc => sql.write_str(" DESC").unwrap(),
            Order::Field(values) => self.prepare_field_order(order_expr, values, sql),
        }
    }

    /// Translate [`Order::Field`] into SQL statement
    fn prepare_field_order(
        &self,
        order_expr: &OrderExpr,
        values: &Values,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("CASE ").unwrap();
        let mut i = 0;
        for value in &values.0 {
            sql.write_str("WHEN ").unwrap();
            self.prepare_expr(&order_expr.expr, sql);
            sql.write_str("=").unwrap();
            self.write_value(sql, value).unwrap();
            sql.write_str(" THEN ").unwrap();
            write_int(sql, i);
            sql.write_str(" ").unwrap();
            i += 1;
        }

        sql.write_str("ELSE ").unwrap();
        write_int(sql, i);
        sql.write_str(" END").unwrap();
    }

    /// Write [`Value`] into SQL statement as parameter.
    fn prepare_value(&self, value: Value, sql: &mut impl SqlWriter);

    /// Write [`Value`] inline.
    fn prepare_constant(&self, value: &Value, sql: &mut impl SqlWriter) {
        self.write_value(sql, value).unwrap();
    }

    /// Translate a `&[ValueTuple]` into a VALUES list.
    fn prepare_values_list(&self, value_tuples: &[ValueTuple], sql: &mut impl SqlWriter) {
        sql.write_str("VALUES ").unwrap();
        let mut tuples = value_tuples.iter();
        join_io!(
            tuples,
            value_tuple,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                sql.write_str(self.values_list_tuple_prefix()).unwrap();
                sql.write_str("(").unwrap();

                let mut values = value_tuple.clone().into_iter();
                join_io!(
                    values,
                    value,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        self.prepare_value(value, sql);
                    }
                );

                sql.write_str(")").unwrap();
            }
        );
    }

    fn prepare_values_rows(&self, values: &[Values], sql: &mut impl SqlWriter) {
        sql.write_str("VALUES ").unwrap();
        let mut rows = values.iter();
        join_io!(
            rows,
            row,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                sql.write_str("(").unwrap();

                let mut vals = row.clone().into_iter();
                join_io!(
                    vals,
                    val,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        self.prepare_value(val, sql);
                    }
                );

                sql.write_str(")").unwrap();
            }
        );
    }

    /// Translate [`Expr::Tuple`] into SQL statement.
    fn prepare_tuple(&self, exprs: &[Expr], sql: &mut impl SqlWriter) {
        sql.write_str("(").unwrap();
        for (i, expr) in exprs.iter().enumerate() {
            if i != 0 {
                sql.write_str(", ").unwrap();
            }
            self.prepare_expr(expr, sql);
        }
        sql.write_str(")").unwrap();
    }

    /// Translate [`Keyword`] into SQL statement.
    fn prepare_keyword(&self, keyword: &Keyword, sql: &mut impl SqlWriter) {
        match keyword {
            Keyword::Null => sql.write_str("NULL").unwrap(),
            Keyword::CurrentDate => sql.write_str("CURRENT_DATE").unwrap(),
            Keyword::CurrentTime => sql.write_str("CURRENT_TIME").unwrap(),
            Keyword::CurrentTimestamp => sql.write_str("CURRENT_TIMESTAMP").unwrap(),
            Keyword::Default => sql.write_str("DEFAULT").unwrap(),
            Keyword::Custom(iden) => sql.write_str(&iden.0).unwrap(),
        }
    }

    /// Convert a SQL value into syntax-specific string
    fn value_to_string(&self, v: &Value) -> String {
        self.value_to_string_common(v)
    }

    fn value_to_string_common(&self, v: &Value) -> String {
        let mut s = String::new();
        self.write_value(&mut s, v).unwrap();
        s
    }

    #[doc(hidden)]
    #[allow(clippy::cognitive_complexity)]
    fn write_value(&self, buf: &mut impl Write, value: &Value) -> fmt::Result {
        macro_rules! write_opt {
            ($opt:expr, $val:ident => $body:expr) => {
                match $opt {
                    Some($val) => $body,
                    None => buf.write_str("NULL").unwrap(),
                }
            };
        }

        match value {
            Value::Bool(v) => write_opt!(v, val => self.write_bool_to(buf, *val)),
            Value::TinyInt(v) => write_opt!(v, val => self.write_i8_to(buf, *val)),
            Value::SmallInt(v) => write_opt!(v, val => self.write_i16_to(buf, *val)),
            Value::Int(v) => write_opt!(v, val => self.write_i32_to(buf, *val)),
            Value::BigInt(v) => write_opt!(v, val => self.write_i64_to(buf, *val)),
            Value::TinyUnsigned(v) => write_opt!(v, val => self.write_u8_to(buf, *val)),
            Value::SmallUnsigned(v) => write_opt!(v, val => self.write_u16_to(buf, *val)),
            Value::Unsigned(v) => write_opt!(v, val => self.write_u32_to(buf, *val)),
            Value::BigUnsigned(v) => write_opt!(v, val => self.write_u64_to(buf, *val)),
            Value::Float(v) => write_opt!(v, val => self.write_f32_to(buf, *val)),
            Value::Double(v) => write_opt!(v, val => self.write_f64_to(buf, *val)),
            Value::String(v) => write_opt!(v, val => self.write_str_to(buf, val)),
            Value::Char(v) => write_opt!(v, val => self.write_char_to(buf, *val)),
            Value::Enum(v) => write_opt!(v, val => self.write_enum_to(buf, val.as_ref())),
            Value::Bytes(v) => {
                write_opt!(v, val => ValueEncoder::write_bytes_to(self, buf, val))
            }
            #[cfg(feature = "with-json")]
            Value::Json(v) => write_opt!(v, val => self.write_json_to(buf, val)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => {
                write_opt!(v, val => self.write_naive_date_to(buf, *val))
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => {
                write_opt!(v, val => self.write_naive_time_to(buf, *val))
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => {
                write_opt!(v, val => self.write_naive_datetime_to(buf, *val))
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => {
                write_opt!(v, val => self.write_datetime_utc_to(buf, val))
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => write_opt!(v, val => self
                .write_datetime_local_to(buf, val)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => {
                write_opt!(v, val => self.write_datetime_fixed_to(buf, val))
            }
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => write_opt!(v, val => self.write_time_date_to(buf, *val)),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => write_opt!(v, val => self.write_time_time_to(buf, *val)),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => write_opt!(v, val => self.write_time_datetime_to(buf, *val)),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => write_opt!(v, val => self
                .write_time_datetime_tz_to(buf, *val)),
            #[cfg(feature = "with-jiff")]
            Value::JiffDate(v) => {
                write_opt!(v, val => self.write_jiff_date_to(buf, *val))
            }
            #[cfg(feature = "with-jiff")]
            Value::JiffTime(v) => {
                write_opt!(v, val => self.write_jiff_time_to(buf, *val))
            }
            #[cfg(feature = "with-jiff")]
            Value::JiffDateTime(v) => write_opt!(v, val => self
                .write_jiff_datetime_to(buf, *val)),
            #[cfg(feature = "with-jiff")]
            Value::JiffTimestamp(v) => {
                write_opt!(v, val => self.write_jiff_timestamp_to(buf, *val))
            }
            #[cfg(feature = "with-jiff")]
            Value::JiffZoned(v) => write_opt!(v, val => self.write_jiff_zoned_to(buf, val)),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(v) => write_opt!(v, val => self.write_decimal_to(buf, *val)),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => {
                write_opt!(v, val => self.write_bigdecimal_to(buf, val))
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => write_opt!(v, val => self.write_uuid_to(buf, *val)),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(v) => write_opt!(v, val => self.write_vector_to(buf, val)),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => write_opt!(v, val => self.write_ipnetwork_to(buf, *val)),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => write_opt!(v, val => self.write_mac_address_to(buf, *val)),
            #[cfg(feature = "postgres-array")]
            Value::Array(v) => self.write_array_to(buf, v),
            #[cfg(feature = "postgres-range")]
            Value::Range(v) => write_opt!(v, val => {
                buf.write_str("'")?;
                write!(buf, "{val}")?;
                buf.write_str("'")?;
            }),
        }

        Ok(())
    }

    #[doc(hidden)]
    /// Write ON CONFLICT expression
    fn prepare_on_conflict(&self, on_conflict: &Option<OnConflict>, sql: &mut impl SqlWriter) {
        if let Some(on_conflict) = on_conflict {
            self.prepare_on_conflict_keywords(sql);
            self.prepare_on_conflict_target(&on_conflict.targets, sql);
            self.prepare_on_conflict_condition(&on_conflict.target_where, sql);
            self.prepare_on_conflict_action(&on_conflict.action, sql);
            self.prepare_on_conflict_condition(&on_conflict.action_where, sql);
        }
    }

    #[doc(hidden)]
    /// Write ON CONFLICT target
    fn prepare_on_conflict_target(
        &self,
        on_conflict_targets: &OnConflictTarget,
        sql: &mut impl SqlWriter,
    ) {
        match on_conflict_targets {
            OnConflictTarget::Identifiers(identifiers) => {
                self.prepare_on_conflict_target_identifiers(identifiers, sql)
            }
            OnConflictTarget::Constraint(constraint) => {
                self.prepare_on_conflict_target_constraint(constraint, sql)
            }
        }
    }

    #[doc(hidden)]
    /// Write ON CONFLICT target
    fn prepare_on_conflict_target_identifiers(
        &self,
        identifiers: &[OnConflictIdentifier],
        sql: &mut impl SqlWriter,
    ) {
        let mut targets = identifiers.iter();
        join_io!(
            targets,
            target,
            first {
                sql.write_str("(").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                match target {
                    OnConflictIdentifier::Column(col) => {
                        self.prepare_iden(col, sql);
                    }
                    OnConflictIdentifier::Expr(expr) => {
                        self.prepare_expr(expr, sql);
                    }
                }
            },
            last {
                sql.write_str(")").unwrap();
            }
        );
    }

    #[doc(hidden)]
    fn prepare_on_conflict_target_constraint(&self, constraint: &str, sql: &mut impl SqlWriter) {
        sql.write_fmt(format_args!("ON CONSTRAINT \"{}\"", constraint))
            .unwrap();
    }

    #[doc(hidden)]
    /// Write ON CONFLICT action
    fn prepare_on_conflict_action(
        &self,
        on_conflict_action: &Option<OnConflictAction>,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_on_conflict_action_common(on_conflict_action, sql);
    }

    fn prepare_on_conflict_action_common(
        &self,
        on_conflict_action: &Option<OnConflictAction>,
        sql: &mut impl SqlWriter,
    ) {
        if let Some(action) = on_conflict_action {
            match action {
                OnConflictAction::DoNothing(_) => {
                    sql.write_str(" DO NOTHING").unwrap();
                }
                OnConflictAction::Update(update_strats) => {
                    self.prepare_on_conflict_do_update_keywords(sql);
                    let mut update_strats_iter = update_strats.iter();
                    join_io!(
                        update_strats_iter,
                        update_strat,
                        join {
                            sql.write_str(", ").unwrap();
                        },
                        do {
                            match update_strat {
                                OnConflictUpdate::Column(col) => {
                                    self.prepare_iden(col, sql);
                                    sql.write_str(" = ").unwrap();
                                    self.prepare_on_conflict_excluded_table(col, sql);
                                }
                                OnConflictUpdate::Expr(col, expr) => {
                                    self.prepare_iden(col, sql);
                                    sql.write_str(" = ").unwrap();
                                    self.prepare_expr(expr, sql);
                                }
                            }
                        }
                    );
                }
            }
        }
    }

    #[doc(hidden)]
    /// Write ON CONFLICT keywords
    fn prepare_on_conflict_keywords(&self, sql: &mut impl SqlWriter) {
        sql.write_str(" ON CONFLICT ").unwrap();
    }

    #[doc(hidden)]
    /// Write ON CONFLICT keywords
    fn prepare_on_conflict_do_update_keywords(&self, sql: &mut impl SqlWriter) {
        sql.write_str(" DO UPDATE SET ").unwrap();
    }

    #[doc(hidden)]
    /// Write ON CONFLICT update action by retrieving value from the excluded table
    fn prepare_on_conflict_excluded_table(&self, col: &DynIden, sql: &mut impl SqlWriter) {
        sql.write_char(self.quote().left()).unwrap();
        sql.write_str("excluded").unwrap();
        sql.write_char(self.quote().right()).unwrap();
        sql.write_str(".").unwrap();
        self.prepare_iden(col, sql);
    }

    #[doc(hidden)]
    /// Write ON CONFLICT conditions
    fn prepare_on_conflict_condition(
        &self,
        on_conflict_condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_condition(on_conflict_condition, "WHERE", sql)
    }

    #[doc(hidden)]
    /// Hook to insert "OUTPUT" expressions.
    fn prepare_output(&self, _returning: &Option<ReturningClause>, _sql: &mut impl SqlWriter) {}

    #[doc(hidden)]
    /// Hook to insert "RETURNING" statements.
    fn prepare_returning(&self, returning: &Option<ReturningClause>, sql: &mut impl SqlWriter) {
        if let Some(returning) = returning {
            sql.write_str(" RETURNING ").unwrap();
            match &returning {
                ReturningClause::All => sql.write_str("*").unwrap(),
                ReturningClause::Columns(cols) => {
                    let mut cols_iter = cols.iter();
                    join_io!(
                        cols_iter,
                        column_ref,
                        join {
                            sql.write_str(", ").unwrap();
                        },
                        do {
                            self.prepare_column_ref(column_ref, sql);
                        }
                    );
                }
                ReturningClause::Exprs(exprs) => {
                    let mut exprs_iter = exprs.iter();
                    join_io!(
                        exprs_iter,
                        expr,
                        join {
                            sql.write_str(", ").unwrap();
                        },
                        do {
                            self.prepare_expr(expr, sql);
                        }
                    );
                }
            }
        }
    }

    #[doc(hidden)]
    /// Translate a condition to a "WHERE" clause.
    fn prepare_condition(
        &self,
        condition: &ConditionHolder,
        keyword: &str,
        sql: &mut impl SqlWriter,
    ) {
        match &condition.contents {
            ConditionHolderContents::Empty => (),
            ConditionHolderContents::Chain(conditions) => {
                sql.write_str(" ").unwrap();
                sql.write_str(keyword).unwrap();
                sql.write_str(" ").unwrap();
                for (i, log_chain_oper) in conditions.iter().enumerate() {
                    self.prepare_logical_chain_oper(log_chain_oper, i, conditions.len(), sql);
                }
            }
            ConditionHolderContents::Condition(c) => {
                sql.write_str(" ").unwrap();
                sql.write_str(keyword).unwrap();
                sql.write_str(" ").unwrap();
                self.prepare_condition_where(c, sql);
            }
        }
    }

    #[doc(hidden)]
    /// Translate part of a condition to part of a "WHERE" clause.
    fn prepare_condition_where(&self, condition: &Condition, sql: &mut impl SqlWriter) {
        let simple_expr = condition.clone().into();
        self.prepare_expr(&simple_expr, sql);
    }

    #[doc(hidden)]
    /// Translate [`Frame`] into SQL statement.
    fn prepare_frame(&self, frame: &Frame, sql: &mut impl SqlWriter) {
        match *frame {
            Frame::UnboundedPreceding => sql.write_str("UNBOUNDED PRECEDING").unwrap(),
            Frame::Preceding(v) => {
                self.prepare_value(v.into(), sql);
                sql.write_str("PRECEDING").unwrap();
            }
            Frame::CurrentRow => sql.write_str("CURRENT ROW").unwrap(),
            Frame::Following(v) => {
                self.prepare_value(v.into(), sql);
                sql.write_str("FOLLOWING").unwrap();
            }
            Frame::UnboundedFollowing => sql.write_str("UNBOUNDED FOLLOWING").unwrap(),
        }
    }

    #[doc(hidden)]
    /// Translate [`WindowStatement`] into SQL statement.
    fn prepare_window_statement(&self, window: &WindowStatement, sql: &mut impl SqlWriter) {
        let mut partition_iter = window.partition_by.iter();
        join_io!(
            partition_iter,
            expr,
            first {
                sql.write_str("PARTITION BY ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_expr(expr, sql);
            }
        );

        let mut order_iter = window.order_by.iter();
        join_io!(
            order_iter,
            expr,
            first {
                sql.write_str(" ORDER BY ").unwrap();
            },
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_order_expr(expr, sql);
            }
        );

        if let Some(frame) = &window.frame {
            match frame.r#type {
                FrameType::Range => sql.write_str(" RANGE ").unwrap(),
                FrameType::Rows => sql.write_str(" ROWS ").unwrap(),
            };
            if let Some(end) = &frame.end {
                sql.write_str("BETWEEN ").unwrap();
                self.prepare_frame(&frame.start, sql);
                sql.write_str(" AND ").unwrap();
                self.prepare_frame(end, sql);
            } else {
                self.prepare_frame(&frame.start, sql);
            }
        }
    }

    #[doc(hidden)]
    /// Translate a binary expr to SQL.
    fn binary_expr(&self, left: &Expr, op: &BinOper, right: &Expr, sql: &mut impl SqlWriter) {
        // If left has higher precedence than op, we can drop parentheses around left
        let drop_left_higher_precedence =
            self.inner_expr_well_known_greater_precedence(left, &(*op).into());

        // Figure out if left associativity rules allow us to drop the left parenthesis
        let drop_left_assoc = left.is_binary()
            && op == left.get_bin_oper().unwrap()
            && self.well_known_left_associative(op);

        let left_paren = !drop_left_higher_precedence && !drop_left_assoc;
        if left_paren {
            sql.write_str("(").unwrap();
        }
        self.prepare_expr(left, sql);
        if left_paren {
            sql.write_str(")").unwrap();
        }

        sql.write_str(" ").unwrap();
        self.prepare_bin_oper(op, sql);
        sql.write_str(" ").unwrap();

        // If right has higher precedence than op, we can drop parentheses around right
        let drop_right_higher_precedence =
            self.inner_expr_well_known_greater_precedence(right, &(*op).into());

        let op_as_oper = Oper::BinOper(*op);
        // Due to representation of trinary op between as nested binary ops.
        let drop_right_between_hack = op_as_oper.is_between()
            && right.is_binary()
            && matches!(right.get_bin_oper(), Some(&BinOper::And));

        // Due to representation of trinary op like/not like with optional arg escape as nested binary ops.
        let drop_right_escape_hack = op_as_oper.is_like()
            && right.is_binary()
            && matches!(right.get_bin_oper(), Some(&BinOper::Escape));

        // Due to custom representation of casting AS datatype
        let drop_right_as_hack = (op == &BinOper::As) && matches!(right, Expr::Custom(_));

        let right_paren = !drop_right_higher_precedence
            && !drop_right_escape_hack
            && !drop_right_between_hack
            && !drop_right_as_hack;
        if right_paren {
            sql.write_str("(").unwrap();
        }
        self.prepare_expr(right, sql);
        if right_paren {
            sql.write_str(")").unwrap();
        }
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut impl Write) {
        buffer.write_str("'").unwrap();
        self.write_escaped(buffer, string);
        buffer.write_str("'").unwrap();
    }

    #[doc(hidden)]
    /// The name of the function that represents the "if null" condition.
    fn if_null_function(&self) -> &str {
        "IFNULL"
    }

    #[doc(hidden)]
    /// The name of the function that represents the "greatest" function.
    fn greatest_function(&self) -> &str {
        "GREATEST"
    }

    #[doc(hidden)]
    /// The name of the function that represents the "least" function.
    fn least_function(&self) -> &str {
        "LEAST"
    }

    #[doc(hidden)]
    /// The name of the function that returns the char length.
    fn char_length_function(&self) -> &str {
        "CHAR_LENGTH"
    }

    #[doc(hidden)]
    /// The name of the function that returns a random number
    fn random_function(&self) -> &str {
        // Returning it with parens as part of the name because the tuple preparer can't deal with empty lists
        "RANDOM"
    }

    #[doc(hidden)]
    /// The name of the function that returns the lock phrase including the leading 'FOR'
    fn lock_phrase(&self, lock_type: LockType) -> &'static str {
        match lock_type {
            LockType::Update => "FOR UPDATE",
            LockType::NoKeyUpdate => "FOR NO KEY UPDATE",
            LockType::Share => "FOR SHARE",
            LockType::KeyShare => "FOR KEY SHARE",
        }
    }

    /// The keywords for insert default row.
    fn insert_default_keyword(&self) -> &str {
        "(DEFAULT)"
    }

    /// Write insert default rows expression.
    fn insert_default_values(&self, num_rows: u32, sql: &mut impl SqlWriter) {
        sql.write_str("VALUES ").unwrap();
        if num_rows > 0 {
            sql.write_str(self.insert_default_keyword()).unwrap();

            for _ in 1..num_rows {
                sql.write_str(", ").unwrap();
                sql.write_str(self.insert_default_keyword()).unwrap();
            }
        }
    }

    /// Write TRUE constant
    fn prepare_constant_true(&self, sql: &mut impl SqlWriter) {
        self.prepare_constant(&true.into(), sql);
    }

    /// Write FALSE constant
    fn prepare_constant_false(&self, sql: &mut impl SqlWriter) {
        self.prepare_constant(&false.into(), sql);
    }
}

impl SubQueryStatement {
    pub(crate) fn prepare_statement(
        &self,
        query_builder: &impl QueryBuilder,
        sql: &mut impl SqlWriter,
    ) {
        use SubQueryStatement::*;
        match self {
            SelectStatement(stmt) => query_builder.prepare_select_statement(stmt, sql),
            InsertStatement(stmt) => query_builder.prepare_insert_statement(stmt, sql),
            UpdateStatement(stmt) => query_builder.prepare_update_statement(stmt, sql),
            DeleteStatement(stmt) => query_builder.prepare_delete_statement(stmt, sql),
            WithStatement(stmt) => query_builder.prepare_with_query(stmt, sql),
        }
    }
}

pub(crate) struct CommonSqlQueryBuilder;

impl OperLeftAssocDecider for CommonSqlQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        common_well_known_left_associative(op)
    }
}

impl PrecedenceDecider for CommonSqlQueryBuilder {
    fn inner_expr_well_known_greater_precedence(&self, inner: &Expr, outer_oper: &Oper) -> bool {
        common_inner_expr_well_known_greater_precedence(inner, outer_oper)
    }
}

impl ValueEncoder for CommonSqlQueryBuilder {}

impl QueryBuilder for CommonSqlQueryBuilder {
    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut impl SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_value(&self, value: Value, sql: &mut impl SqlWriter) {
        sql.push_param(value, self as _);
    }
}

impl QuotedBuilder for CommonSqlQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for CommonSqlQueryBuilder {}

impl TableRefBuilder for CommonSqlQueryBuilder {}

#[cfg_attr(
    feature = "option-more-parentheses",
    allow(unreachable_code, unused_variables)
)]
pub(crate) fn common_inner_expr_well_known_greater_precedence(
    inner: &Expr,
    outer_oper: &Oper,
) -> bool {
    match inner {
        // We only consider the case where an inner expression is contained in either a
        // unary or binary expression (with an outer_oper).
        // We do not need to wrap with parentheses:
        // Columns, tuples (already wrapped), constants, function calls, values,
        // keywords, subqueries (already wrapped), case (already wrapped)
        Expr::Column(_)
        | Expr::Tuple(_)
        | Expr::Constant(_)
        | Expr::FunctionCall(_)
        | Expr::Value(_)
        | Expr::Keyword(_)
        | Expr::Case(_)
        | Expr::SubQuery(_, _)
        | Expr::TypeName(_) => true,
        Expr::Binary(_, inner_oper, _) => {
            #[cfg(feature = "option-more-parentheses")]
            {
                return false;
            }
            let inner_oper: Oper = (*inner_oper).into();
            if inner_oper.is_arithmetic() || inner_oper.is_shift() {
                outer_oper.is_comparison()
                    || outer_oper.is_between()
                    || outer_oper.is_in()
                    || outer_oper.is_like()
                    || outer_oper.is_logical()
            } else if inner_oper.is_comparison()
                || inner_oper.is_in()
                || inner_oper.is_like()
                || inner_oper.is_is()
            {
                outer_oper.is_logical()
            } else {
                false
            }
        }
        _ => false,
    }
}

pub(crate) fn common_well_known_left_associative(op: &BinOper) -> bool {
    matches!(
        op,
        BinOper::And | BinOper::Or | BinOper::Add | BinOper::Sub | BinOper::Mul | BinOper::Mod
    )
}

macro_rules! join_io {
    ($iter:ident, $item:ident $(, first $first:expr)?, join $join:expr, do $do:expr $(, last $last:expr)?) => {
        if let Some($item) = $iter.next() {
            $($first)?
            $do

            for $item in $iter {
                $join
                $do
            }

            $($last)?
        }
    };
}

pub(crate) use join_io;

#[cfg(test)]
mod tests {
    #[cfg(feature = "with-chrono")]
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    #[cfg(feature = "with-chrono")]
    use crate::QueryBuilder;

    /// [Postgresql reference](https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-DATETIME-INPUT-TIMES)
    ///
    /// [Mysql reference](https://dev.mysql.com/doc/refman/8.4/en/fractional-seconds.html)
    ///
    /// [Sqlite reference](https://sqlite.org/lang_datefunc.html)
    #[test]
    #[cfg(feature = "with-chrono")]
    fn format_time_constant() {
        use crate::{MysqlQueryBuilder, PostgresQueryBuilder, QueryBuilder, SqliteQueryBuilder};

        let time = NaiveTime::from_hms_micro_opt(1, 2, 3, 123456)
            .unwrap()
            .into();

        let mut string = String::new();
        macro_rules! compare {
            ($a:ident, $b:literal) => {
                PostgresQueryBuilder.prepare_constant(&$a, &mut string);
                assert_eq!(string, $b);

                string.clear();

                MysqlQueryBuilder.prepare_constant(&$a, &mut string);
                assert_eq!(string, $b);

                string.clear();

                SqliteQueryBuilder.prepare_constant(&$a, &mut string);
                assert_eq!(string, $b);

                string.clear();
            };
        }

        compare!(time, "'01:02:03.123456'");

        let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
        let t = NaiveTime::from_hms_micro_opt(12, 34, 56, 123456).unwrap();

        let dt = NaiveDateTime::new(d, t);

        let date_time = dt.into();

        compare!(date_time, "'2015-06-03 12:34:56.123456'");

        let date_time_utc = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).into();

        compare!(date_time_utc, "'2015-06-03 12:34:56.123456 +00:00'");

        let date_time_tz = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            dt,
            FixedOffset::east_opt(8 * 3600).unwrap(),
        )
        .into();

        compare!(date_time_tz, "'2015-06-03 20:34:56.123456 +08:00'");
    }

    #[test]
    #[cfg(feature = "postgres-array")]
    fn prepare_array_null_and_empty() {
        use crate::{Array, ArrayType, PostgresQueryBuilder, QueryBuilder, Value};

        let mut string = String::new();
        PostgresQueryBuilder
            .prepare_value(Value::Array(Array::Null(ArrayType::String)), &mut string);
        assert_eq!(string, "NULL");

        string.clear();
        PostgresQueryBuilder.prepare_value(Vec::<String>::new().into(), &mut string);
        assert_eq!(string, "'{}'");
    }
}
