pub mod perceptron_tagger;
use self::perceptron_tagger::PerceptronTagger;
use serde_json ;

fn main() {
    let tagger = PerceptronTagger::new( "tagger/weights.json" , "tagger/classes.txt" , "tagger/tags.json" )  ; 
    let tags = tagger.tag( "shubham was good" ) ;
    for tag in &tags {
        println!( "{} {} {}" , tag.word , tag.tag , tag.conf ) ; 
    }
}
