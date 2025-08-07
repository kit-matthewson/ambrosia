use nalgebra::{DMatrix, DVector};
use osqp::{CscMatrix, Problem, Settings};
use serde::Deserialize;
use std::{collections::HashMap, fs, io};

#[derive(Deserialize, Debug)]
struct Config {
    min: f64,
    max: f64,
    cutoff: f64,
    targets: HashMap<String, f64>,
    items: HashMap<String, HashMap<String, f64>>,
}

fn main() -> io::Result<()> {
    // Process the config toml
    let toml_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&toml_content).expect("Failed to deserialize toml config");

    // We use the attribute names as given in the target list for reference
    let mut attribute_names: Vec<String> = config.targets.keys().cloned().collect();
    attribute_names.sort(); // Original order is not preserved on deserialization

    let mut item_names: Vec<String> = config.items.keys().cloned().collect();
    item_names.sort();

    // Construct the matrices
    let m = attribute_names.len();
    let n = config.items.len();

    let g = DVector::from_row_iterator(
        m,
        attribute_names.iter().map(|name| {
            config
                .targets
                .get(name)
                .expect("attribute not in target")
                .clone()
        }),
    );

    let mut c = DMatrix::<f64>::zeros(m, n);

    for (i, attribute) in attribute_names.iter().enumerate() {
        for (j, item) in item_names.iter().enumerate() {
            c[(i, j)] = *config
                .items
                .get(item)
                .expect("item does not exist")
                .get(attribute)
                .unwrap_or(&0.0); // TODO: Warn if attribute missing
        }
    }

    let p = 2.0 * c.transpose() * &c;
    let q = -2.0 * &c.transpose() * &g;

    // Solve with OSQP
    let a = DMatrix::<f64>::identity(n, n);
    let l = DMatrix::<f64>::from_element(n, 1, config.min / 100.0); // The Minimum
    let u = DMatrix::<f64>::from_element(n, 1, config.max / 100.0); // The Maximum

    let settings = Settings::default().verbose(false);
    let mut prob = Problem::new(
        CscMatrix::from_column_iter_dense(p.nrows(), p.ncols(), p.iter().map(|x| *x))
            .into_upper_tri(),
        &q.iter().map(|x| *x).collect::<Vec<f64>>(),
        CscMatrix::from_column_iter_dense(a.nrows(), a.ncols(), a.iter().map(|x| *x)),
        &l.iter().map(|x| *x).collect::<Vec<f64>>(),
        &u.iter().map(|x| *x).collect::<Vec<f64>>(),
        &settings,
    )
    .expect("failed to setup problem");

    let result = prob.solve();

    if let Some(x) = result.x() {
        let mut processed_result = Vec::new();
        let mut totals = HashMap::new();

        for (item, amount) in item_names.iter().zip(x) {
            if *amount > config.cutoff / 100.0 {
                processed_result.push((item, amount * 100.0));

                for attribute in attribute_names.iter() {
                    totals.insert(
                        attribute,
                        totals.get(attribute).unwrap_or(&0.0)
                            + config
                                .items
                                .get(item)
                                .unwrap()
                                .get(attribute)
                                .unwrap_or(&0.0)
                                * amount,
                    );
                }
            }
        }

        println!("From the possible ingredients:");
        for item in &item_names {
            println!("- {}", item);
        }

        println!("\nOptimal Mix:");
        for (item, amount) in &processed_result {
            println!("- {:.2}g {}", amount, item);
        }

        println!("\nTotal Nutrition:");
        for (attribute, amount) in totals {
            println!(
                "- {}: {:.2} ({:.2}%)",
                attribute,
                amount,
                amount / config.targets.get(attribute).unwrap() * 100.0
            )
        }

        let total_weight = processed_result.iter().fold(0.0, |acc, (_, a)| acc + a);
        println!("Total Weight: {:.2}g", total_weight);
    }

    Ok(())
}
