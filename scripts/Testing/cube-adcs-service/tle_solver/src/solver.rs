use levenberg_marquardt::{LeastSquaresProblem, LevenbergMarquardt};
use nalgebra::storage::Owned;
use nalgebra::{DMatrix, DVector, Dyn, Vector3};
use sgp4::{Classification, Constants, Elements};
use chrono::NaiveDateTime;

const NUM_PARAMS: usize = 7;
const EPSILON: f64 = 1e-6;

pub struct OrbitFitter {
    pub obs_times_mins: Vec<f64>,
    pub obs_positions_teme: Vec<Vector3<f64>>,
    pub epoch_datetime: NaiveDateTime,
    
    pub params: DVector<f64>,
}

impl OrbitFitter {
    fn vector_to_elements(&self, p: &DVector<f64>) -> Elements {
        Elements {
            object_name: None,
            international_designator: None,
            norad_id: 0,
            classification: Classification::Unclassified,
            datetime: self.epoch_datetime,
            mean_motion_dot: 0.0,
            mean_motion_ddot: 0.0,
            drag_term: p[6],
            element_set_number: 0,
            inclination: p[0],
            right_ascension: p[1],
            eccentricity: p[2].abs(),
            argument_of_perigee: p[3],
            mean_anomaly: p[4],
            mean_motion: p[5],
            revolution_number: 0,
            ephemeris_type: 0,
        }
    }
}

impl LeastSquaresProblem<f64, Dyn, Dyn> for OrbitFitter {
    type ParameterStorage = Owned<f64, Dyn>;
    type ResidualStorage = Owned<f64, Dyn>;
    type JacobianStorage = Owned<f64, Dyn, Dyn>;

    fn set_params(&mut self, p: &DVector<f64>) {
        self.params = p.clone();
    }

    fn params(&self) -> DVector<f64> {
        self.params.clone()
    }

    fn residuals(&self) -> Option<DVector<f64>> {
        let elements = self.vector_to_elements(&self.params);
        let constants = match Constants::from_elements(&elements) {
            Ok(c) => c,
            Err(_) => return None, // Reject invalid physical states
        };

        let mut residuals = DVector::zeros(self.obs_times_mins.len() * 3);

        for (i, &t) in self.obs_times_mins.iter().enumerate() {
            // sgp4 no longer uses MinutesSinceEpoch wrapper, pass f64 directly
            let prediction = match constants.propagate(t) {
                Ok(p) => p,
                Err(_) => return None,
            };
            let pred_pos = Vector3::new(prediction.position[0], prediction.position[1], prediction.position[2]);
            let obs_pos = self.obs_positions_teme[i];

            let diff = pred_pos - obs_pos;
            residuals[i * 3] = diff.x;
            residuals[i * 3 + 1] = diff.y;
            residuals[i * 3 + 2] = diff.z;
        }

        Some(residuals)
    }

    fn jacobian(&self) -> Option<DMatrix<f64>> {
        let num_residuals = self.obs_times_mins.len() * 3;
        let mut jacobian = DMatrix::zeros(num_residuals, NUM_PARAMS);

        let base_residuals = self.residuals()?;

        for col in 0..NUM_PARAMS {
            let mut perturbed_params = self.params.clone();
            perturbed_params[col] += EPSILON;
            
            let temp_fitter = OrbitFitter {
                obs_times_mins: self.obs_times_mins.clone(),
                obs_positions_teme: self.obs_positions_teme.clone(),
                epoch_datetime: self.epoch_datetime,
                params: perturbed_params,
            };

            let perturbed_residuals = temp_fitter.residuals()?;

            for row in 0..num_residuals {
                jacobian[(row, col)] = (perturbed_residuals[row] - base_residuals[row]) / EPSILON;
            }
        }

        Some(jacobian)
    }
}

pub fn fit_tle_elements(observations: OrbitFitter) -> Result<Elements, String> {
    let (result, _report) = LevenbergMarquardt::new()
        .minimize(observations);
        
    Ok(result.vector_to_elements(&result.params))
}