const LITERAL = /This is a literal string/;
const NORMAL = /^(?:a|b)?/;
const CHARACTER = /[^aoeu_0-9]/;
const CHOICE_ODD = /a(b|c|d)e"/;
const CHOICE_EVEN = /a(b|cd{2}|e|f)g/;
const REPEAT = /one(two){5}three/;
const EMAIL = /^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$/;
