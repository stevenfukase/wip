use application::mappers::presenter::Presenter;
use domain::{
    entities::post_entity::PostEntity,
    value_structs::{
        hashtag::Hashtag,
        id::{PostId, UserId},
        image_url::ImageUrl,
    },
};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug)]
pub struct PostPresenter {
    pub id: PostId,
    pub user_id: UserId,
    pub image_urls: [ImageUrl; 4],
    pub text: String,
    pub likers: Vec<UserId>,
    pub hashtags: Vec<Hashtag>,
}

impl Presenter<PostEntity> for PostPresenter {
    fn to_api(post_entity: &PostEntity) -> Self {
        Self {
            id: post_entity.id.to_owned(),
            user_id: post_entity.user_id.to_owned(),
            image_urls: post_entity.image_urls.to_owned(),
            text: post_entity.text.to_owned(),
            likers: post_entity.likers.to_vec(),
            hashtags: Vec::from_iter(post_entity.hashtags.clone()),
        }
    }
}
