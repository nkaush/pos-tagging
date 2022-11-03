mod hapax_patterns;
mod trainer;
mod model;
 
pub use trainer::*;
pub use model::*;

use crate::NestedStringCounter;
use crate::nlp::{extract_word_and_tag, TaggedSentence};
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use std::fs::File;
use std::io;

pub fn evaluate(model: &POSTaggingHMM, data_file: PathBuf) -> Result<(), io::Error> {
    let f = File::open(data_file)?;
    let rdr = BufReader::new(f);

    let samples: Vec<String> = rdr.lines().filter_map(Result::ok).collect();
    let (sentences, correct_taggings): (Vec<Vec<String>>, Vec<Vec<String>>) = 
        samples.iter()
            .map(|s| extract_word_and_tag(s))
            .map(Vec::into_iter)
            .map(Iterator::unzip)
            .unzip();

    let predictions: Vec<TaggedSentence> = sentences.into_iter()
        .map(|s| model.predict(s))
        .collect();

    evaluate_accuracies(predictions, correct_taggings);

    Ok(())
}

fn evaluate_accuracies(predictions: Vec<TaggedSentence>, correct_tags: Vec<Vec<String>>) {
    let mut correct_wordtagcounter = NestedStringCounter::new();
    let mut wrong_wordtagcounter = NestedStringCounter::new();

    let mut correct: usize = 0;
    let mut wrong: usize = 0;

    for (predicted, answer) in predictions.into_iter().zip(correct_tags.into_iter()) {
        assert_eq!(predicted.len(), answer.len());

        let (_, pred): (Vec<_>, Vec<_>) = predicted.iter().cloned().unzip();
        if pred != answer {
            println!("{:?}", pred);
            println!("{:?}", answer);
            println!("------------");
        }
        
        for ((word, pred_tag), ans_tag) in predicted.into_iter().zip(answer.into_iter()) {
            if pred_tag == ans_tag {
                correct_wordtagcounter.increment(&word, &ans_tag);
                correct += 1;
            } else {
                wrong_wordtagcounter.increment(&word, &ans_tag);
                wrong += 1;
            }
        }
    } 

    let correct = correct as f64;
    let wrong = wrong as f64;

    let accuracy = (correct / (correct + wrong)) * 100.0;
    println!("Accuracy: {:.03}%", accuracy);
}