// To match ^1.50.5-alpha.123+build123 like version strings
pub const SEMVER_REGEX: &str = r"^(?P<operator>\^|~|=)?\s?(?P<major>\d+|x|\*)(?:\.(?P<minor>\d+|x|\*))?(?:\.(?P<patch>\d+|x|\*))?(?:[-.](?P<alpha>[a-zA-Z0-9-]+(?:\.\w+)?))?(?:\+(?P<build>[a-zA-Z0-9-+]+))?$";

// To match >=1.50.5 <2.0.0 like version strings
pub const RANGE_REGEX: &str = r"^(\s+)?(?P<start_operator>~?[<>]=?|~|\^)?\s?(?P<start_major>\d+|x|\*)(?:\.(?P<start_minor>\d+|x|\*))?(?:\.(?P<start_patch>\d+|x|\*))?(?:(?P<connector>,|\|\|)?\s*(?P<end_operator>[<>]=?|~|\^)?\s*(?P<end_major>\d+|x|\*)(?:\.(?P<end_minor>\d+|x|\*))?(?:\.(?P<end_patch>\d+|x|\*))?)?$";

// To match 1.50.5 - 2.0.0 like version strings
pub const LINEAR_RANGE_REGEX: &str = r"^(?P<operator>\^|~|)?(?P<start_major>(\d)+|x|\*)\.?(?P<start_minor>(\d)+|x|\*)?\.?(?P<start_patch>(\d)+|x|\*)?\s?-(\^|~|)?(\s+)?(?P<end_major>(\d)+|x|\*)\.?(?P<end_minor>(\d)+|x|\*)?\.?(?P<end_patch>(\d)+|x|\*)?";