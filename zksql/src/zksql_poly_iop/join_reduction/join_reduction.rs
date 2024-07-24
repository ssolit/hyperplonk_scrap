use ark_ec::pairing::Pairing;
use std::marker::PhantomData;
use ark_std::One;
use std::ops::Neg;

use subroutines::pcs::PolynomialCommitmentScheme;
use crate::{
    tracker::prelude::*,
    zksql_poly_iop::{
        bag_multitool::{
            bag_multitool::BagMultitoolIOP, 
            bag_inclusion::BagInclusionIOP, 
            bag_sum::BagSumIOP
        }, 
        bag_no_zeros::BagNoZerosIOP, 
        set_disjoint::set_disjoint::SetDisjointIOP,
    },
};

pub struct JoinReductionIOP<E: Pairing, PCS: PolynomialCommitmentScheme<E>>(PhantomData<E>, PhantomData<PCS>);

impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> JoinReductionIOP<E, PCS> 
where PCS: PolynomialCommitmentScheme<E> {
    pub fn prove(
        prover_tracker: &mut ProverTrackerRef<E, PCS>,
        bag_a: &Bag<E, PCS>,
        bag_b: &Bag<E, PCS>,
        l_sel: &TrackedPoly<E, PCS>,
        r_sel: &TrackedPoly<E, PCS>,
        mid_a_inclusion_m: &TrackedPoly<E, PCS>,
        mid_b_inclusion_m: &TrackedPoly<E, PCS>,
        range_bag: &Bag<E, PCS>, // needed for SetDisjointIOP
    ) -> Result<(), PolyIOPErrors> {

        let ma_sel = &l_sel.mul_scalar(E::ScalarField::one().neg()).add_scalar(E::ScalarField::one());
        let mb_sel = &r_sel.mul_scalar(E::ScalarField::one().neg()).add_scalar(E::ScalarField::one());
        let l_bag = Bag::new(bag_a.poly.clone(), bag_a.selector.mul_poly(l_sel));
        let r_bag = Bag::new(bag_b.poly.clone(), bag_b.selector.mul_poly(r_sel));
        let ma_bag = Bag::new(bag_a.poly.clone(), bag_a.selector.mul_poly(ma_sel));
        let mb_bag = Bag::new(bag_b.poly.clone(), bag_b.selector.mul_poly(mb_sel));

        // Prove L and R are disjoint
        SetDisjointIOP::<E, PCS>::prove(
            prover_tracker,
            &l_bag,
            &r_bag,
            &range_bag,
        )?;

        // prove mid_a and mid_b have the same support
        BagInclusionIOP::<E, PCS>::prove(
            prover_tracker,
            &ma_bag,
            &mb_bag,
            mid_b_inclusion_m,
        )?;
       BagInclusionIOP::<E, PCS>::prove(
            prover_tracker,
            &mb_bag,
            &ma_bag,
            mid_a_inclusion_m,
        )?;
        
        Ok(())
    }

    pub fn verify(
        verifier_tracker: &mut VerifierTrackerRef<E, PCS>,
        bag_a: &BagComm<E, PCS>,
        bag_b: &BagComm<E, PCS>,
        l_sel: &TrackedComm<E, PCS>,
        r_sel: &TrackedComm<E, PCS>,
        mid_a_inclusion_m: &TrackedComm<E, PCS>,
        mid_b_inclusion_m: &TrackedComm<E, PCS>,
        range_bag: &BagComm<E, PCS>,
    ) -> Result<(), PolyIOPErrors> {

        let ma_sel = &l_sel.mul_scalar(E::ScalarField::one().neg()).add_scalar(E::ScalarField::one());
        let mb_sel = &r_sel.mul_scalar(E::ScalarField::one().neg()).add_scalar(E::ScalarField::one());
        let l_bag = BagComm::new(bag_a.poly.clone(), bag_a.selector.mul_comms(l_sel), bag_a.num_vars());
        let r_bag = BagComm::new(bag_b.poly.clone(), bag_b.selector.mul_comms(r_sel), bag_b.num_vars());
        let ma_bag = BagComm::new(bag_a.poly.clone(), bag_a.selector.mul_comms(ma_sel), bag_a.num_vars());
        let mb_bag = BagComm::new(bag_b.poly.clone(), bag_b.selector.mul_comms(mb_sel), bag_b.num_vars());

        // Prove L and R are disjoint
        SetDisjointIOP::<E, PCS>::verify(
            verifier_tracker,
            &l_bag,
            &r_bag,
            &range_bag,
        )?;

        // verify mid_a and mid_b have the same support
        BagInclusionIOP::<E, PCS>::verify(
            verifier_tracker,
            &ma_bag,
            &mb_bag,
            mid_b_inclusion_m,
        )?;
       BagInclusionIOP::<E, PCS>::verify(
            verifier_tracker,
            &mb_bag,
            &ma_bag,
            mid_a_inclusion_m,
        )?;

        Ok(())
    }
}