pub mod perceptron_tagger;

mod c_binding {

    use std::ffi::{ CStr , c_char , c_float , CString } ;
    use crate::perceptron_tagger::PerceptronTagger; 

    #[repr(C)]
    pub struct CTag {
        word: *const u8 , 
        tag: *const u8 , 
        conf: c_float ,
    }

    #[repr(C)]
    pub struct TagResults {
        tags: *const CTag , 
        num_tags: usize
    } 

    unsafe fn get_str(
        c_char_buffer: *const c_char
    ) -> String {
        CStr::from_ptr( c_char_buffer )
              .to_str()
              .expect( "Could not convert weights_filepath to &str" )
              .to_owned()
    }

    #[no_mangle]
    pub extern "C" fn tagger_create( 
        weights_filepath: *const c_char , 
        classes_filepath: *const c_char , 
        tags_filepath: *const c_char 
    ) -> *mut PerceptronTagger  {
        unsafe {
            let fp_weights = get_str( weights_filepath )  ;
            let fp_tags = get_str( tags_filepath ) ; 
            let fp_classes = get_str( classes_filepath ) ;                
            Box::into_raw( 
                Box::new( 
                    PerceptronTagger::new( fp_weights.as_str() , fp_classes.as_str() , fp_tags.as_str() ) 
                ) 
            ) 
        }
    }

    #[no_mangle]
    pub extern "C" fn tagger_annotate(
        tagger_ptr: *mut PerceptronTagger , 
        sentence: *const c_char
    ) -> *const TagResults {
        unsafe {
            let tagger = &*tagger_ptr ; 
            let sentence = get_str( sentence ) ; 
            let tags: Vec<CTag> = tagger.tag( sentence.as_str() )
                .iter()
                .map( |x| {
                        let word = CString::new( x.0 ).expect( "Could not create CString") ; 
                        let word_ptr = word.as_ptr() ; 
                        let tag = CString::new( x.1.as_str() ).expect( "Could not create CString" ) ; 
                        let tag_ptr = tag.as_ptr() ; 
                        std::mem::forget( word ) ; 
                        std::mem::forget( tag ) ;
                        CTag{ 
                            word: word_ptr.cast::<u8>() , 
                            tag: tag_ptr.cast::<u8>() , 
                            conf: x.2 
                        } 
                    }
                ).collect() ; 
            let num_tags = tags.len() ; 
            let tags_ptr = tags.as_ptr() ; 
            std::mem::forget( tags ) ; 
            Box::into_raw( Box::new( TagResults {
                tags: tags_ptr, 
                num_tags
            } ) )
        }
    }

    #[no_mangle]
    pub extern "C" fn tagger_release(
        tagger_ptr: *mut PerceptronTagger
    )  {
        if tagger_ptr.is_null() {
            return;
        }
        unsafe {
            let _ptr = Box::from_raw( tagger_ptr ) ; 
        }
    }

}