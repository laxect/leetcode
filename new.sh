now=e$(transfer $@)
touch src/$now.rs
echo "pub mod $now;" >> src/lib.rs
