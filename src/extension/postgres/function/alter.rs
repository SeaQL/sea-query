use super::create::FunctionBehavior;
use crate::{ColumnType, DynIden, IntoIden};

/// Represents PostgreSQL function alteration dependency option
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionDependsOption {
    DependsOn(DynIden),
    NoDependsOn(DynIden),
}

/// Represents PostgreSQL function alteration configuration options
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionConfigOption {
    Set(String),
    SetDefault,
    SetFromCurrent,
    Reset,
}

/// Represents PostgreSQL function alteration configuration parameter and option
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAlterConfig {
    pub(crate) param: DynIden,
    pub(crate) option: FunctionConfigOption,
}

/// Represents PostgreSQL function alteration option
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionAlterOption {
    Action(FunctionAlterAction),
    RenameTo(DynIden),
    OwnerTo(DynIden),
    SetSchema(DynIden),
    DependsOn(FunctionDependsOption),
}

/// Represents PostgreSQL function alteration action details
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionAlterAction {
    pub(crate) volatility: Option<FunctionBehavior>,
    pub(crate) strictness: Option<FunctionBehavior>,
    pub(crate) security: Option<FunctionBehavior>,
    pub(crate) parallel: Option<FunctionBehavior>,

    pub(crate) leakproof: Option<bool>,
    pub(crate) cost: Option<f64>,
    pub(crate) rows: Option<f64>,
    pub(crate) support: Option<DynIden>,
    pub(crate) configs: Vec<FunctionAlterConfig>,
    pub(crate) reset_all: bool,
    pub(crate) restrict: bool,
}

/// Creates a new "ALTER FUNCTION" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionAlterStatement {
    pub(crate) schema: Option<DynIden>,
    pub(crate) name: Option<DynIden>,
    pub(crate) if_exists: bool,
    pub(crate) arg_types: Option<Vec<ColumnType>>,
    pub(crate) alter_option: Option<FunctionAlterOption>,
}

