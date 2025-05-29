use crate::Expr;

pub struct PrettyFormatter {
    indent: usize,
    indent_str: String,
    line_width: usize,
}

impl Default for PrettyFormatter {
    fn default() -> Self {
        Self {
            indent: 0,
            indent_str: "  ".to_string(), // 2 spaces by default
            line_width: 80,
        }
    }
}

impl PrettyFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent(mut self, indent_str: &str) -> Self {
        self.indent_str = indent_str.to_string();
        self
    }

    pub fn with_line_width(mut self, width: usize) -> Self {
        self.line_width = width;
        self
    }

    pub fn format(&self, expr: &Expr) -> String {
        self.format_expr(expr, 0)
    }

    fn format_expr(&self, expr: &Expr, current_indent: usize) -> String {
        match expr {
            Expr::Nil => "nil".to_string(),
            Expr::Bool(b) => b.to_string(),
            Expr::Int(i) => i.to_string(),
            Expr::Float(f) => f.to_string(),
            Expr::Str(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            Expr::Id(id) => id.to_string(),

            Expr::Form(list) => self.format_form(list, current_indent),
            Expr::Let { bindings, body } => self.format_let(bindings, body, current_indent),
            Expr::If { cond, then, else_ } => self.format_if(cond, then, else_, current_indent),
            Expr::And(exprs) => self.format_and_or("and", exprs, current_indent),
            Expr::Or(exprs) => self.format_and_or("or", exprs, current_indent),
            Expr::Not(expr) => self.format_not(expr, current_indent),
            Expr::Fn { args, body } => self.format_fn(args, body, current_indent),
            Expr::Def { x, y } => self.format_def(x, y, current_indent),
            Expr::Defun { name, args, body } => self.format_defun(name, args, body, current_indent),
            Expr::LetFun {
                name,
                args,
                fun_body,
                expr_body,
            } => self.format_letfun(name, args, fun_body, expr_body, current_indent),
            Expr::DefClos {
                name,
                freevars,
                args,
                body,
            } => self.format_defclos(name, freevars, args, body, current_indent),
            Expr::LetClos {
                name,
                closid,
                freevars,
                body,
            } => self.format_letclos(name, closid, freevars, body, current_indent),
        }
    }

    fn indent_str(&self, level: usize) -> String {
        self.indent_str.repeat(level + self.indent)
    }

    fn format_form(&self, list: &[Expr], current_indent: usize) -> String {
        if list.is_empty() {
            return "()".to_string();
        }

        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        // Special handling for first item as a function name
        let first_item = match &list[0] {
            Expr::Id(id) => id.clone(),
            _ => self.format_expr(&list[0], next_indent),
        };

        // Try single line format first
        let single_line = format!(
            "({} {})",
            first_item,
            list[1..]
                .iter()
                .map(|e| self.format_expr(e, next_indent))
                .collect::<Vec<_>>()
                .join(" ")
        );

        // If single line is short enough, use it
        if single_line.len() <= self.line_width {
            return single_line;
        }

        // Otherwise, use multi-line format
        let args_formatted = list[1..]
            .iter()
            .map(|e| format!("{}{}", inner_indent, self.format_expr(e, next_indent)))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "({}{}{}{}{})",
            first_item,
            if list.len() > 1 { "\n" } else { "" },
            args_formatted,
            if list.len() > 1 { "\n" } else { "" },
            indent
        )
    }

    fn format_let(
        &self,
        bindings: &[(String, Expr)],
        body: &Expr,
        current_indent: usize,
    ) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent_str = self.indent_str(next_indent);
        let binding_indent_str = self.indent_str(current_indent + 2);

        let bindings_formatted = bindings
            .iter()
            .map(|(name, val)| {
                format!(
                    "{}({} {})",
                    binding_indent_str,
                    name,
                    self.format_expr(val, current_indent + 2)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "(let\n{}(\n{}\n{})\n{}{}\n{})",
            inner_indent_str,
            bindings_formatted,
            inner_indent_str,
            inner_indent_str,
            self.format_expr(body, next_indent),
            indent
        )
    }

    fn format_if(&self, cond: &Expr, then: &Expr, else_: &Expr, current_indent: usize) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        // Try single line format first
        let single_line = format!(
            "(if {} {} {})",
            self.format_expr(cond, next_indent),
            self.format_expr(then, next_indent),
            self.format_expr(else_, next_indent)
        );

        // If single line is short enough, use it
        if single_line.len() <= self.line_width {
            return single_line;
        }

        // Otherwise, use multi-line format
        format!(
            "(if\n{}{}\n{}{}\n{}{}\n{})",
            inner_indent,
            self.format_expr(cond, next_indent),
            inner_indent,
            self.format_expr(then, next_indent),
            inner_indent,
            self.format_expr(else_, next_indent),
            indent
        )
    }

    fn format_and_or(&self, operator: &str, exprs: &[Expr], current_indent: usize) -> String {
        if exprs.is_empty() {
            return format!("({})", operator);
        }

        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        // Try single line format first
        let single_line = format!(
            "({} {})",
            operator,
            exprs
                .iter()
                .map(|e| self.format_expr(e, next_indent))
                .collect::<Vec<_>>()
                .join(" ")
        );

        // If single line is short enough, use it
        if single_line.len() <= self.line_width {
            return single_line;
        }

        // Otherwise, use multi-line format
        let exprs_formatted = exprs
            .iter()
            .map(|e| format!("{}{}", inner_indent, self.format_expr(e, next_indent)))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "({}{}{}{}{})",
            operator,
            if !exprs.is_empty() { "\n" } else { "" },
            exprs_formatted,
            if !exprs.is_empty() { "\n" } else { "" },
            indent
        )
    }

    fn format_not(&self, expr: &Expr, current_indent: usize) -> String {
        format!("(not {})", self.format_expr(expr, current_indent + 1))
    }

    fn format_fn(&self, args: &[String], body: &Expr, current_indent: usize) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        let args_str = format!("({})", args.join(" "));

        // Try single line format first
        let single_line = format!("(fn {} {})", args_str, self.format_expr(body, next_indent));

        // If single line is short enough, use it
        if single_line.len() <= self.line_width {
            return single_line;
        }

        // Otherwise, use multi-line format
        format!(
            "(fn {}\n{}{}\n{})",
            args_str,
            inner_indent,
            self.format_expr(body, next_indent),
            indent
        )
    }

    fn format_def(&self, name: &str, value: &Expr, current_indent: usize) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        // Try single line format first
        let single_line = format!("(def {} {})", name, self.format_expr(value, next_indent));

        // If single line is short enough, use it
        if single_line.len() <= self.line_width {
            return single_line;
        }

        // Otherwise, use multi-line format
        format!(
            "(def {}\n{}{}\n{})",
            name,
            inner_indent,
            self.format_expr(value, next_indent),
            indent
        )
    }

    fn format_defun(
        &self,
        name: &str,
        args: &[String],
        body: &Expr,
        current_indent: usize,
    ) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        let args_str = format!("({})", args.join(" "));

        // // Try single line format first
        // let single_line = format!(
        //     "(defun {} {} {})",
        //     name,
        //     args_str,
        //     self.format_expr(body, next_indent)
        // );

        // // If single line is short enough, use it
        // if single_line.len() <= self.line_width {
        //     return single_line;
        // }

        // Otherwise, use multi-line format
        format!(
            "(defun {} {}\n{}{}\n{})",
            name,
            args_str,
            inner_indent,
            self.format_expr(body, next_indent),
            indent
        )
    }

    fn format_letfun(
        &self,
        name: &str,
        args: &[String],
        fun_body: &Expr,
        expr_body: &Expr,
        current_indent: usize,
    ) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        let args_str = format!("({})", args.join(" "));

        format!(
            "(letfun ({} {}\n{}{}\n{})\n{}{}\n{})",
            name,
            args_str,
            inner_indent,
            self.format_expr(fun_body, next_indent),
            inner_indent,
            inner_indent,
            self.format_expr(expr_body, next_indent),
            indent
        )
    }

    fn format_defclos(
        &self,
        name: &str,
        freevars: &[String],
        args: &[String],
        body: &Expr,
        current_indent: usize,
    ) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let inner_indent = self.indent_str(next_indent);

        let freevars_str = format!("({})", freevars.join(" "));
        let args_str = format!("({})", args.join(" "));

        format!(
            "(defclos {} {} {}\n{}{}\n{})",
            name,
            freevars_str,
            args_str,
            inner_indent,
            self.format_expr(body, next_indent),
            indent
        )
    }

    fn format_letclos(
        &self,
        name: &str,
        closid: &str,
        freevars: &[String],
        body: &Expr,
        current_indent: usize,
    ) -> String {
        let next_indent = current_indent + 1;
        let indent = self.indent_str(current_indent);
        let freevar_indent = self.indent_str(current_indent + 2);
        let inner_indent = self.indent_str(next_indent);

        let freevar_formatted = freevars
            .iter()
            .map(|id| format!("{}", id,))
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "(letclos ({} {}\n{}({})\n{})\n{}{}\n{})",
            name,
            closid,
            freevar_indent,
            freevar_formatted,
            inner_indent,
            inner_indent,
            self.format_expr(body, next_indent),
            indent
        )
    }
}

