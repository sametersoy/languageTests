const { Pool } = require('pg')
const now = new Date();
const start_ticks = ((now.getTime() * 10000) + (621355968000000000));
console.log(start_ticks);


const pool = new Pool({
    user: 'samet',
    database: 'MarketDB',
    password: '123456',
    port: 5432,
    host: '192.168.1.61',
})

module.exports = { pool };


async function retrieveData() {
    try {
        const res = await pool.query("SELECT barcode FROM products_yedek");

        res.rows.forEach( (row) => {
            console.log(row);
        })

        const end_time = new Date();
        const end_ticks =  ((end_time.getTime() * 10000) + (621355968000000000));
        console.log(end_ticks);

        const result_ticks = end_ticks - start_ticks; 
        console.log(result_ticks);
        //console.log(res.rows);
    } catch (error) {
        console.error(error);
    }
}

retrieveData()


