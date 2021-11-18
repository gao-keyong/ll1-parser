use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt{
    /// Expression to be parsed
    #[structopt(help="Expression to be parsed")]
    expr: String,
}

fn main() {
    let opt = Opt::from_args();
    let expr = opt.expr;
    println!("{}", expr);
}
