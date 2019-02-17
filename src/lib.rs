mod core {
    /// this is the principal rendering interface
    pub trait Renderer{

        /// width and height are the dimensions of the screen to create
        /// implementors of this module need to ensure that the method is idempotent.
        /// when the method is called a second time with different parameters, 
        /// then the implementors musst ensure that the deinitialize method (drop) is beeing issued before! 
        fn initialize(width: u32, height: u32) -> Option<String>;

        /// loads texture into the graphicssystem, and returns id of it
        fn init_texture(filename: &str) -> Result<u32,String>;

        fn paint_rect(x: f32,y: f32,width: f32, height: f32)-> Option<String>;


        // make it bulk or something
        //fn draw_texture(texture_id: u32, source_x: u32,source_y: u32, source_width: u32, source_height: u32, target_x: f32, target_y: f32, target_width: f32, target_height: f32) -> Option<String>;
    } 
}
