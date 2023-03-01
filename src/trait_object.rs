pub trait Draw {
    fn draw(&self);
}

//trait object
//this works differently from type param with trait bounds,
//a generic trait bounds can only be substituted with one concrete type at a time.
//with trait object, the componenets field can hold different concrete types
struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn draw_all(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
        // for comp in &self.components {
        //     comp.draw();
        // }
    }

    pub fn add(&mut self, comp: Box<dyn Draw>) {
        self.components.push(comp);
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

pub fn demo() {
    //create a Screen instance
    let mut screen = Screen { components: vec![] };
    let button1 = Box::new(Button {
        width: 32,
        height: 23,
        label: String::from("OK"),
    });

    let sb1 = Box::new(SelectBox {
        width: 25,
        height: 32,
        options: vec![String::from("sunny"), String::from("cloudy")],
    });

    screen.add(button1);
    screen.add(sb1);

    //run screen
    screen.draw_all();
}
