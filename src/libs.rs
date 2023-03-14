pub enum Libs {
    OpenCV,
    ONNX,
    MPI,
}

impl std::str::FromStr for Libs {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opencv" => Ok(Libs::OpenCV),
            "onnx" => Ok(Libs::ONNX),
            "mpi" => Ok(Libs::MPI),
            _ => Err(format!("Unknown variant: {}", s)),
        }
    }
}
