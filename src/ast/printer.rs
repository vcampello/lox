use crate::ast::expr::{self, AcceptVisitor};

pub struct Printer {}

impl Printer {
    fn group(&self, name: &str, exprs: &[impl AcceptVisitor]) -> String {
        let mut s: Vec<String> = vec![name.to_string(), "(".to_string()];

        for e in exprs {
            s.push(" ".to_string());
            let value = e.accept(self);
            s.push(value);
        }

        s.push(")".to_string());

        s.into_iter().collect::<String>()
    }
}

impl expr::Visitor for Printer {
    type Item = String;
    fn visit_binary_expr(&self, expr: &expr::BinaryExpr) -> Self::Item {
        self.group(&expr.operator.to_string(), &[expr.left])
    }

    fn visit_literal_expr(&self, expr: &expr::LiteralExpr) -> Self::Item {
        todo!();
    }

    fn visit_unary_expr(&self, expr: &expr::UnaryExpr) -> Self::Item {
        todo!();
    }

    fn visit_grouping_expr(&self, expr: &expr::GroupingExpr) -> Self::Item {
        todo!();
    }
}
