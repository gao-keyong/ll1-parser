use prettytable::{Cell, Row, Table};
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
        let mut tableT = Table::new();
        tableT.add_row(Row::from(["", "FIRST"]));
        for (key, set) in &self.first {
            let mut first_ele = String::new();
            for symbol in set {
                first_ele += &format!("{} ", symbol);
            }
            let row = [format!("{}", key), first_ele];
            tableT.add_row(Row::from(row));
        }
        tableT.printstd();
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
        let mut tableT = Table::new();
        tableT.add_row(Row::from(["", "FOLLOW"]));
        for (key, set) in &self.follow {
            let mut follow_ele = String::new();
            for symbol in set {
                follow_ele += &format!("{} ", symbol);
            }
            let row = [format!("{}", key), follow_ele];
            tableT.add_row(Row::from(row));
        }
        tableT.printstd();
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
                            break;
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
                            if !self.first[alpha].contains(&Symbol::Nonterminal("".to_string())) {
                                break;
                            }
                        }
                        _ => break,
                    }
                    i += 1;
                }
                if i == term.len() {
                    for b in &self.follow[key] {
                        if let Symbol::Terminal(n) = b {
                            self.table
                                .insert((key.clone(), b.clone()), (key.clone(), term.clone()));
                        }
                    }
                }
            }
        }
    }

    pub fn printTable(&self) {
        let mut tableT = Table::new();
        let head = ["", "+", "-", "*", "/", "(", ")", "num", "$"];
        tableT.add_row(Row::from(head));
        let mut rows: HashMap<Symbol, Vec<String>> = HashMap::new();
        for (key, value) in &self.table {
            // println!("{:?}:{:?}",key,value);
            let row_symbol = &key.0;
            let row = rows.get_mut(row_symbol);
            let row = match row {
                Some(r) => r,
                None => {
                    rows.insert(row_symbol.clone(), vec!["".to_string(); head.len()]);
                    rows.get_mut(row_symbol).unwrap()
                }
            };
            let column_symbol = &key.1;
            let column_index = head
                .iter()
                .position(|t| {
                    if let Symbol::Terminal(i) = column_symbol {
                        return i == t;
                    } else {
                        false
                    }
                })
                .unwrap();
            let mut rule = format!("{}->", value.0);
            for symbol in &value.1 {
                rule += &format!("{}", symbol);
            }
            if value.1.is_empty() {
                rule += "ε";
            }
            row[column_index] = rule.clone();
        }
        for (key, value) in rows {
            let row_head = match key {
                Symbol::Nonterminal(n) => n,
                _ => panic!(),
            };
            let mut row = value.clone();
            row[0] = row_head;
            tableT.add_row(Row::from(row));
        }
        tableT.printstd();
    }

    pub fn parse(&mut self, expr: &str) {
        println!("1. 生成FIRST和FOLLOW集合");
        self.getFirst();
        println!("First集合");
        self.printFirst();
        self.getFollow();
        println!("Follow集合");
        self.printFollow();
        println!("2. 生成预测分析表");
        self.getTable();
        self.printTable();
        println!("3. 语法预测分析");
        let mut input = expr.to_string() + "$";
        let num_re = Regex::new(r"[1-9]\d*(\.\d+)?").unwrap();
        let letter_re = Regex::new(r"num|\+|\-|\*|/|\(|\)|\$").unwrap();
        // println!("{:?}",num_re.captures(&input));
        input = num_re.replace_all(&input, "num").to_string();
        let mut stack = vec![
            Symbol::Terminal("$".to_string()),
            Symbol::Nonterminal(self.start_symbol.to_string()),
        ];
        let mut table_t = Table::new();
        table_t.add_row(Row::from(["栈", "输入", "输出"]));
        loop {
            // println!("{:?} | {:?}",stack,input);
            let row_stack: String = stack
                .iter()
                .map(|s| match s {
                    Symbol::Terminal(t) => t.clone(),
                    Symbol::Nonterminal(n) => n.clone(),
                })
                .collect::<Vec<String>>()
                .join(" ");
            let row_input: String = input.clone();
            let mut row_output: String = "".to_string();
            let x = stack.last().unwrap();
            let x = x.clone();
            let a = letter_re.captures(&input).unwrap().get(0).unwrap().as_str();
            match x {
                Symbol::Terminal(x) => {
                    if x == a {
                        stack.pop();
                        input = input[a.len()..].to_string();
                        if x == "$" {
                            break;
                        }
                    } else {
                        panic!("输入不匹配");
                    }
                }
                Symbol::Nonterminal(x) => {
                    let rule = self.table.get(&(
                        Symbol::Nonterminal(x.to_string()),
                        Symbol::Terminal(a.to_string()),
                    ));
                    match rule {
                        Some(r) => {
                            row_output = format!("{}->", r.0);
                            stack.pop();
                            let mut rule_vec = r.1.clone();
                            rule_vec.reverse();
                            if rule_vec.is_empty() {
                                row_output += "ε";
                            }
                            for r in rule_vec {
                                row_output += &format!("{}", r);
                                stack.push(r);
                            }
                        }
                        None => {
                            panic!("输入不匹配: {:?} {:?}", x, a);
                        }
                    }
                }
                _ => {
                    panic!("输入不匹配");
                }
            }
            let row = Row::from([row_stack, row_input, row_output]);
            table_t.add_row(row);
        }
        table_t.printstd();
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
