use crate::{ColumnType, DynIden, IntoIden};
use super::create::FunctionBehavior;

/// Represents PostgreSQL function alteration options
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionAlterOption {
    RenameTo(DynIden),
    OwnerTo(DynIden),
    SetSchema(DynIden),
    Behavior(FunctionBehavior),
    Leakproof(bool),
    Cost(f64),
    Rows(f64),
    Support(DynIden),
    DependsOnExtension(DynIden),
    NoDependsOnExtension(DynIden),
    SetConfig(DynIden, String),
    SetConfigDefault(DynIden),
    SetConfigFromCurrent(DynIden),
    ResetConfig(DynIden),
    ResetAll,
}

/// Creates a new "ALTER FUNCTION" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionAlterStatement {
    pub(crate) name: Option<DynIden>,
    pub(crate) if_exists: bool,
    pub(crate) arg_types: Option<Vec<ColumnType>>,
    pub(crate) options: Vec<FunctionAlterOption>,
    pub(crate) restrict: bool,
}

impl FunctionAlterStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the function name to alter
    pub fn name<T: IntoIden>(&mut self, name: T) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Specify the argument types to uniquely identify the function overload to alter
    pub fn arg_types<I: IntoIterator<Item = ColumnType>>(&mut self, types: I) -> &mut Self {
        self.arg_types = Some(types.into_iter().collect());
        self
    }

    /// Add a "RENAME TO" option to the ALTER FUNCTION statement
    pub fn rename_to<T: IntoIden>(&mut self, new_name: T) -> &mut Self {
        self.options
            .push(FunctionAlterOption::RenameTo(new_name.into_iden()));
        self
    }

    /// Add an "OWNER TO" option to the ALTER FUNCTION statement
    pub fn owner_to<T: IntoIden>(&mut self, new_owner: T) -> &mut Self {
        self.options
            .push(FunctionAlterOption::OwnerTo(new_owner.into_iden()));
        self
    }

    /// Add a "SET SCHEMA" option to the ALTER FUNCTION statement
    pub fn set_schema<T: IntoIden>(&mut self, new_schema: T) -> &mut Self {
        self.options
            .push(FunctionAlterOption::SetSchema(new_schema.into_iden()));
        self
    }

    /// Add a behavior / volatility modifier
    pub fn behavior(&mut self, behavior: FunctionBehavior) -> &mut Self {
        self.options.push(FunctionAlterOption::Behavior(behavior));
        self
    }

    /// Add a "LEAKPROOF" or "NOT LEAKPROOF" modifier
    pub fn leakproof(&mut self, leakproof: bool) -> &mut Self {
        self.options.push(FunctionAlterOption::Leakproof(leakproof));
        self
    }

    /// Add a "COST" option
    pub fn cost(&mut self, cost: f64) -> &mut Self {
        self.options.push(FunctionAlterOption::Cost(cost));
        self
    }

    /// Add a "ROWS" option
    pub fn rows(&mut self, rows: f64) -> &mut Self {
        self.options.push(FunctionAlterOption::Rows(rows));
        self
    }

    /// Add a "SUPPORT" option
    pub fn support<T: IntoIden>(&mut self, support_fn: T) -> &mut Self {
        self.options
            .push(FunctionAlterOption::Support(support_fn.into_iden()));
        self
    }

    /// Add a "DEPENDS ON EXTENSION" option
    pub fn depends_on_extension<T: IntoIden>(&mut self, ext: T) -> &mut Self {
        self.options
            .push(FunctionAlterOption::DependsOnExtension(ext.into_iden()));
        self
    }

    /// Add a "NO DEPENDS ON EXTENSION" option
    pub fn no_depends_on_extension<T: IntoIden>(&mut self, ext: T) -> &mut Self {
        self.options
            .push(FunctionAlterOption::NoDependsOnExtension(ext.into_iden()));
        self
    }

    /// Add a "SET configuration_parameter TO value" option
    pub fn set_config<K: IntoIden, V: Into<String>>(&mut self, param: K, value: V) -> &mut Self {
        self.options
            .push(FunctionAlterOption::SetConfig(param.into_iden(), value.into()));
        self
    }

    /// Add a "SET configuration_parameter TO DEFAULT" option
    pub fn set_config_default<K: IntoIden>(&mut self, param: K) -> &mut Self {
        self.options
            .push(FunctionAlterOption::SetConfigDefault(param.into_iden()));
        self
    }

    /// Add a "SET configuration_parameter FROM CURRENT" option
    pub fn set_config_from_current<K: IntoIden>(&mut self, param: K) -> &mut Self {
        self.options
            .push(FunctionAlterOption::SetConfigFromCurrent(param.into_iden()));
        self
    }

    /// Add an "IMMUTABLE" behavior
    pub fn immutable(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::Immutable)
    }

    /// Add a "STABLE" behavior
    pub fn stable(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::Stable)
    }

    /// Add a "VOLATILE" behavior
    pub fn volatile(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::Volatile)
    }

    /// Add a "CALLED ON NULL INPUT" behavior
    pub fn called_on_null_input(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::CalledOnNullInput)
    }

    /// Add a "RETURNS NULL ON NULL INPUT" behavior
    pub fn returns_null_on_null_input(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::ReturnsNullOnNullInput)
    }

    /// Add a "STRICT" behavior
    pub fn strict(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::Strict)
    }

    /// Add a "SECURITY INVOKER" behavior
    pub fn security_invoker(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::SecurityInvoker)
    }

    /// Add a "SECURITY DEFINER" behavior
    pub fn security_definer(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::SecurityDefiner)
    }

    /// Add a "PARALLEL UNSAFE" behavior
    pub fn parallel_unsafe(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::ParallelUnsafe)
    }

    /// Add a "PARALLEL RESTRICTED" behavior
    pub fn parallel_restricted(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::ParallelRestricted)
    }

    /// Add a "PARALLEL SAFE" behavior
    pub fn parallel_safe(&mut self) -> &mut Self {
        self.behavior(FunctionBehavior::ParallelSafe)
    }

    /// Add a "RESET configuration_parameter" option
    pub fn reset_config<K: IntoIden>(&mut self, param: K) -> &mut Self {
        self.options
            .push(FunctionAlterOption::ResetConfig(param.into_iden()));
        self
    }

    /// Add a "RESET ALL" option
    pub fn reset_all(&mut self) -> &mut Self {
        self.options.push(FunctionAlterOption::ResetAll);
        self
    }

    /// Add a "RESTRICT" option to the ALTER FUNCTION statement
    pub fn restrict(&mut self) -> &mut Self {
        self.restrict = true;
        self
    }
}
