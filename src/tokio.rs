use cel::Context;
use vetis::{
    HookPhases, Request, Response, VetisFutureResult, errors::VetisError, hook::{Hook, HookPhase}, make_bitflags, matcher::Matcher,
};

/// Builder for handler path
pub struct WebTransportHookBuilder {
    matcher: Option<Box<dyn Matcher>>,
}

impl WebTransportHookBuilder {
    /// Allow set a matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn matcher<M>(mut self, matcher: M) -> Self
    where
        M: Matcher + Send + Sync + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Build the handler path
    ///
    /// # Returns
    ///
    /// * `Result<HandlerPath, VetisError>` - The handler path or error
    pub fn build(self) -> Result<WebTransportHook, VetisError> {
        Ok(WebTransportHook {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
        })
    }
}

/// WebTransport path
pub struct WebTransportHook {
    matcher: Option<Box<dyn Matcher>>,
}

impl WebTransportHook {
    /// Allow create a new handler path builder
    ///
    /// # Returns
    ///
    /// * `WebTransportPathBuilder` - The builder
    pub fn builder() -> WebTransportHookBuilder {
        WebTransportHookBuilder { matcher: None }
    }
}

impl Hook for WebTransportHook {
    fn has_matches(&self, _context: &Context) -> bool {
        true
    }

    fn phases(&self) -> HookPhases {
        make_bitflags!(HookPhase::Access)
    }

    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>> {
        self.matcher
            .as_mut()
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        _response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)> {
        let future = async move { Ok((request, Some(Response::builder().empty()))) };

        Box::pin(future)
    }
}
