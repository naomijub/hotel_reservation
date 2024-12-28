# Hotel Reservation API

A while ago I worked on a problem that aimed to return the cheapest hotel given a set of hotel, its rating and its different fares. Example fares are:

* Lake Inn with a rating of 3 has weekday rates as 110$ for regular customer and 80$ 
for rewards customer. The weekend rates are 90$ for regular customer and 80$ for a 
rewards customer.
* Falls Inn with a rating of 4 has weekday rates as 160$ for regular customer and 110$ 
for rewards customer. The weekend rates are 60$ for regular customer and 50$ for a 
rewards customer.
* Forest Inn with a rating of 5 has weekday rates as 220$ for regular customer and 100$ 
for rewards customer. The weekend rates are 150$ for regular customer and 40$ for a 
rewards customer

Given the hotels above, and an input of the format `<customer_type>: <date1>, <date2>, <date3>, ...`, return the cheapest hotel and in case of a tie, return the one with higher rating.

