use std::str;


#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book {title, author} = self {
        //     format!("Book {} by {}", title, author)
        // } else if let Media::Movie {title, director} = self {
        //     format!("Movie {} by {}", title, director)
        // } else if let Media::Audiobook {title} = self {
        //     format!("Audio {}", title)
        // } else {
        //     String::from("Unknown")
        // }

        match self {
            Media::Book {title, author} => format!("Book {} by {}", title, author),
            Media::Movie {title, director} => format!("Movie {} by {}", title, director),
            Media::Audiobook {title} => format!("Audio {}", title),
            Media::Podcast(episode_number) => format!("Podcast with {} episodes", episode_number),
            Media::Placeholder => format!("Placeholder")
        }
    }
    
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Catalog {
        Catalog {items: vec![]}
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }
    
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from ("The Great Gatsby")
    };

    let good_movie = Media::Movie {
        title: String::from("The Dark Knight"), 
        director: String::from("Christopher Nolan")
    };    

    let bad_book = Media::Book {
        title: String::from("Fifty Shades of Grey"), 
        author: String::from("E.L. James")
    };

    let podcast = Media::Podcast(100);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);


}
