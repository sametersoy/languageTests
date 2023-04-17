// See https://aka.ms/new-console-template for more information
using Npgsql;

long startTime = DateTime.Now.Ticks;
Console.WriteLine(startTime);

var connString = "Host=192.168.1.61;Username=samet;Password=123456;Database=MarketDB";

await using var conn = new NpgsqlConnection(connString);
await conn.OpenAsync();

// Retrieve all rows
await using (var cmd = new NpgsqlCommand("SELECT * FROM products_yedek", conn))
await using (var reader = await cmd.ExecuteReaderAsync())

while (await reader.ReadAsync())
{
    
}
    //Console.WriteLine(reader.GetString(1));


long endTime = DateTime.Now.Ticks;
Console.WriteLine(endTime);

long resultTime = endTime- startTime;
Console.WriteLine(resultTime);