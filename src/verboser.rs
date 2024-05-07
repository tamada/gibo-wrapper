#[allow(dead_code)]
pub(crate) trait Verboser {
    fn print(&self, _message: String);
    fn eprint(&self, _message: String);
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
}

impl Verboser for NullVerboser {
    fn print(&self, _message: String) {
        // do nothing.
    }

    fn eprint(&self, _message: String) {
        // do nothing.
    }
}

pub(crate) fn create(verbose: bool) -> Box<dyn Verboser> {
    if verbose {
        Box::new(DefaultVerboser {})
    } else {
        Box::new(NullVerboser {})
    }
}
