use iced::widget::image::Handle;

#[derive(Debug)]
pub(super) struct Image {
    pub(super) handle: Handle,
    pub(super) bytes: Box<[u8]>,
}

impl Image {
    fn new(bytes: Box<[u8]>) -> Self {
        let handle = Handle::from_bytes(bytes.clone());

        Self { handle, bytes }
    }
}

#[derive(Debug, Default)]
pub struct ImageFieldState {
    pub(super) images: Vec<Image>,
    pub(super) limit: Option<usize>,
}

impl ImageFieldState {
    pub fn new(images: impl IntoIterator<Item = impl Into<Box<[u8]>>>) -> Self {
        let collected_images = images.into_iter().map(|i| Image::new(i.into())).collect();

        Self {
            images: collected_images,
            limit: None,
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.images.reserve_exact(limit);
        self.limit = Some(limit);
        self
    }

    pub fn add(&mut self, bytes: Box<[u8]>) {
        // Not entirely sure if this is necessary or not, I not believe it is,
        // but rather be safe than sorry I guess.
        if self.limit.is_none() || self.limit.is_some_and(|l| self.images.len() < l) {
            let image = Image::new(bytes);
            self.images.push(image);
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.images.remove(index);
    }

    // pub fn images(&self) -> Vec<Box<[u8]>> {
    //     self.images.iter().map(|image| &image.bytes).collect()
    // }

    pub(super) fn at_limit(&self) -> bool {
        self.limit.is_some_and(|limit| self.images.len() == limit)
    }
}
