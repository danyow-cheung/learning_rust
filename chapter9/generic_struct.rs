impl <T> Queue <T>{
    // self.older:Vec<char>, //old element ,eldest last
    // self.younger:Vec<char>

    pub fn new()->Queue<T>{
        Queue{older:Vec::new(),younger:Vec::new()}
    }
    pub fn push(&mut self,t:T){
        self.younger.push(t);
    }

    pub fn is_empty(&self)->bool{
        self.older.is_empty() && self.younger.is_empty()

    }
    pub fn new()->Queue{
        Queue{older:Vec::new(),younger:Vec::new()}
    }
}

fn main(){
    let mut q = Queue::<char>::new();
    // 上面可以，下面也可以，让rust决定你的代码
    let mut q = Queue::new();
    let mut r = Queue::new();
    q.push("CAD");  // apparently a Queue<&'static str>
    r.push(0.74);   // apparently a Queue<f64>
    q.push("BTC");  // Bitcoins per USD, 2017-5
    r.push(2737.7); // Rust fails to detect irrational exuberanc

}