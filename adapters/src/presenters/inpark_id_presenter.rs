use application::mappers::presenter::Presenter;
use domain::value_structs::id::InparkId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InparkIdPresenter {
    pub inpark_id: InparkId,
}

impl Presenter<InparkId> for InparkIdPresenter {
    fn to_api(inpark_id: &InparkId) -> Self {
        Self {
            inpark_id: inpark_id.to_owned(),
        }
    }
}
