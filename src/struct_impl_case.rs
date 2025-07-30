struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {

    //构造函数
    fn new_construct(name: String,price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }

    fn get_default_sales_tax() -> f32 {
        0.1
    }
    
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    // whill take ownership of self. auto destory instance after use
    fn buy(self) -> i32 {
        let name: String = self.name;
        println!("{name} was bought!");
        123
    }
}

fn main() {
    let mut book: Product = Product { name: String::from("Book-1"), price: 28.85, in_stock: true };
    let mut sales_tax: f32 = book.calculate_sales_tax();
    println!("sales_tax is: {sales_tax}"); // sales_tax is: 5.77

    book.set_price(1.0);
    book.buy(); // Book-1 was bought!
    // book.buy(); // error: book value used here after move
    // sales_tax = book.calculate_sales_tax(); // error: book value borrowed here after move
    // book.set_price(2.0); // error: book value borrowed here after move

    let book2: Product = Product::new_construct(String::from("Book-2"),39.9);
    book2.buy();// Book-2 was bought!
}

