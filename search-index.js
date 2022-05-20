var searchIndex = JSON.parse('{\
"hyperplonk":{"doc":"","t":[],"n":[],"q":[],"d":[],"i":[],"f":[],"p":[]},\
"pcs":{"doc":"","t":[16,13,13,13,13,3,8,4,16,3,16,16,13,3,3,16,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,12,12,11,11,11,11,11,11,11,11,11,11,12,12,10,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,12,12,12,12,12],"n":["Commitment","InvalidParameters","InvalidProof","InvalidProver","InvalidVerifier","KZGMultilinearPC","MultilinearCommitmentScheme","PCSErrors","Proof","ProverParam","ProverParam","SRS","SerializationError","UniversalParams","VerifierParam","VerifierParam","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","commit","commit","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize_unchecked","deserialize_unchecked","deserialize_unchecked","deserialize_uncompressed","deserialize_uncompressed","deserialize_uncompressed","drop","drop","drop","drop","drop","extract_prover_param","extract_verifier_param","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","g","g","g_mask","g_mask","gen_srs_for_testing","h","h","init","init","init","init","init","into","into","into","into","into","num_vars","num_vars","open","open","powers_of_g","powers_of_h","prover_param","serialize","serialize","serialize","serialize_unchecked","serialize_unchecked","serialize_unchecked","serialize_uncompressed","serialize_uncompressed","serialize_uncompressed","serialized_size","serialized_size","serialized_size","setup","setup","to_owned","to_owned","to_owned","to_string","trim","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","uncompressed_size","uncompressed_size","uncompressed_size","verify","verify","vzip","vzip","vzip","vzip","vzip","0","0","0","0","0"],"q":["pcs","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","pcs::PCSErrors","","","",""],"d":["","Invalid parameters: {0}","Invalid Proof: {0}","Invalid Prover: {0}","Invalid Verifier: {0}","KZG Polynomial Commitment Scheme on multilinear extensions.","","A <code>enum</code> specifying the possible failure modes of the PCS.","","Prover Parameters","","","An error during (de)serialization: {0}","Universal Parameter","Verifier Parameters","","","","","","","","","","","","","","","","","","Generate a commitment for a polynomial","Generate a commitment for a polynomial.","","","","","","","","","","","","","","","","","","","","","","","","","Extract the prover parameters from the public parameters.","Extract the verifier parameters from the public parameters.","","","","","","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","generator for G1","generator of G1","g^randomness: g^t1, g^t2, …","g^t1, g^t2, …","Build SRS for testing. WARNING: THIS FUNCTION IS FOR …","generator for G2","generator of G2","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","number of variables","number of variables","On input a polynomial <code>p</code> and a point <code>point</code>, outputs a proof …","On input a polynomial <code>p</code> and a point <code>point</code>, outputs a proof …","<code>pp_{num_vars}</code>, <code>pp_{num_vars - 1}</code>, <code>pp_{num_vars - 2}</code>, …, …","<code>pp_{num_vars}</code>, <code>pp_{num_vars - 1}</code>, <code>pp_{num_vars - 2}</code>, …, …","prover parameters","","","","","","","","","","","","","Generate SRS from RNG. WARNING: THIS FUNCTION IS FOR …","Generate SRS from RNG. WARNING: THIS FUNCTION IS FOR …","","","","","Trim the universal parameters to specialize the public …","","","","","","","","","","","","","","","","","","","Verifies that <code>value</code> is the evaluation at <code>x</code> of the …","Verifies that <code>value</code> is the evaluation at <code>x</code> of the …","","","","","","","","","",""],"i":[1,2,2,2,2,0,0,0,1,0,1,1,2,0,0,1,3,2,4,5,6,3,2,4,5,6,4,5,6,4,5,6,1,3,3,2,4,5,6,3,2,4,5,6,4,5,6,4,5,6,4,5,6,3,2,4,5,6,4,4,2,2,4,5,6,3,2,2,4,5,6,5,6,4,6,4,5,6,3,2,4,5,6,3,2,4,5,6,5,6,1,3,5,5,4,4,5,6,4,5,6,4,5,6,4,5,6,1,3,4,5,6,2,4,3,2,4,5,6,3,2,4,5,6,3,2,4,5,6,4,5,6,1,3,3,2,4,5,6,7,8,9,10,11],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["universalparams",3]],[[["",0]],["proverparam",3]],[[["",0]],["verifierparam",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[],["result",4,[["pcserrors",4]]]],[[],["result",4,[["pcserrors",4]]]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["read",8]],["result",4,[["serializationerror",4]]]],[[["usize",0]]],[[["usize",0]]],[[["usize",0]]],[[["usize",0]]],[[["usize",0]]],[[["",0]],["proverparam",3]],[[["",0]],["verifierparam",3]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[["serializationerror",4]]],[[]],[[]],[[]],[[]],null,null,null,null,[[["",0],["usize",0]],["result",4,[["pcserrors",4]]]],null,null,[[],["usize",0]],[[],["usize",0]],[[],["usize",0]],[[],["usize",0]],[[],["usize",0]],[[]],[[]],[[]],[[]],[[]],null,null,[[],["result",4,[["pcserrors",4]]]],[[],["result",4,[["pcserrors",4]]]],null,null,null,[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0],["write",8]],["result",4,[["serializationerror",4]]]],[[["",0]],["usize",0]],[[["",0]],["usize",0]],[[["",0]],["usize",0]],[[["",0],["usize",0]],["result",4,[["pcserrors",4]]]],[[["",0],["usize",0]],["result",4,[["pcserrors",4]]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[["",0],["usize",0]],["result",4,[["pcserrors",4]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["usize",0]],[[["",0]],["usize",0]],[[["",0]],["usize",0]],[[],["result",4,[["bool",0],["pcserrors",4]]]],[[],["result",4,[["bool",0],["pcserrors",4]]]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,null],"p":[[8,"MultilinearCommitmentScheme"],[4,"PCSErrors"],[3,"KZGMultilinearPC"],[3,"UniversalParams"],[3,"ProverParam"],[3,"VerifierParam"],[13,"InvalidProver"],[13,"InvalidVerifier"],[13,"InvalidProof"],[13,"InvalidParameters"],[13,"SerializationError"]]},\
"poly_iop":{"doc":"","t":[13,13,13,13,13,3,4,16,16,13,16,16,8,16,16,16,16,3,16,16,8,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,12,11,11,11,11,11,11,11,11,11,11,11,10,10,11,11,11,11,11,11,11,11,11,12,10,10,11,11,11,11,14,11,11,11,11,11,11,11,11,11,11,11,11,10,10,11,11,11,11,11,12,12,12,12,12,12],"n":["InvalidParameters","InvalidProof","InvalidProver","InvalidTranscript","InvalidVerifier","PolyIOP","PolyIOPErrors","Proof","Proof","SerializationError","SubClaim","SubClaim","SumCheck","Transcript","Transcript","VPAuxInfo","VPAuxInfo","VirtualPolynomial","VirtualPolynomial","VirtualPolynomial","ZeroCheck","add_mle_list","aux_info","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","default","default","deref","deref","deref","deref_mut","deref_mut","deref_mut","drop","drop","drop","eq","evaluate","extract_sum","extract_sum","flattened_ml_extensions","fmt","fmt","fmt","fmt","from","from","from","from","init","init","init","init_transcript","init_transcript","init_transcript","init_transcript","into","into","into","mul_by_mle","ne","new","new_from_mle","products","prove","prove","prove","prove","rand","rand_zero","to_bytes","to_owned","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","verify","verify","verify","verify","vzip","vzip","vzip","0","0","0","0","0","0"],"q":["poly_iop","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","poly_iop::PolyIOPErrors","","","","",""],"d":["Invalid parameters: {0}","Invalid Proof: {0}","Invalid Prover: {0}","Invalid Transcript: {0}","Invalid Verifier: {0}","Struct for PolyIOP protocol. It has an associated type <code>F</code> …","A <code>enum</code> specifying the possible failure modes of the …","","","An error during (de)serialization: {0}","","","Trait for doing sum check protocols.","","","","","A virtual polynomial is a sum of products of multilinear …","","","","Add a product of list of multilinear extensions to self …","Aux information about the multilinear polynomial","","","","","","","","","","","","","","","","","","","","","","","Evaluate the virtual polynomial at point <code>point</code>. Returns an …","Extract sum from the proof","Extract sum from the proof","Stores multilinear extensions in which product …","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Initialize the system with a transcript","Initialize the system with a transcript","Initialize the system with a transcript","Initialize the system with a transcript","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Multiple the current VirtualPolynomial by an MLE:","","Creates an empty virtual polynomial with <code>num_variables</code>.","Creates an new virtual polynomial from a MLE and its …","list of reference to products (as usize) of multilinear …","Generate proof of the sum of polynomial over {0,1}^<code>num_vars</code>","initialize the prover to argue for the sum of polynomial …","Generate proof of the sum of polynomial over {0,1}^<code>num_vars</code>","Initialize the prover to argue for the sum of polynomial …","Sample a random virtual polynomial, return the polynomial …","Sample a random virtual polynomial that evaluates to zero …","Takes as input a struct, and converts them to a series of …","","","","","","","","","","","","","Verify the claimed sum using the proof","verify the claimed sum using the proof","Verify the claimed sum using the proof","Verify that the polynomial’s sum is zero using the proof.","","","","","","","","",""],"i":[1,1,1,1,1,0,0,2,3,1,2,3,0,2,3,2,3,0,2,3,0,4,4,1,4,5,1,4,5,4,5,4,5,4,5,1,4,5,1,4,5,1,4,5,4,4,2,5,4,1,1,4,5,1,1,4,5,1,4,5,2,3,5,5,1,4,5,4,4,4,4,4,2,3,5,5,4,4,0,4,5,1,1,4,5,1,4,5,1,4,5,2,3,5,5,1,4,5,6,7,8,9,10,11],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["result",4,[["polyioperrors",4]]]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["virtualpolynomial",3]],[[["",0]],["polyiop",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[],["virtualpolynomial",3]],[[],["polyiop",3]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]]],[[["usize",0]]],[[["usize",0]]],[[["",0],["virtualpolynomial",3]],["bool",0]],[[["",0]],["result",4,[["polyioperrors",4]]]],[[]],[[]],null,[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["serializationerror",4]]],[[]],[[]],[[]],[[],["usize",0]],[[],["usize",0]],[[],["usize",0]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0],["rc",3,[["densemultilinearextension",3]]]],["result",4,[["polyioperrors",4]]]],[[["",0],["virtualpolynomial",3]],["bool",0]],[[["usize",0]]],[[["rc",3,[["densemultilinearextension",3]]]]],null,[[],["result",4,[["polyioperrors",4]]]],[[],["result",4,[["polyioperrors",4]]]],[[],["result",4,[["polyioperrors",4]]]],[[],["result",4,[["polyioperrors",4]]]],[[["usize",0],["usize",0],["",0]],["result",4,[["polyioperrors",4]]]],[[["usize",0],["usize",0],["",0]],["result",4,[["polyioperrors",4]]]],null,[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[],["result",4,[["polyioperrors",4]]]],[[],["result",4,[["polyioperrors",4]]]],[[],["result",4,[["polyioperrors",4]]]],[[],["result",4,[["polyioperrors",4]]]],[[]],[[]],[[]],null,null,null,null,null,null],"p":[[4,"PolyIOPErrors"],[8,"SumCheck"],[8,"ZeroCheck"],[3,"VirtualPolynomial"],[3,"PolyIOP"],[13,"InvalidProver"],[13,"InvalidVerifier"],[13,"InvalidProof"],[13,"InvalidParameters"],[13,"InvalidTranscript"],[13,"SerializationError"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};