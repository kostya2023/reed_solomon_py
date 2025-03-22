use pyo3::prelude::*;
use pyo3::types::PyBytes;
use reed_solomon_erasure::{ReedSolomon, galois_8::Field};

/// Добавляет ECC к сообщению, инвертирует биты и прячет количество байтов для коррекции.
#[pyfunction]
fn pack_data(message: &str, ecc_bytes: usize) -> Py<PyBytes> {
    // Преобразуем строку в байты
    let message_bytes = message.as_bytes();

    // Добавляем ECC к сообщению
    let data_shards = 10;  // Количество data_shards
    let parity_shards = ecc_bytes;

    // Рассчитываем длину шарда
    let shard_len = (message_bytes.len() + data_shards - 1) / data_shards;

    // Создаем шарды
    let mut shards: Vec<Vec<u8>> = vec![vec![0; shard_len]; data_shards + parity_shards];

    // Заполняем данные в шард
    for i in 0..data_shards {
        let start = i * shard_len;
        let end = usize::min(start + shard_len, message_bytes.len());
        if end > start {
            shards[i][..end - start].copy_from_slice(&message_bytes[start..end]);
        }
    }

    // Кодируем данные
    let rs: ReedSolomon<Field> = ReedSolomon::new(data_shards, parity_shards).unwrap();
    rs.encode(&mut shards).unwrap();

    // Собираем все закодированные данные в один вектор
    let mut encoded = Vec::new();
    for shard in shards {
        encoded.extend_from_slice(&shard);
    }

    // Добавляем количество байтов для коррекции в начало данных
    let mut data_with_ecc = Vec::new();
    data_with_ecc.extend_from_slice(&ecc_bytes.to_be_bytes());  // 8 байт для хранения ecc_bytes
    data_with_ecc.extend_from_slice(&encoded);

    // Инвертируем биты
    let inverted_data: Vec<u8> = data_with_ecc.iter().map(|&b| !b).collect();

    // Возвращаем закодированные данные в PyBytes
    Python::with_gil(|py| PyBytes::new(py, &inverted_data).into())
}

/// Извлекает количество байтов для коррекции, восстанавливает данные и возвращает результат.
#[pyfunction]
fn unpack_data(data: &[u8]) -> PyResult<(Option<String>, bool)> {
    // Инвертируем биты обратно
    let restored_data: Vec<u8> = data.iter().map(|&b| !b).collect();

    // Извлекаем количество байтов для коррекции (первые 8 байт)
    let ecc_bytes = usize::from_be_bytes(restored_data[..8].try_into().unwrap());

    // Извлекаем закодированные данные
    let encoded_data = &restored_data[8..];

    // Восстанавливаем данные
    let data_shards = 10;  // Количество data_shards
    let parity_shards = ecc_bytes;

    // Рассчитываем длину шарда
    let shard_len = encoded_data.len() / (data_shards + parity_shards);

    // Разбиваем данные на шарды
    let mut shards: Vec<Option<Vec<u8>>> = encoded_data
        .chunks(shard_len)
        .map(|chunk| Some(chunk.to_vec()))
        .collect();

    // Восстанавливаем данные
    let rs: ReedSolomon<Field> = ReedSolomon::new(data_shards, parity_shards).unwrap();
    if rs.reconstruct(&mut shards).is_ok() {
        // Собираем данные из data_shards
        let corrected_data: Vec<u8> = shards
            .into_iter()
            .take(data_shards)
            .flat_map(|shard| shard.unwrap())
            .collect();

        // Преобразуем в строку
        let message = String::from_utf8(corrected_data).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid UTF-8 in message")
        })?;
        Ok((Some(message), false))  // Успешное восстановление
    } else {
        Ok((None, true))  // Данные невозможно восстановить
    }
}

/// Регистрируем модуль Python.
#[pymodule]
fn reed_solomon_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pack_data, m)?)?;
    m.add_function(wrap_pyfunction!(unpack_data, m)?)?;
    Ok(())
}