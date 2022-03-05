use std::collections::HashMap;
use std::fmt;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1,1);
}

fn function1() -> fmt::Result {}
//fn function2() -> io::Result<()> {}
fn function2() -> IoResult<()> {}
