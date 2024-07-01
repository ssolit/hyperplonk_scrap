/// The Tracker is a data structure for creating and managing virtual commnomials and their commitments.
/// It is in charge of  
///                      1) Recording the structure of virtual commnomials and their products
///                      2) Recording the structure of virtual commnomials and their products
///                      3) Recording the commitments of virtual commnomials and their products
///                      4) Providing methods for adding virtual commnomials together
/// 
/// 

use ark_ec::pairing::Pairing;
use displaydoc::Display;

use ark_serialize::CanonicalSerialize;
use ark_std::One;
use core::panic;
use std::hash::Hash;
use derivative::Derivative;
use subroutines::PolynomialCommitmentScheme;
use std::{
    collections::HashMap,
    ops::Neg,
    // ops::Add,
    sync::Arc,
    cell::RefCell,
    rc::Rc,
    borrow::Borrow,
};
use transcript::{IOPTranscript, TranscriptError};



use crate::utils::tracker_structs::{TrackerID, TrackerSumcheckClaim, TrackerZerocheckClaim};
use crate::utils::prover_tracker::CompiledZKSQLProof;

#[derive(Derivative, Display)]
#[derivative(
    Clone(bound = "PCS: PolynomialCommitmentScheme<E>"),
)]
pub struct VerifierTracker<E: Pairing, PCS: PolynomialCommitmentScheme<E>>{
    pub pcs_params: PCS::VerifierParam,
    pub transcript: IOPTranscript<E::ScalarField>,
    pub id_counter: usize,
    pub materialized_comms: HashMap<TrackerID, Arc<PCS::Commitment>>, // map from id to Commitment
    pub virtual_comms: HashMap<TrackerID, Box<dyn Fn(&[E::ScalarField]) -> Result<(E::ScalarField), PCSError>>>, // id -> eval_fn
    pub query_map: HashMap<(TrackerID, Vec<E::ScalarField>), E::ScalarField>, // (poly_id, point) -> eval
    pub sum_check_claims: Vec<TrackerSumcheckClaim<E::ScalarField>>,
    pub zero_check_claims: Vec<TrackerZerocheckClaim<E::ScalarField>>,
    pub proof: CompiledZKSQLProof<E, PCS>,
}

impl<E: Pairing, PCS: PolynomialCommitmentScheme<E>> VerifierTracker<E, PCS> {
    pub fn new(pcs_params: PCS::VerifierParam, proof: CompiledZKSQLProof<E, PCS>) -> Self {
        Self {
            pcs_params,
            transcript: IOPTranscript::<E::ScalarField>::new(b"Initializing Tracnscript"),
            id_counter: 0,
            materialized_comms: HashMap::new(),
            virtual_comms: HashMap::new(),
            query_map: HashMap::new(),
            sum_check_claims: Vec::new(),
            zero_check_claims: Vec::new(),
            proof,
        }
    }

    pub fn gen_id(&mut self) -> TrackerID {
        let id = self.id_counter;
        self.id_counter += 1;
        TrackerID(id)
    }

    pub fn get_next_id(&mut self) -> TrackerID {
        TrackerID(self.id_counter)
    }

    pub fn track_mat_comm(
        &mut self, 
        comm: Arc<PCS::Commitment>
    ) -> TrackerID {
        // Create the new TrackerID
        let id = self.gen_id();
        
        // // Add the commitment to the materialized map
        // self.materialized_comms.insert(id.clone(), comm);
        
        // Create a virtual commitment for the interaction and decision phases
        let mut eval_fn = Box::new(|point: &[E::ScalarField]| {self.eval_map.get(id, point)});
        self.virtual_comms.insert(id.clone(), eval_fn);

         // add the commitment to the transcript
         self.transcript.append_serializable_element(b"comm", &comm);
        id
    }

