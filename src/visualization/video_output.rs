use opencv::core::*;
use opencv::videoio::*;

pub struct video_instance {
    writer : VideoWriter,
}

pub trait video_wrapper {
    fn open( self: &mut Self, 
             file_name : String,
             frame_rate : f64,
             size : Size2i );
    fn add_frame( self: &mut Self, image : &Mat );
    fn close( self: &mut Self );
}

impl video_wrapper for video_instance {
    fn open( self: &mut Self, 
             file_name : String,
             frame_rate : f64,
             size : Size2i ) {
        self.writer = VideoWriter::new( file_name.as_str(),
                                        VideoWriter::fourcc( 'm','p','4','v' ).unwrap(),
                                        frame_rate,
                                        size,
                                        false ).unwrap();
    }

    fn add_frame( self: &mut Self, image : &Mat ) {
        self.writer.write( image );
    }

    fn close( self: &mut Self ) {
        self.writer.release();
    }
}