#[derive(Debug, Default)]
pub struct Sink {
	id: String,
	name: String,
}

impl Sink {
	pub fn new(id: String, name: String) -> Self {
		Sink { id, name }
	}

	pub fn name(&self) -> &str {
		&self.name
	}
	pub fn id(&self) -> &str {
		&self.id
	}
}