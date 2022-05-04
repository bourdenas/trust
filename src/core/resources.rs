use crate::core::Status;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct ResourceManager<'l, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: 'l + ResourceLoader<'l, Resource>,
{
    resource_path: String,
    loader: &'l Loader,
    cache: HashMap<Key, Rc<Resource>>,
}

impl<'l, Key, Resource, Loader> ResourceManager<'l, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: ResourceLoader<'l, Resource>,
{
    pub fn new(resource_path: &str, loader: &'l Loader) -> Self {
        ResourceManager {
            resource_path: resource_path.to_owned(),
            loader,
            cache: HashMap::new(),
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<Resource>, Status>
    where
        Loader: ResourceLoader<'l, Resource, Args = D>,
        D: Eq + Hash + ?Sized,
        Key: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(&self.resource_path, details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

/// Generic trait to load resource.
pub trait ResourceLoader<'l, Resource> {
    type Args: ?Sized;

    fn load(&'l self, path: &str, resource: &Self::Args) -> Result<Resource, Status>;
}
