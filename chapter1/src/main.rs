use std::collections::HashMap;
use std::env::vars;

fn title(s: &str) {
    let border = format!["----{}----", s.chars().map(|_| '-').collect::<String>()];
    println!["\n{}\n :: {}\n{}", border, s, border];
}

fn main() {
    strings();
    numbers();
    arrays_and_vectors();
    hashmaps();
    results_and_errors();
    scopes_with_strings();
    scopes_with_numbers();
    lifetimes();
    structs();
    hashmaps_revisited();
}

fn strings() {
    title("STRINGS");
    fn print(input_string: &str) {
        println!["{}", input_string];
    }

    let test_string_slice = "Hello, world!";
    print(test_string_slice);
}

fn numbers() {
    title("NUMBERS");
    let small_unsigned = 255u8;
    let small_float: f32 = 20.6;
    println!["{}, {}", small_unsigned, small_float];
}

fn arrays_and_vectors() {
    title("ARRAYS AND VECTORS");
    let int_array = [1, 2, 3];
    for i in int_array.iter() {
        println!["{}", i];
    }
    let second_int_array = [1, 2, 3];
    let two = second_int_array[1];
    println!["{}", two];

    let mut str_vector = vec!["one", "two", "three"];
    str_vector.push("four");
    for i in str_vector.iter() {
        println!["{}", i];
    }
}

fn hashmaps() {
    title("HASHMAPS");

    let mut general_map: HashMap<&str, i8> = HashMap::new();
    general_map.insert("test", 25);

    match general_map.get("test") {
        None => println!["It failed!"],
        Some(result) => println!["Here is the result: {}", result]
    };

    let result = match general_map.get("should fail") {
        None => match general_map.get("test") {
            None => "Nothing found for either key".to_string(),
            Some(found) => format!["Found {} for 'test', but 'testing' failed", found]
        }
        Some(found) => format!["Got original key: {}", found]
    };
    println!["{}", result];
}

fn results_and_errors() {
    title("RESULTS & ERRORS");

    fn error_check(check: bool) -> Result<i8, &'static str> {
        if check == true {
            Err("this is an error")
        } else {
            Ok(1)
        }
    }

    fn describe_result(result: Result<i8, &'static str>) {
        match result {
            Ok(x) => println!["it's a result of: {}", x],
            Err(x) => println!["{}", x]
        }
    }

    let result = error_check(false);
    describe_result(result);

    let caught_result = error_check(true);
    if caught_result.is_err() {
        println!["Error was caught"];
    }
}

fn scopes_with_strings() {
    title("SCOPES with Strings");

    let one = String::from("one");
    {
        println!["{}", one];
        let two = String::from("two");
        println!["Two is in scope here: {}", two];
    }
    println!["But only `one` remains in scope here: {}", one];
    // Cannot do, as `two` is out of scope:
    // println!["{}", two];

    fn print_number(number: String) {
        // This function takes ownership of number, and so it will be dropped after it ends
        println!["{}", number];
    }

    print_number(one); // `one` is moved here, and so is inaccessible beyond.
    // so this println! can't be called
    // println!["{}", one];

    fn print_borrowed_number(borrowed_number: &String) {
        println!["{}", borrowed_number];
    }
    let borrowed_one = String::from("borrowed one");
    print_borrowed_number(&borrowed_one);
    print_borrowed_number(&borrowed_one);
    println!["After borrowing twice: {}", borrowed_one];

    fn alter_number(number: &mut String) {
        number.push('!');
    }

    let mut altered_one = String::from("altered one");
    print_borrowed_number(&altered_one);
    alter_number(&mut altered_one);
    print_borrowed_number(&altered_one);
    println!["Before moving altered_one: {}", altered_one];
    print_number(altered_one); // and altered_one is no more, as it's been moved here.
}

