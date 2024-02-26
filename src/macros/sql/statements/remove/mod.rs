mod database;
mod event;
mod field;
mod index;
mod namespace;
mod param;
mod scope;
mod table;
mod token;
mod user;

use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

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

pub enum Statement {
	Namespace(namespace::Statement),
	Database(database::Statement),
	User(user::Statement),
	Token(token::Statement),
	Scope(scope::Statement),
	Param(param::Statement),
	Table(table::Statement),
	Event(event::Statement),
	Field(field::Statement),
	Index(index::Statement),
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => Ok(Statement::Namespace(input.parse()?)),
			"DATABASE" | "DB" => Ok(Statement::Database(input.parse()?)),
			"USER" => Ok(Statement::User(input.parse()?)),
			"TOKEN" => Ok(Statement::Token(input.parse()?)),
			"SCOPE" => Ok(Statement::Scope(input.parse()?)),
			"PARAM" => Ok(Statement::Param(input.parse()?)),
			"TABLE" => Ok(Statement::Table(input.parse()?)),
			"EVENT" => Ok(Statement::Event(input.parse()?)),
			"FIELD" => Ok(Statement::Field(input.parse()?)),
			"INDEX" => Ok(Statement::Index(input.parse()?)),
			_ => {
				let message = format!("expected one of `NAMESPACE`, `NS`, `DATABASE`, `DB`, `USER`, `TOKEN`, `SCOPE`, `PARAM`, `TABLE`, `EVENT`, `FIELD`, `INDEX`, found `{token}`");
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
            Statement::User(statement) => tokens.append_all(quote!(RemoveStatement::User(#statement))),
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
