package pos.tagger;
import java.util.ArrayList;
import java.util.List;
import org.json.*;

public class POSTagger {

    static {
        System.loadLibrary( "postagger" ) ; 
    }

    private long instance = 0L ; 

    public class POSTag {
        private String word ; 
        private String tag ; 
        private float confidence ; 

        public POSTag(
            String word , 
            String tag , 
            float confidence
        ) {
            this.word = word ; 
            this.tag = tag ; 
            this.confidence = confidence ; 
        }

        public String getWord() {
            return word ; 
        }

        public String getTag()  {
            return tag ; 
        }

        public float getConfidence() {
            return confidence ;
        }
    }

    public POSTagger(
        String weightsPath , 
        String tagsPath , 
        String classesPath
    ) {
        this.instance = create(weightsPath, classesPath, tagsPath) ; 
    }

    public List<POSTag> tag(
        String sentence
    ) {
        String tagsJsonString = annotate(instance, sentence) ; 
        JSONArray tagsArray = new JSONArray( tagsJsonString ) ; 
        ArrayList<POSTag> tags = new ArrayList<>() ; 
        for( int i = 0 ; i < tagsArray.length() ; i++ ) {
            JSONObject tagJSON = tagsArray.getJSONObject( i ) ; 
            POSTag tag = new POSTag(
                tagJSON.getString( "word" ), 
                tagJSON.getString( "tag" ),
                tagJSON.getFloat( "conf" )
            ) ;  
            tags.add( tag ) ;
        }
        return tags ;
    }

    private native long create(
        String weightsPath , 
        String classesPath , 
        String tagsPath
    ) ; 

    private native String annotate(
        long instance ,     
        String sentence 
    ) ;

    private native void release(
        long instance
    ) ; 
    

}
