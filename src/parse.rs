use std::collections::{HashMap, VecDeque};
use serde_json::Value;

// Главная функция для конвертации JSON в Pydantic модель
pub fn to_pydantic(
    json: &Value,
    model_name: &str,
    models: &mut HashMap<String, String>,
    order: &mut VecDeque<String>,
) {
    let mut result = format!("class {}(BaseModel):\n", model_name);

    if let Value::Object(map) = json {
        for (key, value) in map {
            let field_definition = map_json_to_pydantic_field(key, value, model_name, models, order);
            result.push_str(&format!("    {}\n", field_definition));
        }
    } else {
        result.push_str("    ...\n");
    }

    models.insert(model_name.to_string(), result);
    order.push_back(model_name.to_string());
}

// Функция для преобразования каждого поля JSON в строку Pydantic модели
fn map_json_to_pydantic_field(
    key: &str,
    value: &Value,
    model_name: &str,
    models: &mut HashMap<String, String>,
    order: &mut VecDeque<String>,
) -> String {
    let snake_case_key = camel_to_snake_case(key);
    let alias = format!("alias=\"{}\"", key);

    match value {
        Value::String(_) => format!("{}: str | None = Field(None, {})", snake_case_key, alias),
        Value::Number(_) => format!("{}: float | None = Field(None, {})", snake_case_key, alias),
        Value::Bool(_) => format!("{}: bool | None = Field(None, {})", snake_case_key, alias),
        Value::Array(arr) => map_array_field(arr, key, snake_case_key, alias, model_name, models, order),
        Value::Object(_) => map_object_field(value, key, snake_case_key, alias, model_name, models, order),
        _ => format!("{}: Any | None = Field(None, {})", snake_case_key, alias),
    }
}

// Обработка полей, содержащих массивы
fn map_array_field(
    arr: &Vec<Value>,
    key: &str,
    snake_case_key: String,
    alias: String,
    model_name: &str,
    models: &mut HashMap<String, String>,
    order: &mut VecDeque<String>,
) -> String {
    if arr.is_empty() {
        return format!("{}: list | None = Field(None, {})", snake_case_key, alias);
    }

    let array_type = match &arr[0] {
        Value::Object(_) => {
            let sub_model_name = format!("{}", capitalize_first_letter(&key));
            to_pydantic(&arr[0], &sub_model_name, models, order);
            sub_model_name
        }
        Value::String(_) => "str".to_string(),
        Value::Number(_) => "float".to_string(),
        Value::Bool(_) => "bool".to_string(),
        _ => "Any".to_string(),
    };

    format!("{}: list[{}] | None = Field(None, {})", snake_case_key, array_type, alias)
}

// Обработка полей, содержащих объекты
fn map_object_field(
    value: &Value,
    key: &str,
    snake_case_key: String,
    alias: String,
    model_name: &str,
    models: &mut HashMap<String, String>,
    order: &mut VecDeque<String>,
) -> String {
    let sub_model_name = format!("{}", capitalize_first_letter(&key));
    to_pydantic(value, &sub_model_name, models, order);
    format!("{}: {} | None = Field(None, {})", snake_case_key, sub_model_name, alias)
}

// Преобразование camelCase в snake_case
fn camel_to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                snake_case.push('_');
            }
            snake_case.push(ch.to_lowercase().next().unwrap());
        } else {
            snake_case.push(ch);
    }
    }
    snake_case
}

