# pos-tagging

A Rust implementation of the the Viterbi algorithm for part-of-speech tagging.

# Usage
The executable this project produces has the capability to...
* [`train`](#Train) a model, saves it to a file for future use, and optionally evaluates the model on some data
* [`evaluate`](#Evaluate) a pre-trained model on some data
* [`predict`](#Predict) the POS tagging of some sentnces using a pre-trained model either from standard input or a file

These functionalities correspond to the [`train`](#Train), [`evaluate`](#Evaluate), and [`predict`](#Predict) subcommands, respectively. The following blocks indicate how to use each subcommand.

```
A Rust implementation of the the Viterbi algorithm for part-of-speech tagging.

Usage: pos-tagger <COMMAND>

Commands:
  train     Trains a model, saves it to a file for future use, and optionally evaluates the model on some data
  evaluate  Evaluate a pre-trained model on some data
  predict   Predict the POS tagging of some sentnces using a pre-trained model either from standard input or from a file
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Train
```
Trains a model, saves it to a file for future use, and optionally evaluates the model on some data

Usage: pos-tagger train [OPTIONS] -d <DATA_FILES> -o <OUT_FILE>

Options:
  -d <DATA_FILES>      Paths to all of the data files used to train the model
  -o <OUT_FILE>        The path to save the trained model to
  -e <EVAL_FILE>       The path to a data file to evaluate the model
```

## Evaluate
```
Evaluate a pre-trained model on some data

Usage: pos-tagger evaluate -m <MODEL_FILE> -e <EVAL_FILE>

Options:
  -m <MODEL_FILE>      The path to the saved pre-trained model
  -e <EVAL_FILE>       The path to a data file to evaluate the model
```

## Predict
```
Predict the POS tagging of some sentnces using a pre-trained model either from standard input or from a file

Usage: pos-tagger predict [OPTIONS] -m <MODEL_FILE>

Options:
  -m <MODEL_FILE>        The path to the saved pre-trained model
  -p <PREDICT_FILE>      The path to a data file of sentences to predict with. Defaults to STDIN if not specified
```
