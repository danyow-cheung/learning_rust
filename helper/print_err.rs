use std::error::Error;
use std::io::{Write,stederr};

// dump an error message to 'stderr'
fn print_error(mut err:&Error){
  let _ = writln!(stderr(),"errpr{}",err);
  while let Some(cause) ==err.cause(){
    let _ = writln!(stderr(),"caused by {}",cause);
    err = cause;
  }
}