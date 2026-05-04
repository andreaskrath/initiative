use iced::widget::image::Handle;
use uuid::Uuid;

#[derive(Debug)]
pub(super) struct Image {
    pub(super) handle: Handle,
    id: Uuid,
    bytes: Box<[u8]>,
}

impl Image {
    fn new(bytes: Box<[u8]>) -> Self {
        let handle = Handle::from_bytes(bytes.clone());

        Self {
            handle,
            id: Uuid::new_v4(),
            bytes,
        }
    }
}

#[derive(Debug, Default)]
pub struct ImageFieldState {
    pub(super) images: Vec<Image>,
}

impl ImageFieldState {
    pub fn new(images: impl IntoIterator<Item = impl Into<Box<[u8]>>>) -> Self {
        let collected_images = images.into_iter().map(|i| Image::new(i.into())).collect();

        Self {
            images: collected_images,
        }
    }

    pub fn add(&mut self, bytes: Box<[u8]>) {
        let image = Image::new(bytes);
        self.images.push(image);
    }

    pub fn remove(&mut self, index: usize) {
        self.images.remove(index);
    }

    pub fn images(&self) -> Box<[(Uuid, Box<[u8]>)]> {
        self.images
            .iter()
            .map(|image| (image.id, image.bytes.clone()))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
