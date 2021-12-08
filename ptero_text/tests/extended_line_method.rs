mod extended_line_method {
    mod one_bit_test;
    mod two_bit_test;
}

const SINGLE_CHAR_TEXT: &str = "a b ca b ca b ca b ca b c";
const WITH_WORDS_TEXT: &str =
    "A little panda has fallen from a tree. The panda went rolling down the hill";
const WITH_OTHER_WHITESPACE_TEXT: &str = "A\tlittle  panda \
    has fallen from a tree. \
    The panda went rolling \
    down the\t hill";
const WITH_EMOJI_TEXT: &str =
    "A little 🐼 has (fallen) from a \\🌳/. The 🐼 went rolling down the hill.";
const HTML_TEXT: &str = "<div> \
    <button style=\" background: red;\">Click me</button> \
    <div/> \
    <footer> This is the end \
    </footer>";
const TINY_TEXT: &str = "TI NY COVER";
const ONE_WORD_TEXT: &str = "Words.";
const EMPTY_TEXT: &str = "";