use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
        article: String::from("How To work with jsom in Rust"),
        author: String::from("Subhanshu"),
        paragraph: vec![
            Paragraph {
                name: String::from("First Sentence")
            },
            Paragraph {
                name: String::from("Body of Paragraph")
            },
            Paragraph {
                name: String::from("End of Paragraph")
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The Json is: {}", json)
}
