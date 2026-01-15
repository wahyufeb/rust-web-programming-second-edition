use std::collections::HashMap;

fn main() {
    // HashMap
    #[derive(Debug)]
    enum CharacterValue {
        Name(String),
        Age(i32),
        Items(Vec<String>)
    }

    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();
    profile.insert("name", CharacterValue::Name(String::from("Puimek")));
    profile.insert("age", CharacterValue::Age(23));
    profile.insert("items", CharacterValue::Items(vec![String::from("book"), String::from("coffee beans")]));

    match profile.get("name").unwrap() {
        CharacterValue::Name(name) => {
            println!("the name is {}", name);
        },
        _ => panic!("name should be a string")
    }

    println!("{:?}", profile);

    // Immutable borrowing of variables
    fn print_immutable_borrow(value: &String) {
        println!("{}", value);
    }

    let one = "one".to_string();
    print_immutable_borrow(&one);
    println!("{}", one);

    // Mutable borrowing of variables
    fn print_mutable_borrow(value: &mut i8) {
        *value += 1;
        println!("In function the value is: {}", value);
    }

    let mut age: i8 = 23;
    print_mutable_borrow(&mut age);
    println!("In main the value is: {}", age);

    // Scopes
    let scope_one = &"scope one";
    let scope_two: &str;
    {
        println!("{}", scope_one);
        scope_two = &"scope two";
    }
    println!("{}", scope_one);
    println!("{}", scope_two);

    // Building Structs
    #[derive(Debug)]
    enum Friend {
        HUMAN(Box<Human>),
        NIL
    }
    #[derive(Debug)]
    struct Human {
        name: String,
        age: i8,
        current_thought: Option<String>,
        friend: Friend
    }

    let frontend_developer = Human {
        name: String::from("Puimek"),
        age: 30,
        current_thought: Some(String::from("Javascript forever")),
        friend: Friend::NIL
    };

    let developer = Human {
        name: "Alex".to_string(),
        age: 29,
        current_thought: None,
        friend: Friend::HUMAN(Box::new(frontend_developer))
    };

    match &developer.friend {
        Friend::HUMAN(data) => {
            println!("developer name {}", data.name);
        },
        Friend::NIL => {}
    }

    impl Human {
        fn new(name: &str, age: i8) -> Human {
            return Human{
                name: name.to_string(),
                age,
                current_thought: None,
                friend: Friend::NIL
            }
        }

        fn with_thought(mut self, thought: &str) -> Human {
            self.current_thought = Some(thought.to_string());
            return self;
        }

        fn with_friend(mut self, friend: Box<Human>) -> Human {
            self.friend = Friend::HUMAN(friend);
            return self;
        }
    }

    let developer_friend = Human::new("Mangkud", 20);
    let new_developer = Human::new("Napasorn", 24)
        .with_thought("I love Tom Yum")
        .with_friend(Box::new(developer_friend));
    println!("===========================");
    println!("developer = {:?}", new_developer);

    // Trait & Struct

    trait CanEdit {
        fn edit(&self) {
            println!("admin is editing");
        }
    }

    trait CanCreate {
        fn create(&self) {
            println!("admin is creating");
        }
    }

    trait CanDelete {
        fn delete(&self) {
            println!("admin is deleting");
        }
    }

    struct AdminUser {
        username: String,
        password: String,
    }

    struct User {
        username: String,
        password: String,
    }

    impl CanDelete for AdminUser {}
    impl CanCreate for AdminUser {}
    impl CanEdit for AdminUser {}
    impl CanEdit for User {
        fn edit(&self) {
            println!("A standard user {} is editing", self.username);
        }
    }

    fn create<T: CanCreate>(user: &T) -> () {
        user.create();
    }

    fn edit<T: CanEdit>(user: &T) -> () {
        user.edit();
    }

    fn delete<T: CanDelete>(user: &T) -> () {
        user.delete();
    }

    let admin = AdminUser{
        username: "mangkud".to_string(),
        password: "mangkudmangkud".to_string()
    };

    let user = User {
        username: "napasorn".to_string(),
        password: "napasornnapasorn".to_string(),
    };

    create(&admin);

    edit(&admin);

    edit(&user);

    delete(&admin);

    // Metaprogramming with macros
    struct Coordinate <T, X> {
        x: T,
        y: X
    }

    let one = Coordinate{ x: 1, y: 2 };
    let two = Coordinate{ x: 2, y: 3 };
    let three = Coordinate{ x: 3, y: 4 };

    macro_rules! capitalize {
        ($a: expr_2021) => {
            let mut v: Vec<char> = $a.chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            $a = v.into_iter().collect();
        };
    }
    
    let mut x = String::from("capitalize testing");
    capitalize!(x);
    println!("{}", x);
}
