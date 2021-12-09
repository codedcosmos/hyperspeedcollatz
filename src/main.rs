use std::collections::LinkedList;
use std::fmt;
use std::ops::Range;

type Int = u128;

struct RangeList(Vec<(Int, Int)>);

impl RangeList {
	fn new() -> Self {
		Self {
			0: Vec::new()
		}
	}

	fn add(&mut self, index: Int) -> bool {
		let mut connected_first: Option<(usize, Int, Int)> = None;
		let mut delete_first = false;

		// Try to see if it's on the edge of any existing tuples
		// If so grow just that one
		for (i, (a, b)) in self.0.iter_mut().enumerate() {
			// Connects to bottom
			if index == *a-1 {
				*a = index;

				if let Some((_, fa, fb)) = connected_first {
					// Recalculate this item
					*a = fa.min(*a);
					*b = fb.max(*b);

					// Flag as delete first
					delete_first = true;

					// Escape as all conditions are accounted for
					break;
				} else {
					connected_first = Some((i, *a, *b));
				}
			}

			// Connects to top
			else if index == *b+1 {
				*b = index;

				if let Some((_, fa, fb)) = connected_first {
					// Recalculate this item
					*a = fa.min(*a);
					*b = fb.max(*b);

					// Flag as delete first
					delete_first = true;

					// Escape as all conditions are accounted for
					break;
				} else {
					connected_first = Some((i, *a, *b));
				}
			}

			// Within range and therefore already added
			// If this is matched, no further things can happen so just quit function
			else if index >= *a && index <= *b {
				return true;
			}
		}

		// If delete first is flagged, well do that
		if delete_first {
			if let Some((index, _, _)) = connected_first {
				self.0.remove(index);
			}
		}

		// No matches where found, so safely add this as a new one
		// Don't worry about tuples that can merge, since this doesn't connect to any it's not possible
		if connected_first == None {
			self.0.push((index, index));
		}

		return false;
	}

	fn in_range(&self, index: Int) -> bool {
		for (a, b) in self.0.iter() {
			if index >= *a && index <= *b {
				return true;
			}
		}

		return false;
	}

	fn extend(&mut self, other: &RangeList) {
		'outer: for (oa, ob) in other.0.iter() {
			// Check if it intersects with any existing items
			let mut intersections = Vec::new();

			for (i, (a, b)) in self.0.iter_mut().enumerate() {
				// Check for intersection
				if (*a-1 <= *oa && *b+1 >= *oa) || (*a-1 <= *ob && *b+1 >= *ob)
					|| (*oa-1 <= *a && *ob+1 >= *b) || (*oa-1 <= *b && *ob+1 >= *b) {
					intersections.push((i, *a, *b));
				}
			}

			// It intersects with 1 or more, so combine them now
			if let Some((lasti, mut lasta, mut lastb)) = intersections.pop() {
				// Iterate through other intersected elements
				for (_, ia, ib) in intersections.iter() {
					// Get min/max
					lasta = lasta.min(*ia);
					lastb = lastb.max(*ib);
				}

				// Get min max from added element
				lasta = lasta.min(*oa);
				lastb = lastb.max(*ob);

				// Update the only value that matters
				self.0[lasti] = (lasta, lastb);

				// Now that the usize value is safe
				// Remove the ones that we don't need
				let mut remove_increase = 0;
				for (ii, ia, ib) in intersections.iter() {
					// Remove from self
					self.0.remove(*ii-remove_increase);
					remove_increase += 1 ;
				}
			}

			// It never intersects so just add it
			else {
				self.0.push((*oa, *ob));
			}
		}
	}

	fn clear(&mut self) {
		self.0.clear();
	}

	fn sort(&mut self) {
		self.0.sort();
	}
}

impl fmt::Display for RangeList {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

struct CollatzSearcher {
	current_index: Int,
	iteration: Int,
	unvalidated_searches: RangeList,
	validated_searches: RangeList
}

impl CollatzSearcher {
	fn new(starting_index: Int) -> Self {
		let mut validated_searches = RangeList::new();
		validated_searches.add(1);
		validated_searches.add(2);
		validated_searches.add(3);
		validated_searches.add(4);

		Self {
			current_index: starting_index,
			iteration: 4,
			unvalidated_searches: RangeList::new(),
			validated_searches
		}
	}

	fn empty(starting_index: Int) -> Self {
		Self {
			current_index: starting_index,
			iteration: 4,
			unvalidated_searches: RangeList::new(),
			validated_searches: RangeList::new()
		}
	}

	fn calculate_next(&mut self) -> bool {
		if self.iteration % 10000 == 0 {
			self.validated_searches.sort();
			println!("Iteration {}", self.iteration);
			println!("{}", self.validated_searches);
		}

		// Calculate next value
		if self.current_index % 2 == 0 { // Even
			self.current_index = self.current_index / 2;

		} else { // Odd
			self.current_index = self.current_index * 3 + 1
		}

		// Check if already added to unvalidated searches
		// If true, loop is found, exist immediately
		if self.unvalidated_searches.add(self.current_index) {
			println!("LOOP FOUND, THIS IS NOT A DRILL, LOOP FOUND AT INDEX '{}'", self.current_index);
			return true;
		}

		if self.validated_searches.in_range(self.current_index) {
			println!("Conjecture validated for {} at {}", self.iteration, self.current_index);

			// If in validated searches, this number follows the conjecture
			// Updated accordingly
			self.validated_searches.add(self.iteration);
			self.validated_searches.extend(&self.unvalidated_searches);
			self.unvalidated_searches.clear();

			self.iteration += 1;
			self.current_index = self.iteration;
		}

		return false;
	}
}

fn main() {
	println!("Hello, world!");

	let mut searcher = CollatzSearcher::new(5);

	loop {
	    if searcher.calculate_next() {
	        return;
	    }
	}
}
