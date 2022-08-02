fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // превращаем строку в массив байтов

    for (i, &item) in bytes.iter().enumerate() { // проходим по каждому байту
        if item == b' ' {  // если это байт пробела
            return i -1; // возвращаем позицию предыдущего символа
        }
    }

    s.len() // иначе возвращаем длину строки
}

fn main() {
    let mut string = String::from("hello_world"); // создаем строки
    let index = first_word(&string); // передаем ее в функцию
    println!("Last character index - {}.", index); // выводим полученный индекс
    string.clear(); // строка очищена полученный индекс устарел
}
