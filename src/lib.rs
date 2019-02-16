#[cfg(test)]
mod core {
    /// this is the principal rendering interface
    pub trait Renderer{

        /// width and height are the dimensions of the screen to create
        /// implementors of this module need to ensure that the method is idempotent.
        /// when the method is called a second time with different parameters, 
        /// then the implementors musst ensure that the deinitialize method (drop) is beeing issued before! 
        fn initialize(width: i32, height i32) -> Option<String>;

        /// 
        fn initTexture()
    } 
}
