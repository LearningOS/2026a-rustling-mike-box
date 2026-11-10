// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    //数据编译期就存在程序的只读段，整行没有任何堆分配，纯粹是借用。
    string_slice("blue");
    // 把它复制到堆上生成 String，所有权交给 string。
    string("red".to_string());
    // String::from 是构造 String 的写法，数据同样被复制到堆上。
    string(String::from("hi"));
    // to_owned() 对 &str 来说和 to_string() 效果相同，复制出string，强调所有权
    string("rust is fun!".to_owned());
    // 这里会自动进行类型推导
    string_slice("nice weather".into());
    // format! 是格式化字符串的宏，返回一个 String。
    string(format!("Interpolation {}", "Station"));
    // &str 的切片操作，返回一个 &str
    string_slice(&String::from("abc")[0..1]);
    // trim 是 String 的方法，返回的是原字符串的一个子切片，不改变原始字符串的大小
    string_slice("  hello there ".trim());
    // replace 是 String 的方法，返回一个 String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    //  改变原始原始字符串，生成一个新的字符串，因此返回 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// 核心问题
// & str 和 String 的区别
//  &str 是字符串的引用，它是一个指针，指向只读内存，因此不能修改，但是可以读取。
//  String 在堆上构造一个字符串，需要修改字符串，因此需要所有权，因此需要移动所有权。
// 且会重新分配内存，因此会导致性能下降。
