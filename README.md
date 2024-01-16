![postagger-banner](https://github.com/shubham0204/postagger.rs/assets/41076823/5c818699-bf20-4589-a433-16edc5f4b1b1)

# Parts of Speech Tagging - Perceptron Tagger in Rust

[![POSTagger crate](https://img.shields.io/crates/v/postagger.svg)](https://crates.io/crates/postagger)
[![POSTagger documentation](https://docs.rs/postagger/badge.svg)](https://docs.rs/postagger)

> A simple, cross-platform, NLTK-inspired averaged perceptron tagger written in Rust

**Contents**

1. [Usage](#usage)
    1. [Usage in Rust](#usage-in-rust)
    2. [Usage in C](#usage-in-c)
    3. [Usage in Java](#usage-in-java)
2. [Useful External Resources](#useful-external-resources)

## Usage

### Usage in Rust

```rust
use postagger::PerceptronTagger;

fn main() {
    let tagger = PerceptronTagger::new( "tagger/weights.json" , "tagger/classes.txt" , "tagger/tags.json" )  ; 
    let tags = tagger.tag( "the quick brown fox jumps over the lazy dog" ) ;
    for tag in &tags {
        println!( "{} {} {}" , tag.word , tag.tag , tag.conf ) ; 
    }
}
````

### Usage in C

#### Generate C headers and shared library

```
$> cbindgen --lang C --output examples/c/postagger.h
$> cargo build --target=x86_64-unknown-linux-gnu --release
```


```c
#include "postagger.h"
#include <stdio.h>
#include <stdlib.h>

int main( int argc , char** argv ) {
    PerceptronTagger* tagger = tagger_create( "tagger/weights.json" , "tagger/classes.txt" , "tagger/tags.json"  )  ;
    const TagResults* results = tagger_annotate( tagger , "the quick brown fox jumps over the lazy dog" ) ; 
    for( int i = 0 ; i < results -> num_tags ; i++ ) {
        printf( "word=%s , tag=%s , conf=%f \n" , results -> tags[i].word , results -> tags[i].tag , results -> tags[i].conf ) ; 
    }
    tagger_release( tagger ) ;
}
```

### Usage in Java

```java
import java.util.List;
import pos.tagger.POSTagger.POSTag;

public class Main {

    public static void main( String[] args ) {

        /*
         * Replace with the absolute paths of 
         * weights, tags and classes
         * See `tagger` directory at the root of the repository for these files
         */
        POSTagger tagger = new POSTagger(
            "weights.json", 
            "tags.json", 
            "classes.txt"
        ) ; 
        List<POSTag> tags = tagger.tag( "the quick brown fox jumps over the lazy dog" ) ;
        for( POSTag tag : tags ) {
            System.out.println( tag.getWord() + " " + tag.getTag() ) ;
        }
    }

}
```

## Useful External Resources

- [`perceptron.py` on NLTK](https://github.com/nltk/nltk/blob/develop/nltk/tag/perceptron.py)
- [A Good Part-of-Speech Tagger in about 200 Lines of Python](https://explosion.ai/blog/part-of-speech-pos-tagger-in-python)
