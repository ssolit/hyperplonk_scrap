use ark_ec::pairing::Pairing;
use ark_poly::DenseMultilinearExtension;
use ark_std::{end_timer, One, start_timer};
use std::marker::PhantomData;
use subroutines::pcs::PolynomialCommitmentScheme;
use crate::tracker::prelude::*;

use super::bag_multitool::BagMultitoolIOP;

pub struct BagInclusionIOP<E: Pairing, PCS: PolynomialCommitmentScheme<E>>(PhantomData<E>, PhantomData<PCS>);

impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> BagInclusionIOP<E, PCS> 
where PCS: PolynomialCommitmentScheme<E> {
    pub fn prove(
        tracker: &mut ProverTrackerRef<E, PCS>,
        fx: &Bag<E, PCS>,
        gx: &Bag<E, PCS>,
        mg: &TrackedPoly<E, PCS>,
    ) -> Result<(), PolyIOPErrors> {
        let start = start_timer!(|| "BagInclusionIOP prove");
        let nv = fx.num_vars();

        // initialize multiplicity vector
        let one_const_mle = DenseMultilinearExtension::from_evaluations_vec(nv, vec![E::ScalarField::one(); 2_usize.pow(nv as u32)]);
        let mf = tracker.track_mat_poly(one_const_mle);

        // call the bag_multitool prover
        BagMultitoolIOP::<E, PCS>::prove(tracker, &[fx.clone()], &[gx.clone()], &[mf.clone()], &[mg.clone()])?;    
        
        end_timer!(start);
        Ok(())
    }

    pub fn verify(
        tracker: &mut VerifierTrackerRef<E, PCS>,
        fx: &BagComm<E, PCS>,
        gx: &BagComm<E, PCS>,
        mg: &TrackedComm<E, PCS>,
    ) -> Result<(), PolyIOPErrors> {
        let start = start_timer!(|| "BagInclusionIOP verify");

        let one_closure = |_: &[E::ScalarField]| -> Result<<E as Pairing>::ScalarField, PolyIOPErrors> {Ok(E::ScalarField::one())};
        let one_comm = tracker.track_virtual_comm(Box::new(one_closure));
        BagMultitoolIOP::verify(tracker, &[fx.clone()], &[gx.clone()], &[one_comm.clone()], &[mg.clone()])?;
 
        end_timer!(start);
        Ok(())
    }
}