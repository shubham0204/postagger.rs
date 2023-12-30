package pos.tagger;
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
            "/home/shubham/Rust_Projects/postagger-rs/tagger/weights.json", 
            "/home/shubham/Rust_Projects/postagger-rs/tagger/tags.json", 
            "/home/shubham/Rust_Projects/postagger-rs/tagger/classes.txt"
        ) ; 
        List<POSTag> tags = tagger.tag( "the quick brown fox jumps over the lazy dog" ) ;
        for( POSTag tag : tags ) {
            System.out.println( tag.getWord() + " " + tag.getTag() ) ;
        }
        System.out.println( "Done 🚀" ) ;
        
    }

}
