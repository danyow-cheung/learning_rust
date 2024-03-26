use std::rc::Rc ;


// pub struct SpiderRobot{
//     species:String,
//     web_enabled:bool,
//     leg_devices: [fd::FileDesc; 8],
// }

// 机器人的每个主要系统都由其他结构处理，每个结构都有一个指针返回到蜘蛛机
pub struct SpiderSenses{
    rebot:Rc<SpiderRobot>, // - pointer to settings and I/O
    eyes:[Camera;32],
    motion:Accelerometer,
}


use std::cell::Cell;
pub struct SpiderRobot{
    species:String,
    web_enabled:bool,
    leg_devices: [fd::FileDesc; 8],
    hardware_error_count : Cell<u32>,
    log_file:Recall<File>,
}

// and then even non-mut methods of SpiderRobot can access that u32, using
// the .get() and .set() methods
impl  SpiderRobot{
    pub fn add_hardware_error(&self){
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n+1);
    }

    //true if any harware errors have been reported 
    pub fn has_hardware_errors(&self)->bool{
        self.hardware_error_count.get()>0
    }

    //write a line to the log file 
    pub fn log(&self,message:str){
        let mut file = self.log_file.borrow_mut();
        writeln!(file,"{}",message).unwrap();
    }
}


// 这很简单，但并不能解决我们的日志记录问题。Cell不让你
// 对共享值调用mut方法。.get（）方法返回中的值的副本
// 因此，只有当T实现了Copy特性时，它才能工作。对于测井，我们需要一个
// 文件可复制，而文件不可复制


// 在这种情况下，正确的工具是RefCell。与Cell＜T＞一样，RefCell＜T＞是一种泛型类型
// 包含一个T类型的值。与Cell不同，RefCell支持借用引用
// 取决于其T值：
