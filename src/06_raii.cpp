class CarManager
{
private:
    Car* p; // pointer to a Car
public:
    CarManager(Car* p) : p(p) {}
    ~CarManager()
    {
        // clean up memory on the heap
        delete p;
    }
}

class Car {};

void memory_example()
{
    Car* car = new Car();                   // allocate memory on the heap
    function_that_can_throw();              // memory leak idf exception is thrown
    if(!should_continue()) return;          // memory leak if early return
    delete car;                             // clean up memory on the heap
}

//RAII example
void memory_example_2()
{
    CarManager car = CarManager(new Car());
    function_that_can_throw();
    if(!should_continue()) return;
}

//RAII example -> unique_ptr
void memory_example_3()
{
    unique_ptr<Car> car = make_unique<Car>();
    unique_ptr<Car> car2 = move(car);// ownership transfer
    // unique_ptr<Car> car2 = car; // error
    function_that_can_throw();
    if(!should_continue()) return;
}

//RAII example -> shared_ptr
void memory_example_4()
{
    shared_ptr<Car> car = make_shared<Car>();
    shared_ptr<Car> car2 = car; // 引用计数回收
    function_that_can_throw();
    if(!should_continue()) return;
}


class file
{
private:
    ofstream file: // file handle
public:
    File(string file_name) {
        file = ofstream(file_name);
    }
    _File()
    {
        // close file handle
        file.close();
    }
}

void file_example()
{
    ofstream file("example.txt");           // acquire file handle
    function_that_can_thrown();             // file is never closed if exception is thrown
    if(!should_continue()) return;          // file is never closed if early return
    file.close();                           // close file handle
}

//RAII example
void file_example2()
{
    File = File("example.txt");
    function_that_can_thrown();
    if(!should_continue()) return;
}