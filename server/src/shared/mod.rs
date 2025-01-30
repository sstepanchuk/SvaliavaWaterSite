use std::future::Future;
use std::pin::Pin;

use axum::body::Body;
use axum::extract::{FromRef, State};
use axum::http::request::Parts;
use axum::http::{header, HeaderMap, HeaderValue, Request, Response, StatusCode, Uri};
use axum::response::IntoResponse;
use futures_core::*;
use futures_util::stream::{once, StreamExt};
use leptos::prelude::*;
use leptos_axum::*;
use leptos_integration_utils::{BoxedFnOnce, PinnedFuture, PinnedStream, ExtendResponse};
use leptos_meta::ServerMetaContext;
use leptos_router::components::provide_server_redirect;
use leptos_router::location::RequestUrl;
use tower::ServiceExt;
use tower_http::services::ServeDir;


#[derive(Clone, FromRef, Debug)]
pub struct AppState {
  pub leptos_options: LeptosOptions,
  pub optimizer: leptos_image::ImageOptimizer,
}

fn provide_contexts(
  path: &str,
  meta_context: &ServerMetaContext,
  parts: Parts,
  default_res_options: ResponseOptions,
) {
  provide_context(RequestUrl::new(path));
  provide_context(meta_context.clone());
  provide_context(parts);
  provide_context(default_res_options);
  provide_server_redirect(leptos_axum::redirect);
  leptos::nonce::provide_nonce();
}

struct AxumResponse(Response<Body>);

impl ExtendResponse for AxumResponse {
    type ResponseOptions = ResponseOptions;

    fn from_stream(
        stream: impl Stream<Item = String> + Send + 'static,
    ) -> Self {
        AxumResponse(
            Body::from_stream(
                stream.map(|chunk| Ok(chunk) as Result<String, std::io::Error>),
            )
            .into_response(),
        )
    }

    fn extend_response(&mut self, res_options: &Self::ResponseOptions) {
        let mut res_options = res_options.0.write();
        if let Some(status) = res_options.status {
            *self.0.status_mut() = status;
        }
        self.0
            .headers_mut()
            .extend(std::mem::take(&mut res_options.headers));
    }

    fn set_default_content_type(&mut self, content_type: &str) {
        let headers = self.0.headers_mut();
        if !headers.contains_key(header::CONTENT_TYPE) {
            // Set the Content Type headers on all responses. This makes Firefox show the page source
            // without complaining
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str(content_type).unwrap(),
            );
        }
    }
}

pub fn handle_response_inner<IV>(
  additional_context: impl Fn() + 'static + Clone + Send,
  app_fn: impl FnOnce() -> IV + Send + 'static,
  req: Request<Body>,
  stream_builder: fn(
      IV,
      BoxedFnOnce<PinnedStream<String>>,
  ) -> PinnedFuture<PinnedStream<String>>,
) -> PinnedFuture<Response<Body>>
where
  IV: IntoView + 'static,
{
  Box::pin(async move {
      let add_context = additional_context.clone();
      let res_options = ResponseOptions::default();
      let (meta_context, meta_output) = ServerMetaContext::new();

      let additional_context = {
          let meta_context = meta_context.clone();
          let res_options = res_options.clone();
          move || {
              // Need to get the path and query string of the Request
              // For reasons that escape me, if the incoming URI protocol is https, it provides the absolute URI
              let path = req.uri().path_and_query().unwrap().as_str();

              let full_path = format!("http://leptos.dev{path}");
              let (_, req_parts) = generate_request_and_parts(req);
              provide_contexts(
                  &full_path,
                  &meta_context,
                  req_parts,
                  res_options.clone(),
              );
              add_context();
          }
      };

      let res = AxumResponse::from_app(
          app_fn,
          meta_output,
          additional_context,
          res_options,
          stream_builder,
      )
      .await;

      res.0
  })
}

async fn get_static_file(
  uri: Uri,
  root: &str,
  headers: &HeaderMap<HeaderValue>,
) -> Result<Response<Body>, (StatusCode, String)> {
  use axum::http::header::ACCEPT_ENCODING;

  let req = Request::builder().uri(uri);

  let req = match headers.get(ACCEPT_ENCODING) {
      Some(value) => req.header(ACCEPT_ENCODING, value),
      None => req,
  };

  let req = req.body(Body::empty()).unwrap();
  // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
  // This path is relative to the cargo root
  match ServeDir::new(root)
      .precompressed_gzip()
      .precompressed_br()
      .oneshot(req)
      .await
  {
      Ok(res) => Ok(res.into_response()),
      Err(err) => Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Something went wrong: {err}"),
      )),
  }
}

pub fn file_and_error_handler<IV>(
  shell: fn(LeptosOptions) -> IV,
) -> impl Fn(
  Uri,
  State<AppState>,
  Request<Body>,
) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + 'static>>
     + Clone
     + Send
     + 'static
where
  IV: IntoView + 'static,
  LeptosOptions: FromRef<LeptosOptions>
{
  move |uri: Uri, State(state): State<AppState>, req: Request<Body>| {
      Box::pin(async move { 
          let options = state.leptos_options.clone();
          let res = get_static_file(uri, &options.site_root, req.headers());
          let res = res.await.unwrap();

          if res.status() == StatusCode::OK {
              res.into_response()
          } else {
              let mut res = handle_response_inner(
                  move || {
                    provide_context(state.optimizer.clone());
                    provide_context(state.clone());
                  },
                  move || shell(options),
                  req,
                  |app, chunks| {
                      Box::pin(async move {
                          let app = app
                              .to_html_stream_in_order()
                              .collect::<String>()
                              .await;
                          let chunks = chunks();
                          Box::pin(once(async move { app }).chain(chunks))
                              as PinnedStream<String>
                      })
                  },
              )
              .await;
              *res.status_mut() = StatusCode::NOT_FOUND;
              res
          }
      })
  }
}