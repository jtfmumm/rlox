use std::process::Command;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning=***** {} *****", format!($($tokens)*))
    }
}

fn main() {
	Command::new("python")
	        .arg("tools/ast_gen.py")
	        .spawn()
	        .expect("python command failed to start");

	p!("\n");
	p!("Python script generated Expr and Stmt enums");
	p!("\n\n")
}
