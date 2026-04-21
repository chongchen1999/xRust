/* 使用三种方法修复下面的错误  */
fn invalid_output1() -> String { 
    String::from("foo") 
}

fn invalid_output2() -> &'static str { 
    &"foo"
}

fn main() {
    invalid_output1();
    invalid_output2();
}