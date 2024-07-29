#[cfg(test)]
mod test {
    use ark_ec::pairing::Pairing;
    use ark_poly::MultilinearExtension;
    use ark_poly::DenseMultilinearExtension;
    use ark_std::One;

    use subroutines::{
        pcs::PolynomialCommitmentScheme,
        MultilinearKzgPCS
    };

    use ark_bls12_381::{Bls12_381, Fr};
    use ark_std::test_rng;

    use crate::{
        tracker::prelude::*,
        zksql_poly_iop::index_transform::{
            index_transform::IndexTransformIOP,
            utils::{table_row_prover_agg, table_row_verifier_agg},
        },
    };

    fn test_index_transform() -> Result<(), PolyIOPErrors> {
        // testing params
        let mut rng = test_rng();

        // PCS params
        let srs = MultilinearKzgPCS::<Bls12_381>::gen_srs_for_testing(&mut rng, 10)?;
        let (pcs_prover_param, pcs_verifier_param) = MultilinearKzgPCS::<Bls12_381>::trim(&srs, None, Some(10))?;

        // create trackers
        let mut prover_tracker: ProverTrackerRef<Bls12_381, MultilinearKzgPCS<Bls12_381>> = ProverTrackerRef::new_from_pcs_params(pcs_prover_param);
        let mut verifier_tracker: VerifierTrackerRef<Bls12_381, MultilinearKzgPCS<Bls12_381>> = VerifierTrackerRef::new_from_pcs_params(pcs_verifier_param);

        // good path 1: table_in = table_out
        print!("IndexTransformIOP good path 1 test: ");
        let nv = 4;
        let num_cols = 4;
        let table_sel_evals = vec![Fr::one(); 2_usize.pow(nv as u32)];
        let table_sel = DenseMultilinearExtension::from_evaluations_vec(nv, table_sel_evals);
        let mut table_cols = Vec::<DenseMultilinearExtension<Fr>>::new();
        for _ in 0..num_cols {
            table_cols.push(DenseMultilinearExtension::<Fr>::rand(nv, &mut rng));
        }
        test_index_transform_helper(
            &mut prover_tracker, 
            &mut verifier_tracker, 
            &table_sel.clone(), 
            table_cols.clone().as_slice(),
            &table_sel.clone(), 
            table_cols.clone().as_slice(),
        )?;

        Ok(())
    }

    fn test_index_transform_helper<E: Pairing, PCS>(
        prover_tracker: &mut ProverTrackerRef<E, PCS>,
        verifier_tracker: &mut VerifierTrackerRef<E, PCS>,
        table_in_sel_mle: &DenseMultilinearExtension<E::ScalarField>,
        table_in_cols_mle: &[DenseMultilinearExtension<E::ScalarField>],
        table_out_sel_mle: &DenseMultilinearExtension<E::ScalarField>,
        table_out_cols_mle: &[DenseMultilinearExtension<E::ScalarField>],
    ) -> Result<(), PolyIOPErrors>
    where
    E: Pairing,
    PCS: PolynomialCommitmentScheme<E>,
    {
        let table_in_sel = prover_tracker.track_and_commit_poly(table_in_sel_mle.clone())?;
        let mut table_in_cols = Vec::<TrackedPoly<E, PCS>>::new();
        for col in table_in_cols_mle {
            let tracked_col = prover_tracker.track_and_commit_poly(col.clone())?;
            table_in_cols.push(tracked_col);
        }
        let table_in = Table::new(table_in_cols.clone(), table_in_sel.clone());

        let table_out_sel = prover_tracker.track_and_commit_poly(table_out_sel_mle.clone())?;
        let mut table_out_cols = Vec::<TrackedPoly<E, PCS>>::new();
        for col in table_out_cols_mle {
            let tracked_col = prover_tracker.track_and_commit_poly(col.clone())?;
            table_out_cols.push(tracked_col);
        }
        let table_out = Table::new(table_out_cols.clone(), table_out_sel.clone());

        IndexTransformIOP::<E, PCS>::prove(
            prover_tracker,
            &table_in, 
            &table_out,
        )?;
        let proof = prover_tracker.compile_proof()?;

        // set up verifier tracker, create subclaims, and verify IOPProofs
        verifier_tracker.set_compiled_proof(proof);
        let table_in_sel_comm = verifier_tracker.transfer_prover_comm(table_in_sel.id);
        let mut table_in_cols_comm = Vec::<TrackedComm<E, PCS>>::new();
        for col in table_in_cols {
            let tracked_col = verifier_tracker.transfer_prover_comm(col.id);
            table_in_cols_comm.push(tracked_col);
        }
        let table_in_comm = TableComm::new(table_in_cols_comm, table_in_sel_comm, table_in.num_vars());
        let table_out_sel_comm = verifier_tracker.transfer_prover_comm(table_out_sel.id);
        let mut table_out_cols_comm = Vec::<TrackedComm<E, PCS>>::new();
        for col in table_out_cols {
            let tracked_col = verifier_tracker.transfer_prover_comm(col.id);
            table_out_cols_comm.push(tracked_col);
        }
        let table_out_comm = TableComm::new(table_out_cols_comm, table_out_sel_comm, table_out.num_vars());
        IndexTransformIOP::<E, PCS>::verify(
            verifier_tracker,
            &table_in_comm,
            &table_out_comm,
        )?;
        verifier_tracker.verify_claims()?;

        // check that the ProverTracker and VerifierTracker are in the same state
        let p_tracker = prover_tracker.clone_underlying_tracker();
        let v_tracker = verifier_tracker.clone_underlying_tracker();
        assert_eq!(p_tracker.id_counter, v_tracker.id_counter);
        assert_eq!(p_tracker.sum_check_claims, v_tracker.sum_check_claims);
        assert_eq!(p_tracker.zero_check_claims, v_tracker.zero_check_claims);

        Ok(())
    }

    #[test]
    fn index_transform_test() {
        let res = test_index_transform();
        res.unwrap();
    }
}