use crate::*;

use std::fmt::Debug;

fn get_expr<'a>(levels: &Vec<usize>, expr: &'a ExpressionAtom, level: usize) -> &'a ExpressionAtom {
    as_expr(&expr.children()[levels[level] - 1])
}

fn find_next_sibling_expr<'a>(levels: &mut Vec<usize>, expr: &'a ExpressionAtom, level: usize) -> Option<&'a Atom> {
    let mut idx = levels[level];
    while idx < expr.children().len() {
        let child = &expr.children()[idx];
        if let Atom::Expression(_) = child {
            levels[level] = idx + 1;
            log::trace!("find_next_sibling_expr: return: {}", child);
            return Some(child);
        }
        idx += 1;
    }
    levels.pop();
    log::trace!("find_next_sibling_expr: return None");
    return None;
}

fn move_top_down_depth<'a>(levels: &mut Vec<usize>, expr: &'a ExpressionAtom, level: usize) -> Option<&'a Atom> {
    log::trace!("move_top_down_depth: expr: {}, level: {}, levels.len(): {}, idx: {}", expr, level, levels.len(), levels[level]);
    if level < levels.len() - 1 {
        let found = move_top_down_depth(levels, get_expr(levels, expr, level), level + 1);
        if found == None {
            find_next_sibling_expr(levels, expr, level)
        } else {
            found
        }
    } else {
        let idx = levels[level];
        if idx == 0 {
            find_next_sibling_expr(levels, expr, level)
        } else {
            levels.push(0);
            let child = as_expr(&expr.children()[idx - 1]);
            let found = move_top_down_depth(levels, child, level + 1);
            if found == None {
                find_next_sibling_expr(levels, expr, level)
            } else {
                found
            }
        }
    }
}

fn move_bottom_up_depth<'a>(levels: &mut Vec<usize>, expr: &'a ExpressionAtom, level: usize) -> Option<&'a Atom> {
    log::trace!("move_bottom_up_depth: expr: {}, level: {}, levels.len(): {}, idx: {}", expr, level, levels.len(), levels[level]);
    if level < levels.len() - 1 {
        let subexpr = &expr.children()[levels[level] - 1];
        let found = move_bottom_up_depth(levels, as_expr(subexpr), level + 1);
        if found == None {
            log::trace!("move_bottom_up_depth: return: {}", subexpr);
            Some(subexpr)
        } else {
            found
        }
    } else {
        loop {
            let found = find_next_sibling_expr(levels, expr, level);
            if let Some(child) = found {
                levels.push(0);
                let found = move_bottom_up_depth(levels, as_expr(child), level + 1);
                if found == None {
                    log::trace!("move_bottom_up_depth: return: {}, levels.len(): {}", child, levels.len());
                    return Some(child)
                } else {
                    return found
                }
            } else {
                return None;
            }
        }
    }
}

type WalkStrategy = for<'a> fn(&mut Vec<usize>, &'a ExpressionAtom, usize) -> Option<&'a Atom>;

pub static FIND_NEXT_SIBLING_WALK: WalkStrategy = find_next_sibling_expr;
pub static BOTTOM_UP_DEPTH_WALK: WalkStrategy = move_bottom_up_depth;
pub static TOP_DOWN_DEPTH_WALK: WalkStrategy = move_top_down_depth;

#[derive(Clone)]
pub struct SubexprStream {
    expr: Atom,
    levels: Vec<usize>,
    walk: WalkStrategy,
}

impl SubexprStream {
    pub fn from_expr(expr: Atom, walk: WalkStrategy) -> Self {
        Self{
            expr: expr,
            levels: vec![0],
            walk: walk,
        }
    }
    pub fn next(&mut self) -> Option<&Atom> {
        (self.walk)(&mut self.levels, as_expr(&self.expr), 0)
    }

    pub fn as_atom(&self) -> &Atom {
        &self.expr
    }

    pub fn into_atom(self) -> Atom {
        self.expr
    }

    fn get_mut_rec<'a>(levels: &Vec<usize>, atom: &'a mut Atom, level: usize) -> &'a mut Atom {
        if level >= levels.len() {
            atom
        } else {
            let child = &mut (as_expr_mut(atom).children_mut()[levels[level] - 1]);
            Self::get_mut_rec(levels, child, level + 1)
        }
    }

    pub fn get_mut(&mut self) -> &mut Atom {
        Self::get_mut_rec(&self.levels, &mut self.expr, 0)
    }

    fn get_rec<'a>(levels: &Vec<usize>, atom: &'a Atom, level: usize) -> &'a Atom {
        if level >= levels.len() {
            atom
        } else {
            let child = &(as_expr(atom).children()[levels[level] - 1]);
            Self::get_rec(levels, child, level + 1)
        }
    }

    pub fn get(&self) -> &Atom {
        Self::get_rec(&self.levels, &self.expr, 0)
    }
}

impl Iterator for SubexprStream {
    type Item = Atom;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next().cloned()
    }
}

impl Debug for SubexprStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "iter {} from {}", self.get(), self.expr)
    }
}

fn as_expr(atom: &Atom) -> &ExpressionAtom {
    match atom {
        Atom::Expression(ref expr) => expr,
        _ => panic!("Atom::Expression is expected"),
    }
}

fn as_expr_mut(atom: &mut Atom) -> &mut ExpressionAtom {
    match atom {
        Atom::Expression(ref mut expr) => expr,
        _ => panic!("Atom::Expression is expected"),
    }
}

pub fn split_expr(expr: &Atom) -> Option<(&Atom, std::slice::Iter<Atom>)> {
    match expr {
        Atom::Expression(expr) => {
            let mut args = expr.children().iter();
            args.next().map_or(None, |op| Some((op, args)))
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_up_depth_walk() {
        let expr = expr!("+", ("*", "3", ("+", "1", n)), ("-", "4", "3"));

        let iter = SubexprStream::from_expr(expr, BOTTOM_UP_DEPTH_WALK);

        assert_eq!(iter.collect::<Vec<_>>(),
        vec![
        expr!("+", "1", n),
        expr!("*", "3", ("+", "1", n)),
        expr!("-", "4", "3"),
        ]);
    }

    #[test]
    fn bottom_up_depth_walk_two_sub_expr() {
        let expr = expr!("*", ("+", "3", "4"), ("-", "5", "2"));

        let iter = SubexprStream::from_expr(expr, BOTTOM_UP_DEPTH_WALK);

        assert_eq!(iter.collect::<Vec<_>>(),
        vec![
        expr!("+", "3", "4"),
        expr!("-", "5", "2"),
        ]);
    }

    #[test]
    fn top_down_depth_walk() {
        let expr = expr!("+", ("*", "3", ("+", "1", n)), ("-", "4", "3"));

        let iter = SubexprStream::from_expr(expr, TOP_DOWN_DEPTH_WALK);

        assert_eq!(iter.collect::<Vec<_>>(),
        vec![
        expr!("*", "3", ("+", "1", n)),
        expr!("+", "1", n),
        expr!("-", "4", "3"),
        ]);
    }

    #[test]
    fn top_down_depth_walk_two_sub_expr() {
        let expr = expr!("*", ("+", "3", "4"), ("-", "5", "2"));

        let iter = SubexprStream::from_expr(expr, TOP_DOWN_DEPTH_WALK);

        assert_eq!(iter.collect::<Vec<_>>(),
        vec![
        expr!("+", "3", "4"),
        expr!("-", "5", "2"),
        ]);
    }
}
