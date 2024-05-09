pub mod daisy;
pub mod gourd;
pub mod jasmine;
pub mod lily;
pub mod loquat;
pub mod morus;
pub mod musa;
pub mod tuberose;

pub mod azalea {
    pub mod v1 {
        tonic::include_proto!("palm.azalea.v1");
    }
}
pub mod camelia {
    pub mod v1 {
        tonic::include_proto!("palm.camelia.v1");
    }
}
