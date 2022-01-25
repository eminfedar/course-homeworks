mod week1;
mod week2;

fn main() {
    
}

#[cfg(test)]
mod week1_tests {
    use std::f64::consts::PI;
    use crate::week1;

    #[test]
    fn hello() {
        assert_eq!(week1::hello("Emin"), String::from("Hello Emin!"));
        assert_eq!(week1::hello(""), String::from("Hello !"));
        assert_eq!(week1::hello("world"), String::from("Hello world!"));
    }

    #[test]
    fn make_double() {
        assert_eq!(week1::make_it_double(2), 4);
        assert_eq!(week1::make_it_double(5), 10);
        assert_eq!(week1::make_it_double(-2), -4);
    }

    #[test]
    fn multiply_pi() {
        assert_eq!(week1::multiply_pi(1.0), PI);
        assert_eq!(week1::multiply_pi(2.0), PI*2.0);
        assert_eq!(week1::multiply_pi(5.0), PI*5.0);
    }
}

mod week2_tests {
    use crate::week2;

    #[test]
    fn to_letter_grade() {
        // AA = 90-100
        // BA = 85-89
        // BB = 80-84
        // CB = 75-79
        // CC = 70-74
        // DC = 60-69
        // DD = 50-59
        // FD = 40-49
        // FF = 0-39
        assert_eq!(week2::to_letter_grade(100), String::from("AA"));
        assert_eq!(week2::to_letter_grade(90), String::from("AA"));
        assert_eq!(week2::to_letter_grade(88), String::from("BA"));
        assert_eq!(week2::to_letter_grade(80), String::from("BB"));
        assert_eq!(week2::to_letter_grade(75), String::from("CB"));
        assert_eq!(week2::to_letter_grade(70), String::from("CC"));
        assert_eq!(week2::to_letter_grade(60), String::from("DC"));
        assert_eq!(week2::to_letter_grade(55), String::from("DD"));
        assert_eq!(week2::to_letter_grade(45), String::from("FD"));
        assert_eq!(week2::to_letter_grade(38), String::from("FF"));
        assert_eq!(week2::to_letter_grade(0), String::from("FF"));
    }

    #[test]
    fn log() {
        assert_eq!(week2::log(week2::LogLevel::Msg, "mesaj"), String::from("[MSG]: mesaj"));
        assert_eq!(week2::log(week2::LogLevel::Warn, "uyari"), String::from("[WARN]: uyari"));
        assert_eq!(week2::log(week2::LogLevel::Err, "hata"), String::from("[ERR]: hata"));
    }

    #[test]
    fn person() {
        let p1 = week2::Person{name:String::from("Ahmet"), age:24, gender: week2::Gender::Male};
        assert_eq!(p1.to_string(), String::from("Ahmet, 24, Erkek"));

        let p2 = week2::Person{name:String::from("Ayşe"), age:18, gender: week2::Gender::Female};
        assert_eq!(p2.to_string(), String::from("Ayşe, 18, Kadın"));
    }
}