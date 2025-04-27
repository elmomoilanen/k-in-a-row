use actix_web::guard::{Guard, GuardContext};
use actix_web::http::header;

pub struct RefererGuard {
    allowed_referers: Vec<String>,
}

impl RefererGuard {
    pub fn new(allowed_referers: Vec<String>) -> Self {
        Self { allowed_referers }
    }
}

impl Guard for RefererGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        if let Some(referer) = ctx.head().headers().get(header::REFERER) {
            if let Ok(referer_str) = referer.to_str() {
                return self
                    .allowed_referers
                    .iter()
                    .any(|allowed| allowed == referer_str.trim_end_matches('/'));
            }
        }
        false
    }
}
