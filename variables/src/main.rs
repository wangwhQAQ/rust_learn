fn main() {
    // mut_ref(); //可变引用与借用
    // let reference_to_nothing = dangle(); // 悬垂指针
    // let reference_to_nothing = dangle_change(); // 悬垂指针_改
    // print!("{}", reference_to_nothing)

    let r1 = Retangle{wideth:50, length: 80};
    let r2 = Retangle{wideth:21, length: 480};
    let r3 = Retangle{wideth:20, length: 20};

    println!("{:#?}", r1);

    println!("{}", r1.can_hold(&r2));

    println!("{}", r1.can_hold(&r3));

    println!("{}", r3.count_area());

}


fn mut_ref () {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用，如果下文有r3的话，r1 r2的作用域就此结束，所以它们不会与r3冲突

    let r3 = &mut s; // 没问题
    println!("{}", r3);
    // println!("{} and {}", r1, r2);  // 这里就会有问题，因为有了r3之后，r1 r2 还在被使用，导致可变引用和借用同时存在
}

// fn dangle() -> &String {
//     let s = String::from("hello_悬垂指针");
//     &s
// }

fn dangle_change() -> String {
    let s = String::from("hello_悬垂指针_改");
    s
}


#[derive(Debug)]
struct Retangle {
    wideth: u32,
    length: u32,
}

impl Retangle {
    fn count_area(&self) -> u32 {
        self.length * self.wideth
    }

    fn can_hold(&self, other :&Retangle) -> bool {
        self.wideth > other.wideth && self.length > other.length
    }
}