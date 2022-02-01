use crate::command::Command;

pub fn caption() -> Command {
    Command::builder()
        .name("caption")
        .run(|args| {
            println!("Hey!!! Wassup!!")
        })
        .build()
}
