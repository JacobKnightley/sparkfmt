use sparkfmt_core::format_sql;

fn main() {
    // Example 1: Basic query from the spec
    let input1 = "select a,b,count(*) c from t where x=1 and y=2 group by a,b having count(*)>1 order by a limit 10";
    
    println!("=== Example 1: Complete Query ===");
    println!("Input:\n{}\n", input1);
    
    match format_sql(input1) {
        Ok(formatted) => println!("Output:\n{}\n", formatted),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 2: Query with JOINs
    let input2 = "select o.id, c.name, p.product from orders o inner join customers c on o.cust_id=c.id left join products p on o.prod_id=p.id where o.status='active'";
    
    println!("=== Example 2: Query with JOINs ===");
    println!("Input:\n{}\n", input2);
    
    match format_sql(input2) {
        Ok(formatted) => println!("Output:\n{}\n", formatted),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 3: Query with CTEs
    let input3 = "with active_users as (select id, name from users where status='active'), recent_orders as (select user_id, amount from orders where date>'2024-01-01') select u.name, sum(o.amount) as total from active_users u inner join recent_orders o on u.id=o.user_id group by u.name";
    
    println!("=== Example 3: Query with CTEs ===");
    println!("Input:\n{}\n", input3);
    
    match format_sql(input3) {
        Ok(formatted) => println!("Output:\n{}\n", formatted),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 4: UNION query
    let input4 = "select id, name from customers where region='north' union all select id, name from customers where region='south'";
    
    println!("=== Example 4: UNION Query ===");
    println!("Input:\n{}\n", input4);
    
    match format_sql(input4) {
        Ok(formatted) => println!("Output:\n{}\n", formatted),
        Err(e) => println!("Error: {}\n", e),
    }
}
