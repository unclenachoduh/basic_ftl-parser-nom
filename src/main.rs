#[macro_use]
extern crate nom;

use nom::{alphanumeric1, space, types::CompleteStr};

named!(key<CompleteStr,CompleteStr>,
	call!(alphanumeric1)
);

named!(operator<CompleteStr,CompleteStr>,
	tag!("=")
);

named!(value<CompleteStr,CompleteStr>,
	recognize!(many1!(alt_complete!(alphanumeric1 | space)))
);

named!(statement<CompleteStr,(CompleteStr,CompleteStr)>,
	ws!(
		do_parse!(
			k: key >>
			operator >>
			val: value >>
			(k, val)
		)
	)
);

fn main() {
    let mut source = CompleteStr("key = val val val");

    let test = statement(source);

    println!("{:?}", test);

}