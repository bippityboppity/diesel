table! {
    users1 (id) {
        id -> Int4,
    }
}

table! {
    users2 (id) {
        id -> Int4,
    }
}

enable_multi_table_joins!(
    users1,
    users2,
);
