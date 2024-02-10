A restaurant application which accepts menu items from various serving staff in the restaurant. 
The application stores the item along with a cooking time for the item to be completed.

The application exposes endpoints that allow access to:
- removing an order (/remove_order)
- adding an order (/add_order)
- listing all orders for a specific table (/query_all_orders)
- getting a specific order (/get_order)


# Useful commands
## ADD
```curl -X POST -H "Content-Type: application/json" -d '{"item": "mesazza", "table_number": 2}' http://127.0.0.1:8080/add_order```

## Query all
```curl http://127.0.0.1:8080/query_all_orders -d '2' -X GET -H "Content-Type: application/json"```

## Get order
```curl -X GET -H "Content-Type: application/json" -d '{"item": "mesazza", "table_number": 2}' http://127.0.0.1:8080/get_order```

## Remove order
```curl -X POST -H "Content-Type: application/json" -d '{"item": "mesazza", "table_number": 2}' http://127.0.0.1:8080/remove_order```

    
# Testing
```cargo test -- --nocapture```