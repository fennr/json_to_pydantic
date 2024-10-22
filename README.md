# json_to_pydantic

`json_to_pydantic` — это CLI утилита на языке Rust, которая конвертирует JSON файл в файл с Pydantic моделями на языке Python.

## Особенности

- Поддержка вложенных объектов и массивов
- Автоматическое создание вложенных Pydantic моделей
- Возможность вывода результата в файл или в стандартный вывод

## Установка

1. Убедитесь, что у вас установлен [Rust](https://www.rust-lang.org/).
2. Склонируйте репозиторий:

    ```bash
    git clone https://github.com/fennr/json_to_pydantic.git
    cd json_to_pydantic
    ```

3. Соберите проект:

    ```bash
    cargo build --release
    ```

## Использование

### Запись результата в файл

```bash
./target/release/json_to_pydantic --input input.json --output output.py
```

### Вывод результата в stdout

```bash
./target/release/json_to_pydantic --input input.json
```

Пример JSON-файла:

```json
{
  "name": "Bob",
  "age": 30,
  "is_active": true,
  "address": {
    "street": "123 Main St",
    "city": "Berlin"
  },
  "tags": ["friend", "colleague"],
  "projects": [
    {
      "name": "Project A",
      "budget": 100000
    },
    {
      "name": "Project B",
      "budget": 200000
    }
  ]
}
```

Результат:

```py
from pydantic import BaseModel, Field
from typing import Any

class Address(BaseModel):
    city_name: str | None = Field(None, alias="cityName")
    street_name: str | None = Field(None, alias="streetName")


class Projects(BaseModel):
    budget: float | None = Field(None, alias="budget")
    name: str | None = Field(None, alias="name")


class Model(BaseModel):
    address: Address | None = Field(None, alias="address")
    age: float | None = Field(None, alias="age")
    is_active: bool | None = Field(None, alias="is_active")
    name: str | None = Field(None, alias="name")
    projects: list[Projects] | None = Field(None, alias="projects")
    tags: list[str] | None = Field(None, alias="tags")
```
