use pos_tagging::hmm::*;
use std::error::Error;

#[test]
fn test_it_all_works() -> Result<(), Box<dyn Error>> {
    let model = POSTaggingHMMTrainer::new()
        .train("tests/data/mttest-training.txt".into())?
        .finalize();

    assert!(model.is_ok());
    Ok(())
}

#[test]
fn test_training_works() -> Result<(), Box<dyn Error>> {
    let trainer = POSTaggingHMMTrainer::new()
        .train("tests/data/mttest-training.txt".into())?;
    
    println!("{:?}", trainer);
    
    Ok(())
}
