pub enum Libraries {
    BoostProgramOptions,
    OpenCV,
    Onnx,
    Mpi,
}

impl std::str::FromStr for Libraries {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "program_options" | "po" => Ok(Self::BoostProgramOptions),
            "opencv" => Ok(Self::OpenCV),
            "onnx" => Ok(Self::Onnx),
            "mpi" => Ok(Self::Mpi),
            _ => Err(format!(
                "Unknown variant: {}
boost program options (program_options), OpenCV (opencv), ONNX (onnx), and MPI (mpi) are supported",
                s
            )),
        }
    }
}
