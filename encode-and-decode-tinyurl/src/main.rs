struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        return longURL;
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        return shortURL;
    }
}


fn main() {
    println!("Hello, world!");
}