    pub fn track_virt_comm(
        &mut self, 
        eval_fn: Box<dyn Fn(&[E::ScalarField]) -> Result<(E::ScalarField), PCSError> + 'static>, 
    ) -> TrackerID {
        // Create the new TrackerID
        let id = self.gen_id();

        // Add the commitment to the materialized map
        self.virtual_polynomials.insert(id.clone(), eval_fn);

        // Return the new TrackerID
        id
    }

    pub fn track_virt_comm(
        &mut self,
        virt: Vec<(E::ScalarField, Vec<TrackerID>)>
    ) -> TrackerID {
        let id = self.gen_id();
        self.virtual_openings.insert(id.clone(), virt);
        id
    }

    pub fn set_compiled_proof(&mut self, proof: CompiledZKSQLProof<E, PCS>) {
        self.proof = proof;
    }

    pub fn get_mat_comm(&self, id: TrackerID) -> Option<&E::ScalarField> {
        self.materialized_openings.get(&id)
    }

    pub fn get_virt_comm(&self, id: TrackerID) -> Option<&Vec<(E::ScalarField, Vec<TrackerID>)>> {
        self.virtual_openings.get(&id)
    }

    pub fn add_comms(
        &mut self, 
        c1_id: TrackerID, 
        c2_id: TrackerID
    ) -> TrackerID {
        let c1_mat = self.get_mat_comm(c1_id.clone());
        let c1_virt = self.get_virt_comm(c1_id.clone());
        let c2_mat = self.get_mat_comm(c2_id.clone());
        let c2_virt = self.get_virt_comm(c2_id.clone());

        let mut new_virt_rep = Vec::new();
        match (c1_mat.is_some(), c1_virt.is_some(), c2_mat.is_some(), c2_virt.is_some()) {
            // Bad Case: c1 not found
            (false, false, _, _) => {
                panic!("Unknown c1 TrackerID {:?}", c1_id);
            }
            // Bad Case: c2 not found
            (_, _, false, false) => {
                panic!("Unknown c2 TrackerID {:?}", c2_id);
            }
            // Case 1: both c1 and c2 are materialized
            (true, false, true, false) => {
                new_virt_rep.push((E::ScalarField::one(), vec![c1_id]));
                new_virt_rep.push((E::ScalarField::one(), vec![c2_id]));
            },
            // Case 2: c1 is materialized and c2 is virtual
            (true, false, false, true) => {
                new_virt_rep.push((E::ScalarField::one(), vec![c1_id]));
                new_virt_rep.append(&mut c2_virt.unwrap().clone());
            },
            // Case 3: c2 is materialized and c1 is virtual
            (false, true, true, false) => {
                new_virt_rep.append(&mut c1_virt.unwrap().clone());
                new_virt_rep.push((E::ScalarField::one(), vec![c2_id]));
            },
            // Case 4: both c1 and c2 are virtual
            (false, true, false, true) => {
                new_virt_rep.append(&mut c1_virt.unwrap().clone());
                new_virt_rep.append(&mut c2_virt.unwrap().clone());
            },
            // Handling unexpected cases
            _ => {
                panic!("Internal verifier_tracker::add_comms error. This code should be unreachable");
            },
        }

        let comm_id = self.gen_id();
        self.virtual_openings.insert(comm_id.clone(), new_virt_rep);
        comm_id
    }

    pub fn sub_comms(
        &mut self, 
        c1_id: TrackerID, 
        c2_id: TrackerID
    ) -> TrackerID {
        let neg_c2_id = self.track_virt_comm(vec![(E::ScalarField::one().neg(), vec![c2_id])]);
        self.add_comms(c1_id, neg_c2_id)
    }

