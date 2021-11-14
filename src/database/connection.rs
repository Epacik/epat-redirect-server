use std::error::Error;
use std::fs;
use std::panic::panic_any;
use serde::{Deserialize, Serialize};

/// Tworzenie połączenia z serwerem baz danych
pub async fn create() {
    let connectionstring = load_connection_string();
    crate::database::RB.link(&*connectionstring).await.unwrap();
}

/// Otwiera plik konfiguracyjny i na podstawie danych w nim zawartych tworzy ciąg połączeniowy
/// Jeśli nie uda się wczytać danych konfiguracyjnych, wywołuje panic!
/// ### zwraca ciąg połączeniowy do bazy danych
fn load_connection_string() -> String {
    let content :String;

    //załadujmy plik konfiguracyjny
    let  result = load_config_file();

    // jeśli nie wyszło, tworzymy pusty plik konfiguracyjnymi i panikujemy zabijając cały proces
    if result.is_err() {
        create_empty_config();
        panic_any(result.unwrap_err().to_string());
    }

    // odczytujemy zawartość pliku z rezultatu
    content = result.unwrap();

    // konwertujemy konfigurację w formacie JSON do struktury, aby można było z niej łatwiej wyciągnąć dane
    let serialization_result = serde_json::from_str(&*content);

    // jeśli nie udało się przeprowadzić konwersji, tworzymy nowyu pusty plik i zabijamy proces
    if serialization_result.is_err() {
        create_empty_config();
        panic!("error while serializing empty config file");
    }

    // wyciągamy konfigurację, do użycia w tworzeniu ciągu połączeniowego
    let config : Config = serialization_result.unwrap();

    return format!("postgres://{}:{}@{}:5432/{}", config.username, config.password, config.address, config.database);
}

/// Tworzy plik konfiguracyjny do wypełnienia przez użytkownika
fn create_empty_config() {

    // tworzymy pustą strukturę
    let empty_config = Config {
        address: "".to_string(),
        username: "".to_string(),
        password: "".to_string(),
        database: "".to_string(),
    };

    // konwertujemy strukturę do ładnie sformatownego JSONa (takiego rozpisanego na wiele linijek)
    let result = serde_json::to_vec_pretty(&empty_config);

    // jeśli konwersja się nie4 powiodła, panikujemy zabijając proces
    if result.is_err() {
        panic_any(result.unwrap_err().to_string());
    }

    // zapisujemy pustą konfigurację w formacie JSON do pliku config.json
    let result = fs::write("./config.json", result.unwrap());

    // jeśli nie udało się zapisać pliku, panikujemy
    if result.is_err() {
        panic_any(result.unwrap_err().to_string());
    }
}

///Odczytywanie pliku konfiguracyjnego
/// ### jeśli odczytanie pliku się powiodło, zwraca jego zawartość, jeśli nie, zwracany jest błąd
fn load_config_file() -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string("./config.json")?;
    Ok(content)
}

/// Przechowuje dane służące do stworzenia ciągu połączeniowego
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
}
