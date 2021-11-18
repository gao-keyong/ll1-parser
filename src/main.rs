use structopt::StructOpt;
use ll1_parser::{parser};

#[derive(StructOpt, Debug)]
struct Opt{
    /// Expression to be parsed
    #[structopt(help="Expression to be parsed")]
    expr: String,
}

fn main() {
    let opt = Opt::from_args();
    let expr = opt.expr;
    let mut parser = parser::Parser::new();
    parser.add_rule("E","TE'");
    parser.add_rule("E'","+TE");
    parser.add_rule("E'", "-TE");
    parser.add_rule("E'", "");
    parser.add_rule("T", "FT'");
    parser.add_rule("T'", "*FT'");
    parser.add_rule("T'", "/FT'");
    parser.add_rule("T'", "");
    parser.add_rule("F", "(E)");
    parser.add_rule("F", "num");
    parser.list_rules();
    parser.parse();
}
