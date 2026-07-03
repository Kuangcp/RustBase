use std::fmt;

fn main() {
    card_game();
    shape_area();
}

// ====== 示例1: Card Game — 结构体 + 枚举 + Display ======

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Rank::Ace => "Ace",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

fn card_game() {
    println!("--- Card Game ---");

    let hand = vec![
        Card { rank: Rank::Ace, suit: Suit::Spades },
        Card { rank: Rank::King, suit: Suit::Hearts },
        Card { rank: Rank::Five, suit: Suit::Diamonds },
    ];

    println!("Player hand:");
    for card in &hand {
        println!("  {card}");
    }
    println!();
}

// ====== 示例2: Shape Area — 泛型 + trait ======

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// 泛型函数：任何实现了 Area 的类型都可以传入
fn print_area<T: Area + fmt::Display>(shape: &T) {
    println!("{shape}: area = {:.2}", shape.area());
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({}x{})", self.width, self.height)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Triangle(base={}, h={})", self.base, self.height)
    }
}

fn shape_area() {
    println!("--- Shape Area ---");

    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 3.0, height: 4.0 };
    let tri = Triangle { base: 6.0, height: 3.0 };

    print_area(&circle);
    print_area(&rect);
    print_area(&tri);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_equality() {
        let c1 = Card { rank: Rank::Ace, suit: Suit::Spades };
        let c2 = Card { rank: Rank::Ace, suit: Suit::Spades };
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_circle_area() {
        let c = Circle { radius: 5.0 };
        assert!((c.area() - 78.5398).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(r.area(), 12.0);
    }

    #[test]
    fn test_triangle_area() {
        let t = Triangle { base: 6.0, height: 3.0 };
        assert_eq!(t.area(), 9.0);
    }
}
