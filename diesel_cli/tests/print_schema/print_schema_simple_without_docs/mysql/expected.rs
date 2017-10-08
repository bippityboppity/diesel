table! {
    users1 (id) {
        id -> Integer,
    }
}

table! {
    users2 (id) {
        id -> Integer,
    }
}

enable_multi_table_joins!(
    users1,
    users2,
);