fn scopes_with_numbers() {
    title("SCOPES with Numbers");

    fn alter_number(n: &mut i8) {
        *n += 1; // Need to de-reference the number with * since it's copied.  Want to increment the VALUE, not the reference.
    }

    fn print_number(n: i8) {
        println!["Numbers have copy trait and so are not moved here: {}", n];
    }

    let mut two = 2i8;
    print_number(two);
    alter_number(&mut two);
    print_number(two);
}

fn lifetimes() {
    title("LIFETIMES");

    let _one;
    {
        let two = 2i8;
        _one = &two;
    } // The lifetime of `two` ends here, and so the reference stored by `one` is deleted here.
    // so this will cause an error as `one`'s borrowed value of `two` is attempted
    // to be used AFTER the lifetime expired.
    // println!["r: {}", one];

    fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
        // Here the lifetimes are denoted explicitly with the 'a ("tick a").  Can be any text, but
        // simple a, b, c, ... are convention
        // Here the 'a tells the compiler that both numbers must be present for the duration of the function,
        // and the returned result will live equally as long.
        if first_number > second_number {
            return first_number;
        }
        second_number
    }

    let one = 1i8;
    {
        let two = 2i8;
        let outcome = get_highest(&one, &two);
        println!["{}", outcome];
    }

    fn filter<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {
        // Here, the returned result must live as long as the first_number input,
        // which does not necessarily need to be the same as the second_number input.
        if first_number < second_number {
            &0
        } else {
            first_number
        }
    }
    { // This will be lifetime 'a
        let filter_one = 1i8;
        let outcome: &i8;
        { // and this will be lifetime 'b
            let filter_two = 2i8;
            outcome = filter(&filter_one, &filter_two);
            // While filter_two goes out of scope here ('b), the `outcome` has the same lifetime as
            // `filter_one`, so it will persist.
        }
        println!["Filtered outcome persists outside the 'b lifetime block: {}", outcome];
    } // And here, that filter_one is out of scope, so `outcome`'s lifetime has also ended.
}

fn structs() {
    title("STRUCTS");

    struct Human {
        name: String,
        age: i8,
        current_thought: String,
    }

    impl Human {
        fn new(input_name: &str, input_age: i8) -> Self { // Can return `Human` -OR- `Self`, which is understood to be the struct `Human`
            Human {
                name: input_name.to_string(),
                age: input_age,
                current_thought: String::from("nothing"),
            }
        }

        fn with_thought(mut self, thought: &str) -> Self {
            self.current_thought = thought.to_string();
            self
        }

        fn speak(&self) {
            println!["Hello, my name is {} and I'm {} years old.", self.name, self.age];
        }
    }

    let developer = Human::new("Christopher Wahl", 36);
    developer.speak();
    println!["Current thought: {}", developer.current_thought];

    let other_dev = Human::new("John", 41).with_thought("I'm hungry");
    other_dev.speak();
    println!["Other dev thinks: {}", other_dev.current_thought];
}

fn hashmaps_revisited() {
    title("HASHMAPS Revisited");

    enum AllowedData {
        S(String), I(i8)
    }

    struct CustomMap {
        body: HashMap<String, AllowedData>
    }

    impl CustomMap {
        fn new() -> CustomMap {
            CustomMap {
                body: HashMap::new()
            }
        }

        fn get(&self, key: &str) -> Option<&AllowedData> {
            self.body.get(key)
        }
        fn insert(&mut self, key: &str, value: AllowedData) -> () { // `()` is a void return type, and is the same as not annotating at all
            self.body.insert(key.to_string(), value);
        }

        fn display(&self, key: &str) {
            match self.get(key) {
                Some(AllowedData::I(value)) => println!["i8: {}", value],
                Some(AllowedData::S(value)) => println!["String: {}", value],
                None => println!["No entry in map for key: {}", key]
            };
        }
    }

    let mut map = CustomMap::new();
    map.insert("test", AllowedData::I(8));
    map.insert("testing", AllowedData::S("test value!".to_string()));
    map.display("test");
    map.display("testing");
    map.display("Should fail!");
}