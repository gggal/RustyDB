// fn update(it : Iterator<Record>, to_replace : Vec<String>) -> Iterator {
//     for arg in to_replace {
//         it.map(|t| {t.replace()})
//     }
// }

// fn filter(it : Iterator<>, filter: FilterTree) -> Iterator {
//     it.filter(|t| {t.evaluate() })
// }

// fn join(it : Iterator, other : String, on : String) -> Iterator {
//     let joined : iterator = Iterator;
//     let it2 = fs_manager.get_content(other);
//     for row1 in it {
//         for row2 in it2 {
//             if row1.get(on)=row2.get(on) {
//                 joined.add();
//             }
//         }
//     }
// }

// fn left_join(it : Iterator, other : String, on : String) -> Iterator {
//     let joined : iterator = Iterator;
//     let it2 = fs_manager.get_content(other);
//     for row1 in it {
//         for row2 in it2 {
//             if row1.get(on)=row2.get(on) {
//                 joined.add();
//             } else {
//                 joined.add();
//             }
//         }
//     }
// }

// fn right_join(it : Iterator, other : String, on : String) -> Iterator {
//     let joined : iterator = Iterator;
//     let it2 = fs_manager.get_content(other);
//     for row1 in it {
//         for row2 in it2 {
//             if row1.get(on)=row2.get(on) {
//                 joined.add();
//             } else {
//                 joined.add();
//             }
//         }
//     }
// }

// fn full_join(it : Iterator, other : String, on : String) -> Iterator {
//     let joined : iterator = Iterator;
//     let it2 = fs_manager.get_content(other);
//     for row1 in it {
//         for row2 in it2 {
//             if row1.get(on)=row2.get(on) {
//                 joined.add();
//             } else {
//                 joined.add();
//             }
//         }
//     }
// }

// fn select(it : Iterator, to_retain : Map<String>) -> Iterator {
//     it.map(|r| {r.to_retain(to_retain)})
// }

// fn project(it : Iterator) {
//     it.map(|r| {r.map(r.to_string())})
// }
