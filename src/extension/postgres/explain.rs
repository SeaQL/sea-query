use crate::{ExplainFormat, SqlWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PgExplainSerialize {
    None,
    Text,
    Binary,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct PgExplainOptions {
    pub(crate) analyze: Option<bool>,
    pub(crate) verbose: Option<bool>,
    pub(crate) costs: Option<bool>,
    pub(crate) settings: Option<bool>,
    pub(crate) generic_plan: Option<bool>,
    pub(crate) buffers: Option<bool>,
    pub(crate) serialize: Option<PgExplainSerialize>,
    pub(crate) wal: Option<bool>,
    pub(crate) timing: Option<bool>,
    pub(crate) summary: Option<bool>,
    pub(crate) memory: Option<bool>,
    pub(crate) format: Option<ExplainFormat>,
}

impl PgExplainOptions {
    pub(crate) fn write_options(&self, sql: &mut impl SqlWriter) {
        fn write_sep(sql: &mut impl SqlWriter, first: &mut bool) {
            if !*first {
                sql.write_str(", ").unwrap();
            } else {
                *first = false;
            }
        }

        let has_options = self.analyze.is_some()
            || self.verbose.is_some()
            || self.costs.is_some()
            || self.settings.is_some()
            || self.generic_plan.is_some()
            || self.buffers.is_some()
            || self.serialize.is_some()
            || self.wal.is_some()
            || self.timing.is_some()
            || self.summary.is_some()
            || self.memory.is_some()
            || self.format.is_some();
        if !has_options {
            return;
        }

        sql.write_str(" (").unwrap();
        let mut first = true;

        if let Some(analyze) = self.analyze {
            write_sep(sql, &mut first);
            sql.write_str("ANALYZE").unwrap();
            if !analyze {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(verbose) = self.verbose {
            write_sep(sql, &mut first);
            sql.write_str("VERBOSE").unwrap();
            if !verbose {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(costs) = self.costs {
            write_sep(sql, &mut first);
            sql.write_str("COSTS").unwrap();
            if !costs {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(settings) = self.settings {
            write_sep(sql, &mut first);
            sql.write_str("SETTINGS").unwrap();
            if !settings {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(generic_plan) = self.generic_plan {
            write_sep(sql, &mut first);
            sql.write_str("GENERIC_PLAN").unwrap();
            if !generic_plan {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(buffers) = self.buffers {
            write_sep(sql, &mut first);
            sql.write_str("BUFFERS").unwrap();
            if !buffers {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(serialize) = self.serialize {
            write_sep(sql, &mut first);
            sql.write_str("SERIALIZE ").unwrap();
            sql.write_str(match serialize {
                PgExplainSerialize::None => "NONE",
                PgExplainSerialize::Text => "TEXT",
                PgExplainSerialize::Binary => "BINARY",
            })
            .unwrap();
        }

        if let Some(wal) = self.wal {
            write_sep(sql, &mut first);
            sql.write_str("WAL").unwrap();
            if !wal {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(timing) = self.timing {
            write_sep(sql, &mut first);
            sql.write_str("TIMING").unwrap();
            if !timing {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(summary) = self.summary {
            write_sep(sql, &mut first);
            sql.write_str("SUMMARY").unwrap();
            if !summary {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(memory) = self.memory {
            write_sep(sql, &mut first);
            sql.write_str("MEMORY").unwrap();
            if !memory {
                sql.write_str(" FALSE").unwrap();
            }
        }

        if let Some(format) = self.format {
            write_sep(sql, &mut first);
            sql.write_str("FORMAT ").unwrap();
            sql.write_str(format.as_str()).unwrap();
        }
        sql.write_str(")").unwrap();
    }
}
