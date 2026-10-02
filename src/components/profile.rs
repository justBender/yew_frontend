use crate::components::context::auth_context::AuthContext;
use crate::components::context::auth_state::AuthState;
use crate::services::save_image::SaveImageRequest;
use js_sys::Uint8Array;
use js_sys::futures::JsFuture;
use std::borrow::BorrowMut;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, File, HtmlInputElement};
use yew::{Callback, Component, Context, ContextHandle, Html, Properties, html};
use yew_icons::{Icon, IconData};

#[warn(unused_variables)]
pub struct Profile {
    pub _listener: ContextHandle<AuthContext>,
    pub id: String,
    pub name: String,
    pub email: String,
    pub preview_image: Option<String>,
    pub file: Option<File>,
    pub roles: Vec<String>,
    pub extension: String,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ProfileProps {
    pub context_callback: Callback<AuthContext>,
}

pub enum ProfileMsg {
    AuthUpdate(AuthContext),
    SaveImage,
    FileSelected(Event),
}

impl Profile {
    pub fn prefix_host(path: &str) -> String {
        let host = "http://localhost:7878";
        if !path.is_empty() && !path.contains(host) {
            return format!("{}{}", host, path);
        }
        path.to_string()
    }
}

impl Component for Profile {
    type Message = ProfileMsg;
    type Properties = ProfileProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, handle) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(ProfileMsg::AuthUpdate))
            .expect("missing auth context");
        let preview = Some(Profile::prefix_host(
            &auth_context.state.image.clone().unwrap_or_default(),
        ));
        Self {
            id: auth_context.state.id.clone().unwrap_or_default(),
            name: auth_context.state.name.clone().unwrap_or_default(),
            email: auth_context.state.email.clone().unwrap_or_default(),
            roles: auth_context.state.roles.clone().unwrap_or_default(),
            preview_image: preview.or(Some("".to_string())),
            file: None,
            extension: "".to_string(),
            _listener: handle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let copy_self = self.borrow_mut();
        match msg {
            Self::Message::AuthUpdate(auth_context) => {
                let copy_self = self.borrow_mut();
                copy_self.id = auth_context.state.id.unwrap_or_default();
                copy_self.name = auth_context.state.name.clone().unwrap_or_default();
                copy_self.roles = auth_context.state.roles.clone().unwrap_or_default();
                copy_self.preview_image = Some(Profile::prefix_host(
                    auth_context.state.image.as_ref().unwrap(),
                ));
                copy_self.extension = auth_context
                    .state
                    .image
                    .clone()
                    .unwrap_or_default()
                    .split_terminator('.')
                    .next_back()
                    .unwrap_or_default()
                    .to_string();
                true
            }
            Self::Message::FileSelected(event) => {
                let url;
                if let Some(files) = event
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .files()
                    && let Some(file) = files.get(0)
                {
                    url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                    copy_self.preview_image = Some(url.to_owned());
                    copy_self.file = Some(file.clone());
                    copy_self.extension = file
                        .name()
                        .split_terminator('.')
                        .next_back()
                        .unwrap()
                        .to_string();
                }
                true
            }
            Self::Message::SaveImage => {
                let props = ctx.props().clone();
                let image = copy_self.preview_image.clone();
                let file_extension = copy_self.extension.clone();
                let file = copy_self.file.clone().unwrap();
                let user_id = copy_self.id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let buffer = JsFuture::from(file.array_buffer()).await.unwrap();
                    let bytes = Uint8Array::new(&buffer);
                    let result = AuthContext::save_image(SaveImageRequest {
                        user_id,
                        bytes,
                        file_extension,
                    })
                    .await
                    .map_err(|err| web_sys::console::log_1(&JsValue::from_str(err.as_str())));
                    if let Ok(response) = result {
                        let auth_context = AuthContext {
                            state: AuthState {
                                id: response.id,
                                email: response.email,
                                name: response.name,
                                roles: response.roles,
                                image: response.image,
                                logged_in: true,
                            },
                        };
                        props.context_callback.emit(auth_context.clone());
                        link.send_message(Self::Message::AuthUpdate(auth_context));
                    } else {
                        web_sys::console::log_1(&JsValue::from_str("error saving image"));
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image = self.preview_image.clone();
        html! {
            <>
                <div class="profile-container">
                    <div class="profile-picture">{
                             if let Some(img) = image.clone() {
                                        html! {
                                            <div class="image-upload">
                                                <img id="image" class="image" src={img.clone()} alt={"image"} />
                                                <label class="image-label" for="image_upload">
                                                    <Icon
                                                        data={IconData::BOOTSTRAP_CAMERA_FILL}
                                                        width={"1.25rem".to_owned()}
                                                        height={"1.25rem".to_owned()}
                                                        class={"upload-label-icon"}
                                                    />
                                                </label>
                                                <input
                                                    hidden=true
                                                    type="file"
                                                    id="image_upload"
                                                    name="image_upload"
                                                    accept=".jpg, .jpeg, .png"
                                                    onchange={ctx.link().callback(ProfileMsg::FileSelected)}
                                                    multiple=false
                                                />
                                            </div>
                                        }
                             } else {
                                    html! {
                                        <div class="image-upload">
                                                <Icon
                                                    data={IconData::BOOTSTRAP_PERSON}
                                                    width={"10rem".to_owned()}
                                                    height={"10rem".to_owned()}
                                                    class={"image-placeholder"}
                                                />
                                                <label class="image-label" for="image_upload">
                                                    <Icon
                                                        data={IconData::BOOTSTRAP_CAMERA_FILL}
                                                        width={"1.25rem".to_owned()}
                                                        height={"1.25rem".to_owned()}
                                                        class={"upload-label-icon"}
                                                    />
                                                </label>
                                                <input
                                                    type="file"
                                                    id="image_upload"
                                                    name="image_upload"
                                                    accept=".jpg, .jpeg, .png"
                                                    multiple=false
                                                    onchange={ctx.link().callback(ProfileMsg::FileSelected)}
                                                    hidden=true
                                                />
                                        </div>
                                    }
                             }
                    }
                    </div>
                    <button
                        disabled={self.file.is_none()}
                        id="save-image-button"
                        class="save-image-button"
                        type="button"
                        onclick={ctx.link().callback(|_| ProfileMsg::SaveImage)}
                    >
                        {"save image"}
                    </button>
                    <div class="profile-name">{self.name.clone()}</div>
                    <div class="profile-email">{self.email.clone()}</div>
                    <div class="profile-roles-container">
                    {
                        if !&self.roles.is_empty() {
                                self.roles.iter().map(|role|{
            html! {
                                    <div class="profile-roles">{role}</div>
                                }
                            }).collect()
                        } else {
                            html! {}
                        }
                    }
                    </div>
                </div>
            </>
        }
    }
}
