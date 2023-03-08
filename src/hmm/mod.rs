mod trainer;
mod model;
 
pub use trainer::*;
pub use model::*;

use crate::ConditionalStringCounter;
use crate::nlp::{extract_word_and_tag, TaggedSentence};
use indicatif::{ProgressBar, ProgressIterator};
use std::io::{self, BufReader, BufRead};
use std::sync::mpsc::channel;
use std::path::PathBuf;
use std::time::Instant;
use std::fs::File;
use std::thread;

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

    let start = Instant::now();
    let predictions: Vec<TaggedSentence> = sentences.into_iter()
        .map(|s| model.predict(s))
        .progress()
        .collect();
    let duration = Instant::now() - start;
    println!("Model evaluation on {} samples took {:.3}s", correct_taggings.len(), duration.as_secs_f64());

    evaluate_accuracies(predictions, correct_taggings);

    Ok(())
}

pub fn par_evaluate(model: &POSTaggingHMM, data_file: PathBuf) -> Result<(), io::Error> {
    let rdr = BufReader::new(File::open(data_file)?);
    let (sentences, correct_taggings): (Vec<Vec<String>>, Vec<Vec<String>>) = 
        rdr.lines()
            .filter_map(Result::ok)
            .map(|s| extract_word_and_tag(&s))
            .map(Vec::into_iter)
            .map(Iterator::unzip)
            .unzip();

    let start = Instant::now();

    let num_threads = num_cpus::get() - 1;
    let num_predictions = sentences.len();
    let expected_chunk_size = (num_predictions / num_threads) + 1;
    let mut chunks: Vec<Vec<(usize, Vec<String>)>> = vec![Vec::with_capacity(expected_chunk_size); num_threads];
    sentences
        .into_iter()
        .enumerate()
        .for_each(|(i, item)| chunks[i % num_threads].push((i, item)));

    let mut predictions: Vec<_> = Vec::new();
    thread::scope(|s| {
        let (tx, rx) = channel();
        chunks
            .into_iter()
            .for_each(|c| {
                let tx_clone = tx.clone();
                s.spawn(move || {
                    c.into_iter()
                        .map(|(i, s)| (i, model.predict(s)))
                        .for_each(|p| {
                            tx_clone.send(p).unwrap();
                        });
                });
            });
        drop(tx);

        let bar = ProgressBar::new(num_predictions as u64);
        predictions = rx.into_iter()
            .map(|x| { bar.inc(1); x })
            .collect();
        bar.finish_and_clear();
        predictions.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    });

    let duration = Instant::now() - start;
    println!("Model evaluation on {} samples took {:.3}s", correct_taggings.len(), duration.as_secs_f64());

    let predictions = predictions.into_iter().map(|(_, s)| s).collect();
    evaluate_accuracies(predictions, correct_taggings);

    Ok(())
}

fn evaluate_accuracies(predictions: Vec<TaggedSentence>, correct_tags: Vec<Vec<String>>) {
    let mut correct_wordtagcounter = ConditionalStringCounter::new();
    let mut wrong_wordtagcounter = ConditionalStringCounter::new();

    let mut correct: usize = 0;
    let mut wrong: usize = 0;

    for (predicted, answer) in predictions.into_iter().zip(correct_tags.into_iter()) {
        assert_eq!(predicted.len(), answer.len());

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