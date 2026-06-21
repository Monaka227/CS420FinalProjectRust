// for method dispatch
trait Animal {
    fn speak(&self);
}

// Animal class strcut (instead of parent class)
struct BaseAnimal {
    name: String, // field for testing variable inheritance
}

// child class。
// Rust don't have inheritance, so composition base struct
struct Dog {
    base: BaseAnimal, // variable inheritance simulation
    breed: String,    // dog field
}

struct Cat {
    base: BaseAnimal,
    is_lazy: bool,
}

// implement animal tarit(method) for dog
impl Animal for Dog {
    fn speak(&self) {
        // testing whether can access parent variable(name)
        println!("{} (a {}) says: Woof!", self.base.name, self.breed);
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.base.name);
    }
}

fn main() {
    println!("=== OOP Feature Test in Rust ===\n");

    // Test1: variable inheritance simulation
    let my_dog = Dog {
        base: BaseAnimal { name: String::from("Buddy") },
        breed: String::from("Golden Retriever"),
    };

    // test whether can access the parent field
    println!("[Test 1: Variable Inheritance]");
    println!("Dog's name (from base): {}", my_dog.base.name);
    println!("Dog's breed: {}\n", my_dog.breed);


    // Test2: method dispatch
    println!("[Test 2: Dynamic Method Dispatch]");
    
    let my_cat = Cat {
        base: BaseAnimal { name: String::from("Whiskers") },
        is_lazy: true,
    };

    // group different types (dog and cat) into an array as references to common trait
    // by using "&dyn Animal" instruct Rust to perform dynamic dispatch
    let zoo: Vec<&dyn Animal> = vec![&my_dog, &my_cat];

    for animal in zoo {
        // test whether the appropriate "speak()" is called at runtime
        animal.speak();
    }
}