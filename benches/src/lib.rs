/// Computes the full name of a benchmark based on its function name and module path.
///
/// Benchmarks are named in the format of `crate::module::submodule::function_name`. This macro is
/// returns this string from just the `function_name` as an input.
///
/// # Example
///
/// ```
/// // Within a crate named `foo`.
/// mod bar {
///     # use benches::bench;
///     # use criterion::Criterion;
///     #
///     fn my_benchmark(c: &mut Criterion) {
///         // `bench!("my_benchmark")` will expand to `foo::bar::my_benchmark`.
///         c.bench_function(bench!("my_benchmark"), |b| {
///             // ...
///         });
///     }
/// }
/// ```
#[macro_export]
macro_rules! bench {
    ($name:literal) => {
        concat!(module_path!(), "::", $name)
    };
}

mod tests {
    #[test]
    fn bench_macro_output() {
        let bench_name = bench!("function_name");
        assert_eq!(bench_name, "benches::tests::function_name");
    }
}
