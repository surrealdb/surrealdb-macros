mod begin;
mod cancel;
mod commit;
mod create;
mod define;
mod delete;
mod ifelse;
mod info;
mod insert;
mod kill;
mod live;
mod option;
mod output;
mod relate;
mod remove;
mod select;
mod set;
mod update;
mod yuse;

use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

pub enum Statement {
	Begin(begin::Statement),
	Cancel(cancel::Statement),
	Commit(commit::Statement),
	Create(create::Statement),
	Define(define::Statement),
	Delete(delete::Statement),
	Ifelse(ifelse::Statement),
	Info(info::Statement),
	Insert(insert::Statement),
	Kill(kill::Statement),
	Live(live::Statement),
	Option(option::Statement),
	Output(output::Statement),
	Relate(relate::Statement),
	Remove(remove::Statement),
	Select(select::Statement),
	Set(set::Statement),
	Update(update::Statement),
	Use(yuse::Statement),
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match token.to_string().to_ascii_uppercase().as_str() {
			"BEGIN" => Ok(Statement::Begin(input.parse()?)),
			"CANCEL" => Ok(Statement::Cancel(input.parse()?)),
			"COMMIT" => Ok(Statement::Commit(input.parse()?)),
			"CREATE" => Ok(Statement::Create(input.parse()?)),
			"DEFINE" => Ok(Statement::Define(input.parse()?)),
			"DELETE" => Ok(Statement::Delete(input.parse()?)),
			"IF" => Ok(Statement::Ifelse(input.parse()?)),
			"INFO" => Ok(Statement::Info(input.parse()?)),
			"INSERT" => Ok(Statement::Insert(input.parse()?)),
			"KILL" => Ok(Statement::Kill(input.parse()?)),
			"LIVE" => Ok(Statement::Live(input.parse()?)),
			"OPTION" => Ok(Statement::Option(input.parse()?)),
			"OUTPUT" => Ok(Statement::Output(input.parse()?)),
			"RELATE" => Ok(Statement::Relate(input.parse()?)),
			"REMOVE" => Ok(Statement::Remove(input.parse()?)),
			"SELECT" => Ok(Statement::Select(input.parse()?)),
			"SET" => Ok(Statement::Set(input.parse()?)),
			"UPDATE" => Ok(Statement::Update(input.parse()?)),
			"USE" => Ok(Statement::Use(input.parse()?)),
			_ => {
				let message = format!("expected one of `BEGIN`, `CANCEL`, `COMMIT`, `CREATE`, `DEFINE`, `DELETE`, `IF`, `INFO`, `INSERT`, `KILL`, `LIVE`, `OPTION`, `OUTPUT`, `RELATE`, `REMOVE`, `SELECT`, `SET`, `UPDATE` or `USE`, found `{token}`");
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for Statement {
	#[rustfmt::skip]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Statement::Begin(statement) => tokens.append_all(quote!(Statement::Begin(#statement))),
			Statement::Cancel(statement) => tokens.append_all(quote!(Statement::Cancel(#statement))),
			Statement::Commit(statement) => tokens.append_all(quote!(Statement::Commit(#statement))),
			Statement::Create(statement) => tokens.append_all(quote!(Statement::Create(#statement))),
			Statement::Define(statement) => tokens.append_all(quote!(Statement::Define(#statement))),
			Statement::Delete(statement) => tokens.append_all(quote!(Statement::Delete(#statement))),
			Statement::Ifelse(statement) => tokens.append_all(quote!(Statement::Ifelse(#statement))),
			Statement::Info(statement) => tokens.append_all(quote!(Statement::Info(#statement))),
			Statement::Insert(statement) => tokens.append_all(quote!(Statement::Insert(#statement))),
			Statement::Kill(statement) => tokens.append_all(quote!(Statement::Kill(#statement))),
			Statement::Live(statement) => tokens.append_all(quote!(Statement::Live(#statement))),
			Statement::Option(statement) => tokens.append_all(quote!(Statement::Option(#statement))),
			Statement::Output(statement) => tokens.append_all(quote!(Statement::Output(#statement))),
			Statement::Relate(statement) => tokens.append_all(quote!(Statement::Relate(#statement))),
			Statement::Remove(statement) => tokens.append_all(quote!(Statement::Remove(#statement))),
			Statement::Select(statement) => tokens.append_all(quote!(Statement::Select(#statement))),
			Statement::Set(statement) => tokens.append_all(quote!(Statement::Set(#statement))),
			Statement::Update(statement) => tokens.append_all(quote!(Statement::Update(#statement))),
			Statement::Use(statement) => tokens.append_all(quote!(Statement::Use(#statement))),
		}
	}
}
