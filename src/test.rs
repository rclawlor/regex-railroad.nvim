const _LITERAL: &str = r"This is a literal string";
const _NORMAL: &str = "^(a|b)+";
const _CHARACTER: &str = "[^aoeu_0-a]";
const _CHOICE_ODD: &str = "a(b|c|d)e";
const _CHOICE_EVEN: &str = "a(b|cd{2}|e|f)g";
const _REPEAT: &str = "one(two){5}three";
const _EMAIL: &str = r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$";
