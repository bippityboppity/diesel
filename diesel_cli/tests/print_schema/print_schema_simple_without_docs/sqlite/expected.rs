table! {
    users1 (id) {
        id -> Nullable<Integer>,
    }
}

table! {
    users2 (id) {
        id -> Nullable<Integer>,
    }
}

enable_multi_table_joins!(
    users1,
    users2,
);
