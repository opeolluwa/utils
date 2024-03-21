fn main() {
    let security_questions: [&str; 10] = [
        "What is your mother's maiden name?",
        "What is the name of your first pet?",
        "What city were you born in?",
        "What is your favorite movie?",
        "What street did you grow up on?",
        "What is the name of your favorite teacher?",
        "What is your favorite book?",
        "What is your favorite food?",
        "What is the model of your first car?",
        "What is your favorite color?",
    ];

    // Accessing the questions
    for (index, question) in security_questions.iter().enumerate() {
        println!("Question {}: {}", index + 1, question);
    }
}
