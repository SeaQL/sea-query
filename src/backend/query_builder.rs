use crate::*;

pub trait QueryBuilder<DB: QueryBuilder<DB>>: QuotedBuilder {
    /// The type of placeholder the builder uses for values, and whether it is numbered.
    fn placeholder(&self) -> (&str, bool) {
        ("?", false)
    }

    /// Translate [`InsertStatement`] into SQL statement.
    fn prepare_insert_statement<'a>(
        &self,
        insert: &'a InsertStatement<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        write!(sql, "INSERT").unwrap();

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

        write!(sql, " VALUES ").unwrap();
        insert.values.iter().fold(true, |first, row| {
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

        self.prepare_returning(&insert.returning, sql, collector);
    }

    /// Translate [`SelectStatement`] into SQL statement.
    fn prepare_select_statement<'a>(
        &self,
        select: &'a SelectStatement<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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

        if let Some(from) = &select.from {
            write!(sql, " FROM ").unwrap();
            self.prepare_table_ref(from, sql, collector);
        }

        if !select.join.is_empty() {
            for expr in select.join.iter() {
                write!(sql, " ").unwrap();
                self.prepare_join_expr(expr, sql, collector);
            }
        }

        self.prepare_condition(&select.wherei, "WHERE", sql, collector);

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
    }

    /// Translate [`UpdateStatement`] into SQL statement.
    fn prepare_update_statement<'a>(
        &self,
        update: &'a UpdateStatement<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_delete_statement<'a>(
        &self,
        delete: &'a DeleteStatement<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_simple_expr<'a>(
        &self,
        simple_expr: &'a SimpleExpr<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
                };
            }
            SimpleExpr::Unary(op, expr) => {
                self.prepare_un_oper(op, sql, collector);
                write!(sql, " ").unwrap();
                self.prepare_simple_expr(expr, sql, collector);
            }
            SimpleExpr::FunctionCall(func, exprs) => {
                self.prepare_function(func, sql, collector);
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
            SimpleExpr::Binary(left, op, right) => {
                if *op == BinOper::In && right.is_values() && right.get_values().is_empty() {
                    self.binary_expr(
                        &SimpleExpr::Value(&1),
                        &BinOper::Equal,
                        &SimpleExpr::Value(&2),
                        sql,
                        collector,
                    );
                } else if *op == BinOper::NotIn
                    && right.is_values()
                    && right.get_values().is_empty()
                {
                    self.binary_expr(
                        &SimpleExpr::Value(&1),
                        &BinOper::Equal,
                        &SimpleExpr::Value(&1),
                        sql,
                        collector,
                    );
                } else {
                    self.binary_expr(left, op, right, sql, collector);
                }
            }
            SimpleExpr::SubQuery(sel) => {
                write!(sql, "(").unwrap();
                self.prepare_select_statement(sel, sql, collector);
                write!(sql, ")").unwrap();
            }
            SimpleExpr::Value(val) => {
                self.prepare_value(*val, sql, collector);
            }
            SimpleExpr::Values(list) => {
                write!(sql, "(").unwrap();
                list.iter().fold(true, |first, val| {
                    if !first {
                        write!(sql, ", ").unwrap();
                    }
                    self.prepare_value(*val, sql, collector);
                    false
                });
                write!(sql, ")").unwrap();
            }
            SimpleExpr::Custom(s) => {
                write!(sql, "{}", s).unwrap();
            }
            SimpleExpr::CustomWithValues(expr, values) => {
                let mut tokenizer = Tokenizer::new(expr).into_iter().peekable();
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
                                self.prepare_value(values[count], sql, collector);
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
        }
    }

