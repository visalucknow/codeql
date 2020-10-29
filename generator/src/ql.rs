use std::fmt;

pub struct Class {
    pub name: String,
    pub is_abstract: bool,
    pub supertypes: Vec<Type>,
    pub characteristic_predicate: Option<Expression>,
    pub predicates: Vec<Predicate>,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_abstract {
            write!(f, "abstract ")?;
        }
        write!(f, "class {} extends ", &self.name)?;
        for (index, supertype) in self.supertypes.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", supertype)?;
        }
        write!(f, " {{ \n")?;

        if let Some(charpred) = &self.characteristic_predicate {
            write!(
                f,
                "  {}\n",
                Predicate {
                    name: self.name.clone(),
                    overridden: false,
                    return_type: None,
                    formal_parameters: vec![],
                    body: charpred.clone(),
                }
            )?;
        }

        for predicate in &self.predicates {
            write!(f, "  {}\n", predicate)?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

// The QL type of a column.
#[derive(Clone)]
pub enum Type {
    /// Primitive `int` type.
    Int,

    /// Primitive `string` type.
    String,

    /// A user-defined type.
    Normal(String),

    /// A database type that will need to be referred to with an `@` prefix.
    AtType(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Normal(name) => write!(f, "{}", name),
            Type::AtType(name) => write!(f, "@{}", name),
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Var(String),
    String(String),
    Pred(String, Vec<Expression>),
    Or(Vec<Expression>),
    And(Vec<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    Exists(Vec<FormalParameter>, Box<Expression>),
    Dot(Box<Expression>, String, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Var(x) => write!(f, "{}", x),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Pred(n, args) => {
                write!(f, "{}(", n)?;
                for (index, arg) in args.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Or(disjuncts) => {
                if disjuncts.is_empty() {
                    write!(f, "none()")
                } else {
                    for (index, disjunct) in disjuncts.iter().enumerate() {
                        if index > 0 {
                            write!(f, " or ")?;
                        }
                        write!(f, "{}", disjunct)?;
                    }
                    Ok(())
                }
            }
            Expression::And(conjuncts) => {
                if conjuncts.is_empty() {
                    write!(f, "any()")
                } else {
                    for (index, disjunct) in conjuncts.iter().enumerate() {
                        if index > 0 {
                            write!(f, " and ")?;
                        }
                        write!(f, "{}", disjunct)?;
                    }
                    Ok(())
                }
            }
            Expression::Equals(a, b) => write!(f, "{} = {}", a, b),
            Expression::Exists(params, formula) => {
                write!(f, "exists(")?;
                for (index, param) in params.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, " | {})", formula)
            }
            Expression::Dot(x, member_pred, args) => {
                write!(f, "{}.{}(", x, member_pred)?;
                for (index, arg) in args.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Clone)]
pub struct Predicate {
    pub name: String,
    pub overridden: bool,
    pub return_type: Option<Type>,
    pub formal_parameters: Vec<FormalParameter>,
    pub body: Expression,
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.overridden {
            write!(f, "override ")?;
        }
        match &self.return_type {
            None => write!(f, "predicate ")?,
            Some(return_type) => write!(f, "{} ", return_type)?,
        }
        write!(f, "{}(", self.name)?;
        for (index, param) in self.formal_parameters.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ") {{ {} }}", self.body)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct FormalParameter {
    pub name: String,
    pub param_type: Type,
}

impl fmt::Display for FormalParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.param_type, self.name)
    }
}

/// Generates a QL library by writing the given `classes` to the `file`.
pub fn write(
    language_name: &str,
    file: &mut dyn std::io::Write,
    classes: &[Class],
) -> std::io::Result<()> {
    write!(file, "/*\n")?;
    write!(file, " * CodeQL library for {}\n", language_name)?;
    write!(
        file,
        " * Automatically generated from the tree-sitter grammar; do not edit\n"
    )?;
    write!(file, " */\n\n")?;

    for class in classes {
        write!(file, "{}\n\n", &class)?;
    }

    Ok(())
}
