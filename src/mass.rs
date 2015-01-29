use token::Token;
use token::TokenKind::*;

pub fn group_elems(toks: Vec<Token>) -> Vec<Token> {
    let (out, _) = do_grouping(toks, 0);
    out
}

fn do_grouping(toks: Vec<Token>, pos: usize) -> (Vec<Token>, usize) {
    let mut out = Vec::<Token>::new();
    while pos < toks.len() {
        match toks[pos].tok {
            Elem(_) => {
                let elem = toks[pos];
                // if this element is followed by a coefficient token, we use that
                // otherwise we make a new dummy token, so *every* element is followed
                // by a coefficient token in out
                let coef = if let Some(&Token { tok: Coefficient(_), .. }) = toks.get(pos + 1) {
                    pos += 2;
                    toks[pos + 1]
                } else {
                    pos += 1;
                    Token { tok: Coefficient(1), pos: elem.pos, len: 1 }
                };
                if let Some(found_pos) = out.iter().position(|e| e.tok.elem().is_some() &&
                                                                 e.tok == elem.tok) {
                    *out[found_pos + 1].tok.coef().unwrap() += *coef.tok.coef().unwrap();
                } else {
                    // this is a new element so push it *and* and a coefficient token
                    out.push(elem);
                    out.push(coef);
                }
            },
            ParenOpen => {
                let (mut paren_tok, pos) = do_grouping(toks, pos);
                if let Some(Some(mult)) = toks.get(pos + 1).map(|t| t.tok.coef()) {
                    for tok in paren_tok.iter_mut() {
                        if let Some(coef) = tok.tok.coef() {
                            *coef *= *mult;
                        }
                    }
                    pos += 1;
                }
                out.extend(paren_tok.into_iter());
            },
            ParenClose => break,
            // we should never reach any other kind of token, and if we do,
            // then that is a coding error
            _ => unreachable!()
        }
    }
    (out, pos)
}