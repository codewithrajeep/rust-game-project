// io is a module that provides input/output functionality
use std::io;

// using struct over Vec is a better choice for representing a question
struct Question {
    text: String,
    options: Vec<String>,
    answer: String, // just store 1, 2, 3, 4 - easy to compare
}

fn main() {
    // welcome message
    println!("Welcome to the quiz game!");

    // let's set the score for user 0;
    // not using u32 is score can be negative
    // so using default i32 instead
    let mut score = 0;

    // building custome hardcoded Vec questions
    let questions: Vec<Question> = vec![
        // storing answers in numbers like 1, 2, 3, 4
        Question {
            text: "Question 1: Why we use mut in Rust?".to_string(),
            options: vec![
                "It disallows mutable variables".to_string(),
                "It is a keyword".to_string(),
                "It allows mutable variables".to_string(),
                "It is a reserved word".to_string(),
            ],
            answer: "3".to_string(),
        },
        Question {
            text: "Question 2: Why Rust is faster than Javscript?".to_string(),
            options: vec![
                "Rust is compiled to machine code, while Javascript is interpreted".to_string(),
                "Rust has a smaller runtime footprint, while Javascript has a larger one"
                    .to_string(),
                "Rust is faster than Javascript".to_string(),
                "Rust is slower than Javascript".to_string(),
            ],
            answer: "1".to_string(),
        },
        Question {
            text: "Question 3: What is the difference between u32 and i32 in Rust?".to_string(),
            options: vec![
                "u32 is a signed 32-bit integer, while i32 is an unsigned 32-bit integer"
                    .to_string(),
                "u32 is an unsigned 32-bit integer, while i32 is a signed 32-bit integer"
                    .to_string(),
                "u32 is a signed 32-bit integer, while i32 is a signed 32-bit integer".to_string(),
                "u32 is an unsigned 32-bit integer, while i32 is an unsigned 32-bit integer"
                    .to_string(),
            ],
            answer: "2".to_string(),
        },
        Question {
            text: "Question 4: What does Crate mean in Rust?".to_string(),
            options: vec![
                "A library or module in Rust".to_string(),
                "A standard library in Rust".to_string(),
                "A compiler in Rust".to_string(),
                "A runtime in Rust".to_string(),
            ],
            answer: "1".to_string(),
        },
        Question {
            text: "Question 5: Why we use Cargo in Rust?".to_string(),
            options: vec![
                "It is a build system for Rust".to_string(),
                "It is a compiler for Rust".to_string(),
                "It is a runtime for Rust".to_string(),
                "It is a package manager for Rust".to_string(),
            ],
            answer: "4".to_string(),
        },
    ];
    // loop through questions using a for loop
    for question in questions {
        println!("{}", question.text);
        for (i, option) in question.options.iter().enumerate() {
            println!("{}: {}", i + 1, option);
        }
        // loop through user input to check if it matches the answer
        loop {
            // getting user input
            println!("Enter your answer: ");
            // getting user input and trimming whitespace and converting to uppercase
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_uppercase();
            // check if input is valid option ("A", "B", "C", "D")
            if input == "1" || input == "2" || input == "3" || input == "4" {
                // if answers is correct, set score with +1 for each correct answer and -1 for each incorrect answer.
                // after all questions are answered, break out of the loop and display the score
                if input == question.answer {
                    println!("Correct!");
                    score += 1;
                } else {
                    println!("Incorrect. Try again.");
                    score -= 1;
                }
                break;
            } else {
                println!("Invalid option. Try again.");
            }
        }
    }
    // display the final score
    println!("Your score: {}", score);
}
