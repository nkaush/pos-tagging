use std::error::Error;
use pos_tagging::hmm;

fn main() -> Result<(), Box<dyn Error>> {
    let model = hmm::POSTaggingHMMTrainer::new()
        .train("data/brown-training.txt".into())?
        .finalize()?;
    
    hmm::evaluate(&model, "data/brown-dev.txt".into())?;

    Ok(())
}
