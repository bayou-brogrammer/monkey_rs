use std::io::{self, Stdin, Stdout, Write};

const TITLE: &str = r#"
#     #                                      ######   #####     ######                       
##   ##  ####  #    # #    # ###### #   #    #     # #     #    #     # ###### #####  #      
# # # # #    # ##   # #   #  #       # #     #     # #          #     # #      #    # #      
#  #  # #    # # #  # ####   #####    #      ######   #####     ######  #####  #    # #      
#     # #    # #  # # #  #   #        #      #   #         #    #   #   #      #####  #      
#     # #    # #   ## #   #  #        #      #    #  #     #    #    #  #      #      #      
#     #  ####  #    # #    # ######   #      #     #  #####     #     # ###### #      ###### 
                                                                                             
"#;

const HELP: &str = r#"
help:     prints this message
clear:    clears the screen
exit:     exits the repl
<source>: lexed and printed
"#;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    repl(stdin, stdout)
}

fn repl(stdin: Stdin, stdout: Stdout) -> io::Result<()> {
    println!("{TITLE}");
    println!("{HELP}");
}
