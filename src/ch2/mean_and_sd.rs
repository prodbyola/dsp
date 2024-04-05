use super::ext::ext::Sample;

/// Calculates mean
pub fn arithmetic_mean<T: Sample>(samples: &[T]) -> f64 {
    assert!(!samples.is_empty());

    let mut sum: usize = 0;

    for i in samples {
        sum += i.into_usize()
    }

    // dividing as f64 to manage truncation
    sum as f64 / samples.len() as f64
}

///  Calculates standard deviation
pub fn standard_deviation<T: Sample>(samples: &[T]) -> f64 {
    assert!(!samples.is_empty());

    let mut s: f64 = 0.0; // sum of squared difference

    let m = arithmetic_mean(samples);

    for i in samples {
        let p = (i.into_f64() - m).powi(2);
        s += p;
    }

    let v = s / ((samples.len() - 1) as f64);
    v.sqrt()
}

/// For previous implementation of standard deviation procedure, we could get excessive 
/// round-off error if the mean is much larger than the standard deviation.
/// Also, it's a good idea to keep track of some parameters such as the total number of samples processed (`tsp`),
/// the `sum` of `tsp` and the sum of squared `tsp`, in order to get a "real-time" feedback on what's going on 
/// while we process our samples. We also need to be able to add more samples as we go, then resume processing
/// from where we stopped. 
/// 
/// So here's another implementation that cosiders these factors. This type of implementation is known as "running statistics."
/// You can read more here: https://www.analog.com/media/en/technical-documentation/dsp-book/dsp_book_Ch2.pdf
/// 
/// Thanks to my friends [Georgios Christian Antonopoulos](https://www.linkedin.com/in/ACoAAB7am7MBEP8pL_BJk_PSu95vQFKfpC5v76E?lipi=urn%3Ali%3Apage%3Ad_flagship3_feed%3BaNErbRQLQr2UlPsdsRjODQ%3D%3D)
/// and [Martin Grönlund](https://www.linkedin.com/in/ACoAAB0GZiAB85LL5HuQP543uesEgv3UT81FNek?lipi=urn%3Ali%3Apage%3Ad_flagship3_feed%3BaNErbRQLQr2UlPsdsRjODQ%3D%3D)
///  for their insightful feedbacks on previous implementation. I try to factor most of their considerations into this.
#[derive(Default, Debug)]
pub struct RS<T> {
    samples: Vec<T>,
    tsp: usize, // total number of samples processed
    sum_of_tsp: usize,
    sum_of_squares: f64,
    mean: f64,
    sd: f64, // standard deviation
}

impl<T: Sample> RS<T> {
    pub fn new(samples: Vec<T>, run_now: bool) -> RS<T> {
        assert!(!samples.is_empty());

        let mut rs = RS::default();
        rs.samples = samples;

        if run_now {
            rs.run();
        }

        rs
    }

    pub fn add_samples(&mut self, mut samples: Vec<T>, run_now: bool) {
        self.samples.append(&mut samples);

        if run_now {
            self.run();
        }
    }

    pub fn add_sample(&mut self, sample: T, run_now: bool) {
        self.samples.push(sample);

        if run_now {
            self.run();
        }
    }

    /// Computes mean and standard deviation using "running statistics." 
    fn run(&mut self) {
        // get remaining samples for processing
        let (_, rem) = self.samples.split_at(self.tsp);
    
        // process calculate mean and standard deviation
        for s in rem {
            let n = (self.tsp + 1) as f64;
            let sum = (self.sum_of_tsp + s.into_usize()) as f64;
            let sos = self.sum_of_squares + s.into_f64().powi(2);
    
            self.mean = sum / n; // mean
            let v = (sos - (sum as f64) / n) / n - 1.0; // variance
            self.sd = v.sqrt();
    
            self.tsp = n as usize;
            self.sum_of_tsp = sum as usize;
            self.sum_of_squares = sos;
        }
    }
}


#[cfg(test)]
mod test {
    use super::{standard_deviation, arithmetic_mean, RS};

    #[test]
    fn test_sd() {
        let data = vec![100, 12, 34, 73];

        assert_eq!(arithmetic_mean(&data), 54.75);
        assert_eq!(standard_deviation(&data), 39.322385482063524);

        let mut rs = RS::new(data, true);
        
        assert_eq!(rs.tsp, 4);
        assert_eq!(rs.mean, 54.75);
        assert_eq!(rs.sd, 64.36274155130435);
        
        rs.add_samples(vec![70, 22, 70, 35, 62], true);
        assert_eq!(rs.tsp, 9);
        assert_eq!(rs.mean, 53.111111111111114);
        assert_eq!(rs.sd, 59.553793506271745);

        rs.add_sample(240, true);
        assert_eq!(rs.tsp, 10);
    }
}
