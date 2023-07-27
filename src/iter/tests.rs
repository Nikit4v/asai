use crate::iter::*;

// .get() -> Option<Item>
// .list() -> Vec<Item>

#[test]
fn get_test() {
    let data = std::fs::read_to_string("/Users/aleksejzmeevyh/Downloads/_Erai_raws_Masamune_kun_no_Revenge_R_04_1080pMultiple_SubtitleB5C2E59D.ass").unwrap();
    let sections = data.sections();
    let events_section: Section =  sections.get("Events").unwrap();
    let format = events_section.get("Format").unwrap();
    events_section.get_many("Dialogue") // -> Lines; Get lines named "Dialogue"
        .for_each(
            |x: Line| println!("{:?}", x.with_format(format.clone()).get("Style").unwrap()), // Print name of line's style
        );
}
