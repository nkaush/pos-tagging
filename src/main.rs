use std::{path::PathBuf, time::Instant, error::Error, fs::File, io::{self, BufReader, Write}};
use clap::{Args, Parser, Subcommand};
use indicatif::ProgressIterator;
use pos_tagger::hmm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Trains a model, saves it to a file for future use, and optionally 
    /// evaluates the model on some data.
    Train(TrainArgs),
    /// Evaluate a pre-trained model on some data.
    Evaluate(EvaluateArgs),
    /// Predict the POS tagging of some sentnces using a pre-trained model 
    /// either from standard input or from a file.
    Predict(PredictArgs)
}

#[derive(Args)]
struct TrainArgs {
    /// Paths to all of the data files used to train the model.
    #[arg(short, required=true)]
    data_files: Vec<PathBuf>,
    /// The path to save the trained model to.
    #[arg(short, required=true)]
    out_file: PathBuf,
    /// The path to a data file to evaluate the model.
    #[arg(short)]
    eval_file: Option<PathBuf>
}

#[derive(Args)]
struct EvaluateArgs {
    /// The path to the saved pre-trained model.
    #[arg(short, required=true)]
    model_file: PathBuf,
    /// The path to a data file to evaluate the model.
    #[arg(short, required=true)]
    eval_file: PathBuf
}

#[derive(Args)]
struct PredictArgs {
    /// The path to the saved pre-trained model.
    #[arg(short, required=true)]
    model_file: PathBuf,
    /// The path to a data file of sentences to predict with. 
    /// Defaults to STDIN if not specified.
    #[arg(short)]
    predict_file: Option<PathBuf>
}

fn print_input_identifier() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn preprocess_sentence(s: &mut String) {
    let mut iter = s.chars().rev();
    let last = iter.next();
    if iter.next() != Some(' ') {
        match last {
            Some('!') | Some('.') | Some('?') => {
                let last = s.pop().unwrap();
                s.push(' ');
                s.push(last);
            },
            _ => ()
        }
    }
}

fn predict_and_fmt(model: &hmm::POSTaggingHMM, sentence: &str) {
    let sentence: Vec<String> = sentence
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let (_, tags): (Vec<_>, Vec<_>) = model.predict(sentence.clone()).into_iter().unzip();

    let output = sentence.into_iter()
        .zip(tags.into_iter())
        .map(|(w, t)| format!("{}={}", w, t))
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", output);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Train(train_args) => {
            let start = Instant::now();
            let train_result = train_args.data_files
                .into_iter()
                .progress()
                .try_fold(hmm::POSTaggingHMMTrainer::new(), |t, f| t.train(f))
                .map(hmm::POSTaggingHMMTrainer::finalize)
                .map(Result::unwrap);

            if let Err(e) = train_result {
                eprintln!("Failed to train model: {e:?}");
                std::process::exit(1);
            }

            let duration = Instant::now() - start;
            println!("Model training took {:.03}s", duration.as_secs_f64());
            
            let model = train_result.unwrap();
            model.save(train_args.out_file)?;

            if let Some(f) = train_args.eval_file {
                hmm::evaluate(&model, f)?;
            }
        },
        Command::Evaluate(eval_args) => {
            let model = hmm::POSTaggingHMM::from_file(eval_args.model_file)?;
            hmm::evaluate(&model, eval_args.eval_file)?;
        },
        Command::Predict(predict_args) => {
            let model = hmm::POSTaggingHMM::from_file(predict_args.model_file)?;
            let is_file = predict_args.predict_file.is_some();
            let mut input: Box<dyn io::BufRead> = match predict_args.predict_file {
                Some(f) => Box::new(BufReader::new(File::open(f)?)),
                None => Box::new(BufReader::new(io::stdin()))
            };

            let mut buf = String::new();
            print_input_identifier();
            while let Ok(n) = input.read_line(&mut buf) {
                if n == 0 { break }

                buf = buf.trim().into();
                if is_file { println!("{}", buf); }

                preprocess_sentence(&mut buf);
                predict_and_fmt(&model, &buf);
                print_input_identifier();
                buf.clear();
            }
        }
    }

    Ok(())
}
