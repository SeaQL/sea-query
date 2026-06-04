use crate::{ColumnType, DynIden, Expr, IntoIden};

/// Represents PostgreSQL function argument modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionArgMode {
    In,
    Out,
    InOut,
    Variadic,
}

/// Represents a PostgreSQL function argument
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg {
    pub(crate) mode: Option<FunctionArgMode>,
    pub(crate) name: Option<DynIden>,
    pub(crate) arg_type: ColumnType,
    pub(crate) default: Option<Expr>,
}

impl FunctionArg {
    /// Create a new function argument with a type
    pub fn new(arg_type: ColumnType) -> Self {
        Self {
            mode: None,
            name: None,
            arg_type,
            default: None,
        }
    }

    /// Set the argument mode (IN, OUT, INOUT, VARIADIC)
    pub fn mode(mut self, mode: FunctionArgMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set the name of the argument
    pub fn name<T: IntoIden>(mut self, name: T) -> Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Set the default expression for the argument
    pub fn default<T: Into<Expr>>(mut self, expr: T) -> Self {
        self.default = Some(expr.into());
        self
    }
}

/// Represents PostgreSQL function behavior/volatility options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionBehavior {
    Immutable,
    Stable,
    Volatile,
    CalledOnNullInput,
    ReturnsNullOnNullInput,
    Strict,
    SecurityInvoker,
    SecurityDefiner,
    ParallelUnsafe,
    ParallelRestricted,
    ParallelSafe,
}

/// Represents the return type of a PostgreSQL function
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionReturns {
    Type(ColumnType),
    Table(Vec<(DynIden, ColumnType)>),
}

/// Creates a new "CREATE FUNCTION" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionCreateStatement {
    pub(crate) name: Option<DynIden>,
    pub(crate) or_replace: bool,
    pub(crate) args: Vec<FunctionArg>,
    pub(crate) returns: Option<FunctionReturns>,
    pub(crate) language: Option<DynIden>,
    pub(crate) behavior: Vec<FunctionBehavior>,
    pub(crate) as_definition: Option<String>,
    pub(crate) sql_body: Option<String>,
}

impl FunctionCreateStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the function name
    pub fn name<T: IntoIden>(&mut self, name: T) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Use "OR REPLACE" in the CREATE FUNCTION statement
    pub fn or_replace(&mut self) -> &mut Self {
        self.or_replace = true;
        self
    }

    /// Add an argument to the function
    pub fn arg(&mut self, arg: FunctionArg) -> &mut Self {
        self.args.push(arg);
        self
    }

    /// Add multiple arguments to the function
    pub fn args<I: IntoIterator<Item = FunctionArg>>(&mut self, args: I) -> &mut Self {
        self.args.extend(args);
        self
    }

    /// Set the return type
    pub fn returns(&mut self, returns: FunctionReturns) -> &mut Self {
        self.returns = Some(returns);
        self
    }

    /// Set the function language (e.g., PL/pgSQL, SQL)
    pub fn language<T: IntoIden>(&mut self, lang: T) -> &mut Self {
        self.language = Some(lang.into_iden());
        self
    }

    /// Add a behavior modifier (e.g., IMMUTABLE, STRICT, SECURITY DEFINER)
    pub fn behavior(&mut self, behavior: FunctionBehavior) -> &mut Self {
        self.behavior.push(behavior);
        self
    }

    /// Set function definition string (AS '...')
    pub fn as_definition<T: Into<String>>(&mut self, definition: T) -> &mut Self {
        self.as_definition = Some(definition.into());
        self
    }

    /// Set SQL function body
    pub fn sql_body<T: Into<String>>(&mut self, body: T) -> &mut Self {
        self.sql_body = Some(body.into());
        self
    }
}
