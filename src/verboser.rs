#[allow(dead_code)]
pub(crate) trait Verboser {
    fn print(&self, _message: String);
    fn eprint(&self, _message: String);
    fn name(&self) -> String;
}

struct DefaultVerboser {
}

struct NullVerboser {
}

impl Verboser for DefaultVerboser {
    fn print(&self, message: String) {
        println!("{}", message);
    }

    fn eprint(&self, message: String) {
        eprintln!("{}", message);
    }

    fn name(&self) -> String {
        "DefaultVerboser".to_string()
    }
}

impl Verboser for NullVerboser {
    fn print(&self, _message: String) {
        // do nothing.
    }

    fn eprint(&self, _message: String) {
        // do nothing.
    }

    fn name(&self) -> String {
        "NullVerboser".to_string()
    }
}

pub(crate) fn create(verbose: bool) -> Box<dyn Verboser> {
    if verbose {
        Box::new(DefaultVerboser {})
    } else {
        Box::new(NullVerboser {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let v1 = create(true);
        if v1.name() != "DefaultVerboser" {
            assert!(false, "v1 is not DefaultVerboser");
        }

        let v2 = create(false);
        if v2.name() != "NullVerboser" {
            assert!(false, "v2 is not NullVerboser");
        }
    }
}
