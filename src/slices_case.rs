fn main() {
    // Slices are references to a contiguous sequence of element in a collection.
    let tweet: String = String::from(
        "0123456789"
    );

    let trimmed_tweet_1: &str = &tweet[..5];
    println!("trimmed_tweet_1 is: {trimmed_tweet_1}");//trimmed_tweet_1 is: 01234
    let trimmed_tweet_2: &str = &tweet[0..5];
    println!("trimmed_tweet_2 is: {trimmed_tweet_2}");//trimmed_tweet_2 is: 01234
    let trimmed_tweet_3: &str = &tweet[0..10];
    println!("trimmed_tweet_3 is: {trimmed_tweet_3}");//trimmed_tweet_3 is: 0123456789
    let trimmed_tweet_4: &str = &tweet[5..];
    println!("trimmed_tweet_4 is: {trimmed_tweet_4}");//trimmed_tweet_4 is: 56789
    
    
    let trimmed_tweet_5: &str = trim_tweet_1(&tweet);
    println!("trimmed_tweet_5 is: {trimmed_tweet_5}");//trimmed_tweet_5 is: 01234
    let trimmed_tweet_5: &str = trim_tweet_2(&tweet);
    println!("trimmed_tweet_5 is: {trimmed_tweet_5}");//trimmed_tweet_5 is: 01234
    
    let tweet2: &str = "0123456789";
    // let trimmed_tweet_6: &str = trim_tweet_1(&tweet2); // erroe: mismatched types
    let trimmed_tweet_6: &str = trim_tweet_2(&tweet2); // erroe: mismatched types
    println!("trimmed_tweet_6 is: {trimmed_tweet_6}");//trimmed_tweet_6 is: 01234

    let a: [i32; 6] = [0,1,2,3,4,5];
    let a_silce: &[i32] = &a[..3];
    println!("a_silce is: {:?}",a_silce); // a_silce is: [0, 1, 2]
}

fn trim_tweet_1(tweet: &String) -> &str {
    &tweet[0..5]
}

// work for both &String and &str paramters(rust的Dref-Coversion特性，自动类型转换)
fn trim_tweet_2(tweet: &str) -> &str {
    &tweet[0..5]
}