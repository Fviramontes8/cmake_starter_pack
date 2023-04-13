pub enum Features {
    Clangd,
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Doxygen,
}

impl std::str::FromStr for Features {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clangd" => Ok(Self::Clangd),
            "cpp11" => Ok(Self::Cpp11),
            "cpp14" => Ok(Self::Cpp14),
            "cpp17" => Ok(Self::Cpp17),
            "cpp20" => Ok(Self::Cpp20),
            "doxygen" => Ok(Self::Doxygen),
            _ => Err(format!(
                "Unknown variant: {}
C++11/14/17/20 (cpp11/14/17/20), and Doxygen (doxygen) are supported",
                s
            )),
        }
    }
}
