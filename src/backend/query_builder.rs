use crate::*;
use std::ops::Deref;

pub trait QueryBuilder: QuotedBuilder {
    /// The type of placeholder the builder uses for values, and whether it is numbered.
    fn placeholder(&self) -> (&str, bool) {
        ("?", false)
    }

    /// Translate [`InsertStatement`] into SQL statement.
    fn prepare_insert_statement(
        &self,
        insert: &InsertStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_insert(insert.replace, sql);

        if let Some(table) = &insert.table {
            write!(sql, " INTO ").unwrap();
            self.prepare_table_ref(table, sql, collector);
            write!(sql, " ").unwrap();
        }

        write!(sql, "(").unwrap();
        insert.columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap()
            }
            col.prepare(sql, self.quote());
            false
        });
        write!(sql, ")").unwrap();

        if let Some(source) = &insert.source {
            write!(sql, " ").unwrap();
            match source {
                InsertValueSource::Values(values) => {
                    write!(sql, "VALUES ").unwrap();
                    values.iter().fold(true, |first, row| {
                        if !first {
                            write!(sql, ", ").unwrap()
                        }
                        write!(sql, "(").unwrap();
                        row.iter().fold(true, |first, col| {
                            if !first {
                                write!(sql, ", ").unwrap()
                            }
                            self.prepare_simple_expr(col, sql, collector);
                            false
                        });
                        write!(sql, ")").unwrap();
                        false
                    });
                }
                InsertValueSource::Select(select_query) => {
                    self.prepare_select_statement(select_query.deref(), sql, collector);
                }
            }
        }

        self.prepare_on_conflict(&insert.on_conflict, sql, collector);

        self.prepare_returning(&insert.returning, sql, collector);
    }

    /// Translate [`SelectStatement`] into SQL statement.
    fn prepare_select_statement(
        &self,
        select: &SelectStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "SELECT ").unwrap();

        if let Some(distinct) = &select.distinct {
            write!(sql, " ").unwrap();
            self.prepare_select_distinct(distinct, sql, collector);
            write!(sql, " ").unwrap();
        }

        select.selects.iter().fold(true, |first, expr| {
            if !first {
                write!(sql, ", ").unwrap()
            }
            self.prepare_select_expr(expr, sql, collector);
            false
        });

        if !select.from.is_empty() {
            write!(sql, " FROM ").unwrap();
            select.from.iter().fold(true, |first, table_ref| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_table_ref(table_ref, sql, collector);
                false
            });
        }

        if !select.join.is_empty() {
            for expr in select.join.iter() {
                write!(sql, " ").unwrap();
                self.prepare_join_expr(expr, sql, collector);
            }
        }

        self.prepare_condition(&select.r#where, "WHERE", sql, collector);

        if !select.groups.is_empty() {
            write!(sql, " GROUP BY ").unwrap();
            select.groups.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_simple_expr(expr, sql, collector);
                false
            });
        }

        self.prepare_condition(&select.having, "HAVING", sql, collector);

        if !select.unions.is_empty() {
            select.unions.iter().for_each(|(union_type, query)| {
                match union_type {
                    UnionType::Distinct => write!(sql, " UNION ").unwrap(),
                    UnionType::All => write!(sql, " UNION ALL ").unwrap(),
                }
                self.prepare_select_statement(query, sql, collector);
            });
        }

        if !select.orders.is_empty() {
            write!(sql, " ORDER BY ").unwrap();
            select.orders.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_order_expr(expr, sql, collector);
                false
            });
        }

        if let Some(limit) = &select.limit {
            write!(sql, " LIMIT ").unwrap();
            self.prepare_value(limit, sql, collector);
        }

        if let Some(offset) = &select.offset {
            write!(sql, " OFFSET ").unwrap();
            self.prepare_value(offset, sql, collector);
        }

        if let Some(lock) = &select.lock {
            write!(sql, " ").unwrap();
            self.prepare_select_lock(lock, sql, collector);
        }

        if let Some((name, query)) = &select.window {
            write!(sql, " WINDOW ").unwrap();
            name.prepare(sql, self.quote());
            write!(sql, " AS ").unwrap();
            self.prepare_window_statement(query, sql, collector);
        }
    }

    /// Translate [`UpdateStatement`] into SQL statement.
    fn prepare_update_statement(
        &self,
        update: &UpdateStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "UPDATE ").unwrap();

        if let Some(table) = &update.table {
            self.prepare_table_ref(table, sql, collector);
        }

        write!(sql, " SET ").unwrap();

        update.values.iter().fold(true, |first, row| {
            if !first {
                write!(sql, ", ").unwrap()
            }
            let (k, v) = row;
            write!(sql, "{}{}{} = ", self.quote(), k, self.quote()).unwrap();
            self.prepare_simple_expr(v, sql, collector);
            false
        });

        self.prepare_condition(&update.wherei, "WHERE", sql, collector);

        if !update.orders.is_empty() {
            write!(sql, " ORDER BY ").unwrap();
            update.orders.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap();
                }
                self.prepare_order_expr(expr, sql, collector);
                false
            });
        }

        if let Some(limit) = &update.limit {
            write!(sql, " LIMIT ").unwrap();
            self.prepare_value(limit, sql, collector);
        }

        self.prepare_returning(&update.returning, sql, collector);
    }

    /// Translate [`DeleteStatement`] into SQL statement.
    fn prepare_delete_statement(
        &self,
        delete: &DeleteStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "DELETE ").unwrap();

        if let Some(table) = &delete.table {
            write!(sql, "FROM ").unwrap();
            self.prepare_table_ref(table, sql, collector);
        }

        self.prepare_condition(&delete.wherei, "WHERE", sql, collector);

        if !delete.orders.is_empty() {
            write!(sql, " ORDER BY ").unwrap();
            delete.orders.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap();
                }
                self.prepare_order_expr(expr, sql, collector);
                false
            });
        }

        if let Some(limit) = &delete.limit {
            write!(sql, " LIMIT ").unwrap();
            self.prepare_value(limit, sql, collector);
        }

        self.prepare_returning(&delete.returning, sql, collector);
    }

    /// Translate [`SimpleExpr`] into SQL statement.
    fn prepare_simple_expr(
        &self,
        simple_expr: &SimpleExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_simple_expr_common(simple_expr, sql, collector);
    }

    fn prepare_simple_expr_common(
        &self,
        simple_expr: &SimpleExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match simple_expr {
            SimpleExpr::Column(column_ref) => {
                match column_ref {
                    ColumnRef::Column(column) => column.prepare(sql, self.quote()),
                    ColumnRef::TableColumn(table, column) => {
                        table.prepare(sql, self.quote());
                        write!(sql, ".").unwrap();
                        column.prepare(sql, self.quote());
                    }
                    ColumnRef::SchemaTableColumn(schema, table, column) => {
                        schema.prepare(sql, self.quote());
                        write!(sql, ".").unwrap();
                        table.prepare(sql, self.quote());
                        write!(sql, ".").unwrap();
                        column.prepare(sql, self.quote());
                    }
                    ColumnRef::Asterisk => {
                        write!(sql, "*").unwrap();
                    }
                    ColumnRef::TableAsterisk(table) => {
                        table.prepare(sql, self.quote());
                        write!(sql, ".*").unwrap();
                    }
                };
            }
            SimpleExpr::Tuple(exprs) => {
                self.prepare_tuple(exprs, sql, collector);
            }
            SimpleExpr::Unary(op, expr) => {
                self.prepare_un_oper(op, sql, collector);
                write!(sql, " ").unwrap();
                self.prepare_simple_expr(expr, sql, collector);
            }
            SimpleExpr::FunctionCall(func, exprs) => {
                self.prepare_function(func, sql, collector);
                self.prepare_tuple(exprs, sql, collector);
            }
            SimpleExpr::Binary(left, op, right) => {
                if *op == BinOper::In && right.is_values() && right.get_values().is_empty() {
                    self.binary_expr(
                        &SimpleExpr::Value(1.into()),
                        &BinOper::Equal,
                        &SimpleExpr::Value(2.into()),
                        sql,
                        collector,
                    );
                } else if *op == BinOper::NotIn
                    && right.is_values()
                    && right.get_values().is_empty()
                {
                    self.binary_expr(
                        &SimpleExpr::Value(1.into()),
                        &BinOper::Equal,
                        &SimpleExpr::Value(1.into()),
                        sql,
                        collector,
                    );
                } else {
                    self.binary_expr(left, op, right, sql, collector);
                }
            }
            SimpleExpr::SubQuery(sel) => {
                write!(sql, "(").unwrap();
                self.prepare_query_statement(sel.deref(), sql, collector);
                write!(sql, ")").unwrap();
            }
            SimpleExpr::Value(val) => {
                self.prepare_value(val, sql, collector);
            }
            SimpleExpr::Values(list) => {
                write!(sql, "(").unwrap();
                list.iter().fold(true, |first, val| {
                    if !first {
                        write!(sql, ", ").unwrap();
                    }
                    self.prepare_value(val, sql, collector);
                    false
                });
                write!(sql, ")").unwrap();
            }
            SimpleExpr::Custom(s) => {
                write!(sql, "{}", s).unwrap();
            }
            SimpleExpr::CustomWithValues(expr, values) => {
                let mut tokenizer = Tokenizer::new(expr).iter().peekable();
                let mut count = 0;
                while let Some(tok) = tokenizer.next() {
                    match tok {
                        Token::Punctuation(mark) => {
                            if mark == "?" {
                                if let Some(Token::Punctuation(mark)) = tokenizer.peek() {
                                    // escape '??'
                                    if mark == "?" {
                                        write!(sql, "{}", mark).unwrap();
                                        tokenizer.next();
                                        continue;
                                    }
                                }
                                self.prepare_value(&values[count], sql, collector);
                                count += 1;
                            } else {
                                write!(sql, "{}", mark).unwrap();
                            }
                        }
                        _ => write!(sql, "{}", tok).unwrap(),
                    }
                }
            }
            SimpleExpr::Keyword(keyword) => {
                self.prepare_keyword(keyword, sql, collector);
            }
            SimpleExpr::AsEnum(_, expr) => {
                self.prepare_simple_expr(expr, sql, collector);
            }
            SimpleExpr::Case(case_stmt) => {
                self.prepare_case_statement(case_stmt, sql, collector);
            }
        }
    }

    /// Translate [`CaseStatement`] into SQL statement.
    fn prepare_case_statement(
        &self,
        stmts: &CaseStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "(CASE").unwrap();

        let CaseStatement { when, r#else } = stmts;

        for case in when.iter() {
            write!(sql, " WHEN (").unwrap();
            self.prepare_condition_where(&case.condition, sql, collector);
            write!(sql, ") THEN ").unwrap();

            self.prepare_simple_expr(&case.result.clone().into(), sql, collector);
        }
        if let Some(r#else) = r#else.clone() {
            write!(sql, " ELSE ").unwrap();
            self.prepare_simple_expr(&r#else.into(), sql, collector);
        }

        write!(sql, " END) ").unwrap();
    }

    /// Translate [`SelectDistinct`] into SQL statement.
    fn prepare_select_distinct(
        &self,
        select_distinct: &SelectDistinct,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        write!(
            sql,
            "{}",
            match select_distinct {
                SelectDistinct::All => "ALL",
                SelectDistinct::Distinct => "DISTINCT",
                SelectDistinct::DistinctRow => "DISTINCTROW",
            }
        )
        .unwrap();
    }

    /// Translate [`LockType`] into SQL statement.
    fn prepare_select_lock(
        &self,
        lock: &LockClause,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(
            sql,
            "FOR {}",
            match lock.r#type {
                LockType::Update => "UPDATE",
                LockType::NoKeyUpdate => "NO KEY UPDATE",
                LockType::Share => "SHARE",
                LockType::KeyShare => "KEY SHARE",
            }
        )
        .unwrap();
        if !lock.tables.is_empty() {
            write!(sql, " OF ").unwrap();
            lock.tables.iter().fold(true, |first, table_ref| {
                if !first {
                    write!(sql, ", ").unwrap();
                }
                self.prepare_table_ref(table_ref, sql, collector);
                false
            });
        }
        if let Some(behavior) = lock.behavior {
            match behavior {
                LockBehavior::Nowait => write!(sql, " NOWAIT").unwrap(),
                LockBehavior::SkipLocked => write!(sql, " SKIP LOCKED").unwrap(),
            }
        }
    }

    /// Translate [`SelectExpr`] into SQL statement.
    fn prepare_select_expr(
        &self,
        select_expr: &SelectExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_simple_expr(&select_expr.expr, sql, collector);
        match &select_expr.window {
            Some(WindowSelectType::Name(name)) => {
                write!(sql, " OVER ").unwrap();
                name.prepare(sql, self.quote())
            }
            Some(WindowSelectType::Query(window)) => {
                write!(sql, " OVER ").unwrap();
                write!(sql, "( ").unwrap();
                self.prepare_window_statement(window, sql, collector);
                write!(sql, " ) ").unwrap();
            }
            None => {}
        };

        match &select_expr.alias {
            Some(alias) => {
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, self.quote());
            }
            None => {}
        };
    }

    /// Translate [`JoinExpr`] into SQL statement.
    fn prepare_join_expr(
        &self,
        join_expr: &JoinExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_join_type(&join_expr.join, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_join_table_ref(join_expr, sql, collector);
        if let Some(on) = &join_expr.on {
            write!(sql, " ").unwrap();
            self.prepare_join_on(on, sql, collector);
        }
    }

    fn prepare_join_table_ref(
        &self,
        join_expr: &JoinExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if join_expr.lateral {
            write!(sql, "LATERAL ").unwrap();
        }
        QueryBuilder::prepare_table_ref_common(self, &join_expr.table, sql, collector);
    }

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref(
        &self,
        table_ref: &TableRef,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        QueryBuilder::prepare_table_ref_common(self, table_ref, sql, collector);
    }

    fn prepare_table_ref_common(
        &self,
        table_ref: &TableRef,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match table_ref {
            TableRef::Table(iden) => {
                iden.prepare(sql, self.quote());
            }
            TableRef::SchemaTable(schema, table) => {
                schema.prepare(sql, self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql, self.quote());
            }
            TableRef::DatabaseSchemaTable(database, schema, table) => {
                database.prepare(sql, self.quote());
                write!(sql, ".").unwrap();
                schema.prepare(sql, self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql, self.quote());
            }
            TableRef::TableAlias(iden, alias) => {
                iden.prepare(sql, self.quote());
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, self.quote());
            }
            TableRef::SchemaTableAlias(schema, table, alias) => {
                schema.prepare(sql, self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql, self.quote());
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, self.quote());
            }
            TableRef::DatabaseSchemaTableAlias(database, schema, table, alias) => {
                database.prepare(sql, self.quote());
                write!(sql, ".").unwrap();
                schema.prepare(sql, self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql, self.quote());
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, self.quote());
            }
            TableRef::SubQuery(query, alias) => {
                write!(sql, "(").unwrap();
                self.prepare_select_statement(query, sql, collector);
                write!(sql, ")").unwrap();
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, self.quote());
            }
        }
    }

    /// Translate [`UnOper`] into SQL statement.
    fn prepare_un_oper(
        &self,
        un_oper: &UnOper,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        write!(
            sql,
            "{}",
            match un_oper {
                UnOper::Not => "NOT",
            }
        )
        .unwrap();
    }

    fn prepare_bin_oper_common(
        &self,
        bin_oper: &BinOper,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        write!(
            sql,
            "{}",
            match bin_oper {
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
                BinOper::As => "AS",
                #[allow(unreachable_patterns)]
                _ => unimplemented!(),
            }
        )
        .unwrap();
    }

    /// Translate [`BinOper`] into SQL statement.
    fn prepare_bin_oper(
        &self,
        bin_oper: &BinOper,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_bin_oper_common(bin_oper, sql, collector);
    }

    /// Translate [`LogicalChainOper`] into SQL statement.
    fn prepare_logical_chain_oper(
        &self,
        log_chain_oper: &LogicalChainOper,
        i: usize,
        length: usize,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        let (simple_expr, oper) = match log_chain_oper {
            LogicalChainOper::And(simple_expr) => (simple_expr, "AND"),
            LogicalChainOper::Or(simple_expr) => (simple_expr, "OR"),
        };
        if i > 0 {
            write!(sql, " {} ", oper).unwrap();
        }
        let both_binary = match simple_expr {
            SimpleExpr::Binary(_, _, right) => {
                matches!(right.as_ref(), SimpleExpr::Binary(_, _, _))
            }
            _ => false,
        };
        let need_parentheses = length > 1 && both_binary;
        if need_parentheses {
            write!(sql, "(").unwrap();
        }
        self.prepare_simple_expr(simple_expr, sql, collector);
        if need_parentheses {
            write!(sql, ")").unwrap();
        }
    }

    /// Translate [`Function`] into SQL statement.
    fn prepare_function_common(
        &self,
        function: &Function,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        if let Function::Custom(iden) = function {
            iden.unquoted(sql);
        } else {
            write!(
                sql,
                "{}",
                match function {
                    Function::Max => "MAX",
                    Function::Min => "MIN",
                    Function::Sum => "SUM",
                    Function::Avg => "AVG",
                    Function::Coalesce => "COALESCE",
                    Function::Count => "COUNT",
                    Function::IfNull => self.if_null_function(),
                    Function::CharLength => self.char_length_function(),
                    Function::Cast => "CAST",
                    Function::Lower => "LOWER",
                    Function::Upper => "UPPER",
                    Function::Custom(_) => "",
                    #[cfg(feature = "backend-postgres")]
                    Function::PgFunction(_) => unimplemented!(),
                }
            )
            .unwrap();
        }
    }

    /// Translate [`QueryStatement`] into SQL statement.
    fn prepare_query_statement(
        &self,
        query: &SubQueryStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    );

    fn prepare_with_query(
        &self,
        query: &WithQuery,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_with_clause(&query.with_clause, sql, collector);
        self.prepare_query_statement(query.query.as_ref().unwrap().deref(), sql, collector);
    }

    fn prepare_with_clause(
        &self,
        with_clause: &WithClause,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_with_clause_start(with_clause, sql);
        self.prepare_with_clause_common_tables(with_clause, sql, collector);
        if with_clause.recursive {
            self.prepare_with_clause_recursive_options(with_clause, sql, collector);
        }
    }

    fn prepare_with_clause_recursive_options(
        &self,
        with_clause: &WithClause,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if with_clause.recursive {
            if let Some(search) = &with_clause.search {
                write!(
                    sql,
                    "SEARCH {} FIRST BY ",
                    match &search.order.as_ref().unwrap() {
                        SearchOrder::BREADTH => "BREADTH",
                        SearchOrder::DEPTH => "DEPTH",
                    }
                )
                .unwrap();

                self.prepare_simple_expr(&search.expr.as_ref().unwrap().expr, sql, collector);

                write!(sql, " SET ").unwrap();

                search
                    .expr
                    .as_ref()
                    .unwrap()
                    .alias
                    .as_ref()
                    .unwrap()
                    .prepare(sql, self.quote());
                write!(sql, " ").unwrap();
            }
            if let Some(cycle) = &with_clause.cycle {
                write!(sql, "CYCLE ").unwrap();

                self.prepare_simple_expr(cycle.expr.as_ref().unwrap(), sql, collector);

                write!(sql, " SET ").unwrap();

                cycle.set_as.as_ref().unwrap().prepare(sql, self.quote());
                write!(sql, " USING ").unwrap();
                cycle.using.as_ref().unwrap().prepare(sql, self.quote());
                write!(sql, " ").unwrap();
            }
        }
    }

    fn prepare_with_clause_common_tables(
        &self,
        with_clause: &WithClause,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        let mut cte_first = true;
        assert_ne!(
            with_clause.cte_expressions.len(),
            0,
            "Cannot build a with query that has no common table expression!"
        );

        if with_clause.recursive {
            assert_eq!(
                with_clause.cte_expressions.len(),
                1,
                "Cannot build a recursive query with more than one common table! \
                A recursive with query must have a single cte inside it that has a union query of \
                two queries!"
            );
        }
        for cte in &with_clause.cte_expressions {
            if !cte_first {
                write!(sql, ", ").unwrap();
            }
            cte_first = false;

            self.prepare_with_query_clause_common_table(cte, sql, collector);
        }
    }

    fn prepare_with_query_clause_common_table(
        &self,
        cte: &CommonTableExpression,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        cte.table_name.as_ref().unwrap().prepare(sql, self.quote());

        if cte.cols.is_empty() {
            write!(sql, " ").unwrap();
        } else {
            write!(sql, " (").unwrap();

            let mut col_first = true;
            for col in &cte.cols {
                if !col_first {
                    write!(sql, ", ").unwrap();
                }
                col_first = false;
                col.prepare(sql, self.quote());
            }

            write!(sql, ") ").unwrap();
        }

        write!(sql, "AS ").unwrap();

        self.prepare_with_query_clause_materialization(cte, sql);

        write!(sql, "(").unwrap();

        self.prepare_query_statement(cte.query.as_ref().unwrap().deref(), sql, collector);

        write!(sql, ") ").unwrap();
    }

    fn prepare_with_query_clause_materialization(
        &self,
        cte: &CommonTableExpression,
        sql: &mut SqlWriter,
    ) {
        if let Some(materialized) = cte.materialized {
            write!(
                sql,
                "{} MATERIALIZED ",
                if materialized { "" } else { "NOT" }
            )
            .unwrap()
        }
    }

    fn prepare_with_clause_start(&self, with_clause: &WithClause, sql: &mut SqlWriter) {
        write!(sql, "WITH ").unwrap();

        if with_clause.recursive {
            write!(sql, "RECURSIVE ").unwrap();
        }
    }

    fn prepare_insert(&self, replace: bool, sql: &mut SqlWriter) {
        if replace {
            write!(sql, "REPLACE").unwrap();
        } else {
            write!(sql, "INSERT").unwrap();
        }
    }

    fn prepare_function(
        &self,
        function: &Function,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        self.prepare_function_common(function, sql, collector)
    }

    /// Translate [`JoinType`] into SQL statement.
    fn prepare_join_type(
        &self,
        join_type: &JoinType,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        write!(
            sql,
            "{}",
            match join_type {
                JoinType::Join => "JOIN",
                JoinType::InnerJoin => "INNER JOIN",
                JoinType::LeftJoin => "LEFT JOIN",
                JoinType::RightJoin => "RIGHT JOIN",
            }
        )
        .unwrap()
    }

    /// Translate [`OrderExpr`] into SQL statement.
    fn prepare_order_expr(
        &self,
        order_expr: &OrderExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql, collector);
        }
        write!(sql, " ").unwrap();
        self.prepare_order(order_expr, sql, collector);
    }

    /// Translate [`JoinOn`] into SQL statement.
    fn prepare_join_on(
        &self,
        join_on: &JoinOn,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match join_on {
            JoinOn::Condition(c) => self.prepare_condition(c, "ON", sql, collector),
            JoinOn::Columns(_c) => unimplemented!(),
        }
    }

    /// Translate [`Order`] into SQL statement.
    fn prepare_order(
        &self,
        order_expr: &OrderExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match &order_expr.order {
            Order::Asc => write!(sql, "ASC").unwrap(),
            Order::Desc => write!(sql, "DESC").unwrap(),
            Order::Field(values) => self.prepare_field_order(order_expr, values, sql, collector),
        }
    }

    /// Translate [`Order::Field`] into SQL statement
    fn prepare_field_order(
        &self,
        order_expr: &OrderExpr,
        values: &Values,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "CASE ").unwrap();
        let mut i = 0;
        for value in &values.0 {
            write!(sql, " WHEN ").unwrap();
            self.prepare_simple_expr(&order_expr.expr, sql, collector);
            write!(sql, "=").unwrap();
            let value = self.value_to_string(value);
            write!(sql, "{}", value).unwrap();
            write!(sql, " THEN {} ", i).unwrap();
            i += 1;
        }
        write!(sql, "ELSE {} END", i).unwrap();
    }

    /// Translate [`Value`] into SQL statement.
    fn prepare_value(&self, value: &Value, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        let (placeholder, numbered) = self.placeholder();
        sql.push_param(placeholder, numbered);
        collector(value.clone());
    }

    /// Translate [`SimpleExpr::Tuple`] into SQL statement.
    fn prepare_tuple(
        &self,
        exprs: &[SimpleExpr],
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "(").unwrap();
        exprs.iter().fold(true, |first, expr| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            self.prepare_simple_expr(expr, sql, collector);
            false
        });
        write!(sql, ")").unwrap();
    }

    /// Translate [`Keyword`] into SQL statement.
    fn prepare_keyword(
        &self,
        keyword: &Keyword,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        if let Keyword::Custom(iden) = keyword {
            iden.unquoted(sql);
        } else {
            write!(
                sql,
                "{}",
                match keyword {
                    Keyword::Null => "NULL",
                    Keyword::Custom(_) => "",
                }
            )
            .unwrap();
        }
    }

    /// Convert a SQL value into syntax-specific string
    fn value_to_string(&self, v: &Value) -> String {
        let mut s = String::new();
        match v {
            Value::Bool(None)
            | Value::TinyInt(None)
            | Value::SmallInt(None)
            | Value::Int(None)
            | Value::BigInt(None)
            | Value::TinyUnsigned(None)
            | Value::SmallUnsigned(None)
            | Value::Unsigned(None)
            | Value::BigUnsigned(None)
            | Value::Float(None)
            | Value::Double(None)
            | Value::String(None)
            | Value::Bytes(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-json")]
            Value::Json(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-time")]
            Value::TimeDate(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-time")]
            Value::TimeTime(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(None) => write!(s, "NULL").unwrap(),
            #[cfg(feature = "postgres-array")]
            Value::Array(None) => write!(s, "NULL").unwrap(),
            Value::Bool(Some(b)) => write!(s, "{}", if *b { "TRUE" } else { "FALSE" }).unwrap(),
            Value::TinyInt(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::SmallInt(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::Int(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::BigInt(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::TinyUnsigned(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::SmallUnsigned(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::Unsigned(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::BigUnsigned(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::Float(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::Double(Some(v)) => write!(s, "{}", v).unwrap(),
            Value::String(Some(v)) => self.write_string_quoted(v, &mut s),
            Value::Bytes(Some(v)) => write!(
                s,
                "x\'{}\'",
                v.iter().map(|b| format!("{:02X}", b)).collect::<String>()
            )
            .unwrap(),
            #[cfg(feature = "with-json")]
            Value::Json(Some(v)) => self.write_string_quoted(&v.to_string(), &mut s),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(Some(v)) => write!(s, "\'{}\'", v.format("%Y-%m-%d")).unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(Some(v)) => write!(s, "\'{}\'", v.format("%H:%M:%S")).unwrap(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(Some(v)) => {
                write!(s, "\'{}\'", v.format("%Y-%m-%d %H:%M:%S")).unwrap()
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(Some(v)) => {
                write!(s, "\'{}\'", v.format("%Y-%m-%d %H:%M:%S %:z")).unwrap()
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(Some(v)) => {
                write!(s, "\'{}\'", v.format("%Y-%m-%d %H:%M:%S %:z")).unwrap()
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(Some(v)) => {
                write!(s, "\'{}\'", v.format("%Y-%m-%d %H:%M:%S %:z")).unwrap()
            }
            #[cfg(feature = "with-time")]
            Value::TimeDate(Some(v)) => write!(s, "\'{}\'", v.format("%Y-%m-%d")).unwrap(),
            #[cfg(feature = "with-time")]
            Value::TimeTime(Some(v)) => write!(s, "\'{}\'", v.format("%H:%M:%S")).unwrap(),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(Some(v)) => {
                write!(s, "\'{}\'", v.format("%Y-%m-%d %H:%M:%S")).unwrap()
            }
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(Some(v)) => {
                write!(s, "\'{}\'", v.format("%Y-%m-%d %H:%M:%S %z")).unwrap()
            }
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(Some(v)) => write!(s, "{}", v).unwrap(),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(Some(v)) => write!(s, "{}", v).unwrap(),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(Some(v)) => write!(s, "\'{}\'", v).unwrap(),
            #[cfg(feature = "postgres-array")]
            Value::Array(Some(v)) => write!(
                s,
                "\'{{{}}}\'",
                v.iter()
                    .map(|element| self.value_to_string(element))
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .unwrap(),
        };
        s
    }

    #[doc(hidden)]
    /// Write ON CONFLICT expression
    fn prepare_on_conflict(
        &self,
        on_conflict: &Option<OnConflict>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if let Some(on_conflict) = on_conflict {
            self.prepare_on_conflict_keywords(sql, collector);
            self.prepare_on_conflict_target(&on_conflict.target, sql, collector);
            self.prepare_on_conflict_action(&on_conflict.action, sql, collector);
        }
    }

    #[doc(hidden)]
    /// Write ON CONFLICT target
    fn prepare_on_conflict_target(
        &self,
        on_conflict_target: &Option<OnConflictTarget>,
        sql: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        if let Some(target) = on_conflict_target {
            match target {
                OnConflictTarget::ConflictColumns(columns) => {
                    write!(sql, "(").unwrap();
                    columns.iter().fold(true, |first, col| {
                        if !first {
                            write!(sql, ", ").unwrap()
                        }
                        col.prepare(sql, self.quote());
                        false
                    });
                    write!(sql, ")").unwrap();
                }
            }
        }
    }

    #[doc(hidden)]
    /// Write ON CONFLICT action
    fn prepare_on_conflict_action(
        &self,
        on_conflict_action: &Option<OnConflictAction>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if let Some(action) = on_conflict_action {
            match action {
                OnConflictAction::DoNothing => {
                    write!(sql, " DO NOTHING").unwrap();
                }
                OnConflictAction::UpdateColumns(columns) => {
                    self.prepare_on_conflict_do_update_keywords(sql, collector);
                    columns.iter().fold(true, |first, col| {
                        if !first {
                            write!(sql, ", ").unwrap()
                        }
                        col.prepare(sql, self.quote());
                        write!(sql, " = ").unwrap();
                        self.prepare_on_conflict_excluded_table(col, sql, collector);
                        false
                    });
                }
                OnConflictAction::UpdateExprs(column_exprs) => {
                    self.prepare_on_conflict_do_update_keywords(sql, collector);
                    column_exprs.iter().fold(true, |first, (col, expr)| {
                        if !first {
                            write!(sql, ", ").unwrap()
                        }
                        col.prepare(sql, self.quote());
                        write!(sql, " = ").unwrap();
                        self.prepare_simple_expr(expr, sql, collector);
                        false
                    });
                }
            }
        }
    }

    #[doc(hidden)]
    /// Write ON CONFLICT keywords
    fn prepare_on_conflict_keywords(&self, sql: &mut SqlWriter, _: &mut dyn FnMut(Value)) {
        write!(sql, " ON CONFLICT ").unwrap();
    }

    #[doc(hidden)]
    /// Write ON CONFLICT keywords
    fn prepare_on_conflict_do_update_keywords(
        &self,
        sql: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        write!(sql, " DO UPDATE SET ").unwrap();
    }

    #[doc(hidden)]
    /// Write ON CONFLICT update action by retrieving value from the excluded table
    fn prepare_on_conflict_excluded_table(
        &self,
        col: &DynIden,
        sql: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        write!(sql, "{0}excluded{0}", self.quote()).unwrap();
        write!(sql, ".").unwrap();
        col.prepare(sql, self.quote());
    }

    #[doc(hidden)]
    /// Hook to insert "RETURNING" statements.
    fn prepare_returning(
        &self,
        returning: &[SelectExpr],
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if !returning.is_empty() {
            write!(sql, " RETURNING ").unwrap();
            returning.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_select_expr(expr, sql, collector);
                false
            });
        }
    }

    #[doc(hidden)]
    /// Translate a condition to a "WHERE" clause.
    fn prepare_condition(
        &self,
        condition: &ConditionHolder,
        keyword: &str,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if !condition.is_empty() {
            write!(sql, " {} ", keyword).unwrap();
        }
        match &condition.contents {
            ConditionHolderContents::Empty => (),
            ConditionHolderContents::Chain(conditions) => {
                for (i, log_chain_oper) in conditions.iter().enumerate() {
                    self.prepare_logical_chain_oper(
                        log_chain_oper,
                        i,
                        conditions.len(),
                        sql,
                        collector,
                    );
                }
            }
            ConditionHolderContents::Condition(c) => {
                self.prepare_condition_where(c, sql, collector);
            }
        }
    }

    #[doc(hidden)]
    /// Translate part of a condition to part of a "WHERE" clause.
    fn prepare_condition_where(
        &self,
        condition: &Condition,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if condition.negate {
            write!(sql, "NOT (").unwrap();
        }
        let mut is_first = true;
        for cond in &condition.conditions {
            if is_first {
                is_first = false;
            } else {
                match condition.condition_type {
                    ConditionType::Any => write!(sql, " OR ").unwrap(),
                    ConditionType::All => write!(sql, " AND ").unwrap(),
                }
            }
            match cond {
                ConditionExpression::Condition(c) => {
                    if condition.conditions.len() > 1 {
                        write!(sql, "(").unwrap();
                    }
                    self.prepare_condition_where(c, sql, collector);
                    if condition.conditions.len() > 1 {
                        write!(sql, ")").unwrap();
                    }
                }
                ConditionExpression::SimpleExpr(e) => {
                    if condition.conditions.len() > 1 && (e.is_logical() || e.is_between()) {
                        write!(sql, "(").unwrap();
                    }
                    self.prepare_simple_expr(e, sql, collector);
                    if condition.conditions.len() > 1 && (e.is_logical() || e.is_between()) {
                        write!(sql, ")").unwrap();
                    }
                }
            }
        }
        if condition.negate {
            write!(sql, ")").unwrap();
        }
    }

    #[doc(hidden)]
    /// Translate [`Frame`] into SQL statement.
    fn prepare_frame(&self, frame: &Frame, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        match *frame {
            Frame::UnboundedPreceding => write!(sql, " UNBOUNDED PRECEDING ").unwrap(),
            Frame::Preceding(v) => {
                self.prepare_value(&Some(v).into(), sql, collector);
                write!(sql, " PRECEDING ").unwrap();
            }
            Frame::CurrentRow => write!(sql, " CURRENT ROW ").unwrap(),
            Frame::Following(v) => {
                self.prepare_value(&Some(v).into(), sql, collector);
                write!(sql, " FOLLOWING ").unwrap();
            }
            Frame::UnboundedFollowing => write!(sql, " UNBOUNDED FOLLOWING ").unwrap(),
        }
    }

    #[doc(hidden)]
    /// Translate [`WindowStatement`] into SQL statement.
    fn prepare_window_statement(
        &self,
        window: &WindowStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if !window.partition_by.is_empty() {
            write!(sql, " PARTITION BY ").unwrap();
            window.partition_by.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_simple_expr(expr, sql, collector);
                false
            });
        }

        if !window.order_by.is_empty() {
            write!(sql, " ORDER BY ").unwrap();
            window.order_by.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_order_expr(expr, sql, collector);
                false
            });
        }

        if let Some(frame) = &window.frame {
            match frame.r#type {
                FrameType::Range => write!(sql, " RANGE ").unwrap(),
                FrameType::Rows => write!(sql, " ROWS ").unwrap(),
            };
            if let Some(end) = &frame.end {
                write!(sql, "BETWEEN ").unwrap();
                self.prepare_frame(&frame.start, sql, collector);
                write!(sql, " AND ").unwrap();
                self.prepare_frame(end, sql, collector);
            } else {
                self.prepare_frame(&frame.start, sql, collector);
            }
        }
    }

    #[doc(hidden)]
    /// Translate a binary expr to SQL.
    fn binary_expr(
        &self,
        left: &SimpleExpr,
        op: &BinOper,
        right: &SimpleExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        let no_paren = matches!(op, BinOper::Equal | BinOper::NotEqual);
        let left_paren = left.need_parentheses()
            && left.is_binary()
            && *op != left.get_bin_oper().unwrap()
            && !no_paren;
        if left_paren {
            write!(sql, "(").unwrap();
        }
        self.prepare_simple_expr(left, sql, collector);
        if left_paren {
            write!(sql, ")").unwrap();
        }
        write!(sql, " ").unwrap();
        self.prepare_bin_oper(op, sql, collector);
        write!(sql, " ").unwrap();
        let no_right_paren = matches!(op, BinOper::Between | BinOper::NotBetween);
        let right_paren = (right.need_parentheses()
            || right.is_binary() && *op != left.get_bin_oper().unwrap())
            && !no_right_paren
            && !no_paren;
        if right_paren {
            write!(sql, "(").unwrap();
        }
        self.prepare_simple_expr(right, sql, collector);
        if right_paren {
            write!(sql, ")").unwrap();
        }
    }

    #[doc(hidden)]
    /// Write a string surrounded by escaped quotes.
    fn write_string_quoted(&self, string: &str, buffer: &mut String) {
        write!(buffer, "\'{}\'", escape_string(string)).unwrap()
    }

    #[doc(hidden)]
    /// The name of the function that represents the "if null" condition.
    fn if_null_function(&self) -> &str {
        "IFNULL"
    }

    #[doc(hidden)]
    /// The name of the function that returns the char length.
    fn char_length_function(&self) -> &str {
        "CHAR_LENGTH"
    }
}

impl SubQueryStatement {
    pub(crate) fn prepare_statement(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        use SubQueryStatement::*;
        match self {
            SelectStatement(stmt) => query_builder.prepare_select_statement(stmt, sql, collector),
            InsertStatement(stmt) => query_builder.prepare_insert_statement(stmt, sql, collector),
            UpdateStatement(stmt) => query_builder.prepare_update_statement(stmt, sql, collector),
            DeleteStatement(stmt) => query_builder.prepare_delete_statement(stmt, sql, collector),
            WithStatement(stmt) => query_builder.prepare_with_query(stmt, sql, collector),
        }
    }
}

pub(crate) struct CommonSqlQueryBuilder;

impl QueryBuilder for CommonSqlQueryBuilder {
    fn prepare_query_statement(
        &self,
        query: &SubQueryStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        query.prepare_statement(self, sql, collector);
    }
}

impl QuotedBuilder for CommonSqlQueryBuilder {
    fn quote(&self) -> char {
        '"'
    }
}
