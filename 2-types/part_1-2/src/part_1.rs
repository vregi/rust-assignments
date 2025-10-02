macro_rules! states {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, PartialEq)]
            pub struct $name;
        )*
    }
}

states!(New, Unmoderated, Published, Deleted);

pub struct Post<S> {
    title: String,
    excerpt: String,
    content: String,
    date: u32,
    state: S,
}

impl<S> Post<S>
where 
    S: std::fmt::Debug,
{
    pub fn check_state(&self) {
        println!("{:#?}", self.state)
    }
}

impl Post<New> {
    pub fn new(
        title: &str,
        excerpt: &str,
        content: &str,
        date: u32,
    ) -> Self {
        println!("New post\ntitle: {}\nexcerpt: {}\ncontent: {}\ndate: {}", title, excerpt, content, date);
        Post {
            title: title.to_string(),
            excerpt: excerpt.to_string(),
            content: content.to_string(),
            date,
            state: New,
        }
    }


    pub fn publish(self) -> Post<Unmoderated> {
        println!("Post \"{}\" moved to moderation stage.", self.title);
        let Post {title, excerpt, content, date, ..} = self;
        Post {
            title,
            excerpt,
            content,
            date,
            state: Unmoderated,
        }
        
    }
}


impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        println!("Post \"{}\" was published.", self.title);
        let Post {title, excerpt, content, date, ..} = self;
        Post {
            title,
            excerpt,
            content,
            date,
            state: Published,
        }
    }

    pub fn deny(self) -> Post<Deleted> {
        println!("Post \"{}\" was denied. Deleting post.", self.title);
        let Post {title, excerpt, content, date, ..} = self;
        Post {
            title,
            excerpt,
            content,
            date,
            state: Deleted,
        }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted>{
        println!("Post \"{}\" was deleted.", self.title);
        let Post {title, excerpt, content, date, ..} = self;
        Post {
            title,
            excerpt,
            content,
            date,
            state: Deleted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> (&'static str, &'static str, &'static str, u32) {
        ("Title", "Excerpt", "Content", 20250928)
    }

    #[test]
    fn new_post_has_expected_fields_and_state() {
        let (t, e, c, d) = sample();
        let post: Post<New> = Post::new(t, e, c, d);

        assert_eq!(post.title, t);
        assert_eq!(post.excerpt, e);
        assert_eq!(post.content, c);
        assert_eq!(post.date, d);

        assert_eq!(post.state, New);
        assert_eq!(format!("{:#?}", post.state), "New");
    }

    #[test]
    fn publish_moves_to_unmoderated() {
        let (t, e, c, d) = sample();
        let post = Post::new(t, e, c, d);
        let post: Post<Unmoderated> = post.publish();

        assert_eq!(post.state, Unmoderated);
        assert_eq!(format!("{:?}", post.state), "Unmoderated");
    }

    #[test]
    fn allow_moves_to_published() {
        let (t, e, c, d) = sample();
        let post = Post::new(t, e, c, d).publish();
        let post: Post<Published> = post.allow();

        assert_eq!(post.state, Published);
    }

    #[test]
    fn deny_moves_to_deleted_from_unmoderated() {
        let (t, e, c, d) = sample();
        let post = Post::new(t, e, c, d).publish();
        let post: Post<Deleted> = post.deny();

        assert_eq!(post.state, Deleted);
    }

    #[test]
    fn delete_moves_to_deleted_from_published() {
        let (t, e, c, d) = sample();
        let post = Post::new(t, e, c, d).publish().allow();
        let post: Post<Deleted> = post.delete();

        assert_eq!(post.state, Deleted);
    }

    #[test]
    fn full_success_path_compiles_and_transitions() {
        let (t, e, c, d) = sample();
        let published: Post<Published> = Post::new(t, e, c, d).publish().allow();
        assert_eq!(published.state, Published);
    }
}
