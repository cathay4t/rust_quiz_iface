// SPDX-License-Identifier: Apache-2.0

trait GetState {
    fn get_state(&self) -> &str;
}

#[derive(Debug)]
struct Abc;

impl GetState for Abc {
    fn get_state(&self) -> &str {
        "a"
    }
}

#[derive(Debug)]
struct Abd;

impl GetState for Abd {
    fn get_state(&self) -> &str {
        "b"
    }
}

fn foo<T>(data: T)
where
    T: GetState + std::fmt::Debug,
{
    println!("HAHA {:?}", data);
    println!("{}", data.get_state())
}

fn main() {
    let data1 = Abc {};
    let data2 = Abd {};

    foo(data1);
    foo(data2);
}
