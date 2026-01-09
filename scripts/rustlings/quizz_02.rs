/* This is a quiz for the following sections:
        - Strings
        - Vecs
        - Move semantics
        - Modules
        - Enums

   Let's build a little machine in the form of a function. As input, we're going to give a list of
   strings and commands. These commands determine what action is going to be applied to the
   string. It can either be:
        - Uppercase the string
        - Trim the string
        - Append "bar" to the string a specific amount of times

   The exact for of this will be:
        - The input is going to be a `Vector` of 2-length tuples: (string, command).
        - The output element is going to be a vector of strings.
*/

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for (str_in, com_in) in input {
            let str_out = match com_in {
                Command::Uppercase => String::from(str_in.to_uppercase()),
                Command::Trim => String::from(str_in.trim()),
                Command::Append(n) => {
                    let mut s = str_in;
                    for _ in 0..n {
                        s.push_str("bar")
                    }
                    s.to_string()
                },
            };
            output.push(str_out);
        }
        output
    }
}

fn main() {
    let input = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];
    let output = my_module::transformer(input);

    dbg!(&output);

    assert_eq!(
        output,
        [
            "HELLO",
            "all roads lead to rome!",
            "foobar",
            "barbarbarbarbarbar",
        ]
    );

    println!("You passed the quizz!")
}
