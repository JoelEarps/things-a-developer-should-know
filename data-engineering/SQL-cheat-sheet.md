# Commands

`FROM Company c` - say where you are getting the table from and then give it an alias

`COUNT(DISTINCT lm.lead_manager_code)` - Count number of records and count the distinct entries i.e no duplicates

`LEFT JOIN Lead_Manager lm ON c.company_code = lm.company_code` - link the table and join them together using the company code column as the common key. Left join ensures we keep all companies even if they have no employees/ managers, each table joined is given an alias.

`GROUP BY` group the aggregated functions by a specific key to make sure we get on eunqiue row per combination of aggregation - in our case
`GROUP BY c.company_code, c.founder` - give me one row per unique combination of company_code and founder".
