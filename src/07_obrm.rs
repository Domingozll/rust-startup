struct Car {}

// obrm example -> unique ownership
fn mempry_examlpe() {
    let car = Box::new(Car {});
    let car2 = car; // implicit owner transfer(隐式转换所有权)
    let my_string = String::from("LGR");
    function_the_can_panic();
    if !should_continue() { retuen; }
}

// obrm example -> ownership sharing
fn mempry_examlpe_2() {
    let car = RC::new(Car {}); // Rc : reference counting
    let car2 = car.clone(); // share ownership whih car
    let my_string = String::from("LGR");
    function_the_can_panic();
    if !should_continue() { retuen; }
}

// obrm example -> manage file handle
fn file_example() {
    let path = Path::new("example.txt");
    let file = File::open(&path).unwrap();
    function_the_can_panic();
    if !should_continue() { retuen; }
}