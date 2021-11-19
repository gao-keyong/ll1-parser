use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Symbol {
    Nonterminal(String),
    Terminal(String),
}

type Term = Vec<Symbol>;
type Rule = HashMap<Symbol, Vec<Term>>;

pub struct Parser {
    first: HashMap<Symbol, HashSet<Symbol>>,
    follow: HashMap<Symbol, HashSet<Symbol>>,
    rules: Rule,
    start_symbol: Symbol,
    table: HashMap<(Symbol, Symbol), (Symbol, Term)>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            first: HashMap::new(),
            follow: HashMap::new(),
            rules: Rule::new(),
            start_symbol: Symbol::Nonterminal("".to_string()),
            table: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, lhs: &str, rhs: &str) {
        let lhs = Symbol::Nonterminal(lhs.to_string());

        let re = Regex::new(r"[A-Z]'?|(num|\(|\)|\+|\-|\*|/)").unwrap();
        let nt_re = Regex::new(r"[A-Z]'?").unwrap();
        let caps_iter = re.captures_iter(rhs);
        let mut term: Term = Term::new();
        for cap in caps_iter {
            if let Some(symbol) = cap.get(0) {
                let symbol = symbol.as_str();
                if nt_re.is_match(symbol) {
                    term.push(Symbol::Nonterminal(symbol.to_string()));
                } else {
                    term.push(Symbol::Terminal(symbol.to_string()));
                }
            }
        }
        let terms = self.rules.get(&lhs);
        if let Some(terms) = terms {
            let mut new_terms = terms.clone();
            new_terms.push(term);
            self.rules.insert(lhs, new_terms);
        } else {
            self.rules.insert(lhs, vec![term]);
        }
    }

    pub fn set_start_symbol(&mut self, start_symbol: &str) {
        self.start_symbol = Symbol::Nonterminal(start_symbol.to_string());
    }

    pub fn list_rules(&self) {
        for (key, terms) in &self.rules {
            print!("{} -> ", key);
            for i in 0..terms.len() {
                if i != 0 {
                    print!(" | ");
                }
                if terms[i].is_empty() {
                    print!("ε");
                } else {
                    for symbol in &terms[i] {
                        print!("{}", symbol);
                    }
                }
            }
            println!();
        }
    }

    fn getFirst(&mut self) {
        loop {
            let prev_first = self.first.clone();

            for (key, terms) in &self.rules {
                let first = self.first.get_mut(key);
                let first = match first {
                    Some(set) => set,
                    None => {
                        self.first.insert(key.clone(), HashSet::new());
                        self.first.get_mut(key).unwrap()
                    }
                };
                for term in terms {
                    match term.get(0) {
                        Some(symbol) => {
                            if matches!(symbol, Symbol::Terminal(_)) {
                                first.insert(symbol.clone());
                            } else {
                                let first2 = prev_first.get(symbol);
                                let first2 = match first2 {
                                    Some(set) => set.clone(),
                                    None => HashSet::new(),
                                };
                                first.extend(first2.into_iter().map(|i| i.clone()));
                            }
                        }
                        None => {
                            first.insert(Symbol::Terminal("".to_string()));
                        }
                    }
                }
            }

            if self.first == prev_first {
                break;
            }
        }
    }

    fn printFirst(&self) {
        for (key, set) in &self.first {
            print!("{}:", key);
            for symbol in set {
                print!("{} ", symbol);
            }
            println!();
        }
    }

    fn getFollow(&mut self) {
        self.follow.insert(
            self.start_symbol.clone(),
            HashSet::from([Symbol::Terminal("$".to_string())]),
        );
        loop {
            let prev_follow = self.follow.clone();
            for (key, terms) in &self.rules {
                for term in terms {
                    let mut i = 1;
                    while i < term.len() {
                        let symbol1 = term[i - 1].clone();
                        let symbol2 = term[i].clone();
                        if matches!(symbol1, Symbol::Nonterminal(_)) {
                            let follow = self.follow.get_mut(&symbol1);
                            let follow = match follow {
                                Some(set) => set,
                                None => {
                                    self.follow.insert(symbol1.clone(), HashSet::new());
                                    self.follow.get_mut(&symbol1).unwrap()
                                }
                            };
                            if matches!(symbol2, Symbol::Terminal(_)) {
                                follow.insert(symbol2.clone());
                            } else {
                                let first2 = self.first.get(&symbol2);
                                let first2 = match first2 {
                                    Some(set) => set.clone(),
                                    None => HashSet::new(),
                                };
                                follow.extend(
                                    first2
                                        .into_iter()
                                        .filter(|i| {
                                            if let Symbol::Terminal(t) = i {
                                                if t.is_empty() {
                                                    return false;
                                                } else {
                                                    return true;
                                                }
                                            } else {
                                                false
                                            }
                                        })
                                        .map(|i| i.clone()),
                                );
                            }
                        }
                        i += 1;
                    }
                    let mut i = term.len() as isize - 1;
                    while i > 0 {
                        let symbol = term[i as usize].clone();
                        if matches!(symbol, Symbol::Nonterminal(_)) {
                            let follow_to = self.follow.get_mut(&symbol);
                            let follow_to = match follow_to {
                                Some(set) => set,
                                None => {
                                    self.follow.insert(symbol.clone(), HashSet::new());
                                    self.follow.get_mut(&symbol).unwrap()
                                }
                            };
                            let follow_from = prev_follow.get(&key);
                            let follow_from = match follow_from {
                                Some(set) => set.clone(),
                                None => HashSet::new(),
                            };
                            follow_to.extend(follow_from.into_iter().map(|i| i.clone()));
                            let first2 = self.first.get(&symbol);
                            let first2 = match first2 {
                                Some(set) => set.clone(),
                                None => HashSet::new(),
                            };
                            if !first2.contains(&Symbol::Terminal("".to_string())) {
                                break;
                            }
                        } else {
                            break;
                        }
                        i -= 1;
                    }
                }
            }
            if self.follow == prev_follow {
                break;
            }
        }
    }

    pub fn printFollow(&self) {
        for (key, set) in &self.follow {
            print!("{}:", key);
            for symbol in set {
                print!("{} ", symbol);
            }
            println!();
        }
    }

    fn getTable(&mut self) {
        for (key, terms) in &self.rules {
            for term in terms {
                let mut i = 0;
                while i < term.len() {
                    let alpha = &term[i];
                    match alpha {
                        Symbol::Terminal(t) => {
                            self.table
                                .insert((key.clone(), alpha.clone()), (key.clone(), term.clone()));
                        }
                        Symbol::Nonterminal(n) => {
                            for a in &self.first[&alpha] {
                                if a != &Symbol::Terminal("".to_string()) {
                                    self.table.insert(
                                        (key.clone(), a.clone()),
                                        (key.clone(), term.clone()),
                                    );
                                }
                            }
                            if self.first[alpha].contains(&Symbol::Nonterminal("".to_string())) {
                                break;
                            }
                        }
                        _ => break,
                    }
                    i += 1;
                }
                if i == term.len() {
                    for b in &self.follow[key] {
                        if let Symbol::Nonterminal(n) = b {
                            self.table
                                .insert((key.clone(), b.clone()), (key.clone(), term.clone()));
                        }
                    }
                }
            }
        }
    }

    pub fn printTable(&self) {
        for (key, value) in &self.table {
            print!("{:?}:{:?}", key, value);
            println!();
        }
    }

    pub fn parse(&mut self) {
        println!("1. 生成FIRST和FOLLOW集合");
        self.getFirst();
        self.printFirst();
        self.getFollow();
        println!("Follow集合");
        self.printFollow();
        println!("2. 生成预测分析表");
        self.getTable();
        self.printTable();
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Nonterminal(n) => write!(f, "{}", n),
            Symbol::Terminal(t) => {
                if t.is_empty() {
                    write!(f, "ε")
                } else {
                    write!(f, "{}", t)
                }
            }
        }
    }
}
