// a last-in ,first out queue of characters 
pub struct Queue{
    older:Vec<char>, //old element ,eldest last
    younger:Vec<char>//younger elements,younger last
}

impl Queue{
    // push a character onto the back of a queue 
    pub fn push(&mut self,c:char){
        self.younger.push(c);
    }
    //poop a character off the front of a queue ,return "Some(c) " if there 
    //was a character to pop ,or None if the queue was empty
    pub fn pop(&mut self)->Option<char>{
        if self.older.is_empty(){
            if self.younger.is_empty(){
                return None;
            }

            //bring the elements in younger over to older 
            // and put them in the promised order 
            use std::mem::swap;
            swap(&mut self.older,&mut self.younger);
            self.older.reverse();
        }
        // now older is guaranted to have something vec'pop method 
        //already returns an Option,so we're sest
        self.older.pop()
    }
    pub fn is_empty(&self)->bool{
        self.older.is_empty() &&self.younger,is_empty()
    }
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
    
    pub fn new()->Queue{
        Queue{older:Vec::new(),younger:Vec::new()}
    }

}


fn main(){
    //由于Push and Pop需要修改队列，因此它们都采用和Mut self。但是，当您调用方法时，您不需要自己借用可变的参考。普通的方法调用语法隐含地处理了这一点。因此，有了这些定义，您可以使用这样的队列
    let mut q = Queue{older:Vec::new() ,younger:Vec::new()};
    q.push("0");
    q.push("1");
    assert_eq!(q.pop(),Some("0"));

    q.push("=");
    assert_eq!(q.pop(),Some("1"));
    assert_eq!(q.pop(),Some("="));
    assert_eq!(q.pop(),None);
}