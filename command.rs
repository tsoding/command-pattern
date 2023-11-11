extern crate proc_macro;
use proc_macro::*;
use std::iter::FromIterator;

#[derive(Debug)]
struct Command {
    name: String,
    description: String,
    run: String,
}

static mut COMMANDS: Vec<Command> = Vec::new();

fn expect_literal(lex: &mut impl Iterator<Item=TokenTree>) -> Literal {
    match lex.next() {
        Some(TokenTree::Literal(literal)) => literal,
        Some(_token) => panic!("Expected literal but got something else"),
        None => panic!("Expected literal but got nothing"),
    }
}

fn expect_specific_punct(lex: &mut impl Iterator<Item=TokenTree>, ch: char) -> Punct {
    match lex.next() {
        Some(TokenTree::Punct(punct)) => if punct.as_char() == ch {
            punct
        } else {
            panic!("Expected punct `{expected}`, but got `{actual}`", expected = ch, actual = punct.as_char())
        },
        Some(_token) => panic!("Expected punct but got something else"),
        None => panic!("Expected punct but got nothing"),
    }
}

fn expect_specific_ident(lex: &mut impl Iterator<Item=TokenTree>, name: &str) -> Ident {
    match lex.next() {
        Some(TokenTree::Ident(ident)) => {
            if ident.to_string() == name {
                ident
            } else {
                panic!("Expected indent `{expected}` but got `{actual}`", expected = ident, actual = name)
            }
        },
        Some(_token) => panic!("Expected ident but got something else"),
        None => panic!("Expected ident but got nothing"),
    }
}

fn expect_ident(lex: &mut impl Iterator<Item=TokenTree>) -> Ident {
    match lex.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(_token) => panic!("Expected ident but got something else"),
        None => panic!("Expected ident but got nothing"),
    }
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args_iter = args.into_iter();
    let name = expect_literal(&mut args_iter).to_string();
    let _ = expect_specific_punct(&mut args_iter, ',');
    let description = expect_literal(&mut args_iter).to_string();

    let mut input_iter = input.clone().into_iter();
    let _ = expect_specific_ident(&mut input_iter, "fn");
    let run = expect_ident(&mut input_iter).to_string();

    unsafe {
        COMMANDS.push(Command {name, description, run})
    }
    input
}

fn render_command_fields(command: &Command) -> TokenStream {
    let mut tokens: Vec<TokenTree> = Vec::new();
    tokens.push(TokenTree::Ident(Ident::new("name", Span::call_site())));
    tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
    tokens.push(TokenTree::Literal(command.name.parse::<Literal>().unwrap()));
    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

    tokens.push(TokenTree::Ident(Ident::new("description", Span::call_site())));
    tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
    tokens.push(TokenTree::Literal(command.description.parse::<Literal>().unwrap()));
    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

    tokens.push(TokenTree::Ident(Ident::new("run", Span::call_site())));
    tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
    tokens.push(TokenTree::Ident(Ident::new(&command.run, Span::call_site())));
    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

    TokenStream::from_iter(tokens)
}

fn render_commands() -> TokenStream {
    let mut tokens: Vec<TokenTree> = Vec::new();
    unsafe {
        for command in COMMANDS.iter() {
            tokens.push(TokenTree::Ident(Ident::new("Command", Span::call_site())));
            tokens.push(TokenTree::Group(Group::new(Delimiter::Brace, render_command_fields(&command))));
            tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
        }
    }
    TokenStream::from_iter(tokens)
}

#[proc_macro]
pub fn command_list(_item: TokenStream) -> TokenStream {
    let stream = TokenStream::from_iter([
        TokenTree::Punct(Punct::new('&', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Bracket, render_commands())),
    ]);
    stream
}
