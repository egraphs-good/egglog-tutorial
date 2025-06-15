for file in examples/*.la; do cargo run < "$file" >  "$file.optimized"; done
for file in examples/*.la.optimized; do cargo run < "$file"; done