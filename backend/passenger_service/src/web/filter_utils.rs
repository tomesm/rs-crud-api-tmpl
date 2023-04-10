use crate::model::Db;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

// Just clones the Db Arc and returns it as a Filter. This will aloow to include the
// Db in the filter chain.
pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
