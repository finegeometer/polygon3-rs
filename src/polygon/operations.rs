mod queue;
mod sweep_line;

#[cfg(test)]
mod test;

use super::Polygon;
use crate::utils::UnorientedLine;
use queue::{Event, Q};

impl Polygon {
    /// Apply an operation to a collection of polygons.
    /// If the 'inside' function returns true when in none of the polygons, inside and outside will be swapped.
    ///
    /// # Correctness
    /// This function has not been directly tested, but is used in the implementation of `Polygon::difference`.
    pub fn operation<'r>(
        polygons: impl IntoIterator<Item = &'r Self>,
        inside: fn(&bit_vec::BitVec) -> bool,
    ) -> Self {
        // Step 1: Populate the queue.

        let mut events = Q::new();

        let mut polynum = 0;

        polygons.into_iter().for_each(|poly| {
            events.extend(poly.0.iter().flat_map(|edges| {
                crate::utils::pairs(&edges).map(|(&e1, &e2)| {
                    Event::new_vertex(e1.intersect(e2), [(e1, polynum), (e2, polynum)])
                })
            }));
            polynum += 1;
        });

        // Step 2: Sweep line.

        //  \::::::::::::::::::::::/
        //   \::::::::::::::::::::/
        //    \::::::::/\::::::::/
        //     \::::::/@@\::::::/
        //   00 \:10:/@11@\:10:/ 00
        //       \::/@@@@@@\::/
        //        \/@@@@@@@@\/
        //        /\@@@@@@@@/\
        //       /%%\@@@@@@/%%\
        //   00 /%01%\@11@/%01%\ 00
        //     /%%%%%%\@@/%%%%%%\
        //    /%%%%%%%%\/%%%%%%%%\
        //   /%%%%%%%%%%%%%%%%%%%%\
        //  /%%%%%%%%%%%%%%%%%%%%%%\

        let mut sweep_line = sweep_line::SweepLine::new(polynum, inside);

        let mut line_endings: Vec<(UnorientedLine, usize)> = Vec::new();
        while let Some(point) = events.next_event(&mut line_endings) {
            // println!("{:?}", point);

            let mut section = sweep_line.relevant_section_reversed(point);

            line_endings.iter().for_each(|&(line, poly_idx)| {
                section.insert(line, poly_idx);
            });

            events.extend(
                section
                    .boundary_intersections()
                    .filter(|&pt| {
                        let [_, _, z]: [i64; 3] = pt.into();
                        z != 0
                    })
                    .map(Event::new_intersection),
            );
        }

        Self(sweep_line.out)
    }
}

impl Polygon {
    /// Take the union of a collection of polygons.
    ///
    /// # Correctness
    /// This function has not been directly tested, but internally uses the same code as `Polygon::difference`.
    pub fn union<'r>(polygons: impl IntoIterator<Item = &'r Self>) -> Self {
        Self::operation(polygons, bit_vec::BitVec::none)
    }

    /// Take the intersection of a collection of polygons.
    ///
    /// # Correctness
    /// This function has not been directly tested, but internally uses the same code as `Polygon::difference`.
    pub fn intersection<'r>(polygons: impl IntoIterator<Item = &'r Self>) -> Self {
        Self::operation(polygons, bit_vec::BitVec::all)
    }

    /// Subtract a collection of polygons from a polygon.
    ///
    /// # Correctness
    /// This function has been fuzzed for 16 hours through Polygon::test_difference, and reported no errors.
    ///
    /// ```text
    ///                         american fuzzy lop 2.52b (fuzz)
    ///
    /// ┌─ process timing ─────────────────────────────────────┬─ overall results ─────┐
    /// │        run time : 0 days, 16 hrs, 57 min, 48 sec     │  cycles done : 3333   │
    /// │   last new path : 0 days, 3 hrs, 35 min, 23 sec      │  total paths : 529    │
    /// │ last uniq crash : none seen yet                      │ uniq crashes : 0      │
    /// │  last uniq hang : none seen yet                      │   uniq hangs : 0      │
    /// ├─ cycle progress ────────────────────┬─ map coverage ─┴───────────────────────┤
    /// │  now processing : 511* (96.60%)     │    map density : 1.91% / 2.13%         │
    /// │ paths timed out : 0 (0.00%)         │ count coverage : 5.66 bits/tuple       │
    /// ├─ stage progress ────────────────────┼─ findings in depth ────────────────────┤
    /// │  now trying : havoc                 │ favored paths : 36 (6.81%)             │
    /// │ stage execs : 237/384 (61.72%)      │  new edges on : 44 (8.32%)             │
    /// │ total execs : 379M                  │ total crashes : 0 (0 unique)           │
    /// │  exec speed : 11.6k/sec             │  total tmouts : 16 (2 unique)          │
    /// ├─ fuzzing strategy yields ───────────┴───────────────┬─ path geometry ────────┤
    /// │   bit flips : 63/1.67M, 9/1.67M, 13/1.67M           │    levels : 12         │
    /// │  byte flips : 0/208k, 1/168k, 2/168k                │   pending : 0          │
    /// │ arithmetics : 4/9.37M, 2/3.26M, 0/1.22M             │  pend fav : 0          │
    /// │  known ints : 11/1.01M, 9/4.02M, 11/6.90M           │ own finds : 528        │
    /// │  dictionary : 0/0, 0/0, 5/6.71M                     │  imported : n/a        │
    /// │       havoc : 379/121M, 19/220M                     │ stability : 99.86%     │
    /// │        trim : 9.95%/98.4k, 18.93%                   ├────────────────────────┘
    /// └─────────────────────────────────────────────────────┘          [cpu000: 25%]
    /// ```
    ///
    /// It also has fifteen regular tests.
    pub fn difference<'r>(&'r self, clip: impl IntoIterator<Item = &'r Self>) -> Self {
        Self::operation(std::iter::once(self).chain(clip), |bits| {
            let mut iter = bits.iter();
            if let Some(true) = iter.next() {
                iter.all(|x| !x)
            } else {
                false
            }
        })
    }
}

/// Tests
impl Polygon {
    /// Given a point, a polygon, and a lot more polygons,
    /// Test if the point is in the first polygon and not in the rest.
    /// Test if the point is in the difference of the first polygon and the rest.
    /// Make sure the two answers are the same.
    ///
    /// If the library is bug-free, this always passes.
    pub fn test_difference(self, point: crate::utils::Point, other: Vec<Self>) {
        use std::cmp::Ordering;
        let answer_1 = self.contains(point).min(
            other
                .iter()
                .map(|poly| poly.contains(point))
                .max()
                .unwrap_or(Ordering::Less)
                .reverse(),
        );

        if answer_1 != Ordering::Equal {
            let answer_2 = self.difference(&other).contains(point);

            assert_eq!(answer_1, answer_2);
        }
    }
}
