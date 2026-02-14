#[derive(Debug, Clone)]
pub enum Element {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Array(Vec<Element>),
    Object(Vec<(String, Element)>),
}

// json grammar
nano_parser_gen_macro::grammar! {

%token<String> str = "\"([^\\\\\"]|\\\\.)*?\"" => { nano_parser_gen::util::read_strlit($) }
%token<f64> num = "(([0-9]*\\.[0-9]+(([eE][-+]?\\d+)|f)?|[0-9]+\\.[0-9]*(([eE][-+]?\\d+)|f)?|[0-9]+([eE][-+]?\\d+|f))|[0-9]+)"
%token "true"
%token "false"
%token "null"
%token "["
%token "]"
%token "{"
%token "}"
%token ":"
%token ","

%skip "\\s+"

%type<Element> Element
%type<(String, Element)> Pair

%start Element

%%

Element ::= str { $$ = Element::Str($0) }
          | num { $$ = Element::Num($0) }
          | "true" { $$ = Element::Bool(true) }
          | "false" { $$ = Element::Bool(false) }
          | "null" { $$ = Element::Null }
          | "[" SepList<Element, ","> "]" { $$ = Element::Array($1) }
          | "{" SepList<Pair, ","> "}" { $$ = Element::Object($1) }
          ;

Pair ::= str ":" Element { $$ = ($0, $2) }
       ;

}
