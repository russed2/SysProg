use std::env;
use std::sync::{Arc, Mutex};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// Структура для хранения результата анализа одного файла
#[derive(Debug)]
struct FileAnalysis {
    filename: String,
    word_count: usize,
    char_count: usize,
}

// Асинхронная функция анализа одного файла
async fn analyze_file(filepath: String, results: Arc<Mutex<Vec<FileAnalysis>>>) {
    let mut file = match File::open(&filepath).await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Не удалось открыть файл {}: {}", filepath, e);
            return;
        }
    };

    // Читаем содержимое файла
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(_) => {
            // Подсчёт слов и символов
            let words = contents.split_whitespace().count();
            let chars = contents.chars().count();
            // Сохраняем результат
            let analysis = FileAnalysis {
                filename: filepath,
                word_count: words,
                char_count: chars,
            };
            // Добавляем результат в общий список через мьютекс
            let mut data = results.lock().unwrap();
            data.push(analysis);
        }
        Err(e) => {
            eprintln!("Ошибка при чтении файла: {}", e);
        }
    }
}

// Точка входа в программу
#[tokio::main]
async fn main() {
    // Получаем список аргументов (пропускаем имя программы)
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Укажите пути к текстовым файлам в аргументах командной строки.");
        return;
    }

    // Создаём общий защищённый список результатов
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Запускаем асинхронный анализ каждого файла в отдельной задаче
    for path in args {
        let res_clone = Arc::clone(&results);
        let path_clone = path.clone();
        let handle = tokio::spawn(async move {
            analyze_file(path_clone, res_clone).await;
        });
        handles.push(handle);
    }

    // Дожидаемся завершения всех задач
    for handle in handles {
        let _ = handle.await;
    }

    // Вывод результатов
    let data = results.lock().unwrap();
    let mut total_words = 0;
    let mut total_chars = 0;
    println!("Результаты анализа:");
    for analysis in data.iter() {
        println!("{}: {} слов, {} символов", analysis.filename, analysis.word_count, analysis.char_count);
        total_words += analysis.word_count;
        total_chars += analysis.char_count;
    }
    println!("Итог: {} слов, {} символов", total_words, total_chars);
}
