const MAKSIMUM_STOCK: u32 = 50;
const PAJAK: i8 = 2;

fn main() {
    let items: [String; 3] = [
        String::from("indomie"),
        String::from("Beras"),
        String::from("Gula"),
    ];

    let stocks: [u32; 3] = [30, 45, 12];

    let prices: [u32; 3] = [5000, 43000, 12000];

    let mut total_inventaris: u32 = 0;

    println!("===== LAPORAN INVENTARIS TOKO =====");

    for i in 0..3 {
        println!("[{}]  Nama Barang                : {}", i + 1, &items[i]);
        println!("      Stock                     : {}", &stocks[i]);
        println!("      Harga satuan              : {}", &prices[i]);
        println!(
            "      Total harga               : {}",
            &prices[i] * stocks[i]
        );

        let status = if stocks[i] >= MAKSIMUM_STOCK / 2 {
            "Aman"
        } else if stocks[i] >= MAKSIMUM_STOCK / 5 {
            "Segera Restock"
        } else {
            "Kritis"
        };
        println!("      Status Stok               : {status}\n");
    }

    for i in 0..3 {
        total_inventaris += &prices[i] * stocks[i];
    }
    println!("===== RINGKASAN =====");
    println!("      Batas Maksimum Stock      : {MAKSIMUM_STOCK} pcs");
    println!("      Total Inventaris          : Rp {total_inventaris}");
    println!("      Pajak                     : {PAJAK}%");
    println!(
        "      Total Inventaris + Pajak  : Rp {}",
        total_inventaris + (total_inventaris * PAJAK as u32 / 100)
    );
    println!("====================================");
}