// Функция для капитализации первой буквы строки
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Тесты для camel_to_snake_case
    #[test]
    fn test_camel_to_snake_case_with_camelcase() {
        assert_eq!(camel_to_snake_case("camelCase"), "camel_case");
    }

    #[test]
    fn test_camel_to_snake_case_with_pascalcase() {
        assert_eq!(camel_to_snake_case("CamelCase"), "camel_case");
    }

    #[test]
    fn test_camel_to_snake_case_with_snakecase() {
        assert_eq!(camel_to_snake_case("snake_case"), "snake_case");
    }

    #[test]
    fn test_camel_to_snake_case_with_lowercase() {
        assert_eq!(camel_to_snake_case("lowercase"), "lowercase");
    }

    #[test]
    fn test_camel_to_snake_case_with_uppercase() {
        assert_eq!(camel_to_snake_case("Uppercase"), "uppercase");
    }

    // Тесты для capitalize_first_letter
    #[test]
    fn test_capitalize_first_letter_with_lowercase() {
        assert_eq!(capitalize_first_letter("camel"), "Camel");
    }

    #[test]
    fn test_capitalize_first_letter_with_capitalized() {
        assert_eq!(capitalize_first_letter("Camel"), "Camel");
    }

    #[test]
    fn test_capitalize_first_letter_with_underscore() {
        assert_eq!(capitalize_first_letter("snake_case"), "Snake_case");
    }

    #[test]
    fn test_capitalize_first_letter_with_empty_string() {
        assert_eq!(capitalize_first_letter(""), "");
    }

    // Тесты для map_json_to_pydantic_field (преобразование JSON в Pydantic поля)
    #[test]
    fn test_map_json_to_pydantic_field_string() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({"streetName": "123 Main St"});
        let field = map_json_to_pydantic_field("streetName", &json["streetName"], "TestModel", &mut models, &mut order);
        assert_eq!(field, "street_name: str | None = Field(None, alias=\"streetName\")");
    }

    #[test]
    fn test_map_json_to_pydantic_field_number() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({"age": 30});
        let field = map_json_to_pydantic_field("age", &json["age"], "TestModel", &mut models, &mut order);
        assert_eq!(field, "age: float | None = Field(None, alias=\"age\")");
    }

    #[test]
    fn test_map_json_to_pydantic_field_boolean() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({"isActive": true});
        let field = map_json_to_pydantic_field("isActive", &json["isActive"], "TestModel", &mut models, &mut order);
        assert_eq!(field, "is_active: bool | None = Field(None, alias=\"isActive\")");
    }

    // Тесты для map_array_field
    #[test]
    fn test_map_array_field_empty_array() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({"items": []});
        let field = map_json_to_pydantic_field("items", &json["items"], "TestModel", &mut models, &mut order);
        assert_eq!(field, "items: list | None = Field(None, alias=\"items\")");
    }

    #[test]
    fn test_map_array_field_string_array() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({"names": ["Alice", "Bob"]});
        let field = map_json_to_pydantic_field("names", &json["names"], "TestModel", &mut models, &mut order);
        assert_eq!(field, "names: list[str] | None = Field(None, alias=\"names\")");
    }

    #[test]
    fn test_map_array_field_object_array() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({"users": [{"name": "Alice"}, {"name": "Bob"}]});
        let field = map_json_to_pydantic_field("users", &json["users"], "TestModel", &mut models, &mut order);
        assert!(field.contains("users: list[Users] | None = Field(None, alias=\"users\")"));
    }

    // Тесты для map_object_field
    #[test]
    fn test_map_object_field() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({
            "address": {
                "streetName": "123 Main St",
                "city": "New York"
            }
        });
        let field = map_json_to_pydantic_field("address", &json["address"], "TestModel", &mut models, &mut order);
        assert!(field.contains("address: Address | None = Field(None, alias=\"address\")"));
    }

    // Интеграционные тесты для to_pydantic
    #[test]
    fn test_to_pydantic_single_level() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({
            "name": "John Doe",
            "age": 30,
            "isActive": true
        });

        to_pydantic(&json, "UserModel", &mut models, &mut order);

        let user_model = models.get("UserModel").unwrap();
        assert!(user_model.contains("name: str | None = Field(None, alias=\"name\")"));
        assert!(user_model.contains("age: float | None = Field(None, alias=\"age\")"));
        assert!(user_model.contains("is_active: bool | None = Field(None, alias=\"isActive\")"));
    }

    #[test]
    fn test_to_pydantic_nested_objects_and_arrays() {
        let mut models = HashMap::new();
        let mut order = VecDeque::new();

        let json = json!({
            "name": "John Doe",
            "addresses": [
                {
                    "streetName": "123 Main St",
                    "city": "New York"
                },
                {
                    "streetName": "456 Market St",
                    "city": "San Francisco"
                }
            ]
        });

        to_pydantic(&json, "UserModel", &mut models, &mut order);

        let user_model = models.get("UserModel").unwrap();
        assert!(user_model.contains("name: str | None = Field(None, alias=\"name\")"));
        assert!(user_model.contains("addresses: list[Addresses] | None = Field(None, alias=\"addresses\")"));

        let addresses_model = models.get("Addresses").unwrap();
        assert!(addresses_model.contains("street_name: str | None = Field(None, alias=\"streetName\")"));
        assert!(addresses_model.contains("city: str | None = Field(None, alias=\"city\")"));
    }
}

