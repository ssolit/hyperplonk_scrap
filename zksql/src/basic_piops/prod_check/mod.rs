// use crate::{
//     pcs::PolynomialCommitmentScheme,
//     poly_iop::{
//         errors::PolyIOPErrors,
//         prod_check::util::{compute_frac_poly, compute_product_poly, prove_zero_check},
//         zero_check::new_stuff::{ZeroCheckIOP, ZeroCheckIOPSubClaim, ZeroCheckIOPProof},
//         // PolyIOP,
//     },
// };
// use arithmetic::VPAuxInfo;
// use ark_ec::pairing::Pairing;
// use ark_ff::{One, PrimeField, Zero};
// use ark_poly::DenseMultilinearExtension;
// use ark_std::{end_timer, start_timer};
// use std::sync::Arc;
// use std::{fmt::Debug, marker::PhantomData};
// use transcript::IOPTranscript;


// // use super::util;




// /// A product-check proves that two lists of n-variate multilinear polynomials
// /// `(f1, f2, ..., fk)` and `(g1, ..., gk)` satisfy:
// /// \prod_{x \in {0,1}^n} f1(x) * ... * fk(x) = \prod_{x \in {0,1}^n} g1(x) *
// /// ... * gk(x)
// ///
// /// A ProductCheck is derived from ZeroCheck.
// ///
// /// Prover steps:
// /// 1. build MLE `frac(x)` s.t. `frac(x) = f1(x) * ... * fk(x) / (g1(x) * ... *
// /// gk(x))` for all x \in {0,1}^n 2. build `prod(x)` from `frac(x)`, where
// /// `prod(x)` equals to `v(1,x)` in the paper 2. push commitments of `frac(x)`
// /// and `prod(x)` to the transcript,    and `generate_challenge` from current
// /// transcript (generate alpha) 3. generate the zerocheck proof for the virtual
// /// polynomial Q(x):       prod(x) - p1(x) * p2(x)
// ///     + alpha * frac(x) * g1(x) * ... * gk(x)
// ///     - alpha * f1(x) * ... * fk(x)
// /// where p1(x) = (1-x1) * frac(x2, ..., xn, 0)
// ///             + x1 * prod(x2, ..., xn, 0),
// /// and p2(x) = (1-x1) * frac(x2, ..., xn, 1)
// ///           + x1 * prod(x2, ..., xn, 1)
// ///
// /// Verifier steps:
// /// 1. Extract commitments of `frac(x)` and `prod(x)` from the proof, push
// /// them to the transcript
// /// 2. `generate_challenge` from current transcript (generate alpha)
// /// 3. `verify` to verify the zerocheck proof and generate the subclaim for
// /// polynomial evaluations

// pub struct ProductCheckIOP<E: Pairing, PCS: PolynomialCommitmentScheme<E>>(PhantomData<E>, PhantomData<PCS>);

// /// A product check subclaim consists of
// /// - A zero check IOP subclaim for the virtual polynomial
// /// - The random challenge `alpha`
// /// - A final query for `prod(1, ..., 1, 0) = 1`.
// // Note that this final query is in fact a constant that
// // is independent from the proof. So we should avoid
// // (de)serialize it.
// #[derive(Clone, Debug, Default, PartialEq)]
// pub struct ProductCheckIOPSubClaim<F: PrimeField> {
//     // the SubClaim from the ZeroCheck
//     pub zero_check_sub_claim: ZeroCheckIOPSubClaim<F>,
//     // final query which consists of
//     // - the vector `(1, ..., 1, 0)` (needs to be reversed because Arkwork's MLE uses big-endian
//     //   format for points)
//     // The expected final query evaluation is 1
//     pub final_query: (Vec<F>, F),
//     pub alpha: F,
// }
// pub type Transcript<F> = IOPTranscript<F>;

// #[derive(Clone, Debug, Default, PartialEq)]
// pub struct ProductCheckIOPProof<
//     E: Pairing,
//     PCS: PolynomialCommitmentScheme<E>,
// > {
//     pub zero_check_proof: ZeroCheckIOPProof<E::ScalarField>,
//     pub prod_x_comm: PCS::Commitment,
//     pub frac_comm: PCS::Commitment,
// }

// impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> ProductCheckIOP<E, PCS> 
// where PCS: PolynomialCommitmentScheme<E, Polynomial = Arc<DenseMultilinearExtension<E::ScalarField>>>
// {
//     pub fn init_transcript() -> Transcript<E::ScalarField> {
//         IOPTranscript::<E::ScalarField>::new(b"Initializing ProductCheck transcript")
//     }

