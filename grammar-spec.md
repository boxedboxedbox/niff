Expect this to be old (or so new that some of the features haven't been implemented yet).
I probably just write code and don't care about this, or the other way around.
These specifications don't follow any pattern other than being hard to understand.

strings: ", [ any ], "

characters: ', [ any1 ] '

integers: 0-9

functions: "fn", [ name ], "(", ( [ parameter name ], [ type ] ) | "", ")", [ return type ], ( ";" | "{" )

casts: "(", [ type ], ")", [ identifier ]

expression => ( "-" expression ) |
              ( expression binaryop expression ) |
              primary

binaryop   => ( "-" | "+" | "/" | "*" | "%" | "^" | "&" | "|")
unary      => ( ( "-" | "+" )  primary) | ( primary ( "++" | "--" ) ) 

primary    => NUMBER | IDENTIFIER

variables = "var" [name] ( "" | ":" [type] ) "=" expression ";" 

Precedence:
0 	=
1 	||
2 	&&
3 	|
4 	^
5 	&
6 	==, !=
7 	>, >=, <, <=
8 	+, -
9 	>>, <<
10 	*, /, %
11 	!, ~, - (unary)
12 	**
13 	(, )
14  function()
