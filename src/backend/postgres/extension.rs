// impl ExtensionBuilder for ExtensionStatement {
//     fn prepare_extension_statement(&self, sql: &mut dyn SqlWriter) {
//         match self.operation {
//             ExtensionOperation::Create => {
//                 write!(sql, "CREATE EXTENSION ").unwrap();

//                 if self.if_not_exists {
//                     write!(sql, "IF NOT EXISTS ").unwrap()
//                 }

//                 write!(sql, "{}", self.name).unwrap();

//                 if let Some(schema) = self.schema.as_ref() {
//                     write!(sql, " WITH SCHEMA {}", schema).unwrap();
//                 }

//                 if let Some(version) = self.version.as_ref() {
//                     write!(sql, " VERSION {}", version).unwrap();
//                 }

//                 if self.cascade {
//                     write!(sql, " CASCADE").unwrap();
//                 }
//             }
//             ExtensionOperation::Drop => {
//                 write!(sql, "DROP EXTENSION ").unwrap();

//                 if self.if_exists {
//                     write!(sql, "IF EXISTS ").unwrap();
//                 }

//                 write!(sql, "{}", self.name).unwrap();

//                 if self.cascade {
//                     write!(sql, " CASCADE").unwrap();
//                 }

//                 if self.restrict {
//                     write!(sql, "  RESTRICT").unwrap();
//                 }
//             }
//         }
//     }
// }
