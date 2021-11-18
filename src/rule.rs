use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]

pub enum Symbol{
    Nonterminal(String),
    Terminal(String),
}

pub struct Rule{
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

impl Rule{
    pub fn new(lhs: Symbol, rhs: Vec<Symbol>) -> Rule{
        Rule{
            lhs: lhs,
            rhs: rhs,
        }
    }
}

impl fmt::Display for Symbol{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Symbol::Nonterminal(n) => write!(f, "{}", n),
            Symbol::Terminal(t) => write!(f, "{}", t),
        }
    }
}

impl fmt::Display for Rule{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f, "{} -> ", self.lhs);
        Ok(for term in &self.rhs{
            write!(f, "{} ",&term);
        })
    }
}