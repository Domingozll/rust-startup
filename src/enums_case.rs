struct Product {
    name: String,
    category: ProuctCategory,
    price: f32,
    in_stock: bool
}

enum ProuctCategory {
    Books,
    Clothing,
    Electrics
}

enum Commond {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Commond {
    fn serialize(&self) -> String {
        String::from("JSON string")
    }
}

fn main() {
    let category = ProuctCategory::Electrics;
    let product = Product {name: String::from("TV"), category: category,price: 20.98, in_stock: true};

    let cmd = Commond::Undo;
    let cmd  = Commond::Redo;
    let cmd  = Commond::AddText(String::from("test"));
    let cmd  = Commond::MoveCursor(22, 0);
    let cmd  = Commond::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    let json_string = cmd.serialize();
    println!("json_string is: {json_string}"); // json_string is: JSON string
}