# trybuild_test

I am working on an experimental ranged types crate with typestatey/compile-time checks.  One of those checks is an invariant which ensures the `END` bound is not less than the `START` bound.

I created a `trybuild` test and `compile_fail` reports a failure because the [test](https://github.com/U007D/trybuild_test/blob/main/tests/trybuild/range_test.rs) compiles successfully, when it shouldn't.

I tried extracting the [same code as a unit test](https://github.com/U007D/trybuild_test/blob/main/src/range/unit_tests/range_test.rs), where it promptly failed (good), [as an integration test](https://github.com/U007D/trybuild_test/blob/main/tests/integration_tests.rs) and [as a standalone package](https://github.com/U007D/trybuild_test/tree/main/tests/standalone/range_test) all of which failed to compile, as expected.

I am working though debugging `trybuild` to see what assumptions I have for the `trybuild-tests` environment that it creates are false, but I haven't yet worked out how to "capture" `trybuild`'s `trybuild-tests` environment.

I thought that in parallel, I would post a [minimal compiling reproducible example](https://github.com/U007D/trybuild_test) of the issue and the three tests in case there was something simple I am missing and someone with more experience with `trybuild` could point out what I am missing.
