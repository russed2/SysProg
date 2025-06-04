use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main] // Асинхронная главная функция
async fn main() -> io::Result<()> {
    // Открываем локальный текстовый файл, который хотим отправить
    let mut file = File::open("example.txt").await?;

    let mut content = Vec::new(); 
    file.read_to_end(&mut content).await?; // Читаем весь файл в память

    // Устанавливаем соединение с сервером на 127.0.0.1:8080
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Отправляем содержимое файла на сервер
    stream.write_all(&content).await?;

    let mut buffer = vec![0u8; 1024]; 
    let n = stream.read(&mut buffer).await?; // Читаем ответ

    // Показываем полученный результат пользователю
    println!("Ответ от сервера:\n{}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
