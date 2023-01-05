#![allow(non_snake_case)]

#[test]
fn lcnr() {
    const PROGRAM: &str = "[
        crate core {
            trait IsNotU32<> where [] {}
            impl<> IsNotU32<> for i32 where [] {}

            trait Trait<ty T, ty U> where [] {}
            impl<ty T, ty U> Trait<T, U> for () where [T: IsNotU32<>, (): Trait<U, T>] {}
            impl<ty U> Trait<u32, U> for () where [] {}
        }
    ]";

    const GOAL: &str = "exists(<ty T, ty U> is_implemented(Trait((), T, U)))";

    expect_test::expect![[r#"
        Ok(
            maybe(
                env(
                    U(0),
                    [
                        inference_var_data(
                            ty,
                            U(0),
                            None,
                            [],
                            [],
                            [],
                            [],
                        ),
                        inference_var_data(
                            ty,
                            U(0),
                            None,
                            [],
                            [],
                            [],
                            [],
                        ),
                    ],
                    no,
                ),
            ),
        )
    "#]]
    .assert_debug_eq(&formality_rust::test_can_prove_exists(PROGRAM, GOAL));
}
