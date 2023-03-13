use crate::value_structs::{
    hashtag::Hashtag,
    id::{PostId, UserId},
    image_url::ImageUrl,
};
use derive_more::Display;
use std::collections::HashSet;

#[derive(Debug, Display)]
pub enum CreatePostEntityError {
    ImagesMoreThanFour,
}

impl std::error::Error for CreatePostEntityError {}

#[non_exhaustive]
#[derive(Debug)]
pub struct PostEntity {
    pub id: PostId,
    pub user_id: UserId,
    pub image_urls: Vec<ImageUrl>,
    pub text: String,
    pub likers: Vec<UserId>,
    pub hashtags: HashSet<Hashtag>,
}

impl PostEntity {
    pub fn new(
        id: &PostId,
        user_id: &UserId,
        image_urls: &[ImageUrl],
        text: &str,
        likers: &[UserId],
        hashtags: &HashSet<Hashtag>,
    ) -> Result<Self, CreatePostEntityError> {
        if image_urls.len() > 4 {
            return Err(CreatePostEntityError::ImagesMoreThanFour);
        }

        let new_post = Self {
            id: id.to_owned(),
            user_id: user_id.to_owned(),
            image_urls: image_urls.to_owned(),
            text: text.to_owned(),
            likers: likers.to_vec(),
            hashtags: hashtags.to_owned(),
        };

        Ok(new_post)
    }
}
