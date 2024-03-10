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
    num_wins: u32,
}

impl Parse for Card {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_prefix("Card ")?;
        parser.skip_whitespace();
        let _: u32 = parser.parse()?;
        parser.consume_prefix(": ")?;
        let mut win_buffer = [0; 10];
        let mut winning_numbers = 0;
        parser.skip_whitespace();
        while let Ok(n) = parser.parse() {
            if winning_numbers == 10 { panic!("Too many winning numbers") }
            win_buffer[winning_numbers] = n;
            winning_numbers += 1;
            parser.skip_whitespace();
        }
        let wins = &win_buffer[0..winning_numbers];
        parser.consume_prefix("|")?;
        let mut num_wins = 0;
        parser.skip_whitespace();
        while let Ok(n) = parser.parse() {
            if wins.contains(&n) { num_wins += 1 }
            parser.skip_whitespace();
        }
        return Ok(Card{num_wins: num_wins});
    }
}

fn main() {
    // Load the input.
    let mut buffer = [0; 24 * 1024];
    let size = io::stdin().read(&mut buffer).unwrap();
    let input = str::from_utf8(&buffer[0..size]).unwrap();

    // Parse the cards.
    let mut parser = Parser::new(input);
    let mut part1 = 0;
    let mut part2 = 0;
    let mut counts = [1; 10];
    let mut i = 0;
    while !parser.at_end() {
        let card: Card = parser.parse().unwrap();
        parser.newline().unwrap();

        // Part 1: accumulate points based on the number of wins.
        part1 += (1 << card.num_wins) >> 1;

        // Part 2: accumulate cards.
        let n = counts[i];
        part2 += n;
        counts[i] = 1;
        i = if i < 9 { i + 1 } else { 0 };
        for j in 0 .. card.num_wins as usize {
            let k = if i + j < 10 { i + j } else { i + j - 10 };
            counts[k] += n;
        }
    }
    print!("{}\n{}\n", part1, part2);
}
