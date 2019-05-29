use std::fs;

fn add_table(name : String) -> bool {
    match (File::create(name.concat(".db")), File::create(name.concat(".meta"))) {
        (Ok(file),Ok(file2)) => true,
        _ => {
            remove_table();
            false
        }
    }
}

fn remove_table() -> bool {
    File::delete(name.concat(".db"));
    File::delete(name.concat(".meta"));
    true
}

fn list_tables() -> Vec<String> {
    let to_return : Vec<String> = Vec::new();
    for file in read_dir(db_dir) {
        if let file.matches("*.meta")  {
            to_return.insert(file)
        }
    }
}

fn append(table_name : String, to_append : Record) -> boolean {
    match get_schema(table_name) {
        Ok(file) => {
            match to_append.valid(table_name) {
                true => {
                    file.write(to_append.to_str())
                    true
                }
                false => false
            }
        }
        _ => false
    }
}

fn get_schema(table_name : String) -> Vec<String, Type> {
    match File::open(table_name.concat(".meta")) {
        Ok(file) => file.lines().map()
    }
}

fn nth_line(table_name : String, n : u64) -> Option<String> {
    match File::open(table_name.concat(".db")) {
        Ok(file) => Some(file.lines().nth(n))
        _ => None
    }
}

fn save_content(table_name : String, stream : Stream<Record>) -> boolean {
    match File::create(table_name.concat(".db.copy")) {
        Ok(file) => {
            while stream.next() {
                file.write();
            }
        _ => false;
    }
}

fn get_content(table_name : String) -> Option<Stream<Record>> {
    match File::open(table_name.concat(".db")) {
        Ok(file) => Some(file.lines().to_steam())
        _ => None
    }
}