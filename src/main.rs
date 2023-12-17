pub mod perceptron_tagger;
use self::perceptron_tagger::PerceptronTagger;

fn main() {
    let tagger = PerceptronTagger::new( "tagger/weights.json" , "tagger/classes.txt" , "tagger/tags.json" )  ; 
    let tags = tagger.tag( "shubham was good" ) ;
    for tag in &tags {
        println!( "{} {} {}" , tag.0 , tag.1 , tag.2 ) ; 
    }
}
