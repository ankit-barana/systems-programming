// name: Ankit Barana
use core::fmt;
use std::io;
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

// defines a generic type
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct Entry {
    author: Option<String>, // author can be empty
    title: String,
    url: String,
    resources: Vec<Resource>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let author = match &self.author {
            Some(a) => a.to_string(),
            None => String::from("None"),
        };

        let mut info = format!("Author: {}\nTitle: {}\nURL: {}\nResources", author, self.title, self.url);
        let resources_iter = self.resources.iter();
        for resource in resources_iter {
            match resource {
                // checks if resource matches any of the varaints
                Resource::Epub(s) | Resource::Text(s) | Resource::Audio(s) => {
                    info.push_str("\n- ");
                    info.push_str(s);
                }
            }
        }
        // f refers to the formatter used to print the custom types
        write!(f, "{}", info)
    }
}

#[derive(Debug, Clone)]
struct Index {
    bookshelf: Vec<Entry>,
    size: usize,
}

#[derive(Debug, Clone)]
enum Resource {
    Epub(String), 
    Text(String),
    Audio(String),
}

impl Index {
    /// Create a new `Index` by reading from `pgindex.txt`.
    fn new() -> Result<Self> {
        // reades pgindex.txt
        let mut r = BufReader::new(File::open("pgindex.txt")?);
        // contains each entry
        let mut temp_bookshelf = Vec::new();
        // saves individual entry
        let mut line = String::new();
        let mut count = 0;

            while r.read_line(&mut line).unwrap() > 0 {  // readline returns bytes so we want chek if the number of bytes is 0, which would mean that we have reached the end of the file
                let parts: Vec<&str> = line.as_str().split("\t").collect();
                let author = Some(parts[0].to_string());
                let title = parts[1].to_string();
                let url = parts[2].to_string();
                let mut resources: Vec<Resource> = Vec::new();

                // iterates over the remaining line as it contains file-type and link to access it
                for index in 3..parts.len() {
                    // holds file information
                    let file_info = parts[index].to_string();
                    // splits file information at whitespace
                    let file_info_vec: Vec<&str> = file_info.split_whitespace().collect();
                    match file_info_vec[0] {
                        "epub" => resources.push(Resource::Epub(file_info)), // I used clone() because I was getting "creates a temporary value which is freed while still in use" error here
                        "text" => resources.push(Resource::Text(file_info)),
                        "audio" => resources.push(Resource::Audio(file_info)),
                        _ => return Err(format!("Unknown format: {}", parts[0]).into()),
                    }
                }

                let entry = Entry {
                    author,
                    title,
                    url,
                    resources,
                };
                // adds each entry into the bookshelf
                temp_bookshelf.push(entry);
                count += 1;
                // as readlines appends, we need to reset the line
                line = String::new();
            }

        Ok(Self {
            bookshelf: temp_bookshelf,
            size: count,
        })
    }

    /// Returns the number of entries in the index.
    fn len(&self) -> usize {
        self.size
    }

    /// Returns a vector containing references to entries whose titles
    /// contain `search_string`.
    fn title_search(&self, search_string: &str) -> Vec<&Entry> {
        let mut result = Vec::new();
        let query_lowercase = search_string.to_ascii_lowercase();
        for entry in &self.bookshelf {
            if entry.title.to_ascii_lowercase().contains(&query_lowercase) {
                result.push(entry);
            }
        } return result;
    }

    /// Returns a vector containing references to entries whose authors
    /// contain `search_string`.
    fn author_search(&self, search_string: &str) -> Vec<&Entry> {
        let mut result = Vec::new();
        let query_lowercase = search_string.to_ascii_lowercase();
        for entry in &self.bookshelf {
            if entry.author.as_ref().unwrap().to_ascii_lowercase().contains(&query_lowercase) {
                result.push(entry);
            }
        } return result;
    }

    /// Returns a vector containing references to entries whose titles
    /// or authors contain `search_string`.
    fn search(&self, search_string: &str) -> Vec<&Entry> {
        let mut result = Vec::new();
        let query_lowercase = search_string.to_ascii_lowercase();
        for entry in &self.bookshelf {
            if entry.author.as_ref().unwrap().to_ascii_lowercase().contains(&query_lowercase) || entry.title.to_ascii_lowercase().contains(&query_lowercase) {
                result.push(entry);
            }
        } return result;
    }
}


fn run() -> Result<()> {
    let index = Index::new()?;
    println!("Welcome to Gutensearch! The index contains {} entries.", index.len());
    println!("Example queries to search titles, authors, or both:\n  > title: two cities\n  > author: wodehouse\n  > shelley");
    print!("\nEnter a search query (ctrl-d to exit)\n");
    
    let mut results: Vec<&Entry>;

    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        let mut search_string = String::new();
        io::stdin()
            .read_line(&mut search_string)
            .expect("Failed to read line");

        // if input is empty, we exit with code 1
        if search_string.is_empty() {
            std::process::exit(1);

        // if it contains only whitespaces, we continue
        } else if search_string.trim().is_empty() {
            continue;

        // if it constains colon, we check if it the user is looking for title or author
        } else if search_string.trim().contains(":") {

            let i = search_string.trim().find(":");
            let querytype: &str = &search_string.trim()[0..i.unwrap()];
            let query: &str = &search_string.trim()[i.unwrap()+1..];

            // searces by title 
            if querytype.to_ascii_lowercase() == "title" {
                results = index.title_search(query.trim());
                // if results is empty the title does not exist
                if results.is_empty() { println!("No matching results"); }

            } 
            
            // searches by author
            else if querytype.to_ascii_lowercase() == "author" {
                results = index.author_search(query.trim());
                // if results is empty the author does not exist
                if results.is_empty() { println!("No matching results"); }
            } 

            // If a colon appears but the trimmed text before the colon is neither title nor author, print an error message and print the prompt again.
            else {
                eprintln!("Please search by author or title or none (by not specifying either)");
                continue;
            }

        // oterwise, we search across both authors and title
        } else {
            println!("OMG, We are here");
            results = index.search(search_string.trim());
        }
        // prints the entries
        for entry in results {
            println!("{entry}");
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}