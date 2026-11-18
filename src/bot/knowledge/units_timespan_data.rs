// Regular expressions from the pinned PyBot units.py time-span reader.
fancy!(TIME_SHARE, r###"(?<![\w.])(?<word>(?:(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d+)[\s-]+(?:halves?|thirds?|quarters?|fifths?|sixths?|eighths?|tenths?|twelfths?|twentieths?|twenty[\s-]fourths?)|half|\d+\s*/\s*\d+|\d*\.\d+))\s*(?:of\s+)?(?:an?\s+|one\s+)?(?<unit>hours?|hrs?|minutes?|mins?|days?|weeks?)\b"###);
pattern!(MIXED_SPAN, r###"\b(?<whole>a|an|one|two|three|four|five|six|seven|eight|nine|ten|\d+(?:\.\d+)?)\s+(?<unit>hours?|hrs?|days?|weeks?)\s+(?:and\s+)?(?<rest>a\s+half|half|(?<n>\d+|one|two|three|four|five|six|seven|eight|nine|ten|fifteen|twenty|thirty|forty|forty five|fifty)\s+(?<small>minutes?|mins?))\b"###);
fancy!(FRACTION_OF_A_MIXED, r###"(?<![\w.])(?<word>(?:(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d+)[\s-]+(?:halves?|thirds?|quarters?|fifths?|sixths?|eighths?|tenths?|twelfths?|twentieths?|twenty[\s-]fourths?)|half|\d+\s*/\s*\d+|\d*\.\d+))\s*(?:of|out\s+of)\s+(?<span>.+)$"###);
pattern!(HALF_SPAN, r###"\b(?<whole>a|an|one|two|three|four|five|\d+)\s+and\s+a\s+half\s+(?<unit>hours?|hrs?|days?)\b"###);
pattern!(FRACTION_OF, r###"\b(?:what|which|how\s+much)\s+(?:a\s+)?fraction\s+of\s+(?:an?\s+|one\s+)?(?<unit>hour|day|week|minute)\s+(?:is|of)\s+(?<amount>\d+(?:\.\d+)?|a|an|one)\s*(?<small>minutes?|mins?|hours?|hrs?|seconds?|secs?|days?)\b"###);
pattern!(FRACTION_AS, r###"\b(?<amount>\d+(?:\.\d+)?|a|an|one)\s*(?<small>minutes?|mins?|hours?|hrs?|seconds?|secs?|days?)\s+(?:as\s+)?(?:a\s+)?fraction\s+of\s+(?:an?\s+|one\s+)?(?<unit>hour|day|week|minute)\b"###);
pattern!(SPAN_ASK, r###"\b(?:what\s+is|whats|what's|how\s+much|how\s+many|how\s+long|how\s+big)\b"###);
pattern!(FRACTION_ASK, r###"\b(?:what(?:s|'s)?|which|how\s+much|how\s+many)\b"###);
pattern!(TARGET_UNIT, r###"\b(?:in|to|into)\s+(?:minutes?|mins?|seconds?|secs?|hours?|hrs?|days?|weeks?)\b"###);
pattern!(UNIT_ASKED, r###"\b(?:how\s+many|how\s+much|in|to|into)\s+(?:a\s+|an\s+|one\s+)?(minutes?|mins?|seconds?|secs?|hours?|hrs?|days?|weeks?)\b"###);
pattern!(WHOLE_UNIT, r###"^\s*(?:how\s+long\s+is|how\s+long's|what\s+is|what's|whats)\s+(?:an?|one|1)\s+(?<unit>hours?|days?|weeks?|minutes?)\s*$"###);
