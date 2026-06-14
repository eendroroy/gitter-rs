use crate::repository::repositories::{Properties, PropertyLengths, Repositories};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DURATION_REGEX: Regex = Regex::new(r"(\d+)([yMwdhm])").unwrap();
}

fn parse_duration_string(s: &str) -> Option<Duration> {
    let mut total_duration = Duration::zero();
    let mut matched_any = false;

    for cap in DURATION_REGEX.captures_iter(s) {
        matched_any = true;
        let value = cap[1].parse::<i64>().ok()?;
        let unit = &cap[2];

        match unit {
            "y" => total_duration += Duration::days(value * 365),
            "M" => total_duration += Duration::days(value * 30),
            "w" => total_duration += Duration::weeks(value),
            "d" => total_duration += Duration::days(value),
            "h" => total_duration += Duration::hours(value),
            "m" => total_duration += Duration::minutes(value),
            _ => return None,
        }
    }

    if matched_any
        && DURATION_REGEX.find_iter(s).map(|m| m.as_str().len()).sum::<usize>() == s.len()
    {
        Some(total_duration)
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
enum FilterType {
    Path,
    Name,
    Branch,
    Active,
}

#[derive(Debug, PartialEq)]
enum FilterCondition {
    Exact(String),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    ActiveLessThan(Duration),
    ActiveGreaterThan(Duration),
    ActiveExact(Duration),
}

#[derive(Debug, PartialEq)]
struct ParsedFilter {
    filter_type: FilterType,
    condition: FilterCondition,
    negate: bool,
}

impl ParsedFilter {
    fn parse(filter_clause: &str) -> Option<Self> {
        let mut negate = false;
        let mut clause = filter_clause;

        if clause.starts_with('!') {
            negate = true;
            clause = &clause[1..];
        }

        let parts: Vec<&str> = clause.splitn(2, ':').collect();
        if parts.len() != 2 {
            return None;
        }

        let prefix = parts[0].trim();
        let value_str = parts[1].trim();

        let filter_type = match prefix {
            "path" => FilterType::Path,
            "name" => FilterType::Name,
            "branch" => FilterType::Branch,
            "active" => FilterType::Active,
            _ => return None,
        };

        let condition = if filter_type == FilterType::Active {
            let (op, duration_str) = if let Some(stripped) = value_str.strip_prefix('<') {
                (Some('<'), stripped)
            } else if let Some(stripped) = value_str.strip_prefix('>') {
                (Some('>'), stripped)
            } else {
                (None, value_str)
            };

            let duration = parse_duration_string(duration_str)?;

            match op {
                Some('<') => FilterCondition::ActiveLessThan(duration),
                Some('>') => FilterCondition::ActiveGreaterThan(duration),
                None => FilterCondition::ActiveExact(duration),
                _ => return None, // This case should ideally not be reached
            }
        } else if value_str.starts_with('+') && value_str.ends_with('+') && value_str.len() > 2 {
            FilterCondition::Contains(value_str[1..value_str.len() - 1].to_string())
        } else if value_str.starts_with('+') && value_str.len() > 1 {
            FilterCondition::EndsWith(value_str[1..].to_string())
        } else if value_str.ends_with('+') && value_str.len() > 1 {
            FilterCondition::StartsWith(value_str[0..value_str.len() - 1].to_string())
        } else {
            FilterCondition::Exact(value_str.to_string())
        };

        Some(ParsedFilter { filter_type, condition, negate })
    }

    fn matches(&self, repo_prop: &Properties) -> bool {
        let matched = match &self.filter_type {
            FilterType::Path => {
                let target_string = &repo_prop.relative_path;
                match &self.condition {
                    FilterCondition::Exact(s) => target_string == s,
                    FilterCondition::StartsWith(s) => target_string.starts_with(s),
                    FilterCondition::EndsWith(s) => target_string.ends_with(s),
                    FilterCondition::Contains(s) => target_string.contains(s),
                    _ => false,
                }
            }
            FilterType::Name => {
                let target_string = &repo_prop.name;
                match &self.condition {
                    FilterCondition::Exact(s) => target_string == s,
                    FilterCondition::StartsWith(s) => target_string.starts_with(s),
                    FilterCondition::EndsWith(s) => target_string.ends_with(s),
                    FilterCondition::Contains(s) => target_string.contains(s),
                    _ => false,
                }
            }
            FilterType::Branch => {
                let target_string = &repo_prop.branch;
                match &self.condition {
                    FilterCondition::Exact(s) => target_string == s,
                    FilterCondition::StartsWith(s) => target_string.starts_with(s),
                    FilterCondition::EndsWith(s) => target_string.ends_with(s),
                    FilterCondition::Contains(s) => target_string.contains(s),
                    _ => false,
                }
            }
            FilterType::Active => {
                let commit_time = match DateTime::parse_from_rfc3339(&repo_prop.absolute_time) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(_) => return false,
                };
                let now = Utc::now();
                let age = now.signed_duration_since(commit_time);

                match &self.condition {
                    FilterCondition::ActiveLessThan(duration) => age < *duration,
                    FilterCondition::ActiveGreaterThan(duration) => age > *duration,
                    FilterCondition::ActiveExact(duration) => {
                        let tolerance = Duration::minutes(1);
                        age >= *duration - tolerance && age <= *duration + tolerance
                    }
                    _ => false,
                }
            }
        };

        if self.negate { !matched } else { matched }
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Filter(ParsedFilter),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self, repo_prop: &Properties) -> bool {
        match self {
            Expression::Filter(f) => f.matches(repo_prop),
            Expression::Not(expr) => !expr.evaluate(repo_prop),
            Expression::And(left, right) => left.evaluate(repo_prop) && right.evaluate(repo_prop),
            Expression::Or(left, right) => left.evaluate(repo_prop) || right.evaluate(repo_prop),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    LParen,
    RParen,
    And,
    Or,
    Not,
    FilterClause(String),
    Eof,
}

struct Tokenizer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Tokenizer { input, cursor: 0 }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        if self.cursor >= self.input.len() {
            return Ok(Token::Eof);
        }

        let c = self.current_char();
        match c {
            '(' => {
                self.advance();
                Ok(Token::LParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RParen)
            }
            '&' => {
                self.advance();
                Ok(Token::And)
            }
            '|' => {
                self.advance();
                if self.cursor < self.input.len() && self.current_char() == '|' {
                    self.advance();
                    Ok(Token::Or)
                } else {
                    Err("Expected '||' for OR operator, but found single '|'".to_string())
                }
            }
            '!' => {
                self.advance();
                Ok(Token::Not)
            }
            _ => {
                let start = self.cursor;
                while self.cursor < self.input.len() {
                    let current = self.current_char();
                    if current.is_whitespace()
                        || current == '('
                        || current == ')'
                        || current == '&'
                        || current == '|'
                        || current == '!'
                    {
                        break;
                    }
                    self.advance();
                }
                let clause = self.input[start..self.cursor].to_string();
                Ok(Token::FilterClause(clause))
            }
        }
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.cursor).unwrap_or('\0')
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.cursor < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Result<Self, String> {
        let mut tokenizer = Tokenizer::new(input);
        let current_token = tokenizer.next_token()?;
        Ok(Parser { tokenizer, current_token })
    }

    fn consume(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.current_token = self.tokenizer.next_token()?;
            Ok(())
        } else {
            Err(format!("Expected {:?}, but got {:?}", expected, self.current_token))
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token.clone() {
            Token::LParen => {
                self.consume(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            }
            Token::Not => {
                self.consume(Token::Not)?;
                let expr = self.parse_primary()?;
                Ok(Expression::Not(Box::new(expr)))
            }
            Token::FilterClause(clause) => {
                let filter = ParsedFilter::parse(&clause)
                    .ok_or_else(|| format!("Invalid filter clause: {}", clause))?;
                let expr = Expression::Filter(filter);
                self.current_token = self.tokenizer.next_token()?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn parse_and_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;
        while self.current_token == Token::And {
            self.consume(Token::And)?;
            let right = self.parse_primary()?;
            left = Expression::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_or_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_and_expression()?;
        while self.current_token == Token::Or {
            self.consume(Token::Or)?;
            let right = self.parse_and_expression()?;
            left = Expression::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or_expression()
    }

    fn parse(&mut self) -> Result<Expression, String> {
        let expr = self.parse_expression()?;
        if self.current_token != Token::Eof {
            return Err(format!("Unexpected token at end of expression: {:?}", self.current_token));
        }
        Ok(expr)
    }
}

pub fn filter_repositories(repositories: &mut Repositories, filter_str: &str) -> Repositories {
    if filter_str.is_empty() {
        return repositories.clone();
    }

    let mut parser = match Parser::new(filter_str) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error initializing parser: {}", e);
            return repositories.clone();
        }
    };

    let expression = match parser.parse() {
        Ok(expr) => expr,
        Err(e) => {
            eprintln!("Error parsing filter expression: {}", e);
            return repositories.clone();
        }
    };

    let filtered_props: Vec<Properties> = repositories
        .props
        .iter()
        .filter(|prop| expression.evaluate(prop))
        .cloned()
        .collect();

    Repositories {
        props: filtered_props,
        lens: PropertyLengths::default(),
    }
}
