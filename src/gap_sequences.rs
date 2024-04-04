use std::num::ParseIntError;

use self::{
    n_smooth::list_n_smooth,
    math::gcd,
};

mod n_smooth;
mod math;

pub enum GapSequence {
    Shell1959,
    FrankLazarus1960,
    Hibbard1963,
    PapernovStasevich1965,
    Pratt1971,
    Knuth1973,
    Sedgewick1982,
    IncerpiSedgewick1985,
    Sedgewick1986,
    GonnetBaezaYates1991,
    Tokuda1992,
    Ciura2001,
    Lee2021,
    Custom(Vec<usize>),
}

impl GapSequence {
    pub fn to_vec(&self, array_len: usize) -> Vec<usize> {
        match self {
            Self::Shell1959 => shell_1959(array_len),
            Self::FrankLazarus1960 => frank_lazarus_1960(array_len),
            Self::Hibbard1963 => hibbard_1963(array_len),
            Self::PapernovStasevich1965 => papernov_stasevich_1965(array_len),
            Self::Pratt1971 => pratt_1971(array_len),
            Self::Knuth1973 => knuth_1973(array_len),
            Self::Sedgewick1982 => sedgewick_1982(array_len),
            Self::IncerpiSedgewick1985 => incerpi_sedgewick_1985(array_len),
            Self::Sedgewick1986 => sedgewick_1986(array_len),
            Self::GonnetBaezaYates1991 => gonnet_baezayates_1991(array_len),
            Self::Tokuda1992 => tokuda_1992(array_len),
            Self::Ciura2001 => ciura_2001(),
            Self::Lee2021 => lee_2021(array_len),
            Self::Custom(gaps) => gaps.clone(),
        }
    }
}

impl TryFrom<String> for GapSequence {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "" => Ok(Self::Lee2021),
            "shell_1959" => Ok(Self::Shell1959),
            "frank_lazarus_1960" => Ok(Self::FrankLazarus1960),
            "hibbard_1963" => Ok(Self::Hibbard1963),
            "papernov_stasevich_1965" => Ok(Self::PapernovStasevich1965),
            "pratt_1971" => Ok(Self::Pratt1971),
            "knuth_1973" => Ok(Self::Knuth1973),
            "sedgewick_1982" => Ok(Self::Sedgewick1982),
            "incerpi_sedgewick_1985" => Ok(Self::IncerpiSedgewick1985),
            "sedgewick_1986" => Ok(Self::Sedgewick1986),
            "gonnet_baezayates_1991" => Ok(Self::GonnetBaezaYates1991),
            "tokuda_1992" => Ok(Self::Tokuda1992),
            "ciura_2021" => Ok(Self::Ciura2001),
            "lee_2021" => Ok(Self::Lee2021),
            _ => {
                let gaps: Result<Vec<usize>, ParseIntError> = value
                    .split(",")
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<>();

               match gaps {
                   Ok(gaps) => Ok(Self::Custom(gaps)),
                   Err(e) => Err(e),
               }
            }
        }
    }
}


pub fn shell_1959(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = Vec::new();
    
    let mut gap = array_len;
    loop {
        gap >>= 1;

        if gap < 1 {
            gaps.reverse();
            break gaps;
        }

        gaps.push(gap);
    }
}

pub fn frank_lazarus_1960(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = Vec::new();

    let mut gap = array_len;
    loop {
        gap = ((gap >> 2) << 1) + 1;
        gaps.push(gap);

        if gap <= 1 {
            gaps.reverse();
            break gaps;
        }
    }
}

pub fn hibbard_1963(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    let mut gap: usize = 1;
    loop {
        gap = 2 * gap + 1;
        
        if gap >= array_len {
            break gaps;
        }

        gaps.push(gap);
    }
}

pub fn papernov_stasevich_1965(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    let mut gap: usize = 3;
    loop {
        if gap >= array_len {
            break gaps;
        }
        
        gaps.push(gap);
        
        gap = 2 * gap - 1;
    }
}

pub fn pratt_1971(array_len: usize) -> Vec<usize> {
    list_n_smooth(3, array_len)
}

pub fn knuth_1973(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    let mut gap: usize = 1;
    loop {
        gap = 3 * gap + 1;

        if gap >= array_len {
            break gaps;
        }

        gaps.push(gap);
    }
}

pub fn sedgewick_1982(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    let mut a: usize = 4;
    let mut b: usize = 3;
    loop {
        let gap = a + b + 1;

        if gap >= array_len {
            break gaps;
        }

        gaps.push(gap);

        a <<= 2;
        b <<= 1;
    }   
}

pub fn incerpi_sedgewick_1985(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = Vec::new();

    let mut k: usize = 1;
    let mut a_seq: Vec<usize> = vec![3];
    const FIVE_HALVES: f64 = 5.0 / 2.0;
    let mut current_power: f64 = FIVE_HALVES;
    loop {
        let k2: f64 = 2.0 * k as f64;
        let r = (k2 + k2.sqrt()).sqrt().floor() as usize;
            
        for q in a_seq.len()..r {
            current_power = current_power * FIVE_HALVES;
            let mut n = current_power.ceil() as usize;
            loop {
                let coprime = (0..q).all(|p| gcd(n, a_seq[p]) == 1);
                if coprime {
                    a_seq.push(n);
                    break;
                }

                n += 1;
            }
        }

        let exclude_q = ((r * r + r) / 2) - k;
        let gap: usize = (0..r)
        .filter(|q| *q != exclude_q)
        .map(|q| a_seq[q])
        .product();

        if gap >= array_len {
            break gaps;
        }

        gaps.push(gap);
        k += 1;
    }
}

pub fn sedgewick_1986(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    let mut k: usize = 1;
    loop {
        if k & 1 == 0 {
            let gap = 9 * (1 << k) - 9 * (1 << (k >> 1)) + 1;
            if gap >= array_len {
                break gaps;
            }

            gaps.push(gap);
        } else {
            let gap = 8 * (1 << k) - 6 * (1 << ((k + 1) >> 1)) + 1;
            if gap >= array_len {
                break gaps;
            }

            gaps.push(gap);
        }
    
        k += 1;
    }
}

pub fn gonnet_baezayates_1991(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = Vec::new();

    let mut gap = array_len;
    loop {
        gap = (5 * gap - 1) / 11;

        if gap <= 1 {
            gaps.push(1);
            gaps.reverse();
            break gaps;
        }

        gaps.push(gap);
    }
}

pub fn tokuda_1992(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    const NINE_FOURTHS: f64 = 9.0 / 4.0;
    let mut current_power = NINE_FOURTHS;
    loop {
        current_power *= NINE_FOURTHS;
        let gap = ((current_power - 1.0) / (NINE_FOURTHS - 1.0)).ceil() as usize;

        if gap >= array_len {
            break gaps;
        }

        gaps.push(gap);
    }
}

pub fn ciura_2001() -> Vec<usize> {
    vec![1, 4, 10, 23, 57, 132, 301, 701, 1750]
}

pub fn lee_2021(array_len: usize) -> Vec<usize> {
    let mut gaps: Vec<usize> = vec![1];

    const GAMMA: f64 = 2.243609061420001;
    let mut current_power: f64 = GAMMA;
    loop {
        current_power *= GAMMA;
        let gap = ((current_power - 1.0) / (GAMMA - 1.0)).ceil() as usize;

        if gap >= array_len {
            break gaps;
        }

        gaps.push(gap);
    }
}