//     pub fn prove(
//         pcs_param: &PCS::ProverParam,
//         fxs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         gxs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         transcript: &mut IOPTranscript<E::ScalarField>,
//     ) -> Result<
//         (
//             ProductCheckIOPProof<E, PCS>,
//             Arc<DenseMultilinearExtension<E::ScalarField>>,
//             Arc<DenseMultilinearExtension<E::ScalarField>>,
//         ),
//         PolyIOPErrors,
//     > {
//         let start = start_timer!(|| "prod_check prove");

//         if fxs.is_empty() {
//             return Err(PolyIOPErrors::InvalidParameters("ProductCheckIOP Error: fxs is empty".to_string()));
//         }
//         if fxs.len() != gxs.len() {
//             return Err(PolyIOPErrors::InvalidParameters(
//                 "ProductCheckIOP Error: fxs and gxs have different number of polynomials".to_string(),
//             ));
//         }
//         for poly in fxs.iter().chain(gxs.iter()) {
//             if poly.num_vars != fxs[0].num_vars {
//                 return Err(PolyIOPErrors::InvalidParameters(
//                     "ProductCheckIOP Error: fx and gx have different number of variables".to_string(),
//                 ));
//             }
//         }

//         // compute the fractional polynomial frac_p s.t.
//         // frac_p(x) = f1(x) * ... * fk(x) / (g1(x) * ... * gk(x))
//         let frac_poly = compute_frac_poly::<E::ScalarField>(fxs, gxs)?;
//         // compute the product polynomial
//         let prod_x = compute_product_poly(&frac_poly)?;

//         // generate challenge
//         let frac_comm = PCS::commit(pcs_param, &frac_poly)?;
//         let prod_x_comm = PCS::commit(pcs_param, &prod_x)?;
//         transcript.append_serializable_element(b"frac(x)", &frac_comm)?;
//         transcript.append_serializable_element(b"prod(x)", &prod_x_comm)?;
//         let alpha = transcript.get_and_append_challenge(b"alpha")?;

//         // build the zero-check proof
//         let (zero_check_proof, _) =
//             prove_zero_check(fxs, gxs, &frac_poly, &prod_x, &alpha, transcript)?;

//         end_timer!(start);

//         Ok((
//             ProductCheckIOPProof {
//                 zero_check_proof,
//                 prod_x_comm,
//                 frac_comm,
//             },
//             prod_x,
//             frac_poly,
//         ))
//     }

//     pub fn verification_info(
//         _pcs_param: &PCS::ProverParam,
//         fxs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         _gxs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         _transcript: &mut IOPTranscript<E::ScalarField>,
//     ) -> VPAuxInfo<E::ScalarField> {
//         let aux_info = VPAuxInfo {
//             max_degree: fxs.len() + 1,
//             num_variables: fxs[0].num_vars,
//             phantom: PhantomData::default(),
//         };
//         return aux_info;
//     }

//     pub fn verify(
//         proof: &ProductCheckIOPProof<E, PCS>,
//         aux_info: &VPAuxInfo<E::ScalarField>,
//         transcript: &mut Transcript<E::ScalarField>,
//     ) -> Result<ProductCheckIOPSubClaim<E::ScalarField>, PolyIOPErrors> {
//         let start = start_timer!(|| "prod_check verify");

//         // update transcript and generate challenge
//         transcript.append_serializable_element(b"frac(x)", &proof.frac_comm)?;
//         transcript.append_serializable_element(b"prod(x)", &proof.prod_x_comm)?;
//         let alpha = transcript.get_and_append_challenge(b"alpha")?;

//         // invoke the zero check on the iop_proof
//         // the virtual poly info for Q(x)
//         let zero_check_sub_claim = ZeroCheckIOP::<E::ScalarField>::verify(
//             &proof.zero_check_proof,
//             aux_info,
//             transcript,
//         )?;

//         // the final query is on prod_x
//         let mut final_query = vec![E::ScalarField::one(); aux_info.num_variables];
//         // the point has to be reversed because Arkworks uses big-endian.
//         final_query[0] = E::ScalarField::zero();
//         let final_eval = E::ScalarField::one();

//         end_timer!(start);

//         Ok(ProductCheckIOPSubClaim {
//             zero_check_sub_claim,
//             final_query: (final_query, final_eval),
//             alpha,
//         })
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::ProductCheckIOP;
//     use crate::{
//         pcs::{prelude::MultilinearKzgPCS, PolynomialCommitmentScheme},
//         poly_iop::errors::PolyIOPErrors,
//     };
//     use arithmetic::VPAuxInfo;
//     use ark_bls12_381::{Bls12_381, Fr};
//     use ark_ec::pairing::Pairing;
//     use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
//     use ark_std::test_rng;
//     use std::{marker::PhantomData, sync::Arc};

