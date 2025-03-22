# Reed-Solomon Python Module (`reed_solomon_py`)


Библиотека `reed_solomon_py` предоставляет Python-интерфейс для работы с кодами Рида-Соломона, реализованными на Rust. Она позволяет кодировать данные с добавлением избыточности для коррекции ошибок, а также восстанавливать поврежденные данные.

## Особенности

- **Кодирование данных**: Добавление избыточности для коррекции ошибок.
- **Декодирование данных**: Восстановление данных даже при наличии ошибок.
- **Инвертирование битов**: Данные выглядят как "случайные" байты.
- **Встраивание метаданных**: Количество байтов для коррекции встроено в данные.

## Установка

### Требования

- Python 3.7 или выше.
- Rust (для сборки модуля).

### Установка через `maturin`

1. Установите `maturin`:
   ```bash
   pip install maturin
   ```

2. Клонируйте репозиторий:
   ```bash
   git clone https://github.com/kostya2023/reed_solomon_py.git
   cd reed_solomon_py
   ```

3. Соберите и установите модуль:
   ```bash
   maturin develop --release
   ```

## Использование

### Пример кода

```python
import reed_solomon_py

# Исходное сообщение
message = "Hello, Hold!"
ecc_bytes = 10  # Количество байтов для коррекции ошибок

# Упаковываем данные
packed_data = reed_solomon_py.pack_data(message, ecc_bytes)
print("Packed data:", packed_data)

# Распаковываем данные
restored_message, has_errors = reed_solomon_py.unpack_data(packed_data)
if has_errors:
    print("Failed to restore data: The data is corrupted beyond repair.")
else:
    print("Restored message:", restored_message)
```

### Результат

```plaintext
Packed data: b'\xff\xff\xff\xff\xff\xff\xff\xf5\x97\x9a\x9b\x9b\x9c\xb2\xb5\x97\x9f\x9c\x9b\x9a\xb4\xff\xff\xff\xff\xff\xff\xff\xffl\xff\x14h8\x16\x83\xd3\xfe\x0a\x8c6\xef;\x9c\x14\x1c\xc4'
Restored message: Hello, Hold!
```

## API

### `pack_data(message: str, ecc_bytes: int) -> bytes`

Кодирует сообщение с добавлением избыточности для коррекции ошибок, инвертирует биты и прячет количество байтов для коррекции.

- **Параметры:**
  - `message`: Исходное сообщение (строка).
  - `ecc_bytes`: Количество байтов для коррекции ошибок.
- **Возвращает:** Упакованные данные в виде байтов.

### `unpack_data(data: bytes) -> Tuple[Optional[str], bool]`

Распаковывает данные, извлекает количество байтов для коррекции и восстанавливает сообщение.

- **Параметры:**
  - `data`: Упакованные данные в виде байтов.
- **Возвращает:** Кортеж из двух элементов:
  - Восстановленное сообщение (или `None`, если данные невозможно восстановить).
  - Флаг ошибки (`True`, если данные невозможно восстановить, иначе `False`).

## Лицензия

[LICENSE](LICENSE) - [MIT](https://choosealicense.com/licenses/mit/)

## Авторы

- [kostya2023](https://github.com/kostya2023)