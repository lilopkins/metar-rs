use metar::Metar;

fn main() -> anyhow::Result<()> {
    let metar =
        "EGHI 282120Z 19015G32KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006 RMK EXAMPLE METAR=";
    let r = Metar::parse(metar)?;
    println!("{:#?}", r);
    Ok(())
}
