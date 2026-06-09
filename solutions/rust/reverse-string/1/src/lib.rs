use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let grapheme_cluster_input = input.graphemes(true).collect::<Vec<&str>>();
    grapheme_cluster_input.into_iter().rev().collect::<Vec<&str>>().join("")
}
