use crate::*;

pub trait ViewBuilder: QueryBuilder + QuotedBuilder {
    /// Translate [`ViewCreateStatement`] into SQL statement.
    fn prepare_view_create_statement(&self, create: &ViewCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE").unwrap();
        self.prepare_view_create_befor_view(create, sql);

        write!(sql, " VIEW ").unwrap();
        self.prepare_view_create_after_view(create, sql);

        if let Some(view_ref) = &create.view {
            self.prepare_view_ref(view_ref, sql);
        }

        if !create.columns.is_empty() {
            write!(sql, " ( ").unwrap();
            create.columns.iter().fold(true, |first, column| {
                if !first {
                    write!(sql, ", ").unwrap();
                }
                column.prepare(sql, self.quote());
                false
            });
            write!(sql, " ) ").unwrap();
        }

        write!(sql, " AS ").unwrap();
        // self.prepare_select_statement(create.query, sql);
        if let Some(opt) = &create.opt {
            self.prepare_view_create_opt_update(opt, sql);
        }
    }

    fn prepare_view_create_befor_view(&self, _create: &ViewCreateStatement, _sql: &mut SqlWriter) {}

    fn prepare_view_create_after_view(&self, _create: &ViewCreateStatement, _sql: &mut SqlWriter) {}

    fn prepare_view_create_opt_update(&self, opt: &ViewCreateOpt, sql: &mut SqlWriter) {
        write!(sql, " WITH ").unwrap();
        match opt {
            ViewCreateOpt::Cascade => write!(sql, "CASCADED").unwrap(),
            ViewCreateOpt::Local => write!(sql, "LOCAL").unwrap(),
        }
        write!(sql, " CHECK OPTION").unwrap();
    }

    fn prepare_view_rename_statement(&self, _rename: &ViewRenameStatement, _sql: &mut SqlWriter);

    /// Translate [`ViewDropStatement`] into SQL statement.
    fn prepare_view_drop_statement(&self, drop: &ViewDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP VIEW ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        drop.views.iter().fold(true, |first, view| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            self.prepare_view_ref(view, sql);
            false
        });

        let mut view_drop_opt = String::new();
        for drop_opt in drop.options.iter() {
            write!(&mut view_drop_opt, " ").unwrap();
            self.prepare_view_drop_opt(drop_opt, &mut view_drop_opt);
        }
        write!(sql, "{}", view_drop_opt.trim_end()).unwrap();
    }

    /// Translate [`ViewDropOpt`] into SQL statement.
    fn prepare_view_drop_opt(&self, drop_opt: &ViewDropOpt, sql: &mut dyn std::fmt::Write) {
        write!(
            sql,
            "{}",
            match drop_opt {
                ViewDropOpt::Restrict => "RESTRICT",
                ViewDropOpt::Cascade => "CASCADE",
            }
        )
        .unwrap();
    }

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_view_ref(&self, table_ref: &TableRef, sql: &mut SqlWriter) {
        match table_ref {
            TableRef::Table(table) => {
                table.prepare(sql, self.quote());
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
            _ => panic!("Not supported"),
        }
    }
}
