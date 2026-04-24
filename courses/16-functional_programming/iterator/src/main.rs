mod common_adapters;
mod consuming_adapters;
mod creating_iterators;
mod custom_iterator;
mod lazy_evaluation;

fn main() {
    creating_iterators::run();
    lazy_evaluation::run();
    common_adapters::run();
    consuming_adapters::run();
    custom_iterator::run();
}
