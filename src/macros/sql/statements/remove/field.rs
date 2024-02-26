use crate::macros::sql::kw;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use rust_decimal::Decimal;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::Error;
use syn::Ident;
use syn::LitFloat;
use syn::LitInt;
use syn::Token;

#[derive(Debug)]
enum Number {
	Int(i64),
	Float(f64),
	Decimal(String),
}

impl Parse for Number {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(LitInt) {
			let token: LitInt = input.parse()?;
			let Ok(value) = token.base10_parse::<i64>() else {
				let string = token.to_string();
				if string.parse::<Decimal>().is_err() {
					let message = format!("expected an integer, found `{token}`");
					return Err(Error::new_spanned(token, message));
				}
				return Ok(Number::Decimal(string));
			};
			Ok(Number::Int(value))
		} else if input.peek(LitFloat) {
			let token: LitFloat = input.parse()?;
			let Ok(value) = token.base10_parse::<f64>() else {
				let string = token.to_string();
				if string.parse::<Decimal>().is_err() {
					let message = format!("expected a float, found `{token}`");
					return Err(Error::new_spanned(token, message));
				}
				return Ok(Number::Decimal(string));
			};
			Ok(Number::Float(value))
		} else {
			let message = format!("expected a number, found `{input}`");
			Err(Error::new(input.span(), message))
		}
	}
}

impl ToTokens for Number {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Number::Int(int) => tokens.append_all(quote!(Number::Int(#int))),
			Number::Float(float) => tokens.append_all(quote!(Number::Float(#float))),
			Number::Decimal(decimal) => {
				tokens.append_all(quote!(Number::Decimal(#decimal.parse().unwrap())));
			}
		}
	}
}

#[derive(Debug)]
enum Part {
	All,
	Index(Number),
	Field(Ident),
}

impl ToTokens for Part {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Part::All => tokens.append_all(quote!(Part::All)),
			Part::Index(number) => tokens.append_all(quote!(Part::Index(#number))),
			Part::Field(field) => {
				let field = field.to_string();
				tokens.append_all(quote!(Part::Field(#field.to_owned().into())));
			}
		}
	}
}

struct Idiom(Vec<Part>);

impl Parse for Idiom {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parts = Vec::new();
		if input.peek(Ident) {
			parts.push(Part::Field(input.parse()?));
			if input.peek(Bracket) {
				let content;
				syn::bracketed!(content in input);
				if content.peek(Token![*]) {
					let _token: Token![*] = content.parse()?;
					parts.push(Part::All);
				} else if content.peek(LitInt) || content.peek(LitFloat) {
					parts.push(Part::Index(content.parse()?));
				} else {
					let message = format!("expected `*` or a number, found `{content}`");
					return Err(Error::new(content.span(), message));
				}
			}
		} else if input.peek(Token![*]) {
			let _token: Token![*] = input.parse()?;
			parts.push(Part::All);
		} else {
			let message = format!("expected `*` or field name, found `{input}`");
			return Err(Error::new(input.span(), message));
		}
		Ok(Self(parts))
	}
}

struct Name(Punctuated<Idiom, Token![.]>);

impl Parse for Name {
	fn parse(input: ParseStream) -> Result<Self> {
		let parts = Punctuated::parse_separated_nonempty(input)?;
		Ok(Name(parts))
	}
}

impl ToTokens for Name {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let mut name = Vec::new();
		for Idiom(parts) in &self.0 {
			name.extend(parts);
		}
		let len = name.len();
		let mut parts = quote! {
			let mut parts = Vec::with_capacity(#len);
		};
		for part in name {
			parts.append_all(quote! {
				parts.push(#part);
			});
		}
		parts.append_all(quote!(parts));
		tokens.append_all(quote!(Idiom({ #parts })));
	}
}

pub struct Statement {
	name: Name,
	_on: kw::On,
	table: Ident,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		if !input.peek(Ident) {
			return Err(input.parse::<Ident>().unwrap_err());
		}
		Ok(Self {
			name: input.parse()?,
			_on: input.parse()?,
			table: {
				let token: Ident = input.parse()?;
				match uppercase!(token).as_str() {
					"TABLE" => input.parse()?,
					_ => token,
				}
			},
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		let table = self.table.to_string();
		tokens.append_all(quote!(RemoveFieldStatement {
			name: #name,
			what: #table.to_owned().into(),
		}));
	}
}
