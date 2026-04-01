const MAKSIMUM_STOCK: f32 = 50.0;
const PAJAK: i8 = 2;

fn main() {
    let items: [String; 3] = [
        String::from("indomie"),
        String::from("Beras"),
        String::from("Gula"),
    ];

    let stocks: [u8; 3] = [30, 45, 12];

    let prices: [i32; 3] = [5000, 43000, 12000];

    let item1: &String = &items[0];
    let price1: i32 = prices[0];
    let stock1: u8 = stocks[0];
    let total_price1: i32 = stock1 as i32 * price1;

    let item2: &String = &items[1];
    let price2: i32 = prices[1];
    let stock2: u8 = stocks[1];
    let total_price2: i32 = stock2 as i32 * price2;

    let item3: &String = &items[2];
    let price3: i32 = prices[2];
    let stock3: u8 = stocks[2];
    let total_price3: i32 = stock3 as i32 * price3;

    let summary: (
        (String, String, String, String),
        (String, String, String, String),
        (String, String, String, String),
    ) = (
        (
            format!("Nama Barang: {item1}"),
            format!("Stok: {stock1}"),
            format!("Harga satuan: {price1}"),
            format!("Total Harga: {total_price1}"),
        ),
        (
            format!("Nama Barang: {item2}"),
            format!("Stok: {stock2}"),
            format!("Harga satuan: {price2}"),
            format!("Total Harga: {total_price2}"),
        ),
        (
            format!("Nama Barang: {item3}"),
            format!("Stok: {stock3}"),
            format!("Harga satuan: {price3}"),
            format!("Total Harga: {total_price3}"),
        ),
    );

    let total_inventaris: i32 =
        (price1 * stock1 as i32) + (price2 * stock2 as i32) + (price3 * stock3 as i32);

    let total_inventaris_pajak: i32 = total_inventaris + (total_inventaris * PAJAK as i32 / 100);

    println!("===== LAPORAN INVENTARIS TOKO =====\n");

    println!("[1]       {}", summary.0.0);
    println!("          {}", summary.0.1);
    println!("          {}", summary.0.2);
    println!("          {}", summary.0.3);

    println!("");

    println!("[2]       {}", summary.1.0);
    println!("          {}", summary.1.1);
    println!("          {}", summary.1.2);
    println!("          {}", summary.1.3);

    println!("");

    println!("[3]       {}", summary.2.0);
    println!("          {}", summary.2.1);
    println!("          {}", summary.2.2);
    println!("          {}", summary.2.3);

    println!("");

    println!("Batas Maksimum Stock     : {MAKSIMUM_STOCK} pcs");
    println!("Total Inventaris         : Rp {total_inventaris}");
    println!("Pajak                    : {PAJAK} %");
    println!("Total Inventaris + Pajak : Rp {total_inventaris_pajak}");
    println!("==================================================");
}
