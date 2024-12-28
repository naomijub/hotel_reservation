# Hotel Reservation API

A while ago I worked on a problem that aimed to return the cheapest hotel given a set of hotel, its rating and its different fares. Example fares are:

* `Lake Inn` with a rating of 3 has weekday rates as 110 CA$ for regular customer and 80 CA$ 
for rewards customer. The weekend rates are 90 CA$ for regular customer and 80 CA$ for a 
rewards customer.
* `Falls Inn` with a rating of 4 has weekday rates as 160 CA$ for regular customer and 110 CA$ 
for rewards customer. The weekend rates are 60 CA$ for regular customer and 50 CA$ for a 
rewards customer.
* `Forest Inn` with a rating of 5 has weekday rates as 220 CA$ for regular customer and 100 CA$ 
for rewards customer. The weekend rates are 150 CA$ for regular customer and 40 CA$ for a 
rewards customer

Given the hotels above, and an input of the format `<customer_type>: <date1>, <date2>, <date3>, ...`, return the cheapest hotel and in case of a tie, return the one with higher rating.

An addition is that each hotel can pick a **blackout** date period, where there are not rewards discount rates. `Lake Inn` doesnt have a blackout date, `Falls Inn` has picked its black out dates for New Years from 23rd December to 3rd January, while `Forest Inn` has all the entire three months of July, August, September as blackout dates for summer.