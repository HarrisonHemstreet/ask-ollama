use std::env;
use std::io::{self, Write};
use std::process::{Command, Output};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut model = String::from("mistral");
    let mut question =
        String::from("Tell me that I forgot to ask you a question. Ask me to ask you a question.");

    // Starting from the second argument (index 1), as the first is the program name
    let mut args_iter = args.iter().skip(1);
    let mut model_specified = false;

    // Check if the first argument specifies the model
    if let Some(first_arg) = args_iter.next() {
        if first_arg.starts_with("--model=") {
            let model_parts: Vec<&str> = first_arg.splitn(2, '=').collect();
            if model_parts.len() == 2 {
                model = model_parts[1].to_string();
                model_specified = true;
            }
        } else {
            // If the first argument is not a model specifier, treat it as part of the question
            question.push_str(first_arg);
            question.push(' ');
        }
    }

    let mut new_question = String::new();

    // Append the rest of the arguments to form the question
    for arg in args_iter {
        if !model_specified || arg != &model {
            new_question.push_str(arg);
            new_question.push(' ');
        }
    }

    if new_question.len() > 0 {
        question = new_question;
    }

    question = question.trim().to_string();

    // Check if Ollama is installed and install it if not
    if !ollama_installed() {
        println!("Ollama is not installed. Installing Ollama...");
        install_ollama().expect("Failed to install Ollama");
    }

    // Run the command with the specified model and question
    run_ollama(&model, &question).expect("Failed to run Ollama");
}

fn ollama_installed() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("command -v ollama")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_ollama() -> Result<Output, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg("curl https://ollama.ai/install.sh | sh")
        .output()
}

fn run_ollama(model: &str, question: &str) -> Result<Output, std::io::Error> {
    let output = Command::new("ollama")
        .arg("run")
        .arg(model)
        .arg(question)
        .output()?;

    // Check if the command execution was successful
    if output.status.success() {
        // If successful, print stdout and stderr
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
    } else {
        // If failed, print an error message
        eprintln!("Ollama command failed to execute");
    }

    Ok(output)
}