//     fn check_frac_poly<E>(
//         frac_poly: &Arc<DenseMultilinearExtension<E::ScalarField>>,
//         fs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         gs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//     ) where
//         E: Pairing,
//     {
//         let mut flag = true;
//         let num_vars = frac_poly.num_vars;
//         for i in 0..1 << num_vars {
//             let nom = fs
//                 .iter()
//                 .fold(E::ScalarField::from(1u8), |acc, f| acc * f.evaluations[i]);
//             let denom = gs
//                 .iter()
//                 .fold(E::ScalarField::from(1u8), |acc, g| acc * g.evaluations[i]);
//             if denom * frac_poly.evaluations[i] != nom {
//                 flag = false;
//                 break;
//             }
//         }
//         assert!(flag);
//     }
//     // fs and gs are guaranteed to have the same product
//     // fs and hs doesn't have the same product
//     fn test_product_check_helper<E, PCS>(
//         fs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         gs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         hs: &[Arc<DenseMultilinearExtension<E::ScalarField>>],
//         pcs_param: &PCS::ProverParam,
//     ) -> Result<(), PolyIOPErrors>
//     where
//         E: Pairing,
//         PCS: PolynomialCommitmentScheme<
//             E,
//             Polynomial = Arc<DenseMultilinearExtension<E::ScalarField>>,
//         >,
//     {
//         let mut transcript = ProductCheckIOP::<E, PCS>::init_transcript();
//         transcript.append_message(b"testing", b"initializing transcript for testing")?;

//         let (proof, prod_x, frac_poly) = ProductCheckIOP::<E, PCS>::prove(
//             pcs_param,
//             fs,
//             gs,
//             &mut transcript,
//         )?;

//         let mut transcript = ProductCheckIOP::<E, PCS>::init_transcript();
//         transcript.append_message(b"testing", b"initializing transcript for testing")?;

//         // what's aux_info for?
//         let aux_info = VPAuxInfo {
//             max_degree: fs.len() + 1,
//             num_variables: fs[0].num_vars,
//             phantom: PhantomData::default(),
//         };
//         let prod_subclaim = ProductCheckIOP::verify(
//             &proof,
//             &aux_info,
//             &mut transcript,
//         )?;
//         assert_eq!(
//             prod_x.evaluate(&prod_subclaim.final_query.0).unwrap(),
//             prod_subclaim.final_query.1,
//             "different product"
//         );
//         check_frac_poly::<E>(&frac_poly, fs, gs);

//         // bad path
//         let mut transcript = ProductCheckIOP::<E, PCS>::init_transcript();
//         transcript.append_message(b"testing", b"initializing transcript for testing")?;

//         let (bad_proof, prod_x_bad, frac_poly) = ProductCheckIOP::<E, PCS>::prove(
//             pcs_param, fs, hs, &mut transcript
//         )?;

//         let mut transcript = ProductCheckIOP::<E, PCS>::init_transcript();
//         transcript.append_message(b"testing", b"initializing transcript for testing")?;
//         let bad_subclaim = ProductCheckIOP::verify(
//             &bad_proof,
//             &aux_info,
//             &mut transcript,
//         )?;
//         assert_ne!(
//             prod_x_bad.evaluate(&bad_subclaim.final_query.0).unwrap(),
//             bad_subclaim.final_query.1,
//             "can't detect wrong proof"
//         );
//         // the frac_poly should still be computed correctly
//         check_frac_poly::<E>(&frac_poly, fs, hs);

//         Ok(())
//     }

//     fn test_product_check(nv: usize) -> Result<(), PolyIOPErrors> {
//         let mut rng = test_rng();

//         let f1: DenseMultilinearExtension<Fr> = DenseMultilinearExtension::rand(nv, &mut rng);
//         let mut g1 = f1.clone();
//         g1.evaluations.reverse();
//         let f2: DenseMultilinearExtension<Fr> = DenseMultilinearExtension::rand(nv, &mut rng);
//         let mut g2 = f2.clone();
//         g2.evaluations.reverse();
//         let fs = vec![Arc::new(f1), Arc::new(f2)];
//         let gs = vec![Arc::new(g2), Arc::new(g1)];
//         let mut hs = vec![];
//         for _ in 0..fs.len() {
//             hs.push(Arc::new(DenseMultilinearExtension::rand(
//                 fs[0].num_vars,
//                 &mut rng,
//             )));
//         }

//         let srs = MultilinearKzgPCS::<Bls12_381>::gen_srs_for_testing(&mut rng, nv)?;
//         let (pcs_param, _) = MultilinearKzgPCS::<Bls12_381>::trim(&srs, None, Some(nv))?;

//         test_product_check_helper::<Bls12_381, MultilinearKzgPCS<Bls12_381>>(
//             &fs, &gs, &hs, &pcs_param,
//         )?;

//         Ok(())
//     }

//     #[test]
//     fn test_trivial_polynomial() -> Result<(), PolyIOPErrors> {
//         test_product_check(1)
//     }
//     #[test]
//     fn test_normal_polynomial() -> Result<(), PolyIOPErrors> {
//         test_product_check(10)
//     }
// }
