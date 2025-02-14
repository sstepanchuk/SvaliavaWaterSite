use askama::Template;

#[derive(Template)]
#[template(path = "confirm-email.html")]
pub struct ConfirmEmail<'a> {
  pub logo_url: &'a str,
  pub confirmation_link: &'a str,
}