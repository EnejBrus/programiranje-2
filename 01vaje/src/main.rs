/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    if n <= 0 {
        return a0;
    } else if n == 1 {
        return a1;
    } else {
        return fib(a1, a0 + a1, n - 1); 
    }
}


/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno (n: u32) -> bool {
    if n % 400 == 0 {
        return true;
    } else if n % 100 == 0 {
        return false;
    } else if n % 4 == 0 {
        return true;
    } else {
        return false
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);


fn je_veljaven_datum(dan: u32, mesec: u32, leto: u32) -> bool {
    if leto > 2024 {
        return false;
    } else if mesec == 2 {
        if je_prestopno(leto) {
            if dan < 30 {
                return true;                
            } else {
                return false;
            }
        } else {
            if dan < 29 {
                return true;                
            } else {
                return false;
            }
        }
    } else if mesec == 1 || mesec == 3 || mesec == 5 || mesec == 8 || mesec == 7 || mesec == 10 || mesec == 12 {
        if dan < 32 {
            return true;
        } else {
            return false;
        }
    } else {
        if dan < 31 {
            return true;
        } else {
            return false;
        }
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(mut arr: &[u32]) {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0, 1, 0), 0);
        assert_eq!(fib(0, 1, 1), 1);
        assert_eq!(fib(0, 1, 2), 1);
        assert_eq!(fib(0, 1, 3), 2);
        assert_eq!(fib(0, 1, 4), 3);
        assert_eq!(fib(0, 1, 5), 5);
        assert_eq!(fib(0, 1, 6), 8);
        assert_eq!(fib(0, 1, 7), 13);
        assert_eq!(fib(0, 1, 8), 21);
        assert_eq!(fib(0, 1, 9), 34);
        assert_eq!(fib(0, 1, 10), 55);
    }
}
    #[test]
    fn test_je_prestopno() {
        assert_eq!(je_prestopno(2000), true);
        assert_eq!(je_prestopno(2004), true);
        assert_eq!(je_prestopno(1808), true);
        assert_eq!(je_prestopno(1936), true);
        assert_eq!(je_prestopno(1000), false);
        assert_eq!(je_prestopno(13), false);
        assert_eq!(je_prestopno(1111), false);
    }

    #[test]
    fn test_je_veljaven_datum() {
        assert_eq!(je_veljaven_datum(4, 4, 2000), true);
        assert_eq!(je_veljaven_datum(1, 1, 2200), false);
        assert_eq!(je_veljaven_datum(30, 2, 2000), false);
        assert_eq!(je_veljaven_datum(35, 7, 1), false);
    }