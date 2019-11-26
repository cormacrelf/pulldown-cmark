// This file is auto-generated by the build script
// Please, do not modify it manually

#[allow(unused_imports)]
use super::test_markdown_html;
use super::test_markdown_html_smart;

#[test]
fn smart_punctuation_test_1() {
    let original = r##"Manice's dog said, "Woof."
"##;
    let expected = r##"<p>Manice&#8217;s dog said, &#8220Woof.&#8221;</p>
"##;

    test_markdown_html_smart(original, expected);
}

#[test]
fn smart_punctuation_test_2() {
    let original = r##"'Manice's dog said, "Woof."'
"##;
    let expected = r##"<p>&#8216;Manice&#8217;s dog said, &#8220Woof.&#8221;&#8217;</p>
"##;

    test_markdown_html_smart(original, expected);
}

#[test]
fn smart_punctuation_test_3() {
    let original = r##"'*Italicised, quoted*'
"##;
    let expected = r##"<p>&#8216;<em>Italicised, quoted</em>&#8217;</p>
"##;

    test_markdown_html_smart(original, expected);
}

#[test]
fn smart_punctuation_test_4() {
    let original = r##"'Nobody Knows You're a Dog': As iconic Internet cartoon turns 20...
"##;
    let expected = r##"<p>&#8216;Nobody Knows You&#8217;re a Dog&#8217;: As iconic Internet cartoon turns 20...</p>
"##;

    test_markdown_html_smart(original, expected);
}

#[test]
fn smart_punctuation_test_5() {
    let original = r##""A quote, and then a comma," and then done.
"##;
    let expected = r##"<p>&#8220;A quote, and then a comma,&#8221; and then done.</p>
"##;

    test_markdown_html_smart(original, expected);
}

#[test]
fn smart_punctuation_test_6() {
    let original = r##"'A quote, and then a comma,' and then done.
"##;
    let expected = r##"<p>&#8216;A quote, and then a comma,&#8217; and then done.</p>
"##;

    test_markdown_html_smart(original, expected);
}

#[test]
fn smart_punctuation_test_7() {
    let original = r##"Some text, a "Stray quote.
"##;
    let expected = r##"<p>Some text, a &#8220;Stray quote.</p>
"##;

    test_markdown_html_smart(original, expected);
}
