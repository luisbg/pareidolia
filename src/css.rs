pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub enum Selector {
    Simple(SimpleSelector),
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // insert more values here
}

#[derive(Clone, PartialEq)]
pub enum Unit {
    Px,
    // insert more units here
}

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

pub fn example() -> Stylesheet {
    // html rules
    let mut html_selects: Vec<Selector> = Vec::new();
    html_selects.push(Selector::Simple(SimpleSelector {
        tag_name: Some(String::from("html")),
        id: None,
        class: Vec::new()}));
    let mut html_decls: Vec<Declaration> = Vec::new();
    html_decls.push(Declaration {
        name: String::from("background"),
        value: Value::ColorValue(Color {r: 255, g: 255, b: 255, a: 255}) });

    // div main rules
    let mut main_selects: Vec<Selector> = Vec::new();
    main_selects.push(Selector::Simple(SimpleSelector {
        tag_name: Some(String::from("div")),
        id: Some(String::from("main")),
        class: Vec::new()}));
    let mut main_decls: Vec<Declaration> = Vec::new();
    main_decls.push(Declaration {
        name: String::from("color"),
        value: Value::ColorValue(Color {r: 0, g: 0, b: 255, a: 255}) });
    main_decls.push(Declaration {
        name: String::from("height"),
        value: Value::Length (400.0, Unit::Px) });

    // div second rules
    let mut second_selects: Vec<Selector> = Vec::new();
    second_selects.push(Selector::Simple(SimpleSelector {
        tag_name: Some(String::from("div")),
        id: Some(String::from("second")),
        class: Vec::new()}));
    let mut second_decls: Vec<Declaration> = Vec::new();
    second_decls.push(Declaration {
        name: String::from("color"),
        value: Value::ColorValue(Color {r: 255, g: 0, b: 0, a: 255}) });
    second_decls.push(Declaration {
        name: String::from("height"),
        value: Value::Length (250.0, Unit::Px) });

    // Create the stylesheet from the rules
    let mut rules: Vec<Rule> = Vec::new();
    rules.push(Rule { selectors: html_selects, declarations: html_decls });
    rules.push(Rule { selectors: main_selects, declarations: main_decls });
    rules.push(Rule { selectors: second_selects, declarations: second_decls });
    Stylesheet { rules: rules }
}

impl Value {
    /// Return the size of a length in px, or zero for non-lengths.
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0
        }
    }
}
