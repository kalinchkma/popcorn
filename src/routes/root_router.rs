use axum::Router;

#[derive(Debug)]
pub struct RootRouter {
    routes: Vec<Router>,
}

impl RootRouter {
    // Create a new root router
    fn new() -> Self {
        RootRouter { routes: Vec::new() }
    }

    // add new routes to root router
    fn add(&mut self, route: Router) {
        self.routes.push(route);
    }

    // check lenght of root router
    fn length(&self) -> usize {
        self.routes.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_root_router() {
        let root_router = RootRouter::new();
        let raw_root = RootRouter { routes: Vec::new() };
        assert_eq!(root_router.routes.len(), raw_root.routes.len());
    }

    #[test]
    fn test_add_router() {
        let mut root_router = RootRouter::new();
        let mut raw_router = RootRouter { routes: Vec::new() };

        root_router.add(Router::new());
        raw_router.routes.push(Router::new());

        assert_eq!(root_router.length(), raw_router.routes.len())
    }
}
