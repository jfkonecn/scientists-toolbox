use uuid::Uuid;
use yew::use_ref;

pub fn use_random_id() -> String {
    let id_ref = use_ref(|| Uuid::new_v4().to_string());
    id_ref.to_string()
}
