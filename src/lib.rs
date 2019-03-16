pub mod core {
    /// this is the principal rendering interface
    pub trait IRenderer{



        /// loads texture into the graphicssystem, and returns id of it
        fn create_texture(&self, filename: &str) -> Result<u32,String>;

        fn paint_rect2d(&self,x: f32,y: f32,width: f32, height: f32,red: u8, green: u8, blue: u8, alpha: u8)-> Option<String>;


        // displays a texture at the target_x and target_y coordinates
        fn draw_texture2d(  &self,
                            texture_id   : u32, 
                            source_x     : u32,
                            source_y     : u32, 
                            source_width : u32, 
                            source_height: u32, 
                            target_x     : f32, 
                            target_y     : f32, 
                            target_width : f32, 
                            target_height: f32) -> Option<String>;

        fn clear(&self,red: u8, green: u8, blue: u8)-> Option<String>; 
    } 
}
