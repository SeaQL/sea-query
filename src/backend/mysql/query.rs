use super::*;

impl QueryBuilder for MysqlQueryBuilder {
    fn prepare_insert_statement(&self, insert: &InsertStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
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
            col.prepare(sql, '`');
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
                self.prepare_value(col, sql, collector);
                false
            });
            write!(sql, ")").unwrap();
            false
        });
    }

    fn prepare_select_statement(&self, select: &SelectStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
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

        if !select.wherei.is_empty() {
            write!(sql, " WHERE ").unwrap();
        }
        for (i, log_chain_oper) in select.wherei.iter().enumerate() {
            self.prepare_logical_chain_oper(log_chain_oper, i, select.wherei.len(), sql, collector);
        }

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

        if !select.having.is_empty() {
            write!(sql, " HAVING ").unwrap();
        }
        for (i, log_chain_oper) in select.having.iter().enumerate() {
            self.prepare_logical_chain_oper(log_chain_oper, i, select.having.len(), sql, collector);
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
    }

    fn prepare_update_statement(&self, update: &UpdateStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
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
            write!(sql, "`{}` = ", k).unwrap();
            self.prepare_simple_expr(v, sql, collector);
            false
        });

        if let Some(wherei) = &update.wherei {
            write!(sql, " WHERE ").unwrap();
            self.prepare_simple_expr(wherei, sql, collector);
        }

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
    }

    fn prepare_delete_statement(&self, delete: &DeleteStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        write!(sql, "DELETE ").unwrap();
        
        if let Some(table) = &delete.table {
            write!(sql, "FROM ").unwrap();
            self.prepare_table_ref(table, sql, collector);
        }

        if let Some(wherei) = &delete.wherei {
            write!(sql, " WHERE ").unwrap();
            self.prepare_simple_expr(wherei, sql, collector);
        }

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
    }

    fn prepare_simple_expr(&self, simple_expr: &SimpleExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        match simple_expr {
            SimpleExpr::Column(column) => {
                column.prepare(sql, '`');
            },
            SimpleExpr::TableColumn(table, column) => {
                table.prepare(sql, '`');
                write!(sql, ".").unwrap();
                column.prepare(sql, '`');
            },
            SimpleExpr::Unary(op, expr) => {
                self.prepare_un_oper(op, sql, collector);
                write!(sql, " ").unwrap();
                self.prepare_simple_expr(expr, sql, collector);
            },
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
            },
            SimpleExpr::Binary(left, op, right) => {
                let no_paren = matches!(op, BinOper::Equal | BinOper::NotEqual);
                let left_paren =
                    left.need_parentheses() &&
                    left.is_binary() && *op != left.get_bin_oper().unwrap() &&
                    !no_paren;
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
                let right_paren =
                    (right.need_parentheses() ||
                        right.is_binary() && *op != left.get_bin_oper().unwrap()) &&
                    !no_right_paren &&
                    !no_paren;
                if right_paren {
                    write!(sql, "(").unwrap();
                }
                self.prepare_simple_expr(right, sql, collector);
                if right_paren {
                    write!(sql, ")").unwrap();
                }
            },
            SimpleExpr::SubQuery(sel) => {
                write!(sql, "(").unwrap();
                self.prepare_select_statement(sel, sql, collector);
                write!(sql, ")").unwrap();
            },
            SimpleExpr::Value(val) => {
                self.prepare_value(val, sql, collector);
            },
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
            },
            SimpleExpr::Custom(s) => {
                write!(sql, "{}", s).unwrap();
            },
        }
    }

    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        write!(sql, "{}", match select_distinct {
            SelectDistinct::All => "ALL",
            SelectDistinct::Distinct => "DISTINCT",
            SelectDistinct::DistinctRow => "DISTINCTROW",
        }).unwrap();
    }

    fn prepare_select_expr(&self, select_expr: &SelectExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        self.prepare_simple_expr(&select_expr.expr, sql, collector);
        match &select_expr.alias {
            Some(alias) => {
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, '`');
            },
            None => {},
        }
    }

    fn prepare_join_expr(&self, join_expr: &JoinExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        self.prepare_join_type(&join_expr.join, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_table_ref(&join_expr.table, sql, collector);
        if let Some(on) = &join_expr.on {
            write!(sql, " ").unwrap();
            self.prepare_join_on(on, sql, collector);
        }
    }

    fn prepare_table_ref(&self, table_ref: &TableRef, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        match table_ref {
            TableRef::Table(iden) => {
                iden.prepare(sql, '`');
            },
            TableRef::TableAlias(iden, alias) => {
                iden.prepare(sql, '`');
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, '`');
            },
            TableRef::SubQuery(query, alias) => {
                write!(sql, "(").unwrap();
                self.prepare_select_statement(query, sql, collector);
                write!(sql, ")").unwrap();
                write!(sql, " AS ").unwrap();
                alias.prepare(sql, '`');
            },
        }
    }

    fn prepare_un_oper(&self, un_oper: &UnOper, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        write!(sql, "{}", match un_oper {
            UnOper::Not => "NOT",
        }).unwrap();
    }

    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        write!(sql, "{}", match bin_oper {
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
        }).unwrap();
    }

    fn prepare_logical_chain_oper(&self, log_chain_oper: &LogicalChainOper, i: usize, length: usize, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        let (simple_expr, oper) = match log_chain_oper {
            LogicalChainOper::And(simple_expr) => (simple_expr, "AND"),
            LogicalChainOper::Or(simple_expr) => (simple_expr, "OR"),
        };
        if i > 0 {
            write!(sql, " {} ", oper).unwrap();
        }
        let both_binary = match simple_expr {
            SimpleExpr::Binary(_, _, right) => matches!(right.as_ref(), SimpleExpr::Binary(_, _, _)),
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

    fn prepare_function(&self, function: &Function, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        if let Function::Custom(iden) = function {
            iden.unquoted(sql);
        } else {
            write!(sql, "{}", match function {
                Function::Max => "MAX",
                Function::Min => "MIN",
                Function::Sum => "SUM",
                Function::Count => "COUNT",
                Function::IfNull => "IFNULL",
                Function::Custom(_) => "",
            }).unwrap();
        }
    }

    fn prepare_join_type(&self, join_type: &JoinType, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        write!(sql, "{}", match join_type {
            JoinType::Join => "JOIN",
            JoinType::InnerJoin => "INNER JOIN",
            JoinType::LeftJoin => "LEFT JOIN",
            JoinType::RightJoin => "RIGHT JOIN",
        }).unwrap()
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_order(&order_expr.order, sql, collector);
    }

    fn prepare_join_on(&self, join_on: &JoinOn, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        match join_on {
            JoinOn::Condition(c) => {
                write!(sql, "ON ").unwrap();
                self.prepare_simple_expr(c, sql, collector);
            },
            JoinOn::Columns(_c) => unimplemented!(),
        }
    }

    fn prepare_order(&self, order: &Order, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        match order {
            Order::Asc => {
                write!(sql, "ASC").unwrap()
            },
            Order::Desc => {
                write!(sql, "DESC").unwrap()
            },
        }
    }

    fn prepare_value(&self, value: &Value, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        sql.push_param("?", false);
        collector(value.clone());
    }

    fn value_to_string(&self, v: &Value) -> String {
        mysql_value_to_string(v)
    }
}

pub fn mysql_value_to_string(v: &Value) -> String {
    let mut s = String::new();
    match v {
        Value::Null => write!(s, "NULL").unwrap(),
        Value::Bytes(v) => write!(s, "\'{}\'", escape_string(std::str::from_utf8(v).unwrap())).unwrap(),
        Value::Int(v) => write!(s, "{}", v).unwrap(),
        Value::UInt(v) => write!(s, "{}", v).unwrap(),
        Value::Float(v) => write!(s, "{}", v).unwrap(),
        Value::Double(v) => write!(s, "{}", v).unwrap(),
        Value::Date(year, month, day, hour, minutes, seconds, _micro_seconds) => 
            write!(
                s, "{:04}{:02}{:02} {:02}{:02}{:02}",
                year, month, day, hour, minutes, seconds
            ).unwrap(),
        Value::Time(negative, days, hours, minutes, seconds, micro_seconds) => 
            write!(
                s, "{}{:02}{:02} {:02}{:02}.{:03}",
                if *negative { "-" } else { "" }, days, hours, minutes, seconds, micro_seconds / 1000
            ).unwrap(),
    };
    s
}