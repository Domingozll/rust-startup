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
        let json_string = match self {
            Commond::Undo => String::from("{\"cmd\": \"undo\"}"),
            Commond::Redo => String::from("{\"cmd\": \"redo\"}"),
            Commond::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            },
            Commond::MoveCursor(line,column ) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": \"{line}\", \
                        \"column\": \"{column}\" \
                    }}"
                )
            },
            Commond::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        };

        json_string
    }
}

fn main() {

    let age = 35;
    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        x => println!("You are {x} years old!")
    }

    let category = ProuctCategory::Electrics;
    let product = Product {name: String::from("TV"), category: category,price: 20.98, in_stock: true};

    let cmd1 = Commond::Undo;
    let cmd2  = Commond::Redo;
    let cmd3  = Commond::AddText(String::from("test"));
    let cmd4  = Commond::MoveCursor(22, 0);
    let cmd5  = Commond::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    println!("{}",cmd1.serialize()); 
    println!("{}",cmd2.serialize()); 
    println!("{}",cmd3.serialize()); 
    println!("{}",cmd4.serialize()); 
    println!("{}",cmd5.serialize()); 
    /*
        {"cmd": "undo"}
        {"cmd": "redo"}
        { "cmd": "add_text", "text": "test" }
        { "cmd": "move_cursor", "line": "22", "column": "0" }
        { "cmd": "replace", "from": "a", "to": "b" }
    */
}