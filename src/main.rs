mod content;

use content::media::Media;
use content::catalog::Catalog;


enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);

    // println!("{:#?}", item);

    match catalog.get_by_index(9990) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }
    }

    if let Some(value) = catalog.get_by_index(0) {
        println!("Item in pattern match: {:#?}", value)
    } else {
        println!("No value!!!!!!!!!!");
    }
}
