mod lib;

fn main() {
    let mut post = lib::Post::new();
    let text = "I ate a salada for lunch today";

    post.add_text(text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(text, post.content());
}
