use goblin::elf::Elf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read("./main")?;
    let elf = Elf::parse(&file)?;
    // for reloc in elf.pltrelocs.iter() {
    //     println!("{:#?}", reloc);
    // }
    elf.shdr_relocs.iter().for_each(|reloc| {
        println!("{:#?}", reloc);
    });
    let name = elf.section_headers.get(10).unwrap().sh_name;
    for name in elf.strtab.to_vec().into_iter() {
        println!("{:?}", name);
    }
    println!("{:?}", elf.strtab.get_at(1));
    // print!("{:#?}", elf);
    Ok(())
}
