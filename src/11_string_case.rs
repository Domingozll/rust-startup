use std::fmt::format;

//run: cargo add unicode_segmentation
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // create string
    let s1: &str = "hello world! 🤖";
    let s2: String = String::from("hello world! 🤖");
    let s3: String = "hello world! 🤖".to_string();
    let s4: String = "hello world! 🤖".to_owned();
    let s5: String = s3 + "haha";
    let s6: &str = &s4[..];

    // edit string
    let mut s: String = String::from("hah");
    s.push_str(" foo"); // s is: hah foo
    println!("s is: {s}");
    s.replace_range(1.., "bbb");
    println!("s is: {s}"); // s is: hbbb

    // + operator
    let ss1: String = String::from("hello,");
    let ss2: String = String::from("world");
    let ss = ss1 + &ss2; // move ss1 to ss and append ss2, ss will take the ownership of ss1
    // println!("ss1 is: {ss1}"); // error: value borrowed here after move
    println!("ss2 is: {ss2}"); // ss2 is: world
    println!("ss is: {ss}"); // ss is: hello,world

    // format
    let sss1: String = String::from("hello,");
    let sss2: String = "world!".to_string();
    let sss3: String = " 🤖".to_owned();
    let sss: String = format!("{}-{}-{}-{}",sss1,sss2,sss3," ok");// need copy contents of each string. less efficient than + operator. 
    println!("sss1 is: {sss1}"); // sss1 is: hello,
    println!("sss2 is: {sss2}"); // sss2 is: world!
    println!("sss3 is: {sss3}"); // sss3 is:  🤖
    println!("sss is: {sss}"); // sss is: hello,-world!- 🤖- ok


    // concat
    let s: String = ["first ","second ","third ","four."].concat();
    println!("s is: {s}"); // s is: first second third four.
    let ss2: &str = concat!("first ","second ","third ","four.");
    println!("ss2 is: {ss2}"); // ss2 is: first second third four.


    // index of string
    let s_index1: &str = "🌞the🌞🌞🌞🌞🌞🌞🌞🌞🌞"; // a 🌞 is 4 bytes
    let s_index2: &str = &s_index1[0..4];// a 4 bytes slice
    println!("s_index2 is: {s_index2}"); // s_index2 is: 🌞

    // let s_index3: &str = &s_index1[0..3];// a 3 bytes slice
    // println!("s_index3 is: {s_index3}"); // error: byte index 3 is not a char boundary; it is inside '🌞' (bytes 0..4) of `🌞🌞🌞🌞🌞🌞🌞🌞🌞🌞`

    // iterator of string
    for b in "🌞the🌞🌞🌞🌞🌞🌞🌞🌞🌞".bytes() {
        print!("{} ",b); // 240 159 140 158 116 104 101 240 159 140 158 240 159 140 158 240 159 140 158 240 159 140 158 240 159 140 158 240 159 140 158 240 159 140 158 240 159 140 158 240 159 140 158
    }
    println!("");
    
    for b in "🌞the🌞🌞🌞🌞🌞张三🌞🌞里斯本🌞王武🌞".chars() {
        print!("{} ",b); //🌞 t h e 🌞 🌞 🌞 🌞 🌞 张 三 🌞 🌞 里 斯 本 🌞 王 武 🌞
    } 
    println!("");

    //print the Unicode sacler values of string (部分用户看到的字符是由多个Unicode编码组合成的图形值)
    for b in "नमस्ते".chars() {
        print!("{} ",b); // 'न' 'म' 'स' '्' 'त' 'े' 
    }
    println!("");

    // In Unicode，use-reviced characters are known as grapheme clusters


    // iterate over graphing clusters
    for g in "नमस्ते".graphemes(true) {
        print!("{} ",g);//न म स्ते
    }
    println!("");


    let sf1: &str = "hello world! 🤖";
    let sf2: String = String::from("hello world! 🤖");

    let sf3: String = my_function(sf1);
    println!("sf3 is: {sf3}"); // sf3 is: hello world! 🤖
    let sf4: String = my_function(&sf2);
    println!("sf4 is: {sf4}"); // sf4 is: hello world! 🤖

}

fn my_function(a: &str) -> String {
    format!("{}",a)
}

