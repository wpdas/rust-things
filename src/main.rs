struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn sayhi(&self) -> String {
        // faz a mesma coisa que o println! porem retorna o valor
        format!("Meu nome e {} e eu tenho {} anos", self.name, self.age)
    } 
}

fn main() {
    let person = Person {name: "Bruno".to_string(), age: 32};
    let person2 = Person {name: "Flavia".to_string(), age: 26};

    println!("{}", person.sayhi());
    print!("{}", person2.sayhi());
}

// mod my_mod;

// fn main() {
//     my_mod::nested::function();
//     my_mod::function();
// }