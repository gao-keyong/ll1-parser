use std::fmt;
use regex::Regex;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Symbol{
    Nonterminal(String),
    Terminal(String),
}

type Term = Vec<Symbol>;
type Rule = HashMap<Symbol,Vec<Term>>;

pub struct Parser {
    first: HashMap<Symbol, Vec<Symbol>>,
    rules: Rule,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            first: HashMap::new(),
            rules: Rule::new(),
        }
    }

    pub fn add_rule(&mut self, lhs: &str, rhs:&str) {
        let lhs=Symbol::Nonterminal(lhs.to_string());

        let re=Regex::new(r"[A-Z]'?|(num|\(|\)|\+|\-|\*|/)").unwrap();
        let nt_re=Regex::new(r"[A-Z]'?").unwrap();
        let caps_iter=re.captures_iter(rhs);
        let mut term:Term = Term::new();
        for cap in caps_iter{
            if let Some(symbol)=cap.get(0){
                let symbol=symbol.as_str();
                if nt_re.is_match(symbol){
                    term.push(Symbol::Nonterminal(symbol.to_string()));
                }else{
                    term.push(Symbol::Terminal(symbol.to_string()));
                }
            }
        }
        let terms=self.rules.get(&lhs);
        if let Some(terms)=terms{
            let mut new_terms=terms.clone();
            new_terms.push(term);
            self.rules.insert(lhs, new_terms);
        }else{
            self.rules.insert(lhs, vec![term]);
        }
    }

    pub fn list_rules(&self){
        for (key, terms) in &self.rules{
            print!("{} -> ",key);
            for i in 0..terms.len(){
                if i!=0{
                    print!(" | ");
                }
                if terms[i].is_empty(){
                    print!("ε");
                }else {
                    for symbol in &terms[i]{
                        print!("{}",symbol);
                    }
                }
            }
            println!();
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Nonterminal(n) => write!(f, "{}", n),
            Symbol::Terminal(t) => write!(f, "{}", t),
        }
    }
}