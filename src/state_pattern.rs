//self-implementation

use std::error::Error;

use rand::Rng;

fn get_random_string() -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(7)
        .collect()
}

// You may have been wondering why we didn’t use an enum with the different possible post states as variants.
//That’s certainly a possible solution, try it and compare the end results to see which you prefer!
//One disadvantage of using an enum is every place that checks the value of the enum will need a match expression
//or similar to handle every possible variant. This could get more repetitive than this trait object solution.

#[derive(Debug)]
enum BlogState {
    Draft,
    Review,
    Published,
}

#[derive(Debug)]
struct Post {
    id: String,
    title: String,
    content: String,
    state: BlogState,
}

impl Post {
    fn new() -> Self {
        Self {
            id: get_random_string(),
            title: String::new(),
            content: String::new(),
            state: BlogState::Draft,
        }
    }

    fn send_for_review(&mut self) {
        self.state = BlogState::Review
    }

    fn approve(&mut self) {
        self.state = BlogState::Published
    }

    fn print(&self) -> Result<&str, Box<dyn Error>> {
        match self.state {
            BlogState::Published => Ok(&self.content),
            _ => Err(String::from("Post not published!").into()),
        }
    }
}

#[derive(Debug)]
struct Blog {
    posts: Vec<Post>,
}

impl Blog {
    fn add(&mut self) {
        self.posts.push(Post::new());
    }

    //mut an element inside a vector
    fn request_review(&mut self, id: &str) {
        let post = self.posts.iter_mut().find(|p| p.id == id).unwrap();
        post.send_for_review();
    }

    fn request_approval(&mut self, id: &str) {
        let post = self.posts.iter_mut().find(|p| p.id == id).unwrap();
        post.approve();
    }

    fn print(&self, id: &str) -> Result<&str, Box<dyn Error>> {
        let post = self.posts.iter().find(|p| p.id == id);
        if let None = post {
            return Err(String::from("Post not found").into());
        }

        post.unwrap().print()
    }
}

pub fn demo() {
    let mut blog = Blog { posts: vec![] };
    blog.add();
    blog.add();
    println!("current blog: {:?}", blog);

    match blog.print("383") {
        Ok(content) => println!("content is: {content}"),
        Err(msg) => println!("ERROR: {msg}"),
    };
}
