use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Ne,
    StrictEq,
    StrictNe,

    Empty,
}

impl Operator {
    pub fn new<S: Into<String>>(s: S) -> Operator {
        let s = s.into();
        match s.as_str() {
            ">" => Operator::Gt,
            "<" => Operator::Lt,
            ">=" => Operator::Gte,
            "<=" => Operator::Lte,
            "=" => Operator::Eq,
            "==" => Operator::Eq,
            "!=" => Operator::Ne,
            "===" => Operator::StrictEq,
            "!==" => Operator::StrictNe,
            _ => Operator::Empty,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
            Operator::Eq => "",
            Operator::Ne => "!=",
            Operator::StrictEq => "===",
            Operator::StrictNe => "!==",
            Operator::Empty => "",
        };

        write!(f, "{}", s)
    }
}
