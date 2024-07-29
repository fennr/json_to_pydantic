use clap::Parser;
use serde_json::Value;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};
mod parse;

#[derive(Parser)]
#[command(name = "json_to_pydantic")]
#[command(about = "Convert JSON to Pydantic", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let input_path = &cli.input;
    let output_path = &cli.output;

    let data = fs::read_to_string(input_path).expect("Невозможно прочитать файл!");
    let json: Value = serde_json::from_str(&data).expect("Невозможно распарсить JSON!");

    let mut models = HashMap::new();
    let model_name = "Model";
    let mut order = VecDeque::new();
    parse::to_pydantic(&json, model_name, &mut models, &mut order);

    let mut pydantic_model =
        "from pydantic import BaseModel\nfrom typing import Any\n\n".to_string();
    while let Some(model) = order.pop_front() {
        pydantic_model.push_str(models.get(&model).unwrap());
        pydantic_model.push_str("\n\n");
    }

    match output_path {
        Some(path) => {
            fs::write(path, pydantic_model).expect("Невозможно записать файл!");
        }
        None => {
            println!("{}", pydantic_model);
        }
    }
}
