pub fn _access_out_of_scope_variable() {
    let v = vec![1, 2, 3, 4, 5];

    v[99];
}
