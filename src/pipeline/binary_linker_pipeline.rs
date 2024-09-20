use crate::contracts::Pipe;
use crate::errors::ExecutionError;
use crate::package::{BinType, NpmPackage, PackageMetaRecorder};


#[derive(Default)]
pub struct BinaryLinkerPipeline {
    packages: Vec<PackageMetaRecorder>
}


impl BinaryLinkerPipeline {
    pub fn new(packages: Vec<PackageMetaRecorder>) -> BinaryLinkerPipeline {
        BinaryLinkerPipeline{
            packages,
        }
    }

    fn build_bin_for_name_and_path(name: String, path: String) {
        let shell_script =
    }
}


impl Pipe<()> for BinaryLinkerPipeline{
    async fn run(&mut self) -> Result<(), ExecutionError> {
        self.packages.iter().for_each(|p|{
            if let Some(bin) = &p.bin {
                match bin {
                    BinType::BinMappings(bin_mapping) => {

                    }
                    BinType::Bin(bin) => {


                    }
                }
            }
        });
        Ok(())
    }
}