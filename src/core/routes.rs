use axum::Router;

#[derive(Debug)]
pub struct RouterBucket {
    pub routes: Vec<Router>
}

impl RouterBucket {
    // Create new router bucket
    pub fn new() -> Self {
        RouterBucket {
            routes: Vec::new()
        }
    }

    // add new routes to root router
    pub fn push(&mut self, router: Router) {
        self.routes.push(router);
    }

    // combine routers
    pub fn combine_routers(self) -> Router {
        let mut router_bucket = Router::new();
        for route in self.routes {
            router_bucket = router_bucket.merge(route);
        }
        router_bucket
    }

    // check lenght of router bucket
    pub fn len(self) -> usize {
        self.routes.len()
    }

}