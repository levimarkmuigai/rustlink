use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use serde::Deserialize;

use crate::application::service::LinkService;
use crate::domain::{
    errors::LinkError,
    link::LinkId,
    ports::{LinkPersistence, LinkQuery},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState<P, Q>
where
    P: LinkPersistence + Send + Sync + 'static,
    Q: LinkQuery + Send + Sync + 'static,
{
    pub link_service: Arc<LinkService<P, Q>>,
}

#[derive(Clone, Deserialize)]
pub struct CreateLinkForm {
    pub long_url: String,
}

pub async fn create_link<P, Q>(
    State(state): State<AppState<P, Q>>,
    Form(form): Form<CreateLinkForm>,
) -> impl IntoResponse
where
    P: LinkPersistence + Send + Sync + 'static,
    Q: LinkQuery + Send + Sync + 'static,
{
    match state.link_service.create(form.long_url).await {
        Ok(link_id) => (
            StatusCode::CREATED,
            Html(format!(
                "<div id='result'>Link created with ID: {:?} </div>",
                link_id
            )),
        )
            .into_response(),

        Err(LinkError::InvalidUrl) => (
            StatusCode::BAD_REQUEST,
            Html("<h3>The provided URL is invalid.</h3>".to_string()),
        )
            .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<h3>An internal error prevented link creation.</h3>".to_string()),
        )
            .into_response(),
    }
}

pub async fn redirect_link<P, Q>(
    Path(code): Path<String>,
    State(state): State<AppState<P, Q>>,
) -> impl IntoResponse
where
    P: LinkPersistence + Send + Sync + 'static,
    Q: LinkQuery + Send + Sync + 'static,
{
    let short_url = match code.try_into() {
        Ok(s) => s,

        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Html("<h3>Invalid short code format.</h3>".to_string()),
            )
                .into_response()
        }
    };

    match state.link_service.redirect(short_url).await {
        Ok(link) => Redirect::to(link.user_url().as_str()).into_response(),

        Err(LinkError::NotFound) => (
            StatusCode::NOT_FOUND,
            Html("<h3>Link not found</h3>".to_string()),
        )
            .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<h3>And internal error occurred.</h3>".to_string()),
        )
            .into_response(),
    }
}

pub async fn delete_link<P, Q>(
    Path(id): Path<String>,
    State(state): State<AppState<P, Q>>,
) -> impl IntoResponse
where
    P: LinkPersistence + Send + Sync + 'static,
    Q: LinkQuery + Send + Sync + 'static,
{
    let link_id = match LinkId::from_string(id) {
        Ok(id) => id,

        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Html("<h3>Invalid Link ID format.</h3>".to_string()),
            )
                .into_response()
        }
    };

    match state.link_service.delete(link_id).await {
        Ok(Some(_)) => (
            StatusCode::OK,
            Html("<p>Link deleted successfully.</p>".to_string()),
        )
            .into_response(),

        Ok(None) | Err(LinkError::NotFound) => (
            StatusCode::NOT_FOUND,
            Html("<p>Link not found</p>".to_string()),
        )
            .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<h3> An internal error occurred.</h3>".to_string()),
        )
            .into_response(),
    }
}