    fn mul_comms(
        &mut self, 
        c1_id: TrackerID, 
        c2_id: TrackerID
    ) -> TrackerID {
        let c1_mat = self.get_mat_comm(c1_id.clone());
        let c1_virt = self.get_virt_comm(c1_id.clone());
        let c2_mat = self.get_mat_comm(c2_id.clone());
        let c2_virt = self.get_virt_comm(c2_id.clone());

        let mut new_virt_rep = Vec::new();
        match (c1_mat.is_some(), c1_virt.is_some(), c2_mat.is_some(), c2_virt.is_some()) {
            // Bad Case: c1 not found
            (false, false, _, _) => {
                panic!("Unknown c1 TrackerID {:?}", c1_id);
            }
            // Bad Case: c2 not found
            (_, _, false, false) => {
                panic!("Unknown c2 TrackerID {:?}", c2_id);
            }
            // Case 1: both c1 and c2 are materialized
            (true, false, true, false) => {
                new_virt_rep.push((E::ScalarField::one(), vec![c1_id, c2_id]));
            },
            // Case 2: c1 is materialized and c2 is virtual
            (true, false, false, true) => {
                let c2_rep = c2_virt.unwrap();
                c2_rep.iter().for_each(|(coeff, prod)| {
                    let mut new_prod = prod.clone();
                    new_prod.push(c1_id.clone());
                    new_virt_rep.push((coeff.clone(), new_prod));
                });
            },
            // Case 3: c2 is materialized and c1 is virtual
            (false, true, true, false) => {
                let c1_rep = c1_virt.unwrap();
                c1_rep.iter().for_each(|(coeff, prod)| {
                    let mut new_prod = prod.clone();
                    new_prod.push(c2_id.clone());
                    new_virt_rep.push((coeff.clone(), new_prod));
                });
            },
            // Case 4: both c1 and c2 are virtual
            (false, true, false, true) => {
                let c1_rep = c1_virt.unwrap();
                let c2_rep = c2_virt.unwrap();
                c1_rep.iter().for_each(|(c1_coeff, c1_prod)| {
                    c2_rep.iter().for_each(|(c2_coeff, c2_prod)| {
                        let new_coeff = *c1_coeff * c2_coeff;
                        let mut new_prod_vec = c1_prod.clone();
                        new_prod_vec.extend(c2_prod.clone());
                        new_virt_rep.push((new_coeff, new_prod_vec));
                    })
                });
            },
            // Handling unexpected cases
            _ => {
                panic!("Internal tracker::mul_comms error. This code should be unreachable");
            },
        }

        let comm_id = self.gen_id();
        self.virtual_openings.insert(comm_id.clone(), new_virt_rep);
        comm_id
    }

    // like evalutate for prover tracker
    // actually don't thing I need this, since verifier_compiled_proof will do everything?
    // fn open_comm( 
    //     &self, 
    //     comm_id: TrackerID, 
    //     point: Vec<E::ScalarField>, 
    //     value: E::ScalarField, 
    //     proof: PCS::Proof, 
    // ) -> Result<bool, PCSError> {
    //     todo!()
    // }



    pub fn get_and_append_challenge(&mut self, label: &'static [u8]) -> Result<E::ScalarField, TranscriptError> {
        self.transcript.get_and_append_challenge(label)
    }

    pub fn append_serializable_element<S: CanonicalSerialize>(
        &mut self,
        label: &'static [u8],
        group_elem: &S,
    ) -> Result<(), TranscriptError> {
        self.transcript.append_serializable_element(label, group_elem)
    }

    pub fn add_sumcheck_claim(&mut self, poly_id: TrackerID, claimed_sum: E::ScalarField) {
        self.sum_check_claims.push(TrackerSumcheckClaim::new(poly_id, claimed_sum));
    }
    pub fn add_zerocheck_claim(&mut self, poly_id: TrackerID) {
        self.zero_check_claims.push(TrackerZerocheckClaim::new(poly_id));
    }

    pub fn get_prover_comm(&self, id: TrackerID) -> Option<&Arc<PCS::Commitment>> { 
        self.proof.comms.get(&id) 
    }
    pub fn get_prover_polynomial_eval(&self, id: TrackerID) -> Option<&E::ScalarField> {
        self.proof.polynomial_evals.get(&id)
    }

