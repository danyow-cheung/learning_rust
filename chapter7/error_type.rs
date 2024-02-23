// json/src/error.js

#[drive(Debug.Clone)]
pub struct JsonError{
    pub message:String,
    pub line:usize,
    pub column:usize,
}
