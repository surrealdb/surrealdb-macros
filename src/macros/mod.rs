macro_rules! uppercase {
	($token:ident) => {
		$token.to_string().to_ascii_uppercase()
	};
}

pub(super) mod sql;
