
use std::{fs::read_to_string, collections::HashMap};
use serde_json as json;

struct AveragedPerceptron {
    feature_weights: HashMap<String,HashMap<String,f32>> , 
    classes: Vec<String>
}

impl AveragedPerceptron {

    pub fn new() -> AveragedPerceptron {
        let mut feature_weights: HashMap<String,HashMap<String,f32>> = HashMap::new() ; 
        let weights_str: String = read_to_string( "weights.json" ).expect( "Could not read weights.json" ) ;
        let output: json::Value = json::from_str( weights_str.as_str() ).expect( "Could not convert weights.json to Value" ) ;
        for ( feature_name , value ) in output.as_object().unwrap() {
            let mut weights: HashMap<String,f32> = HashMap::new() ; 
            for ( tag , weight ) in value.as_object().unwrap() {
                weights.insert( tag.to_string() , weight.as_f64().unwrap() as f32 ) ; 
            }
            feature_weights.insert( feature_name.to_string() , weights ) ;
        }
        let classes_str = read_to_string( "classes.txt" ).expect( "Could not read classes.txt as string") ; 
        let classes: Vec<String> = classes_str.split( "\n" )
                                                  .map( |class| class.trim().to_string() )  
                                                  .collect() ;
        AveragedPerceptron { 
            feature_weights, 
            classes
        }
    }

    pub fn predict(
        &self ,
        word_features: HashMap<String,usize>
    ) -> (&str , f32) {

        let mut scores: HashMap<&str,f32> = HashMap::new() ; 
        for ( feature , value ) in word_features.into_iter() {
            if self.feature_weights.contains_key( feature.as_str() ) && value != 0 {
                let weights = self.feature_weights.get( feature.as_str() ).unwrap() ; 
                for ( label , weight ) in weights.iter() {
                    match scores.get( label.as_str() ) {
                        Some( score ) => scores.insert( label ,  score + (weight * ( value as f32 )) ) ,
                        None => scores.insert( label , weight * ( value as f32 ) )
                    } ;
                }
            }
        }
        let class: &str = self.classes.iter()
                            .max_by( | a , b | scores.get(a.as_str()).unwrap_or( &0.0 ).partial_cmp( 
                                               scores.get(b.as_str()).unwrap_or( &0.0 ) ).unwrap() )
                            .unwrap() ;
        let exp_scores: Vec<f32> = scores.iter().map( |( _ , v )| v.exp() ).collect() ; 
        let exp_scores_sum: f32 = exp_scores.iter().sum() ; 
        let softmax_scores: Vec<f32> = exp_scores.iter().map( | score | score / exp_scores_sum ).collect() ; 
        let max_softmax_score: f32 = softmax_scores.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() ;
         
        ( class , max_softmax_score )
    }

}

struct PerceptronTagger {
    model: AveragedPerceptron , 
    tags: HashMap<String,String>
}

impl PerceptronTagger {

    pub fn new() -> PerceptronTagger {
        let mut tags: HashMap<String,String> = HashMap::new() ; 
        let tags_str: String = read_to_string( "tags.json" ).expect( "Could not read tags.json" ) ;
        let output: json::Value = json::from_str( &tags_str ).expect( "Could not convert tags.json to Value" ) ;
        for ( word , tag ) in output.as_object().unwrap().into_iter() {
            tags.insert( word.to_string() , tag.to_string() ) ;
        }
        PerceptronTagger { 
            model: AveragedPerceptron::new() , 
            tags
         }
    }

    fn get_features<'a>(
        &self , 
        i: usize , 
        word: &str ,  
        context: &Vec<String> ,
        prev: &'a str , 
        prev2: &'a str
    ) -> HashMap<String,usize> {

        let i = i + 2 ; 

        let mut features: HashMap<String,usize> = HashMap::new() ; 
        features.insert( "bias".to_owned() , 1 ) ; 
        features.insert( format!( "{} {}" , "i suffix" , &word[ (word.len() - 3).. ] ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i pref1" , &word.chars().nth(1).unwrap() ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i-1 tag" , prev ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i-2 tag" , prev2 ) , 1 ) ; 
        features.insert( format!( "{} {} {}" , "i tag+i-2 tag" , prev , prev2 ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i word" , context[i] ) , 1 ) ; 

        features.insert( format!( "{} {} {}" , "i-1 tag+i word" , prev , context[i] ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i-1 word" , context[i - 1] ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i-2 word" , context[i - 2] ) , 1 ) ;
        features.insert( format!( "{} {}" , "i+1 word" , context[i + 1] ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i+2 word" , context[i + 2] ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i+1 suffix" , &context[i + 1][ (context[i + 1].len() - 3).. ] ) , 1 ) ; 
        features.insert( format!( "{} {}" , "i-1 suffix" , &context[i - 1][ (context[i + 1].len() - 3).. ] ) , 1 ) ;
    
        features
    }

    pub fn tag(
        &self ,
        tokens: Vec<&str>
    ) -> Vec<(String,String,f32)> {

        let mut prev: &str = "-START-" ; 
        let mut prev2: &str = "-START2-" ; 
        let mut output: Vec<(String,String,f32)> = Vec::new() ; 

        let mut context: Vec<String> = Vec::new() ; 
        context.push( "-START-".to_owned() ) ; 
        context.push( "-START2-".to_owned() ) ; 
        for token in tokens.iter() {
            context.push( self.normalize( token ) ) 
        }

        context.push( "-END-".to_owned() ) ; 
        context.push( "-END2-".to_owned() ) ; 

        let mut i: usize = 0 ; 
        for token in tokens.iter() {

            if self.tags.get( token.to_owned() ).is_none() {
                let features = self.get_features( i, token, &context, prev, prev2) ; 
                let (tag , conf) = self.model.predict( features ) ; 
                output.push( ( token.to_string() , tag.to_string() , conf ) ) ; 
                prev2 = prev ; 
                prev = tag ; 
            }
            else {
                output.push( ( token.to_string() , self.tags.get( token.to_string().as_str() ).unwrap().to_owned() , 1.0 ) ) ; 
                prev2 = prev ; 
                prev = self.tags.get( token.to_owned() ).unwrap() ; 
            }

            i += 1 ; 
        }

        output
    }

    fn normalize(
        &self ,
        word: &str
    ) -> String {
        if word.contains( "-" ) && word.chars().nth(0).unwrap() != '-' {
            return String::from( "!HYPHEN" )
        }
        if word.parse::<usize>().is_ok() && word.len() == 4 {
            return String::from( "!YEAR" ) 
        }
        if word[0..1].parse::<usize>().is_ok() {
            return String::from( "!DIGITS" )
        }
        word.to_lowercase()
    }

}

fn main() {
    let tagger = PerceptronTagger::new() ; 
    let tags = tagger.tag( vec![ "Shubham" , "was" , "a" , "good" , "boy" ] ) ;
    for tag in tags.iter() {
        println!( "{} {} {}" , tag.0 , tag.1 , tag.2 ) ; 
    }

}
