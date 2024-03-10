use std::io;
use std::io::Read;
use std::str;

#[derive(Copy, Clone, Debug)]
struct Location {
    line: u32,
    column: u32,
}

#[derive(Debug)]
enum ParseError {
    ErrorAt(Location, String),
}

struct Parser<'a> {
    input: &'a str,
    location: Location,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        return Parser{input: input, location: Location{line: 1, column: 1}};
    }
    fn at_end(&self) -> bool { return self.input.is_empty() }
    fn advance_bytes(&mut self, n: usize) {
        if n > self.input.len() { panic!("Advanced beyond EOF") }
        for c in self.input[0..n].chars() {
            if c == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            } else {
                self.location.column += 1;
            }
        }
        self.input = &self.input[n..];
    }
    fn skip_whitespace(&mut self) {
        let n = self.input.find(|c: char| c != ' ')
                          .unwrap_or(self.input.len());
        self.advance_bytes(n);
    }
    fn try_consume_prefix(&mut self, prefix: &str) -> bool {
        if !self.input.starts_with(prefix) { return false }
        self.input = &self.input[prefix.len()..];
        return true;
    }
    fn consume_prefix(&mut self, prefix: &str) -> Result<(), ParseError> {
        if self.try_consume_prefix(prefix) { return Ok(()) }
        return Err(self.error(format!("expected \"{}\"", prefix).as_str()));
    }
    fn newline(&mut self) -> Result<(), ParseError> {
        if self.try_consume_prefix("\n") {
            return Ok(());
        } else {
            return Err(self.error("expected newline"));
        }
    }
    fn error(&mut self, message: &str) -> ParseError {
        return ParseError::ErrorAt(self.location, message.to_string());
    }
    fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
        return Parse::parse(self);
    }
}

trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}

impl Parse for u32 {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let l = parser.input.find(|c: char| !c.is_digit(10))
                            .unwrap_or(parser.input.len());
        if l == 0 { return Err(parser.error("expected number")) }
        let mut n = 0;
        let digits = &parser.input[0 .. l];
        for c in digits.chars().map(|c| c.to_digit(10).unwrap()) {
            n = 10 * n + c;
        }
        parser.advance_bytes(l);
        return Ok(n);
    }
}

struct Card {
    win: Vec<u32>,
    values: Vec<u32>,
}

impl Parse for Card {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_prefix("Card ")?;
        parser.skip_whitespace();
        let _: u32 = parser.parse()?;
        parser.consume_prefix(": ")?;
        let mut win = Vec::new();
        parser.skip_whitespace();
        while let Ok(n) = parser.parse() {
            win.push(n);
            parser.skip_whitespace();
        }
        parser.consume_prefix("|")?;
        let mut values = Vec::new();
        parser.skip_whitespace();
        while let Ok(n) = parser.parse() {
            values.push(n);
            parser.skip_whitespace();
        }
        return Ok(Card{win: win, values: values});
    }
}

fn read_input() -> Result<Vec<Card>, ParseError> {
    // Load the input.
    let mut buffer = [0; 24 * 1024];
    let size = io::stdin().read(&mut buffer).unwrap();
    let input = str::from_utf8(&buffer[0..size]).unwrap();

    // Parse the cards.
    let mut parser = Parser::new(input);
    let mut cards = Vec::new();
    while !parser.at_end() {
        cards.push(parser.parse()?);
        parser.newline()?;
    }
    return Ok(cards);
}

fn part1(cards: &[Card]) -> u32 {
    let mut total = 0;
    for card in cards {
        let mut num_wins = 0;
        for n in card.values.as_slice() {
            if card.win.contains(&n) { num_wins += 1 }
        }
        total += (1 << num_wins) >> 1;
    }
    return total;
}

fn part2(cards: &[Card]) -> u32 {
    let mut counts = vec![1; cards.len()];
    let n = cards.len();
    for (i, card) in cards.iter().enumerate() {
        let mut num_wins = 0;
        for v in card.values.as_slice() {
            if card.win.contains(&v) { num_wins += 1 }
        }
        for j in (i + 1).min(n) .. (i + 1 + num_wins).min(n) {
            counts[j] += counts[i];
        }
    }
    return counts.iter().sum();
}

fn main() {
    let cards = read_input().unwrap();
    print!("{}\n{}\n", part1(&cards), part2(&cards));
}
