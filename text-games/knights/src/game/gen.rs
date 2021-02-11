extern crate console;
use rand::Rng;

pub fn gen_pop() -> Vec<f64> {
    let mut ret: Vec<f64> = Vec::new();

    for _i in 0..25 {
        ret.push(7.0 + rand::thread_rng().gen::<f64>() * 5.0);
    }
    ret
}

pub fn gen_map() -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();

    for i in 0..25 {
        ret.push(i);
    }

    for i in 0..25 {
        for _j in 0..25 {
            let b = rand::thread_rng().gen_range(0..25);
            let temp = ret[b];
            ret[b] = ret[i];
            ret[i] = temp;
        }
    }

    ret
}

pub fn gen_names() -> Vec<String> {
    let names = [
        "Antonia",
        "Boudica",
        "Calgacus",
        "Civilis",
        "Claudius",
        "Clovis",
        "Cyllin",
        "Duras",
        "Italicus",
        "Koson",
        "Marius",
        "Mithridates",
        "Segimer",
        "Tiberius",
        "Theodosius",
        "Tudrus",
        "Vangio",
        "Sido",
        "Vannius",
        "Verica",
        "Zorsine",
        "Abrahm",
        "Achila",
        "Alfonso",
        "Ardo",
        "Aripert",
        "Arthfael",
        "Aurelius",
        "Bela",
        "Bermudo",
        "Boreslaw",
        "Constantin",
        "Demetrius",
        "Dobroslav",
        "Egica",
        "Erik",
        "Eystein",
        "Favila",
        "Ferdinan",
        "Fingal",
        "Fruela",
        "Garciae",
        "Geza",
        "Gwriad",
        "Haakon",
        "Harald",
        "Henry",
        "Helena",
        "Hildeprand",
        "Inigo",
        "Igor",
        "Ivar",
        "Ivan",
        "Ketill",
        "Lambert",
        "Leon",
        "Liutpert",
        "Mauregatus",
        "Mirian",
        "Nepotian",
        "Olaf",
        "Ordono",
        "Oppas",
        "Otto",
        "Ottokar",
        "Pelagius",
        "Piast",
        "Raginpert",
        "Ragnall",
        "Ramiro",
        "Redbad",
        "Reginfrid",
        "Sigurd",
        "Somerled",
        "Sigtrygg",
        "Stephen",
        "Roderic",
        "Sigfred",
        "Silo",
        "Witter",
        "Zayyan",
    ];

    let mut chars: Vec<String> = Vec::new();
    let mut suffix: Vec<String> = Vec::new();

    for _i in 0..24 {
        let idx: usize = rand::thread_rng().gen_range(0..names.len());
        let name: String = names[idx].to_string();

        let mut cnt: i32 = 1;
        for c in &chars {
            if str::eq(c, &name) {
                cnt = cnt + 1;
            }
        }

        suffix.push(
            match cnt {
                2 => "II  ",
                3 => "III ",
                4 => "IV  ",
                5 => "V   ",
                6 => "VI  ",
                7 => "VII ",
                8 => "VIII",
                9 => "IX  ",
                10 => "X   ",
                _ => "    ",
            }
            .to_string(),
        );

        chars.push(name);
    }

    for i in 0..chars.len() {
        chars[i] = format!("{} {}", chars[i], suffix[i]);
    }

    chars
}
