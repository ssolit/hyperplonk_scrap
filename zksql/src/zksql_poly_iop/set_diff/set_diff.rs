use ark_ec::pairing::Pairing;
use std::marker::PhantomData;

use subroutines::pcs::PolynomialCommitmentScheme;
use crate::{
    tracker::prelude::*,
    zksql_poly_iop::{
        bag_multitool::{bag_subset::BagSubsetIOP, bag_sum::BagSumIOP}, set_disjoint::set_disjoint::SetDisjointIOP, set_union::set_union::SetUnionIOP
    },
};

pub struct SetDiffIOP<E: Pairing, PCS: PolynomialCommitmentScheme<E>>(PhantomData<E>, PhantomData<PCS>);

impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> SetDiffIOP<E, PCS> 
where PCS: PolynomialCommitmentScheme<E> {
    pub fn prove(
        prover_tracker: &mut ProverTrackerRef<E, PCS>,
        bag_a: &Bag<E, PCS>,
        bag_b: &Bag<E, PCS>,
        bag_l: &Bag<E, PCS>,
        bag_m: &Bag<E, PCS>,
        bm_multiplicities: &TrackedPoly<E, PCS>,
        range_bag: &Bag<E, PCS>
    ) -> Result<(), PolyIOPErrors> {

        // Prove L and B are disjoint
        SetDisjointIOP::<E, PCS>::prove(
            prover_tracker,
            bag_l,
            bag_b,
            range_bag,
        )?;

        // prove L \union M = A
        BagSumIOP::<E, PCS>::prove(
            prover_tracker,
            bag_l,
            bag_m,
            bag_a,
        )?;

        // prove M \subseteq B
        BagSubsetIOP::<E, PCS>::prove(
            prover_tracker,
            bag_m,
            bag_b,
            bm_multiplicities,
        )?;
        
        Ok(())
    }

    pub fn verify(
        verifier_tracker: &mut VerifierTrackerRef<E, PCS>,
        bag_a: &BagComm<E, PCS>,
        bag_b: &BagComm<E, PCS>,
        bag_l: &BagComm<E, PCS>,
        bag_m: &BagComm<E, PCS>,
        bm_multiplicities: &TrackedComm<E, PCS>,
        range_bag: &BagComm<E, PCS>,
    ) -> Result<(), PolyIOPErrors> {

        // verify L and B are disjoint
        SetDisjointIOP::<E, PCS>::verify(
            verifier_tracker,
            bag_l,
            bag_b,
            range_bag,
        )?;

        // veruft L \union M = A
        BagSumIOP::<E, PCS>::verify(
            verifier_tracker,
            bag_l,
            bag_m,
            bag_a,
        )?;

        // verify M \subseteq B
        BagSubsetIOP::<E, PCS>::verify(
            verifier_tracker,
            bag_m,
            bag_b,
            bm_multiplicities,
        )?;

        Ok(())
    }
}