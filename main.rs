use std::io::{self, Write};

/// Menyimpan informasi sensor suhu virtual.
/// Dalam sistem nyata, value dapat berasal dari DS18B20/termokopel/RTD.
struct Sensor {
    name: String,
    value: f32,
}

impl Sensor {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: 0.0,
        }
    }

    fn update_value(&mut self, value: f32) {
        self.value = value;
    }

    fn display(&self) {
        println!("Sensor              : {}", self.name);
        println!("Nilai Suhu Sensor   : {:.2} C", self.value);
    }
}

/// Menyimpan status aktuator dan alarm.
/// Controller bertugas mengambil keputusan berdasarkan suhu.
struct Controller {
    heater_on: bool,
    cooling_on: bool,
    alarm_on: bool,
}

impl Controller {
    fn new() -> Self {
        Self {
            heater_on: false,
            cooling_on: false,
            alarm_on: false,
        }
    }

    fn evaluate(&mut self, temperature: f32) -> &'static str {
        self.heater_on = false;
        self.cooling_on = false;
        self.alarm_on = false;

        if temperature > 35.0 {
            self.cooling_on = true;
            self.alarm_on = true;
            "BAHAYA"
        } else if temperature > 30.0 {
            self.cooling_on = true;
            "PANAS"
        } else if temperature < 24.0 {
            self.heater_on = true;
            "DINGIN"
        } else {
            "NORMAL"
        }
    }

    fn display(&self) {
        println!("Heater              : {}", if self.heater_on { "ON" } else { "OFF" });
        println!("Aerator/Cooling     : {}", if self.cooling_on { "ON" } else { "OFF" });
        println!("Alarm               : {}", if self.alarm_on { "AKTIF" } else { "NONAKTIF" });
    }
}

/// Sistem utama yang menggabungkan sensor, controller, histori data,
/// dan perhitungan moving average.
struct MonitoringSystem {
    sensor: Sensor,
    controller: Controller,
    history: Vec<f32>,
    moving_window: usize,
}

impl MonitoringSystem {
    fn new(sensor_name: &str, moving_window: usize) -> Self {
        Self {
            sensor: Sensor::new(sensor_name),
            controller: Controller::new(),
            history: Vec::new(),
            moving_window,
        }
    }

    fn add_temperature(&mut self, temperature: f32) {
        self.sensor.update_value(temperature);
        self.history.push(temperature);
    }

    fn moving_average(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }

        let len = self.history.len();
        let start_index = if len > self.moving_window {
            len - self.moving_window
        } else {
            0
        };

        let data_window = &self.history[start_index..];
        let sum: f32 = data_window.iter().sum();
        sum / data_window.len() as f32
    }

    fn display_history(&self) {
        print!("Histori Suhu        : ");
        for (index, value) in self.history.iter().enumerate() {
            if index > 0 {
                print!(", ");
            }
            print!("{:.2}", value);
        }
        println!(" C");
    }

    fn process_temperature(&mut self, temperature: f32) {
        self.add_temperature(temperature);
        let status = self.controller.evaluate(temperature);
        let avg = self.moving_average();

        println!("\n====================================================");
        println!(" SISTEM MONITORING SUHU KOLAM IKAN LELE");
        println!("====================================================");
        self.sensor.display();
        println!("Status Kondisi      : {}", status);
        println!("Moving Average      : {:.2} C", avg);
        self.controller.display();
        self.display_history();
        println!("====================================================");

        match status {
            "DINGIN" => println!("Rekomendasi         : Heater menyala untuk menaikkan suhu air."),
            "NORMAL" => println!("Rekomendasi         : Suhu ideal, sistem menjaga pemantauan."),
            "PANAS" => println!("Rekomendasi         : Cooling/aerator menyala untuk menurunkan suhu."),
            "BAHAYA" => println!("Rekomendasi         : Alarm aktif, operator harus segera mengecek kolam."),
            _ => println!("Rekomendasi         : Status tidak dikenal."),
        }
    }
}

/// Validasi input agar program tidak menerima data kosong, huruf,
/// atau nilai suhu yang tidak realistis untuk simulasi kolam budidaya.
fn read_temperature_input() -> Option<f32> {
    print!("Masukkan suhu air kolam (C) atau ketik q untuk keluar: ");
    io::stdout().flush().expect("Gagal melakukan flush output");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    let trimmed = input.trim();

    if trimmed.eq_ignore_ascii_case("q") {
        return None;
    }

    if trimmed.is_empty() {
        println!("Input tidak boleh kosong.");
        return read_temperature_input();
    }

    match trimmed.parse::<f32>() {
        Ok(value) => {
            if value < 0.0 || value > 60.0 {
                println!("Input tidak realistis. Masukkan suhu antara 0 C sampai 60 C.");
                read_temperature_input()
            } else {
                Some(value)
            }
        }
        Err(_) => {
            println!("Input tidak valid. Gunakan angka, contoh: 28.5");
            read_temperature_input()
        }
    }
}

fn main() {
    let mut system = MonitoringSystem::new("Sensor Suhu Virtual Kolam Lele", 5);

    println!("====================================================");
    println!(" ETS ALGORITMA DAN PEMROGRAMAN - RUST");
    println!(" Monitoring dan Kontrol Suhu Kolam Ikan Lele");
    println!("====================================================");
    println!("Batas sistem:");
    println!("< 24 C       : Dingin  -> Heater ON");
    println!("24 - 30 C    : Normal  -> Sistem stabil");
    println!("> 30 C       : Panas   -> Aerator/Cooling ON");
    println!("> 35 C       : Bahaya  -> Alarm AKTIF");
    println!("Moving average menggunakan 5 data terakhir.\n");

    loop {
        match read_temperature_input() {
            Some(temperature) => system.process_temperature(temperature),
            None => {
                println!("Program selesai. Data monitoring telah disimpan di histori selama program berjalan.");
                break;
            }
        }
    }
}
