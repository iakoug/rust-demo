// use url::Url;
// use std::io::Read;

// fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     // 解析 URL
//     let url = Url::parse(url)?;

//     // 创建一个 TCP 连接
//     let mut response = std::net::TcpStream::connect(format!("{}:{}",
//         url.host_str().unwrap(),
//         url.port_or_known_default().unwrap_or(80)))?;

//     // 构造 HTTP 请求
//     let request = format!(
//         "GET {} HTTP/1.1\r\n\
//          Host: {}\r\n\
//          Connection: close\r\n\
//          \r\n",
//         url.path(),
//         url.host_str().unwrap()
//     );

//     // 发送请求
//     response.write_all(request.as_bytes())?;

//     // 读取响应
//     let mut buffer = String::new();
//     response.read_to_string(&mut buffer)?;

//     Ok(buffer)
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "http://www.example.com";
//     match fetch_url(url) {
//         Ok(content) => println!("Response:\n{}", content),
//         Err(e) => eprintln!("Error: {}", e),
//     }
//     Ok(())
// }

// use std::{collections::BTreeMap, hash::Hash};

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct Name {
//     pub name: String,
//     pub flags: u32,
// }

// // impl Ord for Name {
// //     fn cmp(&self, other: &Self) -> Ordering {
// //         (self.flags, &self.name).cmp(&(other.flags, &other.name))
// //     }
// // }

// // impl PartialOrd for Name {
// //     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
// //         Some(self.cmp(other))
// //     }
// // }

// // impl PartialEq for Name {
// //     fn eq(&self, other: &Self) -> bool {
// //         (self.flags, &self.name) == (other.flags, &other.name)
// //     }
// // }

// impl Name {
//     pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
//         Self {
//             name: name.as_ref().to_string(),
//             flags,
//         }
//     }
// }

// fn main() {
//     let mut map = BTreeMap::new();
//     map.insert(Name::new("/etc/password", 0x1), 12);
//     map.insert(Name::new("/etc/hosts", 0x1), 4);
//     map.insert(Name::new("/home/tchen", 0x0), 28);

//     for item in map.iter() {
//         println!("{:?}", item);
//     }
// }

// use std::{
//     collections::{hash_map::DefaultHasher, HashMap},
//     hash::{Hash, Hasher},
// };

// // 如果要支持 Hash，可以用 #[derive(Hash)]，前提是每个字段都实现了 Hash
// // 如果要能作为 HashMap 的 key，还需要 PartialEq 和 Eq
// #[derive(Debug, Hash, PartialEq, Eq)]
// struct Student<'a> {
//     name: &'a str,
//     age: u8,
// }

// impl<'a> Student<'a> {
//     pub fn new(name: &'a str, age: u8) -> Self {
//         Self { name, age }
//     }
// }
// fn main() {
//     let mut hasher = DefaultHasher::new();
//     let student = Student::new("Tyr", 18);
//     // 实现了 Hash 的数据结构可以直接调用 hash 方法
//     student.hash(&mut hasher);
//     let mut map = HashMap::new();
//     // 实现了 Hash / PartialEq / Eq 的数据结构可以作为 HashMap 的 key
//     map.insert(student, vec!["Math", "Writing"]);
//     println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
// }
// fn main() {
//     // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T]，所以可以访问 iter()
//     let result = vec![1, 2, 3, 4]
//         .iter()
//         .map(|v| v * v)
//         .filter(|v| *v < 16)
//         .take(2)
//         .collect::<Vec<_>>();

//     println!("{:?}", result);
// }
// use std::{fmt, ops::Deref, str};

// const MINI_STRING_MAX_LEN: usize = 30;

// // MyString 里，String 有 3 个 word，供 24 字节，所以它以 8 字节对齐
// // 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节。
// // MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节.
// struct MiniString {
//     len: u8,
//     data: [u8; MINI_STRING_MAX_LEN],
// }

// impl MiniString {
//     // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
//     fn new(v: impl AsRef<str>) -> Self {
//         let bytes = v.as_ref().as_bytes();
//         // 我们在拷贝内容时一定要使用字符串的字节长度
//         let len = bytes.len();
//         let mut data = [0u8; MINI_STRING_MAX_LEN];
//         data[..len].copy_from_slice(bytes);
//         Self {
//             len: len as u8,
//             data,
//         }
//     }
// }

// impl Deref for MiniString {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         // 由于生成 MiniString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
//         str::from_utf8(&self.data[..self.len as usize]).unwrap()
//         // 也可以直接用 unsafe 版本
//         // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
//     }
// }

// impl fmt::Debug for MiniString {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
//         write!(f, "{}", self.deref())
//     }
// }

// #[derive(Debug)]
// enum MyString {
//     Inline(MiniString),
//     Standard(String),
// }

// // 实现 Deref 接口对两种不同的场景统一得到 &str
// impl Deref for MyString {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         match *self {
//             MyString::Inline(ref v) => v.deref(),
//             MyString::Standard(ref v) => v.deref(),
//         }
//     }
// }

// impl From<&str> for MyString {
//     fn from(s: &str) -> Self {
//         match s.len() > MINI_STRING_MAX_LEN {
//             true => Self::Standard(s.to_owned()),
//             _ => Self::Inline(MiniString::new(s)),
//         }
//     }
// }

// impl fmt::Display for MyString {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.deref())
//     }
// }

// fn main() {
//     let len1 = std::mem::size_of::<MyString>();
//     let len2 = std::mem::size_of::<MiniString>();
//     println!("Len: MyString {}, MiniString {}", len1, len2);

//     let s1: MyString = "hello world".into();
//     let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

//     // debug 输出
//     println!("s1: {:?}, s2: {:?}", s1, s2);
//     // display 输出
//     println!(
//         "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
//         s1,
//         s1.len(),
//         s1.chars().count(),
//         s2,
//         s2.len(),
//         s2.chars().count()
//     );

//     // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
//     assert!(s1.ends_with("world"));
//     assert!(s2.starts_with("这"));
// }

// use std::borrow::Cow;

// use url::Url;
// fn main() {
//     let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
//     let mut pairs = url.query_pairs();

//     assert_eq!(pairs.count(), 3);

//     let (mut k, v) = pairs.next().unwrap();
//     // 因为 k, v 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
//     // 此刻，他们都是 Borrowed
//     println!("key: {}, v: {}", k, v);
//     // 当修改发生时，k 变成 Owned
//     k.to_mut().push_str("_lala");

//     print_pairs((k, v));

//     print_pairs(pairs.next().unwrap());
//     // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
//     // 所以这里 value 是 Owned
//     print_pairs(pairs.next().unwrap());
// }

// fn print_pairs(pair: (Cow<str>, Cow<str>)) {
//     println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
// }

// fn show_cow(cow: Cow<str>) -> String {
//     match cow {
//         Cow::Borrowed(v) => format!("Borrowed {}", v),
//         Cow::Owned(v) => format!("Owned {}", v),
//     }
// }

// use std::alloc::{GlobalAlloc, Layout, System};

// struct MyAllocator;

// unsafe impl GlobalAlloc for MyAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         let data = System.alloc(layout);
//         eprintln!("ALLOC: {:p}, size {}", data, layout.size());
//         data
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         System.dealloc(ptr, layout);
//         eprintln!("FREE: {:p}, size {}", ptr, layout.size());
//     }
// }

// #[global_allocator]
// static GLOBAL: MyAllocator = MyAllocator;

// #[allow(dead_code)]
// struct Matrix {
//     // 使用不规则的数字如 505 可以让 dbg! 的打印很容易分辨出来
//     data: [u8; 505],
// }

// impl Default for Matrix {
//     fn default() -> Self {
//         Self { data: [0; 505] }
//     }
// }

// fn main() {
//     // 在这句执行之前已经有好多内存分配
//     let data = Box::new(Matrix::default());

//     // 输出中有一个 1024 大小的内存分配，是 println! 导致的
//     println!(
//         "!!! allocated memory: {:p}, len: {}",
//         &*data,
//         std::mem::size_of::<Matrix>()
//     );

//     // data 在这里 drop，可以在打印中看到 FREE
//     // 之后还有很多其它内存被释放
// }

// use std::ops::{Deref, DerefMut};

// #[derive(Debug)]
// struct Buffer<T>(Vec<T>);

// impl<T> Buffer<T> {
//     pub fn new(v: impl Into<Vec<T>>) -> Self {
//         Self(v.into())
//     }
// }

// impl<T> Deref for Buffer<T> {
//     type Target = [T];

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for Buffer<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// fn main() {
//     let mut buf = Buffer::new([1, 3, 2, 4]);
//     // 因为实现了 Deref 和 DerefMut，这里 buf 可以直接访问 Vec<T> 的方法
//     // 下面这句相当于：(&mut buf).deref_mut().sort()，也就是 (&mut buf.0).sort()
//     buf.sort();
//     println!("buf: {:?}", buf);
// }

// #[allow(dead_code)]
// enum Language {
//     Rust,
//     TypeScript,
//     Elixir,
//     Haskell,
// }

// impl AsRef<str> for Language {
//     fn as_ref(&self) -> &str {
//         match self {
//             Language::Rust => "Rust",
//             Language::TypeScript => "TypeScript",
//             Language::Elixir => "Elixir",
//             Language::Haskell => "Haskell",
//         }
//     }
// }

// fn print_ref(v: impl AsRef<str>) {
//     println!("{}", v.as_ref());
// }

// fn main() {
//     let lang = Language::Rust;
//     // &str 实现了 AsRef<str>
//     print_ref("Hello world!");
//     // String 实现了 AsRef<str>
//     print_ref("Hello world!".to_string());
//     // 我们自己定义的 enum 也实现了 AsRef<str>
//     print_ref(lang);
// }

// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// fn print(v: impl Into<IpAddr>) {
//     println!("{:?}", v.into());
// }

// fn main() {
//     let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
//     let v6: Ipv6Addr = "::1".parse().unwrap();

//     println!("aaa{:?}",v4);

//     // IPAddr 实现了 From<[u8; 4]，转换 IPv4 地址
//     print([1, 1, 1, 1]);
//     // IPAddr 实现了 From<[u16; 8]，转换 IPv6 地址
//     print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
//     // IPAddr 实现了 From<Ipv4Addr>
//     print(v4);
//     // IPAddr 实现了 From<Ipv6Addr>
//     print(v6);
// }

// fn main() {
//     let num = 42;
//     let r1 = &num as *const i32; // 不可变裸指针
//     // let r2 = &num as *mut i32; // 可变裸指针

//     unsafe {
//         println!("r1 points to: {}", *r1); // 使用裸指针读取数据
//                                            // *r2 = 50; // 如果允许修改，将会触发未定义行为，因为 `num` 是不可变的
//     }
// }

// #[derive(Clone, Copy, Debug)]
// struct Developer {
//     name: String,
//     age: u8,
//     lang: Language,
// }

// #[derive(Clone, Copy, Debug)]
// enum Language {
//     Rust,
//     TypeScript,
//     Elixir,
//     Haskell,
// }

// #[derive(Clone, Debug)]
// struct Developer {
//     name: String,
//     age: u8,
//     lang: Language,
// }

// #[allow(dead_code)]
// #[derive(Clone, Debug)]
// enum Language {
//     Rust,
//     TypeScript,
//     Elixir,
//     Haskell,
// }

// fn main() {
//     let dev = Developer {
//         name: "Tyr".to_string(),
//         age: 18,
//         lang: Language::Rust,
//     };

//     let dev1 = dev.clone();
//     println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
//     println!(
//         "dev1: {:?}, addr of dev1 name: {:p}",
//         dev1,
//         dev1.name.as_str()
//     )
// }

// pub trait Formatter {
//     fn format(&self, input: &mut String) -> bool;
// }

// struct MarkdownFormatter;
// impl Formatter for MarkdownFormatter {
//     fn format(&self, input: &mut String) -> bool {
//         input.push_str("\nformatted with Markdown formatter");
//         true
//     }
// }

// struct RustFormatter;
// impl Formatter for RustFormatter {
//     fn format(&self, input: &mut String) -> bool {
//         input.push_str("\nformatted with Rust formatter");
//         true
//     }
// }

// struct HtmlFormatter;
// impl Formatter for HtmlFormatter {
//     fn format(&self, input: &mut String) -> bool {
//         input.push_str("\nformatted with HTML formatter");
//         true
//     }
// }

// pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
//     for formatter in formatters {
//         formatter.format(input);
//     }
// }

// fn main() {
//     let mut text = "Hello world!".to_string();
//     let html: &dyn Formatter = &HtmlFormatter;
//     let rust: &dyn Formatter = &RustFormatter;
//     let formatters = vec![html, rust];
//     format(&mut text, formatters);

//     println!("text: {}", text);
// }

// struct Cat;
// struct Dog;

// trait Animal {
//     fn name(&self) -> &'static str;
// }

// impl Animal for Cat {
//     fn name(&self) -> &'static str {
//         "Cat"
//     }
// }

// impl Animal for Dog {
//     fn name(&self) -> &'static str {
//         "Dog"
//     }
// }

// fn name(animal: impl Animal) -> &'static str {
//     animal.name()
// }

// fn main() {
//     println!("cat: {}", name(Cat));
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Complex {
//     real: f64,
//     imagine: f64,
// }

// impl Complex {
//     pub fn new(real: f64, imagine: f64) -> Self {
//         Self { real, imagine }
//     }
// }

// // 对 Complex 类型的实现
// impl Add for Complex {
//     type Output = Self;

//     // 注意 add 第一个参数是 self，会移动所有权
//     fn add(self, rhs: Self) -> Self::Output {
//         let real = self.real + rhs.real;
//         let imagine = self.imagine + rhs.imagine;
//         Self::new(real, imagine)
//     }
// }

// fn main() {
//     let c1 = Complex::new(1.0, 1f64);
//     let c2 = Complex::new(2 as f64, 3.0);
//     println!("{:?}", c1 + c2);
//     // c1、c2 已经被移动，所以下面这句无法编译
//     // println!("{:?}", c1 + c2);
// }

// use std::str::FromStr;

// use regex::Regex;
// pub trait Parse {
//     type Error;
//     fn parse(s: &str) -> Result<Self, Self::Error>
//     where
//         Self: Sized;
// }

// impl<T> Parse for T
// where
//     T: FromStr + Default,
// {
//     // 定义关联类型 Error 为 String
//     type Error = String;
//     fn parse(s: &str) -> Result<Self, Self::Error> {
//         let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
//         if let Some(captures) = re.captures(s) {
//             // 当出错时我们返回 Err(String)
//             captures
//                 .get(0)
//                 .map_or(Err("failed to capture".to_string()), |s| {
//                     s.as_str()
//                         .parse()
//                         .map_err(|_err| "failed to parse captured string".to_string())
//                 })
//         } else {
//             Err("failed to parse string".to_string())
//         }
//     }
// }

// #[test]
// fn parse_should_work() {
//     assert_eq!(u32::parse("123abcd"), Ok(123));
//     assert_eq!(
//         u32::parse("123.45abcd"),
//         Err("failed to parse captured string".into())
//     );
//     assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
//     assert!(f64::parse("abcd").is_err());
// }

// fn main() {
//     println!("result: {:?}", u8::parse("255 hello world"));
// }

// use std::str::FromStr;

// use regex::Regex;
// pub trait Parse {
//     fn parse(s: &str) -> Self;
// }

// // 我们约束 T 必须同时实现了 FromStr 和 Default
// // 这样在使用的时候我们就可以用这两个 trait 的方法了
// impl<T> Parse for T
// where
//     T: FromStr + Default,
// {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
//         // 生成一个创建缺省值的闭包，这里主要是为了简化后续代码
//         // Default::default() 返回的类型根据上下文能推导出来，是 Self
//         // 而我们约定了 Self，也就是 T 需要实现 Default trait
//         let d = || Default::default();
//         if let Some(captures) = re.captures(s) {
//             captures
//                 .get(0)
//                 .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
//         } else {
//             d()
//         }
//     }
// }

// #[test]
// fn parse_should_work() {
//     assert_eq!(u32::parse("123abcd"), 123);
//     assert_eq!(u32::parse("123.45abcd"), 0);
//     assert_eq!(f64::parse("123.45abcd"), 123.45);
//     assert_eq!(f64::parse("abcd"), 0f64);
// }

// fn main() {
//     println!("result: {}", u8::parse("255 hello world"));
// }
// use regex::Regex;
// pub trait Parse {
//     fn parse(s: &str) -> Self;
// }

// impl Parse for u8 {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^[0-9]+").unwrap();
//         if let Some(captures) = re.captures(s) {
//             // 取第一个 match，将其捕获的 digits 换成 u8
//             captures
//                 .get(0)
//                 .map_or(0, |s| s.as_str().parse().unwrap_or(0))
//         } else {
//             0
//         }
//     }
// }

// #[test]
// fn parse_should_work() {
//     assert_eq!(u8::parse("123abcd"), 123);
//     assert_eq!(u8::parse("1234abcd"), 0);
//     assert_eq!(u8::parse("abcd"), 0);
// }

// fn main() {
//     println!("result: {}", u8::parse("255 hello world"));
// }

// use std::fmt;
// use std::io::Write;

// struct BufBuilder {
//     buf: Vec<u8>,
// }

// impl BufBuilder {
//     pub fn new() -> Self {
//         Self {
//             buf: Vec::with_capacity(1024),
//         }
//     }
// }

// // 实现 Debug trait，打印字符串
// impl fmt::Debug for BufBuilder {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", String::from_utf8_lossy(&self.buf))
//     }
// }

// impl Write for BufBuilder {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         // 把 buf 添加到 BufBuilder 的尾部
//         self.buf.extend_from_slice(buf);
//         Ok(buf.len())
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         // 由于是在内存中操作，所以不需要 flush
//         Ok(())
//     }
// }

// fn main() {
//     let mut buf = BufBuilder::new();
//     buf.write_all(b"Hello world!").unwrap();
//     println!("{:?}", buf);
// }

// use std::io::{BufWriter, Write};
// use std::net::TcpStream;

// #[derive(Debug)]
// struct MyWriter<W> {
//     writer: W,
// }

// impl<W: Write> MyWriter<W> {
//     pub fn new(writer: W) -> Self {
//         Self { writer }
//     }

//     pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
//         self.writer.write_all(buf.as_bytes())
//     }
// }

// fn main() {
//     let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//     let mut writer = MyWriter::new(BufWriter::new(stream));

//     writer.write("hello world!");
// }

// use std::str::Chars;

// // 错误，为什么？
// fn lifetime1() -> &'static str {
//     let name = "Tyr".to_string();
//     &name[1..]
// }

// // 错误，为什么？
// fn lifetime2(name: String) -> &str {
//     &name[1..]
// }

// // 正确，为什么？
// fn lifetime3(name: &str) -> Chars {
//     name.chars()
// }

// fn main() {}

// use std::collections::HashMap;
// use std::mem::size_of;

// enum E {
//     A(f64),
//     B(HashMap<String, String>),
//     C(Result<Vec<u8>, String>),
// }

// // 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
// macro_rules! show_size {
//     (header) => {
//         println!(
//             "{:<24} {:>4}    {}    {}",
//             "Type", "T", "Option<T>", "Result<T, io::Error>"
//         );
//         println!("{}", "-".repeat(64));
//     };
//     ($t:ty) => {
//         println!(
//             "{:<24} {:4} {:8} {:12}",
//             stringify!($t),
//             size_of::<$t>(),
//             size_of::<Option<$t>>(),
//             size_of::<Result<$t, std::io::Error>>(),
//         )
//     };
// }

// fn main() {
//     show_size!(header);
//     show_size!(u8);
//     show_size!(f64);
//     show_size!(&u8);
//     show_size!(Box<u8>);
//     show_size!(&[u8]);

//     show_size!(String);
//     show_size!(Vec<u8>);
//     show_size!(HashMap<String, String>);
//     show_size!(E);
// }

// fn main() {
//     let arr = vec![1];

//     let handle = std::thread::spawn(move || {
//         println!("{:?}", arr);
//     });

//     handle.join().unwrap();
// }

// use std::sync::Arc;
// fn main() {
//     let arr = Arc::new("123");
//     // 生成一个不可变引用
//     let d = arr.clone();
//     // 创建一个线程 同时使用 move 进行所有权的转移
//     let computation = std::thread::spawn(move || {
//         println!("{:?}", d);
//     });
//     println!("{:?}", arr);
//     // 将线程进行加入
//     computation.join().expect("The sender thread has panicked");
// }
