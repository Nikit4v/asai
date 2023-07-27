// Low Level API
use asai::iter::*;

// .get() -> Option<Item>
// .list() -> Vec<Item>

fn foo() {
	let data = "";
	data.sections() // Get section iterator
	.get("Events") // Get lines of section
	.list("Dialogue") // Get lines named "Dialogue" 
	.map(|x|
		println!("{}", x.get("Style")) // Print name of line's style
	)
}