impl FunctionAlterStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the function schema
    pub fn schema(&mut self, schema: impl IntoIden) -> &mut Self {
        self.schema = Some(schema.into_iden());
        self
    }

    /// Set the function name to alter
    pub fn name(&mut self, name: impl IntoIden) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Specify the argument types to uniquely identify the function overload to alter
    pub fn arg_types(&mut self, types: impl IntoIterator<Item = ColumnType>) -> &mut Self {
        self.arg_types = Some(types.into_iter().collect());
        self
    }

    /// Add a "RENAME TO" option to the ALTER FUNCTION statement
    pub fn rename_to(&mut self, new_name: impl IntoIden) -> &mut Self {
        self.alter_option = Some(FunctionAlterOption::RenameTo(new_name.into_iden()));
        self
    }

    /// Add an "OWNER TO" option to the ALTER FUNCTION statement
    pub fn owner_to(&mut self, new_owner: impl IntoIden) -> &mut Self {
        self.alter_option = Some(FunctionAlterOption::OwnerTo(new_owner.into_iden()));
        self
    }

    /// Add a "SET SCHEMA" option to the ALTER FUNCTION statement
    pub fn set_schema(&mut self, new_schema: impl IntoIden) -> &mut Self {
        self.alter_option = Some(FunctionAlterOption::SetSchema(new_schema.into_iden()));
        self
    }

    fn get_or_insert_action(&mut self) -> &mut FunctionAlterAction {
        if !matches!(self.alter_option, Some(FunctionAlterOption::Action(_))) {
            self.alter_option = Some(FunctionAlterOption::Action(FunctionAlterAction::default()));
        }
        match self.alter_option.as_mut().unwrap() {
            FunctionAlterOption::Action(action) => action,
            _ => unreachable!(),
        }
    }

    /// Add a behavior / volatility modifier
    pub fn behavior(&mut self, behavior: FunctionBehavior) -> &mut Self {
        let action = self.get_or_insert_action();
        match behavior {
            FunctionBehavior::Immutable | FunctionBehavior::Stable | FunctionBehavior::Volatile => {
                action.volatility = Some(behavior);
            }
            FunctionBehavior::CalledOnNullInput
            | FunctionBehavior::ReturnsNullOnNullInput
            | FunctionBehavior::Strict => {
                action.strictness = Some(behavior);
            }
            FunctionBehavior::SecurityInvoker | FunctionBehavior::SecurityDefiner => {
                action.security = Some(behavior);
            }
            FunctionBehavior::ParallelUnsafe
            | FunctionBehavior::ParallelRestricted
            | FunctionBehavior::ParallelSafe => {
                action.parallel = Some(behavior);
            }
        }
        self
    }

    /// Add a "LEAKPROOF" or "NOT LEAKPROOF" modifier
    pub fn leakproof(&mut self, leakproof: bool) -> &mut Self {
        self.get_or_insert_action().leakproof = Some(leakproof);
        self
    }

    /// Add a "COST" option
    pub fn cost(&mut self, cost: f64) -> &mut Self {
        self.get_or_insert_action().cost = Some(cost);
        self
    }

    /// Add a "ROWS" option
    pub fn rows(&mut self, rows: f64) -> &mut Self {
        self.get_or_insert_action().rows = Some(rows);
        self
    }

    /// Add a "SUPPORT" option
    pub fn support(&mut self, support_fn: impl IntoIden) -> &mut Self {
        self.get_or_insert_action().support = Some(support_fn.into_iden());
        self
    }

    /// Add a "DEPENDS ON EXTENSION" option
    pub fn depends_on_extension(&mut self, ext: impl IntoIden) -> &mut Self {
        self.alter_option = Some(FunctionAlterOption::DependsOn(
            FunctionDependsOption::DependsOn(ext.into_iden()),
        ));
        self
    }

    /// Add a "NO DEPENDS ON EXTENSION" option
    pub fn no_depends_on_extension(&mut self, ext: impl IntoIden) -> &mut Self {
        self.alter_option = Some(FunctionAlterOption::DependsOn(
            FunctionDependsOption::NoDependsOn(ext.into_iden()),
        ));
        self
    }

    fn set_config_option(&mut self, param: DynIden, option: FunctionConfigOption) {
        let action = self.get_or_insert_action();
        if let Some(config) = action
            .configs
            .iter_mut()
            .find(|c| c.param.to_string() == param.to_string())
        {
            config.option = option;
        } else {
            action.configs.push(FunctionAlterConfig { param, option });
        }
    }

    /// Add a "SET configuration_parameter TO value" option
    pub fn set_config(&mut self, param: impl IntoIden, value: impl Into<String>) -> &mut Self {
        self.set_config_option(param.into_iden(), FunctionConfigOption::Set(value.into()));
        self
    }

    /// Add a "SET configuration_parameter TO DEFAULT" option
    pub fn set_config_default(&mut self, param: impl IntoIden) -> &mut Self {
        self.set_config_option(param.into_iden(), FunctionConfigOption::SetDefault);
        self
    }

    /// Add a "SET configuration_parameter FROM CURRENT" option
    pub fn set_config_from_current(&mut self, param: impl IntoIden) -> &mut Self {
        self.set_config_option(param.into_iden(), FunctionConfigOption::SetFromCurrent);
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
    pub fn reset_config(&mut self, param: impl IntoIden) -> &mut Self {
        self.set_config_option(param.into_iden(), FunctionConfigOption::Reset);
        self
    }

    /// Add a "RESET ALL" option
    pub fn reset_all(&mut self) -> &mut Self {
        self.get_or_insert_action().reset_all = true;
        self
    }

    /// Add a "RESTRICT" option to the ALTER FUNCTION statement
    pub fn restrict(&mut self) -> &mut Self {
        self.get_or_insert_action().restrict = true;
        self
    }
}
