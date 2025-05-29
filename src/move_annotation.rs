use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum MoveAnnotation {
    Excellent,
    Fair,
    Inaccuracy,
    Mistake,
    Blunder,
}
impl Serialize for MoveAnnotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let annotation_index = match self {
            MoveAnnotation::Excellent => 1,
            MoveAnnotation::Fair => 2,
            MoveAnnotation::Inaccuracy => 3,
            MoveAnnotation::Mistake => 4,
            MoveAnnotation::Blunder => 5,
        };
        serializer.serialize_i32(annotation_index)
    }
}

impl MoveAnnotation {
    pub fn from_parameters(win_percent_loss: f32, current_win_percent: f32) -> Self {
        if win_percent_loss < 3.0 && current_win_percent > 20.0 {
            MoveAnnotation::Excellent
        } else if win_percent_loss < 10.0 {
            MoveAnnotation::Fair
        } else if win_percent_loss < 20.0 {
            MoveAnnotation::Inaccuracy
        } else if win_percent_loss < 30.0 {
            MoveAnnotation::Mistake
        } else {
            MoveAnnotation::Blunder
        }
    }
}