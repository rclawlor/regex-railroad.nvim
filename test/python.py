LITERAL = r"T\\whis is a literal string";
NORMAL = "^(?:a|b)?";
CHARACTER = "[^aoeu_0-9]";
CHOICE_ODD = "a(b|c|d)e";
CHOICE_EVEN = "a(b|cd{2}|e|f)g";
REPEAT = "one(two){5}three";
EMAIL = r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$";