// Convenience function to quickly format an expression
pub fn pretty_format(expr: &Expr) -> String {
    PrettyFormatter::default().format(expr)
}

pub fn format_prog(prog: &[Expr]) -> String {
    prog.iter()
        .map(|expr| PrettyFormatter::default().format(expr))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_atoms() {
        let formatter = PrettyFormatter::new();

        assert_eq!(formatter.format(&Expr::Nil), "nil");
        assert_eq!(formatter.format(&Expr::Bool(true)), "true");
        assert_eq!(formatter.format(&Expr::Int(42)), "42");
        assert_eq!(formatter.format(&Expr::Float(3.14)), "3.14");
        assert_eq!(
            formatter.format(&Expr::Str("hello".to_string())),
            "\"hello\""
        );
        assert_eq!(formatter.format(&Expr::Id("x".to_string())), "x");
    }

    #[test]
    fn test_format_form() {
        let formatter = PrettyFormatter::new();

        // Simple form
        let form = Expr::Form(vec![
            Expr::Id("add".to_string()),
            Expr::Int(1),
            Expr::Int(2),
        ]);

        assert_eq!(formatter.format(&form), "(add 1 2)");

        // Nested form
        let nested_form = Expr::Form(vec![
            Expr::Id("add".to_string()),
            Expr::Form(vec![
                Expr::Id("mul".to_string()),
                Expr::Int(2),
                Expr::Int(3),
            ]),
            Expr::Int(4),
        ]);

        assert_eq!(formatter.format(&nested_form), "(add (mul 2 3) 4)");
    }

    #[test]
    fn test_format_clos() {
        let closure = Expr::LetClos {
            name: "f".to_string(),
            closid: "f_tmp_0".to_string(),
            freevars: vec!["x".to_string(), "y".to_string()],
            body: Box::new(Expr::Form(vec![
                Expr::Id("+".to_string()),
                Expr::Int(1),
                Expr::Form(vec![Expr::Id("f".to_string()), Expr::Int(2)]),
            ])),
        };

        println!("{}", pretty_format(&closure));
    }
}
