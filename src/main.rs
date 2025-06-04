use std::env;
use std::fs;
use std::io;

/// Функция для чтения содержимого файла по указанному пути.
/// Использует передачу строки через заимствование (&str) и возвращает результат типа String.
fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

/// Функция для подсчёта количества вхождений искомого слова в тексте.
/// Принимает неизменяемую ссылку на содержимое файла и слово для поиска.
fn count_word_occurrences(content: &str, word: &str) -> usize {
    content
        .split_whitespace()
        .filter(|s| *s == word)
        .count()
}

/// Функция для подсчёта общего количества слов в тексте.
fn total_word_count(content: &str) -> usize {
    content.split_whitespace().count()
}

fn main() {
    // Считывание аргументов командной строки.
    // Приложению требуется два аргумента: путь к файлу и слово для поиска.
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Использование: {} <путь_к_файлу> <слово_для_поиска>",
            args[0]
        );
        std::process::exit(1);
    }
    let file_path = &args[1];
    let search_word = &args[2];

    // Чтение содержимого файла.
    let content = match read_file_contents(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Ошибка при чтении файла '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    // Подсчёт общего количества слов и количества повторений заданного слова.
    let total_words = total_word_count(&content);
    let occurrences = count_word_occurrences(&content, search_word);

    // Вывод результатов в консоль.
    println!("Общее количество слов в файле: {}", total_words);
    println!("Количество повторений слова '{}': {}", search_word, occurrences);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_word_occurrences() {
        let text = "rust rust code rust";
        // Ожидаем 3 вхождения "rust"
        assert_eq!(count_word_occurrences(text, "rust"), 3);
    }

    #[test]
    fn test_total_word_count() {
        let text = "one two three four";
        // Ожидаем 4 слова
        assert_eq!(total_word_count(text), 4);
    }
}