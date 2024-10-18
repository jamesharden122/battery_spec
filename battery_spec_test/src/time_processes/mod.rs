use chrono::prelude::*;
use chrono::{DateTime, TimeDelta, Utc};

use nalgebra;
use rand::prelude::*;
use rand_distr::Poisson;
use std::path::PathBuf;
fn to_utc_date_time<Tz: TimeZone>(dt: DateTime<Tz>) -> DateTime<Utc> {
    dt.with_timezone(&Utc)
}

fn create_poisson_distributions(lambdas: Vec<f64>) -> Result<Vec<Poisson<f64>>, anyhow::Error> {
    Ok(lambdas
        .into_iter()
        .map(|lambda| Poisson::new(lambda).expect("Invalid lambda"))
        .collect())
}

pub async fn sample_from_poisson_distributions(
    poisson_distributions: Vec<Poisson<f64>>,
    mut rng: rand::rngs::ThreadRng,
    time_steps: i32,
) -> Result<Vec<f64>, anyhow::Error> {
    // Create a random number generator
    let mut pois_full_sample: Vec<f64> = Vec::new();
    let mut count = 1;
    loop {
        pois_full_sample.append(
            &mut poisson_distributions
                .iter()
                .map(|dist| dist.sample(&mut rng) / 60.0) // Sample from each distribution
                .collect::<Vec<f64>>(),
        );
        if count == time_steps {
            break;
        }
        count += 1;
    }
    Ok(pois_full_sample) // Collect the samples into a new Vec<u64>
}
pub async fn modulated_markov_poison_process(
    date1: &str,
    date2: &str,
    mut lambda: Vec<f64>,
) -> Result<
    (
        nalgebra::base::DMatrix<f64>,
        nalgebra::base::DMatrix<f64>,
        nalgebra::base::DMatrix<DateTime<Utc>>,
    ),
    anyhow::Error,
> {
    let rng = thread_rng();
    println!(
        "Lambda rate is: {}",
        lambda.clone().into_iter().sum::<f64>()
    );
    let date_fmt = "%Y-%m-%d %H:%M:%S%:z"; // Ensure this format matches your input data
    let d1 = DateTime::parse_from_str(date1, date_fmt)?;
    let d2 = DateTime::parse_from_str(date2, date_fmt)?;
    let mut times: Vec<f64> = Vec::new();
    let mut day_hours = Vec::new();
    let mut curr_time = d1;
    loop {
        if curr_time == d2 {
            break;
        }
        day_hours.push(to_utc_date_time(curr_time));
        curr_time += TimeDelta::try_hours(1).unwrap();
        if curr_time <= (d1 + TimeDelta::try_hours(24).unwrap()) {
            times.push(curr_time.hour() as f64);
        }
    }
    let time_steps = day_hours.len() as i32;
    let hours_in_day: i32 = 24;

    let poisson_dist_vec: Vec<Poisson<f64>> = create_poisson_distributions(lambda.clone())?;
    let poisson_sample =
        sample_from_poisson_distributions(poisson_dist_vec, rng, time_steps / 24).await?;
    // `2014-07-08T09:10:11
    let mut matrix = times;
    matrix.append(&mut lambda);
    let parameter_matrix = nalgebra::base::DMatrix::from_vec(24, 2, matrix);
    let data_matrix = nalgebra::base::DMatrix::from_vec(
        24,
        (time_steps / 24).try_into().unwrap(),
        poisson_sample,
    );
    let days_matrix =
        nalgebra::base::DMatrix::from_vec(24, (time_steps / 24).try_into().unwrap(), day_hours);
    Ok((data_matrix, parameter_matrix, days_matrix))
}

