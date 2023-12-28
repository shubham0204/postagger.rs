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