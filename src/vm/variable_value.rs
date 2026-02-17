use ordered_float::OrderedFloat;

pub enum VariableValue {
    // Value is location in string pool
    StringLiteral(usize),

    Number(i32),
    Float(OrderedFloat<f64>),
    Null,
    Boolean(bool)
}
