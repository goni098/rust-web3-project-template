use crate::exception::HttpException;
use axum::{
    Form, Json,
    extract::{
        FromRequest, FromRequestParts, Path, Query, Request,
        rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection},
    },
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[allow(dead_code)]
pub struct ValidatedPath<P>(pub P);

#[allow(dead_code)]
pub struct ValidatedParams<Q>(pub Q);

#[allow(dead_code)]
pub struct ValidatedPayload<P>(pub P);

#[allow(dead_code)]
pub struct ValidatedForm<F>(pub F);

impl<S, P> FromRequestParts<S> for ValidatedPath<P>
where
    S: Send + Sync,
    P: DeserializeOwned + Validate,
    Path<P>: FromRequestParts<S, Rejection = PathRejection>,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(path) = Path::<P>::from_request_parts(parts, state).await?;
        path.validate()?;
        Ok(ValidatedPath(path))
    }
}

impl<S, Q> FromRequestParts<S> for ValidatedParams<Q>
where
    S: Send + Sync,
    Q: DeserializeOwned + Validate,
    Query<Q>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<Q>::from_request_parts(parts, state).await?;
        query.validate()?;
        Ok(ValidatedParams(query))
    }
}

impl<S, P> FromRequest<S> for ValidatedPayload<P>
where
    S: Send + Sync,
    P: DeserializeOwned + Validate,
    Json<P>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = HttpException;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<P>::from_request(req, state).await?;
        payload.validate()?;
        Ok(ValidatedPayload(payload))
    }
}

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = HttpException;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(form) = Form::<T>::from_request(req, state).await?;
        form.validate()?;
        Ok(ValidatedForm(form))
    }
}
