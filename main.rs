use std::iter::FromIterator;
use std::ops::Range;

enum State {
    NotIn,
    In,
}

struct Parser {
    input: String, 
    state: State, 
    index: usize,
}

#[derive(Debug, PartialEq, Eq, Copy)]
enum TokenType {
    Empty, 
    Unicode,
    Single,
    Any,
    Group,
}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        return *self
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: char,
    group_values: Vec<char>
}

impl Clone for Token {
    fn clone(&self) -> Token {
        return Token {
            token_type: self.token_type,
            value: self.value,
            group_values: self.group_values.clone()
        }
    }
}

impl Parser {
    fn new(pattern: &str) -> Parser {
        return Parser {
            input: pattern.to_string(), 
            state: State::NotIn,
            index: 0
        }
    }

    fn Next(&mut self) -> Result<Token, &'static str> {
       if self.index == self.input.len() {
            return Ok(Token {
                token_type: TokenType::Empty, 
                value: '-',
                group_values: vec![]
            })
       } 
       match self.input.chars().nth(self.index).unwrap() {
        '[' => {
            let n = self.input.len();
            
            if self.index + 2 >= n {
               return Err("Got a group opening with no closing"); 
            }
            
            self.index += 1;
            let mut first = self.input.chars().nth(self.index).unwrap();
            if !first.is_alphanumeric() {
                return Err("Expected alphanumeric char after group opening");
            }
            
            if self.input.chars().nth(self.index + 1).unwrap() == '-' {
                self.index += 1;

                if self.index + 1 >= n {
                    return Err("Got a group range with no end to it");
                } 
                self.index += 1;
                let last = self.input.chars().nth(self.index).unwrap();
                if !last.is_alphanumeric() {
                    return Err("Expected alphanumeric char after '-' in group");
                }

                if self.index + 1 >= n {
                    return Err("Got a group range with no group closing");
                }
                self.index += 1;
                if self.input.chars().nth(self.index).unwrap() != ']' {
                    return Err("Expected ]");
                }
                self.index += 1;

                return Ok(Token {
                    token_type: TokenType::Group,
                    value: '-', 
                    group_values: Vec::from_iter(first..=last)
                }) 
            }
            let mut values = vec![first];
            self.index += 1;
            while self.index < self.input.len() {
                let mut v = self.input.chars().nth(self.index).unwrap();
                if  v == ']' {
                    self.index += 1;
                    return Ok(Token{
                        token_type: TokenType::Group,
                        value: '-',
                        group_values: values
                    })
                }
                values.push(v);
                self.index += 1;
            }
            return Err("Expected ] at the end of a group");
        }
        '*' => {
            self.index += 1; 
            return Ok(Token {
                token_type: TokenType::Any, 
                value: '*', 
                group_values: vec![]
            })
        },
        '?' => {
            self.index += 1; 
            return Ok(Token {
                token_type: TokenType::Single,
                value: '?',
                group_values: vec![]
            })
        }
        x => {
            if !x.is_alphanumeric() {
                return Err("Expected alphanumeric char");
            }
            self.index += 1;
            return Ok(Token {
                token_type: TokenType::Unicode,     
                value: x,
                group_values: vec![]
            }) 
       }
       }
    }
}

struct Matcher {
    tokens: Vec<Token>
}

struct Curser {
    index: usize,
    state: usize
}

impl Matcher {
    fn new() -> Matcher {
        return Matcher {
            tokens: vec![]
        }
    }

    fn match_str(&mut self, to_match: &str) -> bool {
        let mut cursers = vec![Curser {
            index: 0,
            state: 0
        }];
        
        let t: Vec<_> = to_match.chars().collect();
        let n = to_match.len();

        while cursers.len() > 0 {
            let c = cursers.remove(0);
            if c.index == n {                
                if self.tokens[c.state].token_type == TokenType::Empty {
                    return true;
                }
                println!("state is: {:?} at index: {}", self.tokens[c.state].token_type, c.state);
                if c.state + 1 == self.tokens.len() - 1 {
                    return true;
                } 

                continue
            }
            
            match self.tokens[c.state].token_type  {
            TokenType::Any => {
                cursers.push(Curser {
                    index: c.index + 1,
                    state: c.state
                });
                cursers.push(Curser {
                    index: c.index + 1,
                    state: c.state + 1
                });
            }
            TokenType::Single => {
                cursers.push(Curser {
                    index: c.index + 1,
                    state: c.state + 1
                })
            }
            TokenType::Unicode => {
                if t[c.index] != self.tokens[c.state].value {
                    continue 
                }
                cursers.push(Curser {
                    index: c.index + 1,
                    state: c.state + 1
                })
            }
            TokenType::Group => {
                if !self.tokens[c.state].group_values.contains(&t[c.index]) {
                    continue
                }
                cursers.push(Curser {
                    index: c.index + 1,
                    state: c.state + 1
                })
            }
            TokenType::Empty => { return false } 
            }

        } 

        return false
    }
}

fn main() {
    let mut p = Parser::new("s*me");
    let mut m = Matcher::new();   

    let mut t = p.Next().unwrap(); 
    while t.token_type != TokenType::Empty {
        m.tokens.push(t.clone());
        t = p.Next().unwrap();
    }
    m.tokens.push(t);

    println!("{}", m.match_str("sooooome"));
}
