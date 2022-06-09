#[macro_export]
macro_rules! wrap_link_methods {
    ($wrapper_type:ident < $value_type:ident >) => {
        impl $wrapper_type {
            pub fn from_cid(cid: Cid) -> Self {
                Self(Link::from_cid(cid))
            }

            pub async fn get_owned_value<B: BlockStore>(self, store: &B) -> Result<$value_type> {
                self.0.get_owned_value(store).await
            }

            pub fn get_value(&self) -> Option<&$value_type> {
                self.0.get_value()
            }

            pub fn get_cid(&self) -> Option<&Cid> {
                self.0.get_cid()
            }

            pub async fn resolve_value<B: BlockStore>(&self, store: &B) -> Result<&$value_type> {
                self.0.resolve_value(store).await
            }

            pub async fn resolve_cid<B: BlockStore>(&self, store: &mut B) -> Result<&Cid> {
                self.0.resolve_cid(store).await
            }

            pub fn has_value(&self) -> bool {
                self.0.has_value()
            }

            pub fn has_cid(&self) -> bool {
                self.0.has_cid()
            }
        }
    };
}
