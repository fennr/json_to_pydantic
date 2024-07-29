use std::collections::{HashMap, VecDeque};

use serde_json::Value;

pub fn to_pydantic(
    json: &Value,
    model_name: &str,
    models: &mut HashMap<String, String>,
    order: &mut VecDeque<String>,
) {
    let mut result = format!("class {}(BaseModel):\n", model_name);

    if let Value::Object(map) = json {
        for (key, value) in map {
            match value {
                Value::String(_) => result.push_str(&format!("    {}: str\n", key)),
                Value::Number(_) => result.push_str(&format!("    {}: float\n", key)),
                Value::Bool(_) => result.push_str(&format!("    {}: bool\n", key)),
                Value::Array(arr) => {
                    if arr.is_empty() {
                        result.push_str(&format!("    {}: list\n", key));
                    } else {
                        let array_type = match &arr[0] {
                            Value::Object(_) => {
                                let sub_model_name =
                                    format!("{}{}", model_name, capitalize_first_letter(key));
                                to_pydantic(&arr[0], &sub_model_name, models, order);
                                sub_model_name
                            }
                            Value::String(_) => "str".to_string(),
                            Value::Number(_) => "float".to_string(),
                            Value::Bool(_) => "bool".to_string(),
                            _ => "Any".to_string(),
                        };
                        result.push_str(&format!("    {}: list[{}]\n", key, array_type));
                    }
                }
                Value::Object(_) => {
                    let sub_model_name = format!("{}{}", model_name, capitalize_first_letter(key));
                    to_pydantic(value, &sub_model_name, models, order);
                    result.push_str(&format!("    {}: {}\n", key, sub_model_name));
                }
                _ => result.push_str(&format!("    {}: Any\n", key)),
            }
        }
    } else {
        result.push_str("    ...\n");
    }

    models.insert(model_name.to_string(), result);
    order.push_back(model_name.to_string());
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
