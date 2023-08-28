pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPostNoReview {
    content: String,
}

pub struct PendingReviewPostOneReview {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPostNoReview {
        PendingReviewPostNoReview {
            content: self.content,
        }
    }
}

impl PendingReviewPostNoReview {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(self) -> PendingReviewPostOneReview {
        PendingReviewPostOneReview {
            content: self.content,
        }
    }
}

impl PendingReviewPostOneReview {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
