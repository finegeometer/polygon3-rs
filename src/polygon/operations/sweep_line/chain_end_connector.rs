use chain_end::*;
use crate::utils::UnorientedLine;

pub(crate) struct ChainEndConnector(Option<ChainEnd<UnorientedLine>>);

impl ChainEndConnector {
	pub fn new() -> Self {
		Self(None)
	}

	pub fn end(&mut self, out: &mut Vec<Vec<UnorientedLine>>, e1: ChainEnd<UnorientedLine>) {
		if let Some(e2) = self.0.take() {
			if let Some(edges) = e1.connect(e2) {
				let mut poly = Vec::new();
				for edge in edges {
					if Some(&edge) != poly.last() {
						poly.push(edge);
					}
				}
				while poly.first() == poly.last() {
					poly.pop();
				}
				if !poly.is_empty() {
					debug_assert!(poly.len() >= 3);
					out.push(poly);
				}
			}
		} else {
			self.0 = Some(e1);
		}
	}
}

impl Drop for ChainEndConnector {
	fn drop(&mut self) {
		debug_assert!(self.0.is_none())
	}
}
