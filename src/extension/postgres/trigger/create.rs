use crate::{DynIden, Expr, IntoIden, IntoTableRef, TableRef};

/// Represents PostgreSQL trigger execution timing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerTime {
    Before,
    After,
    InsteadOf,
}

/// Represents PostgreSQL trigger events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerEvent {
    Insert,
    Update(Vec<DynIden>),
    Delete,
    Truncate,
}

/// Represents PostgreSQL trigger REFERENCING transition relations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerReferencing {
    OldTable(DynIden),
    NewTable(DynIden),
}

/// Represents PostgreSQL trigger FOR EACH (ROW | STATEMENT)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerEach {
    Row,
    Statement,
}

/// Represents PostgreSQL trigger DEFERRABLE options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerInitially {
    Immediate,
    Deferred,
}

/// Represents PostgreSQL trigger execution type (FUNCTION or PROCEDURE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerExecutionType {
    Function,
    Procedure,
}

/// Creates a new "CREATE TRIGGER" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TriggerCreateStatement {
    pub(crate) name: Option<DynIden>,
    pub(crate) or_replace: bool,
    pub(crate) is_constraint: bool,
    pub(crate) table: Option<TableRef>,
    pub(crate) referenced_table: Option<TableRef>,
    pub(crate) deferrable: Option<bool>,
    pub(crate) initially: Option<TriggerInitially>,
    pub(crate) time: Option<TriggerTime>,
    pub(crate) events: Vec<TriggerEvent>,
    pub(crate) referencing: Vec<TriggerReferencing>,
    pub(crate) each: Option<TriggerEach>,
    pub(crate) r#when: Option<Expr>,
    pub(crate) function: Option<DynIden>,
    pub(crate) function_args: Vec<Expr>,
    pub(crate) execution_type: Option<TriggerExecutionType>,
}

impl TriggerCreateStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the trigger name
    pub fn name(&mut self, name: impl IntoIden) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Use "OR REPLACE" in the CREATE TRIGGER statement
    pub fn or_replace(&mut self) -> &mut Self {
        self.or_replace = true;
        self
    }

    /// Use "CONSTRAINT" in the CREATE TRIGGER statement
    pub fn constraint(&mut self) -> &mut Self {
        self.is_constraint = true;
        self
    }

    /// Set the target table
    pub fn table(&mut self, table: impl IntoTableRef) -> &mut Self {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Set the referenced table (FROM referenced_table_name)
    pub fn from_table(&mut self, table: impl IntoTableRef) -> &mut Self {
        self.referenced_table = Some(table.into_table_ref());
        self
    }

    /// Set whether the constraint trigger is DEFERRABLE
    pub fn deferrable(&mut self, deferrable: bool) -> &mut Self {
        self.deferrable = Some(deferrable);
        self
    }

    /// Set the trigger check timing (INITIALLY IMMEDIATE or INITIALLY DEFERRED)
    pub fn initially(&mut self, initially: TriggerInitially) -> &mut Self {
        self.initially = Some(initially);
        self
    }

    /// Set the trigger check timing to INITIALLY IMMEDIATE
    pub fn initially_immediate(&mut self) -> &mut Self {
        self.initially(TriggerInitially::Immediate)
    }

    /// Set the trigger check timing to INITIALLY DEFERRED
    pub fn initially_deferred(&mut self) -> &mut Self {
        self.initially(TriggerInitially::Deferred)
    }

    /// Set trigger timing (BEFORE, AFTER, INSTEAD OF)
    pub fn time(&mut self, time: TriggerTime) -> &mut Self {
        self.time = Some(time);
        self
    }

    /// Set trigger timing to BEFORE
    pub fn before(&mut self) -> &mut Self {
        self.time(TriggerTime::Before)
    }

    /// Set trigger timing to AFTER
    pub fn after(&mut self) -> &mut Self {
        self.time(TriggerTime::After)
    }

    /// Set trigger timing to INSTEAD OF
    pub fn instead_of(&mut self) -> &mut Self {
        self.time(TriggerTime::InsteadOf)
    }

    /// Add a trigger event
    pub fn event(&mut self, event: TriggerEvent) -> &mut Self {
        self.events.push(event);
        self
    }

    /// Add multiple trigger events
    pub fn events(&mut self, events: impl IntoIterator<Item = TriggerEvent>) -> &mut Self {
        self.events.extend(events);
        self
    }

    /// Add transition relation referencing
    pub fn referencing(&mut self, referencing: TriggerReferencing) -> &mut Self {
        self.referencing.push(referencing);
        self
    }

    /// Referencing OLD TABLE AS name
    pub fn referencing_old_table(&mut self, name: impl IntoIden) -> &mut Self {
        self.referencing(TriggerReferencing::OldTable(name.into_iden()))
    }

    /// Referencing NEW TABLE AS name
    pub fn referencing_new_table(&mut self, name: impl IntoIden) -> &mut Self {
        self.referencing(TriggerReferencing::NewTable(name.into_iden()))
    }

    /// Set trigger iteration frequency (FOR EACH ROW or FOR EACH STATEMENT)
    pub fn each(&mut self, each: TriggerEach) -> &mut Self {
        self.each = Some(each);
        self
    }

    /// Set trigger frequency to FOR EACH ROW
    pub fn for_each_row(&mut self) -> &mut Self {
        self.each(TriggerEach::Row)
    }

    /// Set trigger frequency to FOR EACH STATEMENT
    pub fn for_each_statement(&mut self) -> &mut Self {
        self.each(TriggerEach::Statement)
    }

    /// Set WHEN condition
    pub fn r#when(&mut self, condition: impl Into<Expr>) -> &mut Self {
        self.r#when = Some(condition.into());
        self
    }

    /// Set trigger execution function
    pub fn function(&mut self, name: impl IntoIden) -> &mut Self {
        self.function = Some(name.into_iden());
        self.execution_type = Some(TriggerExecutionType::Function);
        self
    }

    /// Set trigger execution procedure
    pub fn procedure(&mut self, name: impl IntoIden) -> &mut Self {
        self.function = Some(name.into_iden());
        self.execution_type = Some(TriggerExecutionType::Procedure);
        self
    }

    /// Add a trigger execution argument
    pub fn function_arg(&mut self, arg: impl Into<Expr>) -> &mut Self {
        self.function_args.push(arg.into());
        self
    }

    /// Add multiple trigger execution arguments
    pub fn function_args(&mut self, args: impl IntoIterator<Item = Expr>) -> &mut Self {
        self.function_args.extend(args);
        self
    }
}
