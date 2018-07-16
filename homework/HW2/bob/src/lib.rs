/// Copyright (c) 2018 Jason Graalum
///
/// Function to emulate a lackadasical teenager.
///
pub fn reply(message: &str) -> &str {
    // Did we ask him a question?
    let question: bool = message.trim_right().ends_with('?');

    // Did we yell at him?
    // Yell is defined as has alphabetic characters, but no lower casee
    let yell: bool = message.chars().any(|c| (c >= 'A' && c <= 'Z'))
        && !message.chars().any(|c| (c >= 'a' && c <= 'z'));

    //
    // Teenage replies.
    //
    if message.trim_right().is_empty() {
        "Fine. Be that way!" // Just staring at him
    } else if question && !yell {
        "Sure." // Just a question
    } else if !question && yell {
        "Whoa, chill out!" // Yelling
    } else if question && yell {
        "Calm down, I know what I'm doing!" // Yelling a question
    } else {
        "Whatever." // Something else
    }
}
