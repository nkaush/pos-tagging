use std::time::Instant;
use std::error::Error;
use pos_tagging::hmm;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    // let model = hmm::POSTaggingHMMTrainer::new()
    //     .train("data/brown-training.txt".into())?
    //     .finalize()?;
    let model = hmm::POSTaggingHMM::from_file("models/brown.bin".into())?;

    let duration = Instant::now() - start;
    println!("Model training took {:.03}s", duration.as_secs_f64());

    model.save("models/brown.bin".into())?;
    
    hmm::evaluate(&model, "data/brown-dev.txt".into())?;

    Ok(())
}
