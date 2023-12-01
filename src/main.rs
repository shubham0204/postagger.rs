
use std::{fs::read_to_string, collections::HashMap};
use serde_json as json;

struct AveragedPerceptron {
    feature_weights: HashMap<String,HashMap<String,f32>> , 
    classes: Vec<String>
}

impl AveragedPerceptron {

    pub fn new(
        weights_filepath: &str , 
        classes_filepath: &str
    ) -> AveragedPerceptron {

        // Read weights for each feature
        // from the JSON file
        // The key indicates the feature name, and value, its weight
        let mut feature_weights: HashMap<String,HashMap<String,f32>> = HashMap::new() ;
        let weights_str: String = read_to_string( weights_filepath ).expect( "Could not read weights.json" ) ;
        let weights_json: json::Value = json::from_str( weights_str.as_str() ).expect( "Could not convert weights.json to json::Value" ) ;
        for ( feature_name , value ) in weights_json.as_object().unwrap() {
            let mut weights: HashMap<String,f32> = HashMap::new() ; 
            for ( tag , weight ) in value.as_object().unwrap() {
                weights.insert( tag.to_string() , weight.as_f64().unwrap() as f32 ) ; 
            }
            feature_weights.insert( feature_name.to_string() , weights ) ;
        }

        // Read names of classes from the text file
        // Each line contains a single class name
        let classes_str = read_to_string( classes_filepath ).expect( "Could not read classes.txt as string") ; 
        let classes: Vec<String> = classes_str.split( '\n' )
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
        for ( feature , value ) in word_features {
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
        let exp_scores: Vec<f32> = scores.values().map( |v| v.exp() ).collect() ; 
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

    pub fn new(
        weights_filepath: &str , 
        classes_filepath: &str , 
        tags_filepath: &str
    ) -> PerceptronTagger {

        // Read tags from the JSON file
        // The key represents a word and the value its POS-tag
        let mut tags: HashMap<String,String> = HashMap::new() ; 
        let tags_str: String = read_to_string( tags_filepath ).expect( "Could not read tags.json" ) ;
        let tags_json: json::Value = json::from_str( &tags_str ).expect( "Could not convert tags.json to json::Value" ) ;
        for ( word , tag ) in tags_json.as_object().unwrap() {
            tags.insert( word.to_string() , tag.to_string() ) ;
        }

        PerceptronTagger { 
            model: AveragedPerceptron::new( weights_filepath , classes_filepath ) , 
            tags
         }
    }

    pub fn tag<'a>(
        &'a self , 
        sentence: &'a str
    ) -> Vec<(&'a str,String,f32)> {
        self.assign_tags( sentence.split_whitespace().collect::<Vec<&str>>() )
    }

    fn assign_tags<'a>(
        &'a self ,
        tokens: Vec<&'a str>
    ) -> Vec<(&'a str,String,f32)> {

        let mut prev: &str = "-START-" ; 
        let mut prev2: &str = "-START2-" ; 
        let mut output: Vec<(&str,String,f32)> = Vec::new() ; 

        let mut context: Vec<&str> = Vec::new() ; 
        context.push( prev ) ; 
        context.push( prev2 ) ; 
        for token in &tokens {
            context.push( 
                if token.contains( "'-'" ) && !token.starts_with( '-' ) {
                    "!HYPHEN"
                }
                else if token.parse::<usize>().is_ok() && token.len() == 4 {
                    "!YEAR"
                }
                else if token[0..1].parse::<usize>().is_ok() {
                    "!DIGITS" 
                }
                else {
                    token
                }
            ) ; 
        }

        context.push( "-END-" ) ; 
        context.push( "-END2-" ) ; 

        for ( i , token ) in tokens.into_iter().enumerate() {

            if self.tags.get( token ).is_none() {
                let features = Self::get_features( i + 2 , token, &context, prev, prev2 ) ; 
                let (tag , conf) = self.model.predict( features ) ; 
                output.push( ( token , tag.to_string() , conf ) ) ; 
                prev2 = prev ; 
                prev = tag ; 
            }
            else {
                output.push( ( token , self.tags.get( token ).unwrap().clone() , 1.0 ) ) ; 
                prev2 = prev ; 
                prev = self.tags.get( token ).unwrap() ; 
            }

        }

        output
    }

    fn get_features<'a>(
        i: usize , 
        word: &'a str ,  
        context: &'a [&'a str] ,
        prev: &'a str , 
        prev2: &'a str
    ) -> HashMap<String,usize> {

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

}

fn main() {
    let tagger = PerceptronTagger::new( "weights.json" , "classes.txt" , "tags.json" )  ; 

    let tags = tagger.tag( "shubham was a good boy" ) ;
    for tag in &tags {
        println!( "{} {} {}" , tag.0 , tag.1 , tag.2 ) ; 
    }
}
