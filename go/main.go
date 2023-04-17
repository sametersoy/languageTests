package main
 
import (
    "database/sql"
    "fmt"
    _ "github.com/lib/pq"
    "time"
)
 
const (
    host     = "192.168.1.61"
    port     = 5432
    user     = "samet"
    password = "123456"
    dbname   = "MarketDB"
)

func main() {

start_time := time.Now()
start_ticks := start_time.UnixNano() / 100
fmt.Println(start_ticks)
psqlconn := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbname)
	 
	// open database
db, err := sql.Open("postgres", psqlconn)
CheckError(err)
 
rows, err := db.Query(`SELECT barcode FROM products_yedek`)
CheckError(err)
 
defer rows.Close()
for rows.Next() {
    var barcode string
 
    err = rows.Scan(&barcode)
    CheckError(err)
 
    //fmt.Println(barcode)
}
end_time := time.Now()
end_ticks := end_time.UnixNano() / 100
fmt.Println(end_ticks)

//result_ticks := ""//strconv.ParseInt(end_ticks, 10, 64) - strconv.ParseInt(start_ticks, 10, 64)
fmt.Println(end_ticks - start_ticks)

} 

func CheckError(err error) {
if err != nil {
	panic(err)
}
}