    pub fn get_prover_claimed_sum(&self, id: TrackerID) -> Option<&E::ScalarField> {
        self.proof.sum_check_claims.get(&id)
    }

    

}

#[derive(Derivative)]
#[derivative(
    Clone(bound = "PCS: PolynomialCommitmentScheme<E>"),
)]
pub struct VerifierTrackerRef<E: Pairing, PCS: PolynomialCommitmentScheme<E>> {
    tracker_rc: Rc<RefCell<VerifierTracker<E, PCS>>>,
}
impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> PartialEq for VerifierTrackerRef<E, PCS> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.tracker_rc, &other.tracker_rc)
    }
}

impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> VerifierTrackerRef<E, PCS> {
    pub fn new(tracker_rc: Rc<RefCell<VerifierTracker<E, PCS>>>) -> Self {
        Self {tracker_rc}
    }

    pub fn new_from_tracker(tracker: VerifierTracker<E, PCS>) -> Self {
        Self {tracker_rc: Rc::new(RefCell::new(tracker)) }
    }

    // pub fn track_mat_comm(
    //     &mut self,
    //     comm: PCS::Commitment,
    //     open_eval: E::ScalarField,
    // ) -> TrackedComm<E, PCS> {
    //     let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
    //     let res_id = tracker_ref_cell.borrow_mut().track_mat_comm(Arc::new(comm), open_eval);
    //     TrackedComm::new(res_id, self.tracker_rc.clone())
    // }

    pub fn get_next_id(&mut self) -> TrackerID {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow_mut().get_next_id()
    }

    pub fn set_compiled_proof(&mut self, proof: CompiledZKSQLProof<E, PCS>) {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow_mut().set_compiled_proof(proof);
    }

    pub fn get_mat_comm<'a>(&'a self, id: TrackerID) -> E::ScalarField {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow().get_mat_comm(id).unwrap().clone()
    }

    pub fn get_virt_comm(&self, id: TrackerID) -> Vec<(E::ScalarField, Vec<TrackerID>)> {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow().get_virt_comm(id).unwrap().clone()
    }

   
    pub fn get_and_append_challenge(&mut self, label: &'static [u8]) -> Result<E::ScalarField, TranscriptError> {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow_mut().get_and_append_challenge(label)
    }

    pub fn append_serializable_element<S: CanonicalSerialize>(
        &mut self,
        label: &'static [u8],
        group_elem: &S,
    ) -> Result<(), TranscriptError> {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow_mut().append_serializable_element(label, group_elem)
    }

    pub fn add_sumcheck_claim(&mut self, poly_id: TrackerID, claimed_sum: E::ScalarField) {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow_mut().add_sumcheck_claim(poly_id, claimed_sum);
    }
    pub fn add_zerocheck_claim(&mut self, poly_id: TrackerID) {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        tracker_ref_cell.borrow_mut().add_zerocheck_claim(poly_id);
    }

    pub fn get_prover_polynomial_eval(&self, id: TrackerID) -> E::ScalarField {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        let tracker = tracker_ref_cell.borrow();
        let eval = tracker.get_prover_polynomial_eval(id).unwrap().clone();
        return eval;
    }

    pub fn get_prover_claimed_sum(&self, id: TrackerID) -> E::ScalarField {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        let tracker = tracker_ref_cell.borrow();
        let sum = tracker.get_prover_claimed_sum(id).unwrap().clone();
        return sum;
    }

    pub fn transfer_prover_comm(&mut self, id: TrackerID) -> TrackedComm<E, PCS> {
        let new_id: TrackerID;
        let comm: Arc<PCS::Commitment>;
        let val: E::ScalarField;
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        {
            // Scope the immutable borrow
            let tracker = tracker_ref_cell.borrow();
            let comm_opt: Option<&Arc<PCS::Commitment>> = tracker.proof.comms.get(&id);
            match comm_opt {
                Some(value) => {
                    comm = value.clone();
                },
                None => {
                    panic!("VerifierTracker Error: attempted to transfer prover comm, but id not found: {}", id);
                }
            }
            val = tracker.get_prover_polynomial_eval(id).unwrap().clone();
        } 
        let mut tracker = tracker_ref_cell.borrow_mut();
        new_id = tracker.track_mat_comm(comm);

        #[cfg(debug_assertions)] {
            assert_eq!(id, new_id, "VerifierTracker Error: attempted to transfer prover comm, but ids don't match: {}, {}", id, new_id);
        }
    
        TrackedComm::new(new_id, self.tracker_rc.clone())
    }

    // used for testing
    pub fn clone_underlying_tracker(&self) -> VerifierTracker<E, PCS> {
        let tracker_ref_cell: &RefCell<VerifierTracker<E, PCS>> = self.tracker_rc.borrow();
        let tracker = tracker_ref_cell.borrow();
        (*tracker).clone()
    }
}

