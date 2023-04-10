#![allow(dead_code)]
#![allow(unused)]

use core::any::TypeId;
// Declare the QueryType enum to represent different types of SQL queries
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Truncate,
}

pub trait FormatSqlValue {
    fn format_sql_value(&self) -> String;
}

impl FormatSqlValue for String {
    fn format_sql_value(&self) -> String {
        format!("'{}'", self)
    }
}

impl FormatSqlValue for i64 {
    fn format_sql_value(&self) -> String {
        self.to_string()
    }
}

pub struct SqlBuilder {
    query_type: QueryType,       // The type of SQL query (SELECT, INSERT, etc.)
    table: String,               // The table on which the query will be executed
    select_columns: Vec<String>, // The columns to be selected in a SELECT query
    insert_columns: Vec<String>,
    insert_values: Vec<String>,
    update_columns: Vec<String>,
    update_values: Vec<String>,
    where_conditions: Vec<String>,
    order_by_column: Option<String>,
}

impl SqlBuilder {
    // Constructor to create a new SqlBuilder instance with an empty query
    pub fn new() -> Self {
        // Initialize the SqlBuilder with default values
        SqlBuilder {
            query_type: QueryType::Select,
            table: String::new(),
            select_columns: Vec::new(),
            insert_columns: Vec::new(),
            insert_values: Vec::new(),
            update_columns: Vec::new(),
            update_values: Vec::new(),
            where_conditions: Vec::new(),
            order_by_column: None,
        }
    }

    // Create a SELECT query targeting the specified table
    pub fn select_from(mut self, table: &str) -> Self {
        self.query_type = QueryType::Select;
        self.table = table.to_string();
        self
    }

    // Specify the columns to be selected in a SELECT query
    pub fn select_columns(mut self, columns: &[&str]) -> Self {
        self.select_columns = columns.iter().map(|col| col.to_string()).collect();
        self
    }

    // Appends "INSERT INTO" followed by the table name to the query
    pub fn insert_into(mut self, table: &str) -> Self {
        self.query_type = QueryType::Insert;
        self.table = table.to_string();
        self
    }

    // Appends the list of columns to the query, surrounded by parentheses
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.insert_columns = columns.iter().map(|col| col.to_string()).collect();
        self
    }

    // Appends list of values to the query, surrounded by parentheses
    pub fn values<F: FormatSqlValue>(mut self, values: &[&F]) -> Self {
        self.insert_values = values.iter().map(|v| v.format_sql_value()).collect();
        self
    }

    pub fn update(mut self, table: &str) -> Self {
        self.query_type = QueryType::Update;
        self.table = table.to_string();
        self
    }

    pub fn set_columns_and_values<F: FormatSqlValue>(mut self, columns: &[&str], values: &[&F]) -> Self {
        self.update_columns = columns.iter().map(|col| col.to_string()).collect();
        self.update_values = values.iter().map(|val| val.format_sql_value()).collect();
        self
    }

    pub fn delete_from(mut self, table: &str) -> Self {
        self.query_type = QueryType::Delete;
        self.table = table.to_string();
        self
    }

    pub fn truncate(mut self, table: &str) -> Self {
        self.query_type = QueryType::Truncate;
        self.table = table.to_string();
        self
    }

    pub fn where_clause<F: FormatSqlValue>(mut self, clause: &str, value: F) -> Self {
        let formatted_value = value.format_sql_value();
        let formatted_clause = clause.replace("{}", &formatted_value);
        self.where_conditions.push(format!(" WHERE {}", formatted_clause));
        self
    }

    pub fn and(mut self, condition: &str) -> Self {
        self.where_conditions.push(format!("AND {}", condition));
        self
    }

    pub fn order_by(mut self, column: &str) -> Self {
        self.order_by_column = Some(column.to_string());
        self
    }

    pub fn build(&self) -> String {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
            QueryType::Truncate => self.build_truncate(),
        }
    }

    fn build_select(&self) -> String {
        let columns = if self.select_columns.is_empty() {
            "*".to_string()
        } else {
            self.select_columns.join(", ")
        };

        let where_clause = if self.where_conditions.is_empty() {
            String::new()
        } else {
            self.where_conditions.join(" ")
        };
        let mut query = format!("SELECT {} FROM {}{}", columns, self.table, where_clause);
        if let Some(ref order_by_column) = self.order_by_column {
            query.push_str(&format!(" ORDER BY {}", order_by_column));
        }
        query
    }

    fn build_update(&self) -> String {
        let sets: Vec<String> = self
            .update_columns
            .iter()
            .zip(self.update_values.iter())
            .map(|(col, val)| format!("{} = {}", col, val))
            .collect();
        let set_clause = sets.join(", ");

        let where_clause = if self.where_conditions.is_empty() {
            String::new()
        } else {
            self.where_conditions.join(" ")
        };
        format!("UPDATE {} SET {} {} RETURNING *", self.table, set_clause, where_clause)
    }

    fn build_insert(&self) -> String {
        let columns = self.insert_columns.join(", ");
        let values = self.insert_values.join(", ");
        format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.table, columns, values
        )
    }

    fn build_delete(&self) -> String {
        let where_clause = if self.where_conditions.is_empty() {
            String::new()
        } else {
            self.where_conditions.join(" ")
        };
        format!("DELETE FROM {} {} RETURNING *", self.table, where_clause)
    }

    fn build_truncate(&self) -> String {
        format!("TRUNCATE {};", self.table)
    }
}
