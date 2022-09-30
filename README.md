# Torrent Parser

## Working

Read to torrent file as bytes, one byte at a time - 

1. Match the character with data types. If a match is found, then create the object and push the data inside it.
2. If the match fails, panic! or return None 

## Bencoding
Bencoding is a way to specify and organize data in a terse format. It supports the following types: byte strings, integers, lists, and dictionaries.

### Byte Strings
Byte strings are encoded as follows: &lt;string length encoded in base ten ASCII&gt;:&lt;string data&gt;
Note that there is no constant beginning delimiter, and no ending delimiter.

Example: 4: spam represents the string "spam"
Example: 0: represents the empty string ""

### Integers
Integers are encoded as follows: i&lt;integer encoded in base ten ASCII&gt;e
The initial i and trailing e are beginning and ending delimiters.

Example: i3e represents the integer "3"
Example: i-3e represents the integer "-3"
i-0e is invalid. All encodings with a leading zero, such as i03e, are invalid, other than i0e, which of course corresponds to the integer "0".

NOTE: The maximum number of bit of this integer is unspecified, but to handle it as a signed 64bit integer is mandatory to handle "large files" aka .torrent for more that 4Gbyte.

### Lists
Lists are encoded as follows: l&lt;bencoded values&gt;e
The initial l and trailing e are beginning and ending delimiters. Lists may contain any bencoded type, including integers, strings, dictionaries, and even lists within other lists.

Example: l4:spam4:eggse represents the list of two strings: [ "spam", "eggs" ]
Example: le represents an empty list: []

### Dictionaries
Dictionaries are encoded as follows: d&lt;bencoded string&gt;&lt;bencoded element&gt;e
The initial d and trailing e are the beginning and ending delimiters. Note that the keys must be bencoded strings. The values may be any bencoded type, including integers, strings, lists, and other dictionaries. Keys must be strings and appear in sorted order (sorted as raw strings, not alphanumerics). The strings should be compared using a binary comparison, not a culture-specific "natural" comparison.

Example: d3:cow3:moo4:spam4:eggse represents the dictionary { "cow" =&gt; "moo", "spam" =&gt; "eggs" }
Example: d4:spaml1:a1:bee represents the dictionary { "spam" =&gt; [ "a", "b" ] }
Example: d9:publisher3:bob17:publisher-webpage15:www.example.com18:publisher.location4:homee represents { "publisher" =&gt; "bob", "publisher-webpage" =&gt; "www.example.com", "publisher.location" =&gt; "home" }
Example: de represents an empty dictionary {}