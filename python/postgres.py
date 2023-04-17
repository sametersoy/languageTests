import psycopg2
import datetime

start_datetime = datetime.datetime.now()
start_ticks = int((start_datetime - datetime.datetime(1, 1, 1)).total_seconds() * 10000000)
print(start_ticks)

#establishing the connection
conn = psycopg2.connect(
   database="MarketDB", user='samet', password='123456', host='192.168.1.61', port= '5432'
)

#Setting auto commit false
conn.autocommit = True

#Creating a cursor object using the cursor() method
cursor = conn.cursor()

#Retrieving data
cursor.execute('''SELECT * FROM products_yedek''')

#Fetching 1st row from the table
result = cursor.fetchone();

#Fetching 1st row from the table
result = cursor.fetchall();

#Commit your changes in the database
conn.commit()

#Closing the connection
conn.close()

end_datetime = datetime.datetime.now()
end_ticks = int((end_datetime - datetime.datetime(1, 1, 1)).total_seconds() * 10000000)
print(end_ticks)

result_ticks = end_ticks - start_ticks
print(result_ticks)