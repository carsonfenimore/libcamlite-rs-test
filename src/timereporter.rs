use std::time::Instant;

pub struct RateReporter {
    timeOfLastReport: Instant,
    count: u32, 
    tag: String,
}

impl RateReporter {
    const REPORTING_SECS:f32 = 2.0;

    pub fn new(tag: &str) -> Self{
        RateReporter {
            timeOfLastReport: Instant::now(),
            count: 0,
            tag: tag.to_string()
        }
    }
    pub fn tick(&mut self){
        self.count += 1;
        let dur = self.timeOfLastReport.elapsed();
        let deltaSecs: f32 = dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0;
        if deltaSecs >= Self::REPORTING_SECS {
            println!("{}: {} calls in past {} seconds", self.tag, self.count, deltaSecs);
            self.count = 0;
            self.timeOfLastReport = Instant::now();
        }
    }
}

