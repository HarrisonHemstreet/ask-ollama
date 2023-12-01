use spinners_rs::{Spinner, Spinners};
use std::env;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the first argument is --help
    if args.len() >= 1 && args[1] == "--help" {
        print_help();
        return;
    }

    // Check for --version argument
    if args.len() >= 1 && args[1] == "--version" {
        println!("ask-ollama version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

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

fn print_help() {
    println!("Usage: ask [OPTIONS] [PROMPT]");
    println!("Ask questions to Ollama.");
    println!("\nOptions:");
    println!("  --model=[MODEL]    Specify the model to use. Default is 'mistral' if no model is provided");
    println!("  --version          Show version information");
    println!("  [PROMPT]           The question to ask Ollama");
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
    let loading_msg: String = format!("Running {}...", model);
    let mut spinner = Spinner::new(Spinners::Aesthetic, loading_msg);
    spinner.start();

    let output = Arc::new(Mutex::new(None));
    let output_clone = Arc::clone(&output);

    let model_clone = model.to_string();
    let question_clone = question.to_string();
    thread::spawn(move || {
        let command_output = Command::new("ollama")
            .arg("run")
            .arg(&model_clone)
            .arg(&question_clone)
            .stdout(Stdio::piped()) // Capture stdout
            .stderr(Stdio::piped()) // Capture stderr
            .output();

        let mut output = output_clone.lock().unwrap();
        *output = Some(command_output);
    });

    let mut command_output: Option<Output> = None;
    while command_output.is_none() {
        let output = output.lock().unwrap();
        if let Some(result) = &*output {
            command_output = result.as_ref().ok().cloned();
            break;
        }
        drop(output); // Release the lock before sleeping
        thread::sleep(Duration::from_millis(100));
    }

    spinner.stop();

    let output = command_output.unwrap();
    // Print the captured output
    println!("\r{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(output)
}
