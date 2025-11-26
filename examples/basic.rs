use metar::Metar;

fn main() -> anyhow::Result<()> {
    let metar =
        "EGHI 282120Z 19015G32KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006 RMK EXAMPLE METAR=";
    match Metar::parse(metar) {
        Ok(metar) => println!("{metar:#?}"),
        Err(es) => {
            for e in es {
                eprintln!("{e}");
            }
        }
    }
    Ok(())
}
