use std::collections::HashMap;

use datafusion::scalar::ScalarValue;

use crate::sql::database_variables::{DatabaseVariable, DatabaseVariables};

pub fn defaults() -> DatabaseVariables {
    let mut variables: DatabaseVariables = HashMap::new();

    variables.insert(
        "max_allowed_packet".to_string(),
        DatabaseVariable::new(
            "max_allowed_packet".to_string(),
            ScalarValue::UInt32(Some(67108864)),
            None,
        ),
    );
    variables.insert(
        "auto_increment_increment".to_string(),
        DatabaseVariable::new(
            "auto_increment_increment".to_string(),
            ScalarValue::UInt32(Some(1)),
            None,
        ),
    );
    variables.insert(
        "version_comment".to_string(),
        DatabaseVariable::new(
            "version_comment".to_string(),
            ScalarValue::Utf8(Some("mysql".to_string())),
            None,
        ),
    );
    variables.insert(
        "system_time_zone".to_string(),
        DatabaseVariable::new(
            "system_time_zone".to_string(),
            ScalarValue::Utf8(Some("UTC".to_string())),
            None,
        ),
    );
    variables.insert(
        "time_zone".to_string(),
        DatabaseVariable::new(
            "time_zone".to_string(),
            ScalarValue::Utf8(Some("SYSTEM".to_string())),
            None,
        ),
    );

    variables.insert(
        "tx_isolation".to_string(),
        DatabaseVariable::new(
            "tx_isolation".to_string(),
            ScalarValue::Utf8(Some("REPEATABLE-READ".to_string())),
            None,
        ),
    );
    variables.insert(
        "tx_read_only".to_string(),
        DatabaseVariable::new(
            "tx_read_only".to_string(),
            ScalarValue::Boolean(Some(false)),
            None,
        ),
    );
    variables.insert(
        "transaction_isolation".to_string(),
        DatabaseVariable::new(
            "transaction_isolation".to_string(),
            ScalarValue::Utf8(Some("REPEATABLE-READ".to_string())),
            None,
        ),
    );
    variables.insert(
        "transaction_read_only".to_string(),
        DatabaseVariable::new(
            "transaction_read_only".to_string(),
            ScalarValue::Boolean(Some(false)),
            None,
        ),
    );
    variables.insert(
        "sessiontransaction_isolation".to_string(),
        DatabaseVariable::new(
            "sessiontransaction_isolation".to_string(),
            ScalarValue::Utf8(Some("REPEATABLE-READ".to_string())),
            None,
        ),
    );
    variables.insert(
        "sessionauto_increment_increment".to_string(),
        DatabaseVariable::new(
            "sessionauto_increment_increment".to_string(),
            ScalarValue::Int64(Some(1)),
            None,
        ),
    );
    variables.insert(
        "character_set_client".to_string(),
        DatabaseVariable::new(
            "character_set_client".to_string(),
            ScalarValue::Utf8(Some("utf8mb4".to_string())),
            None,
        ),
    );
    variables.insert(
        "character_set_connection".to_string(),
        DatabaseVariable::new(
            "character_set_connection".to_string(),
            ScalarValue::Utf8(Some("utf8mb4".to_string())),
            None,
        ),
    );
    variables.insert(
        "character_set_results".to_string(),
        DatabaseVariable::new(
            "character_set_results".to_string(),
            ScalarValue::Utf8(Some("utf8mb4".to_string())),
            None,
        ),
    );
    variables.insert(
        "character_set_server".to_string(),
        DatabaseVariable::new(
            "character_set_server".to_string(),
            ScalarValue::Utf8(Some("utf8mb4".to_string())),
            None,
        ),
    );
    variables.insert(
        "collation_connection".to_string(),
        DatabaseVariable::new(
            "collation_connection".to_string(),
            ScalarValue::Utf8(Some("utf8mb4_general_ci".to_string())),
            None,
        ),
    );
    variables.insert(
        "collation_server".to_string(),
        DatabaseVariable::new(
            "collation_server".to_string(),
            ScalarValue::Utf8(Some("utf8mb4_0900_ai_ci".to_string())),
            None,
        ),
    );
    variables.insert(
        "init_connect".to_string(),
        DatabaseVariable::new(
            "init_connect".to_string(),
            ScalarValue::Utf8(Some("".to_string())),
            None,
        ),
    );
    variables.insert(
        "interactive_timeout".to_string(),
        DatabaseVariable::new(
            "interactive_timeout".to_string(),
            ScalarValue::Int32(Some(28800)),
            None,
        ),
    );
    variables.insert(
        "license".to_string(),
        DatabaseVariable::new(
            "license".to_string(),
            ScalarValue::Utf8(Some("Apache 2".to_string())),
            None,
        ),
    );
    variables.insert(
        "lower_case_table_names".to_string(),
        DatabaseVariable::new(
            "lower_case_table_names".to_string(),
            ScalarValue::Int32(Some(0)),
            None,
        ),
    );
    variables.insert(
        "net_buffer_length".to_string(),
        DatabaseVariable::new(
            "net_buffer_length".to_string(),
            ScalarValue::Int32(Some(16384)),
            None,
        ),
    );
    variables.insert(
        "net_write_timeout".to_string(),
        DatabaseVariable::new(
            "net_write_timeout".to_string(),
            ScalarValue::Int32(Some(600)),
            None,
        ),
    );
    variables.insert(
        "wait_timeout".to_string(),
        DatabaseVariable::new(
            "wait_timeout".to_string(),
            ScalarValue::Int32(Some(28800)),
            None,
        ),
    );
    variables.insert(
  "sql_mode".to_string(),
  DatabaseVariable::new(
      "sql_mode".to_string(),
      ScalarValue::Utf8(Some("ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION".to_string())),
      None,
  ),
);
    variables
}