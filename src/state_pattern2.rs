//from the book, using State Object
pub fn demo() {}

trait State {
    fn send_for_review(self: Box<Self>) -> Box<dyn State>; //can only be called inside a box
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, s: &'a str) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn is_editable(&self) -> bool {
        false
    }
}

struct Draft {}

struct Review {}
struct Published {}

struct Pending {}

impl State for Draft {
    fn send_for_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Review {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_editable(&self) -> bool {
        true
    }
}

impl State for Review {
    fn send_for_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for Published {
    fn send_for_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, s: &'a str) -> &'a str {
        s
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for Pending {
    fn send_for_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Post {
    content: String,
    state: Option<Box<dyn State>>, //state object
}

impl Post {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }

    pub fn add_text(&mut self, s: &str) {
        //only change state if state is editable
        if self.state.as_ref().unwrap().is_editable() {
            self.content.push_str(s);
        }
    }

    pub fn request_review(&mut self) {
        //use of take //unwrap takes ownership
        //take state value, leave none in its place
        // let k = self.state; //this not causes error
        //take lets us move the state value out of post
        //temporarily set state to None
        if let Some(state) = self.state.take() {
            self.state = Some(state.send_for_review());
        }
        // self.state = self.state.take().unwrap().send_for_review();
    }

    fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.send_for_review());
        }
    }

    fn content(&self) -> &str {
        // if let Some(s) = &self.state {
        //     return s.content(&self.content);
        // }
        // ""

        self.state.as_ref().unwrap().content(&self.content)
    }

    fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}
