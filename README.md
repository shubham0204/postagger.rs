# Parts of Speech Tagging - Perceptron Tagger in Rust

## C API

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

## Java API

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

## Generate C headers and shared library

```
$> cbindgen --lang C --output examples/c/postagger.h
$> cargo build --target=x86_64-unknown-linux-gnu --release
```