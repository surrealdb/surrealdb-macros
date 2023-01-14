use bigdecimal::BigDecimal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
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

pub enum Statement {
	Namespace(NamespaceStatement),
	Database(DatabaseStatement),
	Login(LoginStatement),
	Token(TokenStatement),
	Scope(ScopeStatement),
	Param(ParamStatement),
	Table(TableStatement),
	Event(EventStatement),
	Field(FieldStatement),
	Index(IndexStatement),
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => Ok(Statement::Namespace(input.parse()?)),
			"DATABASE" | "DB" => Ok(Statement::Database(input.parse()?)),
			"LOGIN" => Ok(Statement::Login(input.parse()?)),
			"TOKEN" => Ok(Statement::Token(input.parse()?)),
			"SCOPE" => Ok(Statement::Scope(input.parse()?)),
			"PARAM" => Ok(Statement::Param(input.parse()?)),
			"TABLE" => Ok(Statement::Table(input.parse()?)),
			"EVENT" => Ok(Statement::Event(input.parse()?)),
			"FIELD" => Ok(Statement::Field(input.parse()?)),
			"INDEX" => Ok(Statement::Index(input.parse()?)),
			_ => {
				let message = format!("expected one of `NAMESPACE`, `NS`, `DATABASE`, `DB`, `LOGIN`, `TOKEN`, `SCOPE`, `PARAM`, `TABLE`, `EVENT`, `FIELD`, `INDEX`, found `{token}`");
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for Statement {
	#[rustfmt::skip]
	fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Statement::Namespace(statement) => tokens.append_all(quote!(RemoveStatement::Namespace(#statement))),
            Statement::Database(statement) => tokens.append_all(quote!(RemoveStatement::Database(#statement))),
            Statement::Login(statement) => tokens.append_all(quote!(RemoveStatement::Login(#statement))),
            Statement::Token(statement) => tokens.append_all(quote!(RemoveStatement::Token(#statement))),
            Statement::Scope(statement) => tokens.append_all(quote!(RemoveStatement::Scope(#statement))),
            Statement::Param(statement) => tokens.append_all(quote!(RemoveStatement::Param(#statement))),
            Statement::Table(statement) => tokens.append_all(quote!(RemoveStatement::Table(#statement))),
            Statement::Event(statement) => tokens.append_all(quote!(RemoveStatement::Event(#statement))),
            Statement::Field(statement) => tokens.append_all(quote!(RemoveStatement::Field(#statement))),
            Statement::Index(statement) => tokens.append_all(quote!(RemoveStatement::Index(#statement))),
        }
	}
}

pub struct NamespaceStatement {
	name: String,
}

impl Parse for NamespaceStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		Ok(Self {
			name: token.to_string(),
		})
	}
}

impl ToTokens for NamespaceStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		tokens.append_all(quote!(RemoveNamespaceStatement {
			name: #name.to_owned().into(),
		}));
	}
}

pub struct DatabaseStatement {
	name: String,
}

impl Parse for DatabaseStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		Ok(Self {
			name: token.to_string(),
		})
	}
}

impl ToTokens for DatabaseStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		tokens.append_all(quote!(RemoveDatabaseStatement {
			name: #name.to_owned().into(),
		}));
	}
}

enum Base {
	Ns,
	Db,
}

impl Parse for Base {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => Ok(Self::Ns),
			"DATABASE" | "DB" => Ok(Self::Db),
			_ => {
				let message =
					format!("expected one of `NAMESPACE`, `NS`, `DATABASE`, `DB`, found `{token}`");
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for Base {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match &self {
			Base::Ns => tokens.append_all(quote!(Base::Ns)),
			Base::Db => tokens.append_all(quote!(Base::Db)),
		}
	}
}

pub struct LoginStatement {
	name: String,
	base: Base,
}

impl Parse for LoginStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		let name = token.to_string();
		let token: Ident = input.parse()?;
		if uppercase!(token) != "ON" {
			let message = format!("expected `ON`, found `{token}`");
			return Err(Error::new_spanned(token, message));
		}
		let base: Base = input.parse()?;
		Ok(Self {
			name,
			base,
		})
	}
}

impl ToTokens for LoginStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		let base = &self.base;
		tokens.append_all(quote!(RemoveLoginStatement {
			name: #name.to_owned().into(),
			base: #base,
		}));
	}
}

enum BaseOrScope {
	Base(Base),
	Sc(String),
}

impl Parse for BaseOrScope {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => Ok(Self::Base(Base::Ns)),
			"DATABASE" | "DB" => Ok(Self::Base(Base::Db)),
			"SCOPE" => {
				let token: Ident = input.parse()?;
				Ok(Self::Sc(token.to_string()))
			}
			_ => {
				let message = format!(
					"expected one of `NAMESPACE`, `NS`, `DATABASE`, `DB`, `SCOPE`, found `{token}`"
				);
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for BaseOrScope {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match &self {
			BaseOrScope::Base(base) => tokens.append_all(quote!(#base)),
			BaseOrScope::Sc(scope) => tokens.append_all(quote!(Base::Sc(#scope.to_owned().into()))),
		}
	}
}

pub struct TokenStatement {
	name: String,
	base: BaseOrScope,
}

impl Parse for TokenStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		let name = token.to_string();
		let token: Ident = input.parse()?;
		if uppercase!(token) != "ON" {
			let message = format!("expected `ON`, found `{token}`");
			return Err(Error::new_spanned(token, message));
		}
		let base: BaseOrScope = input.parse()?;
		Ok(Self {
			name,
			base,
		})
	}
}

impl ToTokens for TokenStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		let base = &self.base;
		tokens.append_all(quote!(RemoveTokenStatement {
			name: #name.to_owned().into(),
			base: #base,
		}));
	}
}

pub struct ScopeStatement {
	name: String,
}

impl Parse for ScopeStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		Ok(Self {
			name: token.to_string(),
		})
	}
}

impl ToTokens for ScopeStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		tokens.append_all(quote!(RemoveScopeStatement {
			name: #name.to_owned().into(),
		}));
	}
}

pub struct ParamStatement {
	name: String,
}

impl Parse for ParamStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		Ok(Self {
			name: token.to_string(),
		})
	}
}

impl ToTokens for ParamStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		tokens.append_all(quote!(RemoveParamStatement {
			name: #name.to_owned().into(),
		}));
	}
}

pub struct TableStatement {
	name: String,
}

impl Parse for TableStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		Ok(Self {
			name: token.to_string(),
		})
	}
}

impl ToTokens for TableStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		tokens.append_all(quote!(RemoveTableStatement {
			name: #name.to_owned().into(),
		}));
	}
}

struct Name(String);
struct Table(String);

fn extract_name_and_table(input: ParseStream) -> Result<(Name, Table)> {
	let token: Ident = input.parse()?;
	let name = token.to_string();
	let token: Ident = input.parse()?;
	if uppercase!(token) != "ON" {
		let message = format!("expected `ON`, found `{token}`");
		return Err(Error::new_spanned(token, message));
	}
	let token: Ident = input.parse()?;
	let table = match uppercase!(token).as_str() {
		"TABLE" => {
			let token: Ident = input.parse()?;
			token.to_string()
		}
		_ => token.to_string(),
	};
	Ok((Name(name), Table(table)))
}

pub struct EventStatement {
	name: Name,
	table: Table,
}

impl Parse for EventStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let (name, table) = extract_name_and_table(input)?;
		Ok(Self {
			name,
			table,
		})
	}
}

impl ToTokens for EventStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Name(name) = &self.name;
		let Table(table) = &self.table;
		tokens.append_all(quote!(RemoveEventStatement {
			name: #name.to_owned().into(),
			what: #table.to_owned().into(),
		}));
	}
}

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
                if string.parse::<BigDecimal>().is_err() {
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
                if string.parse::<BigDecimal>().is_err() {
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
	Field(String),
}

impl ToTokens for Part {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Part::All => tokens.append_all(quote!(Part::All)),
			Part::Index(number) => tokens.append_all(quote!(Part::Index(#number))),
			Part::Field(field) => tokens.append_all(quote!(Part::Field(#field.to_owned().into()))),
		}
	}
}

struct Idiom(Vec<Part>);

impl Parse for Idiom {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parts = Vec::new();
		if input.peek(Ident) {
			let token: Ident = input.parse()?;
			parts.push(Part::Field(token.to_string()));
			if input.peek(Bracket) {
				let content;
				syn::bracketed!(content in input);
				if content.peek(Token![*]) {
					let _token: Token![*] = content.parse()?;
					parts.push(Part::All);
				} else if content.peek(LitInt) || content.peek(LitFloat) {
					let number = content.parse()?;
					parts.push(Part::Index(number));
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

pub struct FieldStatement {
	name: Vec<Part>,
	table: String,
}

impl Parse for FieldStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		if !input.peek(Ident) {
			return Err(input.parse::<Ident>().unwrap_err());
		}
		let mut name = Vec::new();
		let idioms = Punctuated::<Idiom, Token![.]>::parse_separated_nonempty(input)?;
		for Idiom(parts) in idioms {
			name.extend(parts);
		}
		let token: Ident = input.parse()?;
		if uppercase!(token) != "ON" {
			let message = format!("expected `ON`, found `{token}`");
			return Err(Error::new_spanned(token, message));
		}
		let token: Ident = input.parse()?;
		let table = match uppercase!(token).as_str() {
			"TABLE" => {
				let token: Ident = input.parse()?;
				token.to_string()
			}
			_ => token.to_string(),
		};
		Ok(Self {
			name,
			table,
		})
	}
}

impl ToTokens for FieldStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self {
			name,
			table,
		} = self;
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
		tokens.append_all(quote!(RemoveFieldStatement {
			name: Idiom({
				#parts
			}),
			what: #table.to_owned().into(),
		}));
	}
}

pub struct IndexStatement {
	name: Name,
	table: Table,
}

impl Parse for IndexStatement {
	fn parse(input: ParseStream) -> Result<Self> {
		let (name, table) = extract_name_and_table(input)?;
		Ok(Self {
			name,
			table,
		})
	}
}

impl ToTokens for IndexStatement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Name(name) = &self.name;
		let Table(table) = &self.table;
		tokens.append_all(quote!(RemoveIndexStatement {
			name: #name.to_owned().into(),
			what: #table.to_owned().into(),
		}));
	}
}
