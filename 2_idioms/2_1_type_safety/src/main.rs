mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}

mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

struct New;
struct Unmoderated;
struct Published;
struct Deleted;

#[derive(Clone)]
pub struct Post<S> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: S,
}

impl Post<New> {
    pub fn new(id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> Self {
        Self {
            id,
            user_id,
            title,
            body,
            state: New,
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            state: Unmoderated,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            state: Published,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }

    pub fn deny(self) -> Post<Deleted> {
        Post {
            state: Deleted,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post {
            state: Deleted,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_publish_and_delete_post() {
        let post = Post::new(
            post::Id(1),
            user::Id(1),
            post::Title(String::from("title")),
            post::Body(String::from("body")),
        );

        let unmoderated_post = post.publish();
        let published_post = unmoderated_post.allow();
        let deleted_post = published_post.delete();
    }

    #[test]
    fn should_deny_post_on_moderation() {
        let post = Post::new(
            post::Id(1),
            user::Id(1),
            post::Title(String::from("title")),
            post::Body(String::from("body")),
        );

        let unmoderated_post = post.publish();
        let denied_post = unmoderated_post.deny();
    }
}
