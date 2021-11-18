use crate::rule::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Parser {
    first: HashMap<Symbol, Vec<Symbol>>,
    rules: Vec<Rule>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            first: HashMap::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, lhs: &str, rhs:&str) {
        let lhs=Symbol::Nonterminal(lhs.to_string());

        let re=Regex::new(r"[A-Z]'?|(num|\(|\)|\+|\-|\*|/)").unwrap();
        let nt_re=Regex::new(r"[A-Z]'?").unwrap();
        let t_re=Regex::new(r"num|\(|\)|\+|\-|\*|/").unwrap();
        let mut caps_iter=re.captures_iter(rhs);
        let mut rhs:Vec<Symbol> = Vec::new();
        for cap in caps_iter{
            if let Some(symbol)=cap.get(0){
                let symbol=symbol.as_str();
                if nt_re.is_match(symbol){
                    rhs.push(Symbol::Nonterminal(symbol.to_string()));
                }else{
                    rhs.push(Symbol::Terminal(symbol.to_string()));
                }
            }
        }
        self.rules.push(Rule::new(lhs, rhs));
    }

    pub fn list_rules(&self){
        for rule in &self.rules{
            println!("{}",rule);
        }
    }

    pub fn parse(&mut self, rules:Vec<Rule>) {
        self.rules = rules;
    }
}