    /// Translate [`SelectDistinct`] into SQL statement.
    fn prepare_select_distinct<'a>(
        &self,
        select_distinct: &SelectDistinct,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_select_lock<'a>(
        &self,
        select_lock: &LockType,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        write!(
            sql,
            "{}",
            match select_lock {
                LockType::Shared => "FOR SHARE",
                LockType::Exclusive => "FOR UPDATE",
            }
        )
        .unwrap();
    }

    /// Translate [`SelectExpr`] into SQL statement.
    fn prepare_select_expr<'a>(
        &self,
        select_expr: &'a SelectExpr<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        self.prepare_simple_expr(&select_expr.expr, sql, collector);
        match &select_expr.alias {
            Some(alias) => {
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, self.quote());
            }
            None => {}
        }
    }

    /// Translate [`JoinExpr`] into SQL statement.
    fn prepare_join_expr<'a>(
        &self,
        join_expr: &'a JoinExpr<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        self.prepare_join_type(&join_expr.join, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_table_ref(&join_expr.table, sql, collector);
        if let Some(on) = &join_expr.on {
            write!(sql, " ").unwrap();
            self.prepare_join_on(on, sql, collector);
        }
    }

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref<'a>(
        &self,
        table_ref: &'a TableRef<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_un_oper<'a>(
        &self,
        un_oper: &UnOper,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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

    fn prepare_bin_oper_common<'a>(
        &self,
        bin_oper: &BinOper,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
                _ => unimplemented!(),
            }
        )
        .unwrap();
    }

    /// Translate [`BinOper`] into SQL statement.
    fn prepare_bin_oper<'a>(
        &self,
        bin_oper: &BinOper,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        self.prepare_bin_oper_common(bin_oper, sql, collector);
    }

    /// Translate [`LogicalChainOper`] into SQL statement.
    fn prepare_logical_chain_oper<'a>(
        &self,
        log_chain_oper: &'a LogicalChainOper<'a, DB>,
        i: usize,
        length: usize,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_function_common<'a>(
        &self,
        function: &Function,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
                    Function::Count => "COUNT",
                    Function::IfNull => self.if_null_function(),
                    Function::CharLength => self.char_length_function(),
                    Function::Cast => "CAST",
                    Function::Custom(_) => "",
                    #[cfg(feature = "backend-postgres")]
                    Function::PgFunction(_) => unimplemented!(),
                }
            )
            .unwrap();
        }
    }

    fn prepare_function<'a>(
        &self,
        function: &Function,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        self.prepare_function_common(function, sql, collector)
    }

    /// Translate [`JoinType`] into SQL statement.
    fn prepare_join_type<'a>(
        &self,
        join_type: &JoinType,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_order_expr<'a>(
        &self,
        order_expr: &'a OrderExpr<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_order(&order_expr.order, sql, collector);
    }

    /// Translate [`JoinOn`] into SQL statement.
    fn prepare_join_on<'a>(
        &self,
        join_on: &'a JoinOn<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        match join_on {
            JoinOn::Condition(c) => self.prepare_condition(c, "ON", sql, collector),
            JoinOn::Columns(_c) => unimplemented!(),
        }
    }

    /// Translate [`Order`] into SQL statement.
    fn prepare_order<'a>(
        &self,
        order: &Order,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        match order {
            Order::Asc => write!(sql, "ASC").unwrap(),
            Order::Desc => write!(sql, "DESC").unwrap(),
        }
    }

    /// Translate [`Value`] into SQL statement.
    fn prepare_value<'a>(
        &self,
        value: &'a dyn QueryValue<DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
        let (placeholder, numbered) = self.placeholder();
        sql.push_param(placeholder, numbered);
        collector(value);
    }

    /// Translate [`Keyword`] into SQL statement.
    fn prepare_keyword<'a>(
        &self,
        keyword: &Keyword,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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

    #[doc(hidden)]
    /// Hook to insert "RETURNING" statements.
    fn prepare_returning<'a>(
        &self,
        _returning: &'a [SelectExpr<'a, DB>],
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
    }

    #[doc(hidden)]
    /// Translate a condition to a "WHERE" clause.
    fn prepare_condition<'a>(
        &self,
        condition: &'a ConditionHolder<'a, DB>,
        keyword: &str,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn prepare_condition_where<'a>(
        &self,
        condition: &'a Condition<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
    ) {
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
    }

    #[doc(hidden)]
    /// Translate a binary expr to SQL.
    fn binary_expr<'a>(
        &self,
        left: &'a SimpleExpr<'a, DB>,
        op: &BinOper,
        right: &'a SimpleExpr<'a, DB>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<DB>),
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
    fn if_null_function(&self) -> &'static str {
        "IFNULL"
    }

    #[doc(hidden)]
    /// The name of the function that returns the char length.
    fn char_length_function(&self) -> &'static str {
        "CHAR_LENGTH"
    }
}

#[derive(Default)]
pub(crate) struct CommonSqlQueryBuilder;

impl QueryBuilder<CommonSqlQueryBuilder> for CommonSqlQueryBuilder {}

impl QuotedBuilder for CommonSqlQueryBuilder {
    fn quote(&self) -> char {
        '"'
    }
}
