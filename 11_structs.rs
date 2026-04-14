// ============================================================
// 11 - STRUCTS
// ============================================================
// Structs let you package related values into a single type.
//
// THREE KINDS OF STRUCTS:
//   1. Named-field structs   -> struct Name { field: Type, ... }
//   2. Tuple structs         -> struct Name(T1, T2, ...);
//   3. Unit-like structs     -> struct Name;   // no fields
//
// METHODS vs ASSOCIATED FUNCTIONS:
//   - Methods take &self / &mut self / self.
//   - Associated functions (like constructors) do not take self;
//     called with Type::function().
// ============================================================

// ---------- NAMED-FIELD STRUCT ----------
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// ---------- TUPLE STRUCT ----------
struct Color(i32, i32, i32);

// ---------- UNIT-LIKE STRUCT ----------
struct AlwaysEqual;

// ---------- METHODS via impl BLOCK ----------
#[derive(Debug)]    // auto-implements Debug so we can {:?} print
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method with immutable self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with another instance
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Method with mutable self
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    // ---------- CREATING INSTANCES ----------
    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("a@b.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("new@b.com");
    println!("user: {} / {}", user1.username, user1.email);

    // ---------- FIELD INIT SHORTHAND ----------
    let user2 = build_user(String::from("bob@x.com"), String::from("bob"));
    println!("user2: {}", user2.username);

    // ---------- STRUCT UPDATE SYNTAX ----------
    let user3 = User {
        email: String::from("c@b.com"),
        ..user1    // copy remaining fields from user1 (moves Strings!)
    };
    println!("user3: {} / {}", user3.username, user3.email);

    // ---------- TUPLE STRUCT ----------
    let black = Color(0, 0, 0);
    println!("R={} G={} B={}", black.0, black.1, black.2);

    // ---------- UNIT STRUCT ----------
    let _u = AlwaysEqual;

    // ---------- METHODS ----------
    let mut rect = Rectangle::new(30, 50);
    println!("area = {}", rect.area());
    println!("rect = {:?}", rect);     // uses Debug trait
    rect.scale(2);
    println!("scaled = {:?}", rect);

    let small = Rectangle::new(10, 20);
    println!("rect holds small? {}", rect.can_hold(&small));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,          // shorthand for email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}
