// src/parser/ast.rs

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int,
    Text,
    Float,
    Boolean,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<ColumnDef>,
    },
    DropTable {
        name: String,
    },
    Insert {
        table: String,
        values: Vec<String>,
    },
    Select {
        table: String,
        columns: Vec<SelectColumn>,  // String -> SelectColumn으로 변경
        condition: Option<Condition>,
        join: Option<Join>,
        order_by: Option<OrderBy>,
        group_by: Option<Vec<String>>,
        limit: Option<usize>,
    },
    Update {
        table: String,
        assignments: Vec<(String, String)>,
        condition: Option<Condition>,
    },
    Delete {
        table: String,
        condition: Option<Condition>,
    },
    Begin,
    Commit,
    Rollback,
    AlterTable {
        table: String,
        action: AlterAction,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum AggFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SelectColumn {
    All,                              // *
    Column(String),                   // col
    Agg { func: AggFunc, col: String }, // COUNT(col), SUM(col) 등
}

#[derive(Debug, PartialEq, Clone)]
pub struct OrderBy {
    pub column: String,
    pub ascending: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AlterAction {
    AddColumn(ColumnDef),
    DropColumn(String),
    RenameColumn { from: String, to: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Eq,   // =
    Ne,   // !=
    Gt,   // >
    Lt,   // 
    Gte,  // >=
    Lte,  // <=
}

#[derive(Debug, PartialEq, Clone)]
pub struct Join {
    pub table: String,
    pub left_col: String,
    pub right_col: String,
}