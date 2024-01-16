slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_calculate_taxes(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let mut tax_sum: f64 = 0.0;
        
        // federal taxes
        // https://taxfoundation.org/data/all/federal/2024-tax-brackets/
        if num > 0.0 && num <= 11600.0 {
            tax_sum += num * 0.1;
        } else if num > 11600.0 && num <= 47150.0 {
            tax_sum += ((num - 11600.0)  * 0.12) + (11600.0  * 0.1);
        } else if num > 47150.0 && num <= 100525.0 {
            tax_sum += ((num - 47150.0)  * 0.22) + (47150.0  * 0.12) + (11600.0  * 0.1);
        } else if num > 100525.0 && num <= 191950.0 {
            tax_sum += ((num - 100525.0) * 0.24) + (100525.0  * 0.22) + (47150.0  * 0.12) + (11600.0  * 0.1);
        } else if num > 191950.0 && num <= 243725.0 {
            tax_sum += ((num - 191950.0)  * 0.32) + (191950.0  * 0.24) + (100525.0  * 0.22) + (47150.0  * 0.12) + (11600.0  * 0.1);
        } else if num > 243725.0 && num <= 609350.0 {
            tax_sum += ((num - 243725.0)  * 0.35) + (243725.0  * 0.32) + (191950.0  * 0.24) + (100525.0  * 0.22) + (47150.0  * 0.12) + (11600.0  * 0.1);
        } else if num > 609350.0 {
            tax_sum += ((num - 609350.0)  * 0.37) + (609350.0  * 0.35) + (243725.0  * 0.32) + (191950.0  * 0.24) + (100525.0  * 0.22) + (47150.0  * 0.12) + (11600.0  * 0.1);
        } else {
            tax_sum = 0.0;
        }

/*
income: 5125
E: 3750
subtract: 5125 - 3750 = 1375
D: 4%
multiply: 1375 * 0.04 = 55
C: 83
add: 55 + 83 = 138


income:750
E: 0
subtract: 750 - 0 = 750
D: 1%
multiply: 750 * 0.01 = 7.5
C: 0
add: 7.5 + 0 = 7.5
 */
        // state taxes (GA) P.S. These are dumb and arbitrary
        // https://dor.georgia.gov/tax-tables-georgia-tax-rate-schedule
        if num > 0.0 && num <= 750.0 {
            tax_sum += num * 0.01;
        }
        else if num > 750.0 &&  num <= 2250.0 {
            tax_sum += ((num - 750.0)  * 0.02) + 8.0;
        }
        else if num > 2250.0 &&  num <= 3750.0 {
            tax_sum += ((num - 2250.0)  * 0.03) + 38.0;
        }
        else if num > 3750.0 &&  num <= 5250.0 {
            tax_sum += ((num - 3750.0)  * 0.04) + 83.0;
        }
        else if num > 5250.0 &&  num <= 7000.0 {
            tax_sum += ((num - 5250.0)  * 0.05) + 143.0;
        }
        else if num > 7000.0 {
            tax_sum += ((num - 7000.0)  * 0.0575) + 230.0;
        }
        else {}

        tax_sum += (num * 0.062) + (num * 0.0145);

        let result = format!("Tax: {:.2}", tax_sum);
        ui.set_results(result.into());
    });

    ui.run()
}
