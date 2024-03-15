use std::rc::Rc ;


pub struct SpiderRobot{
    species:String,
    web_enabled:bool,
    leg_devices: [fd::FileDesc; 8],
}

// 机器人的每个主要系统都由其他结构处理，每个结构都有一个指针返回到蜘蛛机
pub struct SpiderSenses{
    rebot:Rc<SpiderRobot>, // - pointer to settings and I/O
    eyes:[Camera;32],
    motion:Accelerometer,
}


