use std::io;

fn main() {
    // Загадываем 4 числа
    let digits: String = get_digits();
    println!("Загадано: {}", digits);
    // Устанавливаем 5 попыток
    let mut attemts: u8 = 5;
    println!("Введите 4 цифры:");
    loop {
        // Проверям остались ли попытки
        if attemts == 0 {
            println!("Попытки закончились! Вы проиграли!");
            break;
        }
        // Получаем ввод предположения
        let mut guess: String = String::new();
        let input: Result<usize, io::Error> = io::stdin().read_line(&mut guess);
        guess.pop();
        // При ошибке ввода, длины предположения или парсинга в число просим ввести снова
        if input.is_err() || guess.len() != 4 || guess.parse::<u16>().is_err()  { 
            println!("Ошибка! Повторите ввод...");
            continue;
        };
        // Если в предположении число повторяется больше 1 раза просим ввести снова
        if char_repeats(&guess) {
            println!("Ошибка! В вашем вводе есть 2 одинаковых числа! Повторите ввод...");
            continue;
        }
        // Если предположение верно, объявляем победу
        if guess == digits {
            println!("Вы угадали! Победа!");
            break;
        } else {
            // Иначе считаем количество быков, коров, затем выводим быков и коров (считаем совпадения и вычитаем быков)
            let bulls = get_bulls(digits.as_str(), guess.as_str());
            println!("Количество быков: {}", bulls);
            println!("Количество коров: {}", get_cows(digits.as_str(), guess.as_str()) - bulls);
            // Тратим попытку
            attemts -= 1;
            println!("Осталось попыток: {}", attemts);
        }
    }
}

// Короткое получение символа по индексу (сахар)
fn get_char(string: &str, index: usize) -> Option<char> {
    return string.chars().nth(index);
}

// Проверка повтора в строке одного и того же символа
fn char_repeats(string: &str) -> bool {
    // Перебираем все символы, фильтруем остальные символы строки и проверяем количество оставшихся символов
    // Если больше 1 - значит повтаряется, иначе - не повторяется
    for char in string.chars() {
        if string.chars().filter(|el: &char| *el == char).count() != 1 {
            return true;
        }
    }
    return false;
}

// Загадываем цифру от 0 до 9
fn get_random_digit() -> String {
    // Берём число от 0 до 255, делим его на 255 чтобы получить дробное число
    // А потом умножаем на 10, чтобы получить число от 0 до 9
    let number: f32 = rand::random::<u8>() as f32 / 255.0 * 10.0;
    // Возвращаем число в типе String после каста до u8 и округляя в меньшую сторону
    return (number.floor() as u8).to_string();
}

// Получаем 4 цифры в виде строки
fn get_digits() -> String {
    let mut string: String = String::new();
    for _ in 0..4 {
        loop {
            // Пролучаем рандомное число
            // Если оно уже есть - повторяем операцию заново
            // Иначе переходим к другому символу из 4
            let number: String = get_random_digit();
            if string.find(&number).is_none() {
                string += &number;
                break;
            }
        }
    }
    return string;
}

// Считаем быков
fn get_bulls(digits: &str, guess: &str) -> u8 {
    // Если предположение и загаданное не совпадает по длине - выводим ошибку
    if digits.len() != guess.len() {
        println!("Ошибка: длина загаданного и предположения не совпадают!");
        return 0;
    }
    // Перебираем символы и сравниваем по индексу в строке
    let mut bulls: u8 = 0;
    for index in 0..digits.len() {
        // Получаем символы из digits и guess по индексу
        let digits_curr: Option<char> = get_char(digits, index);
        let guess_curr: Option<char> = get_char(guess, index);
        // Если возвращается none - выводим ошибку (что-то очень пошло не так)
        if digits_curr.is_none() || guess_curr.is_none() {
            println!("Ошибка: во время итерации был возвращён none");
            return 0;
        }
        // Сравниваем символы и, если они совпадают, увеличиваем количество быков
        if digits_curr.unwrap() == guess_curr.unwrap() { 
            bulls += 1; 
        }
    }
    // Возвращаем быков
    return bulls;
}

// Считаем коров (включая быков, они же тоже коровы (наверное))
fn get_cows(digits: &str, guess: &str) -> u8 {
    // Если предположение и загаданное не совпадает по длине - выводим ошибку
    if digits.len() != guess.len() {
        println!("Ошибка: длина загаданного и предположения не совпадают!");
        return 0;
    }
    // Перебираем символы и ищем их в загаданном
    let mut cows: u8 = 0;
    for index in 0..digits.len() {
        // Получаем символ из загаданной строки
        let guess_curr: Option<char> = get_char(guess, index);
        // Проверяем на none
        if guess_curr.is_none() {
            println!("Ошибка: во время итерации был возвращён none");
            return 0;
        }
        // Ищем в строке и, если нашли, увеличиваем количество коров
        if digits.chars().any(|el: char| el == guess_curr.unwrap()) { 
            cows += 1; 
        }
    }
    // Возвращаем коров
    return cows;
}