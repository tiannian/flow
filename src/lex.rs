use std::io::BufRead;

pub enum Token {
    Unknown,
    ModuleBegin,
    ModuleEnd,
    ImportPackage(String),
    PackageAlias(String),
    VariableName(String),
    VariableType(String),
    VariableWire(String,String),
    StatementEnd,
}

pub struct Lex <T> {
    reader: T,
    buffer: String,
    pos: usize,
}

impl<T: BufRead> Lex<T> {
    fn new(reader: T) -> Lex<T> {
        Lex {
            reader,
            buffer: String::new(),
            pos:0
        }
    }
    
    fn next_word(&mut self) -> Token {
        let mut result = String::new();
        let mut token = Token::Unknown;
        self.reader.read_line(&mut self.buffer);
        let chars = self.buffer.chars();
        for x in chars {
            self.pos = self.pos + 1;
            token = match x {
                ';' => Token::StatementEnd,
                'i' => {
                    result.push(x);
                    Token::ImportPackage(result)
                }
                _ => {
                    result.push(x);
                    Token::Unknown
                },
            }
        }
        if self.pos + 1 == self.buffer.len() {
            Token::StatementEnd
        } else {
            Token::Unknown
        }
    }
    
}
