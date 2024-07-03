
const KEYWORDS = "as cond const constraint contract else enum exists false forall if in @macro macro predicate pub struct satisfy storage state solve true type use var where";

const LITERALS = "true false nil";
const BUILTINS = "__this_set_address __this_address __mut_keys_len __mut_keys_contains __recover_secp256k1 __sha256 __this_set_address __this_address __this_pathway __state_len __verify_ed25519";
const TYPES = "int b256 bool";

hljs.registerLanguage("pint", (hljs) => ({
    name: 'Pint',
    aliases: ['pnt'],
    keywords: {
        type: TYPES,
        keyword: KEYWORDS,
        literal: LITERALS,
        built_in: BUILTINS
    },
    contains: [
        hljs.C_LINE_COMMENT_MODE,
        {
            className: 'title',
            begin: '[a-zA-Z_][a-zA-Z0-9_]+\'',
        },
        {
            className: 'attribute',
            begin: ': ',
            end: '[a-z_]+',
            excludeEnd: false,
            excludeBegin: true,
        },
        {
            className: 'symbol',
            begin: "/'[a-zA-Z_][a-zA-Z0-9_]*/"
        },
        {
            className: 'number',
            variants: [
                { begin: '\\b0x([A-Fa-f0-9_]+)' },
            ],
            relevance: 0
        },
        hljs.QUOTE_STRING_MODE,
        hljs.C_NUMBER_MODE,
        hljs.C_LINE_COMMENT_MODE,
    ]
}));

hljs.initHighlightingOnLoad();
  