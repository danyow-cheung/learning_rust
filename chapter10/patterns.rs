enum RoughTime{
    InThePast(TimeUnit,u32),
    JustNow,
    InTheFuture(TimeUnit,u32)
}

fn rough_time_to_englist(rt:RoughTime)->String{
    match rs {
        RoughTime::InThePast(units,count)=>format!("{}{} ago",count,units.plural()),
        RoughTime::JustNow=>format!("just now"),
        RoughTime::InTheFuture(units,count)=>format!("{} {} from now",count,units.plural())
    }
}


// match执行模式匹配；在本例中，图案是出现在第3、5和7行=>符号之前的部分。与RoughTime值匹配的模式看起来就像用于创建RoughTime的表达式。这绝非巧合。表达式产生值；模式消耗价值。两者使用了很多相同的语法
