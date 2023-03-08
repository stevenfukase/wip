use crate::value_structs::{
    hashtag::Hashtag,
    id::{PostId, UserId},
    image_url::ImageUrl,
};
use std::collections::HashSet;

#[non_exhaustive]
#[derive(Debug)]
pub struct PostEntity {
    pub id: PostId,
    pub user_id: UserId,
    pub image_urls: [ImageUrl; 4],
    pub text: String,
    pub likers: Vec<UserId>,
    pub hashtags: HashSet<Hashtag>,
}

impl PostEntity {
    pub fn new(
        id: &PostId,
        user_id: &UserId,
        image_urls: &[ImageUrl; 4],
        text: &str,
        likers: &[UserId],
        hashtags: &HashSet<Hashtag>,
    ) -> Self {
        Self {
            id: id.to_owned(),
            user_id: user_id.to_owned(),
            image_urls: image_urls.to_owned(),
            text: text.to_owned(),
            likers: likers.to_vec(),
            hashtags: hashtags.to_owned(),
        }
    }
}
