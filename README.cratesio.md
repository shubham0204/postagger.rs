# Parts of Speech Tagging - Perceptron Tagger in Rust
> NLTK-inspired PerceptronTagger in Rust

## Usage

### 1. Adding the `postagger` crate in `Cargo.toml`

The [`postagger`](https://crates.io/crates/postagger) crate is available on `crates.io` which can be added with,

```toml
postagger = "<latest-version>"
```

or

```bash
$> cargo add postagger
```

### 2. Download required files for the tagger

Download `weights.json`, `classes.txt` and `tags.json` from the [`tagger` directory](https://github.com/shubham0204/postagger.rs/tree/main/tagger) in the GitHub repo.

### 3. Use `PerceptronTagger` to perform POS tagging

Create an instance of `PerceptronTagger`, which requires file-paths to the three files downloaded in step-2. Then, use `PerceptronTagger::tag` function which returns POS tags for all words present in the sentence as a `Vec<Tag>`.

Each `Tag` contains three attributes,
* `word`: the word for which the tag was predicted
* `tag`: the POS tag of the word. Check the [list of all possible NLTK POS tags](https://stackoverflow.com/questions/15388831/what-are-all-possible-pos-tags-of-nltk)
* `conf`: the confidence in the prediction of the POS tag

```rust
fn main() {
    let tagger = PerceptronTagger::new( "tagger/weights.json" , "tagger/classes.txt" , "tagger/tags.json" )  ; 
    let tags = tagger.tag( "the quick brown fox jumps over the lazy dog" ) ;
    for tag in &tags {
        println!( "{} {} {}" , tag.word , tag.tag , tag.conf ) ; 
    }
}
```

