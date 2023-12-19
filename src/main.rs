const UNIMODAL_TRAIN_EVAL_TEMPLATE: &str = include_str!("../templates/unimodal_train_eval.template");
const UNIMODAL_MAIN_TEMPLATE: &str = include_str!("../templates/unimodal_main.template");
const IMPORTS_TEMPLATE: &str = include_str!("../templates/imports.template");

use const_format::concatcp;

const UNIMODAL_TEMPLATE: &str = concatcp!(IMPORTS_TEMPLATE, UNIMODAL_TRAIN_EVAL_TEMPLATE, UNIMODAL_MAIN_TEMPLATE);
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long,  default_value_t=false)]
    multimodal: bool,
    #[clap(long, default_value_t=false) ]
    association: bool,
    #[clap(long)]
    out_file: String,
}

fn main() {

    let args: Args = Args::parse();

    match args.multimodal {
        true => {
            match args.association {
                true => {todo!("Association function not implemented yet")},
                false => {},
            }
        },
        false => {
            std::fs::write(&args.out_file, UNIMODAL_TEMPLATE).expect(format!("Unable to write file {}", args.out_file).as_str());
        },
    }

}
