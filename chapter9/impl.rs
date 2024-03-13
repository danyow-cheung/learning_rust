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
}