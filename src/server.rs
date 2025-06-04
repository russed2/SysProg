use tokio::{fs, io::AsyncWriteExt, net::TcpListener};
use std::error::Error;

#[tokio::main] // Асинхронная точка входа
async fn main() -> Result<(), Box<dyn Error>> {
    // Создаём TCP-сервер, который будет слушать порт 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    // Бесконечный цикл ожидания подключений от клиентов
    loop {
        // Принимаем новое соединение
        let (mut socket, _) = listener.accept().await?;

        // Обрабатываем каждое соединение в отдельной асинхронной задаче
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];

            // Ждём, пока можно будет читать из сокета
            let n = match socket.readable().await {
                Ok(_) => match socket.try_read(&mut buffer) {
                    Ok(n) => n, 
                    Err(e) => {
                        eprintln!("Ошибка чтения: {:?}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Ошибка доступа к сокету: {:?}", e);
                    return;
                }
            };

            let content = &buffer[..n]; // Обрезаем до реально прочитанного
            // Создаём уникальное имя файла на основе текущего времени
            let filename = format!(
                "received_{}.txt",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );

            if let Err(e) = fs::write(&filename, content).await {
                eprintln!("Ошибка сохранения файла: {}", e);
                return;
            }

            // Преобразуем текст в строку для анализа
            let text = String::from_utf8_lossy(content);
            let lines = text.lines().count(); 
            let words = text.split_whitespace().count();
            let chars = text.chars().count();

            // Готовим строку с результатами анализа
            let result = format!(
                "Имя файла: {}\nСтрок: {}\nСлов: {}\nСимволов: {}\n",
                filename, lines, words, chars
            );

            // Открываем файл с результатами анализа
            if let Ok(mut file) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("analysis_result.txt")
                .await
            {
                let _ = file.write_all(result.as_bytes()).await;
            }

            // Отправляем результат обратно клиенту
            let _ = socket.write_all(result.as_bytes()).await;
        });
    }
}