#[derive(Derivative)]
#[derivative(
    Clone(bound = "PCS: PolynomialCommitmentScheme<E>"),
)]
pub struct TrackedComm<E: Pairing, PCS: PolynomialCommitmentScheme<E>> {
    pub id: TrackerID,
    pub tracker: Rc<RefCell<VerifierTracker<E, PCS>>>,
}
impl <E: Pairing, PCS: PolynomialCommitmentScheme<E>> PartialEq for TrackedComm<E, PCS> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.tracker, &other.tracker)
    }
}
impl<E: Pairing, PCS: PolynomialCommitmentScheme<E>> TrackedComm<E, PCS> {
    pub fn new(id: TrackerID, tracker: Rc<RefCell<VerifierTracker<E, PCS>>>) -> Self {
        Self { id, tracker }
    }

    pub fn same_tracker(&self, other: &TrackedComm<E, PCS>) -> bool {
        Rc::ptr_eq(&self.tracker, &other.tracker)
    }

    pub fn assert_same_tracker(&self, other: &TrackedComm<E, PCS>) {
        assert!(self.same_tracker(other), "TrackedComms are not from the same tracker");
    }
    
    pub fn add(&self, other: &TrackedComm<E, PCS>) -> Self {
        self.assert_same_tracker(&other);
        let tracker_ref: &RefCell<VerifierTracker<E, PCS>> = self.tracker.borrow();
        let res_id = tracker_ref.borrow_mut().add_comms(self.id.clone(), other.id.clone());
        TrackedComm::new(res_id, self.tracker.clone())
    }

    pub fn sub(&self, other: &TrackedComm<E, PCS>) -> Self {
        self.assert_same_tracker(&other);
        let tracker_ref: &RefCell<VerifierTracker<E, PCS>> = self.tracker.borrow();
        let res_id = tracker_ref.borrow_mut().sub_comms(self.id.clone(), other.id.clone());
        TrackedComm::new(res_id, self.tracker.clone())
    }

    pub fn mul(&self, other: &TrackedComm<E, PCS>) -> Self {
        self.assert_same_tracker(&other);
        let tracker_ref: &RefCell<VerifierTracker<E, PCS>> = self.tracker.borrow();
        let res_id = tracker_ref.borrow_mut().mul_comms(self.id.clone(), other.id.clone());
        TrackedComm::new(res_id, self.tracker.clone())
    }
}

// fn verifier(proof: Proof, tracker: &mut VerifierTracker) -> bool {
//     // Verifier expects a commitment from the prover
//     tracker.add_concrete_cm(&proof.commitments, |id, x| proof.evaluations[(id, x)]);
//     let challenge = tracker.challenge();
//     let evaluation = tracker.evaluate(id, challenge);
//     tracker.verify_compiled_proof() // invokes PCS::verify
// }