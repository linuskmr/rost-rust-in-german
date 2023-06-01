use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fehl" => "Err",
        "Gut" => "Ok",
        "Zeichenkette" => "String",
        "zk" => "str",
        "Wörterbuch" => "HashMap",
        "Standard" => "Default",
        "Fehler" => "Error",
        "Möglichkeit" => "Option",
        "Etwas" => "Some",
        "Nichts" => "None",
        "Ergebnis" => "Result",
        "Selbst" => "Self",
        "sammlungen" => "collections",
        "druckezl" => "println", // print line -> drucke zeile
        "abbruch" => "break",
        "asynchron" => "async",
        "abwarten" => "await",
        "schleife" => "loop",
        "schiebe" => "move",
        "kiste" => "crate",
        "Schachtel" => "Box",
        "unerreichbarer_code" => "unreachable_code",
        "als" => "as",
        "konstante" => "const",
        "merkmal" => "trait",
        "typ" => "type",
        "gefährlich" => "unsafe",
        "in" => "in",
        "von" => "from",
        "dynamisch" => "dyn",
        "entpacken" => "unwrap",
        "standard" => "default",
        "als_ref" => "as_ref",
        "ea" => "io",
        "extern" => "extern",
        "falsch" => "false",
        "funktion" => "fn",
        "übergeordnet" => "super",
        "einfügen" => "insert",
        "ableiten" => "derive", // #[derive(Debug)] -> #[ableiten(Entkäfern)]
        "Entkäfern" => "Debug", // #[derive(Debug)] -> #[ableiten(Entkäfern)]
        "try" => "versuche",
        "Try" => "Versuche", // TryFrom -> VersucheVon
        "Arz" => "Arc", // Atomic reference counter -> Atomare Referenzzähler
        "Rz" => "Rc", // Reference counter -> Referenzzähler
        "klonen" => "clone",
        "kopieren" => "copy",
        "vgl" => "cmp", // compare -> vergleichen
        "formatiere" => "format",
        "vek" => "vec", // Vektor -> Vector
        "Vek" => "Vec", // Vektor -> Vector,
        "Ausschnitt" => "Slice",
        "fortsetzen" => "continue",
        "File" => "Datei",
        "erstelle" => "create",
        "öffne" => "open",
        "lese" => "read",
        "schreibe" => "write",
        "set" => "setze",


        // iterator funktionen
        "wieder" => "iter",
        "zu_wieder" => "into_iter",
        "abbilden" => "map",
        "ausbreiten" => "flat_map",
        "falte" => "fold",
        "leeren" => "drain",
        "sammeln" => "collect",
        "finde" => "find",
        "nehme" => "take", 
        "produkt" => "product",

        // ordering
        "vgl" => "cmp",
        "Ordnung" => "Ordering",
        "Mehr" => "Greater",
        "Weniger" => "Less",
        "Gleich" => "Equal",
        "hole" => "get",
        "erlaube" => "allow",
        "panik" | "scheiße" | "mist" | "ups" => "panic",
        "zutun" => "todo",
        "modul" => "mod",
        "änd" => "mut",
        "neu" => "new",
        "wo" => "where",
        "für" => "for",
        "hole_oder_füge_ein_mit" => "get_or_insert_with",
        "haupt" => "main",
        "öffentlich" => "pub",
        "keins" => None?,
        "zurückgebe" => "return",
        "umsetz" => "impl",
        "ref" => "ref",
        "vergleiche" => "match",
        "wenn" => "if",
        "sonst" => "else",
        "selbst" => "self",
        "lass" => "let",
        "statisch" => "static",
        "struktur" => "struct",
        "erwarte" => "expect",
        "solange" => "while",
        "nutze" => "use",
        "hinein" => "into",
        "wahr" => "true",
        "aufzählung" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rost(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
