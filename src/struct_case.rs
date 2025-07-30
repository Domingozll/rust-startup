struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

fn main() {
    let mut book: Product = Product { name: String::from("Book-1"), price: 28.85, in_stock: true };
    let price: f32 = book.price;
    println!("price is: {price}"); // price is: 28.85
    let mut in_stock: bool = book.in_stock;
    println!("in_stock is: {in_stock}"); // in_stock is: true
    book.in_stock = false;
    in_stock = book.in_stock;
    println!("in_stock is: {in_stock}"); // in_stock is: false

    let sales_tax: f32 = calculate_sales_tax( &book);
    println!("sales_tax is: {sales_tax}"); // sales_tax is: 2.885
}

fn calculate_sales_tax(product: &Product) -> f32 {
    product.price * 0.1
}