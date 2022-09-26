var searchIndex = JSON.parse('{\
"arithmetic":{"doc":"","t":[4,3,13,13,13,3,3,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,12,11,12,11,12,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["ArithErrors","DenseMultilinearExtension","InvalidParameters","SerializationErrors","ShouldNotArrive","VPAuxInfo","VirtualPolynomial","add","add","add","add_assign","add_assign","add_assign","add_mle_list","aux_info","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build_eq_x_r","build_f_hat","clone","clone","clone","clone_into","clone_into","clone_into","default","default","default","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize_unchecked","deserialize_uncompressed","drop","drop","drop","drop","eq","eq","eq","evaluate","evaluate","evaluations","fix_variables","flattened_ml_extensions","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from_evaluations_slice","from_evaluations_vec","get_hash","hash","index","init","init","init","init","into","into","into","into","is_zero","iter","iter_mut","max_degree","mul_by_mle","ne","ne","ne","neg","new","new_from_mle","num_variables","num_vars","num_vars","print_evals","products","rand","rand","rand_zero","random_zero_mle_list","relabel","relabel_inplace","serialize","serialize","serialize_unchecked","serialize_unchecked","serialize_uncompressed","serialize_uncompressed","serialized_size","serialized_size","sub","sub","sub_assign","sub_assign","to_evaluations","to_owned","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","uncompressed_size","uncompressed_size","vzip","vzip","vzip","vzip","zero","0","0"],"q":["arithmetic","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","arithmetic::ArithErrors",""],"d":["A <code>enum</code> specifying the possible failure modes of the …","Stores a multilinear polynomial in dense evaluation form.","Invalid parameters: {0}","An error during (de)serialization: {0}","Should not arrive to this point","Auxiliary information about the multilinear polynomial","A virtual polynomial is a sum of products of multilinear …","","","","","","","Add a product of list of multilinear extensions to self …","Aux information about the multilinear polynomial","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Evaluate the virtual polynomial at point <code>point</code>. Returns an …","","The evaluation over {0,1}^<code>num_vars</code>","","Stores multilinear extensions in which product …","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Construct a new polynomial from a list of evaluations …","Construct a new polynomial from a list of evaluations …","","","Returns the evaluation of the polynomial at a point …","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Returns an iterator that iterates over the evaluations …","Returns a mutable iterator that iterates over the …","max number of multiplicands in each product","Multiple the current VirtualPolynomial by an MLE:","","","","","Creates an empty virtual polynomial with <code>num_variables</code>.","Creates an new virtual polynomial from a MLE and its …","number of variables of the polynomial","","Number of variables","Print out the evaluation map for testing. Panic if the …","list of reference to products (as usize) of multilinear …","Sample a random virtual polynomial, return the polynomial …","","Sample a random virtual polynomial that evaluates to zero …","","","Relabel the point inplace by switching <code>k</code> scalars from …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,5,5,5,0,0,1,2,2,2,2,2,1,1,5,1,9,2,5,1,9,2,0,1,1,9,2,1,9,2,1,9,2,5,1,9,2,5,1,9,2,2,2,2,5,1,9,2,1,9,2,1,2,2,2,1,5,5,1,9,2,5,5,1,9,2,2,2,2,2,2,5,1,9,2,5,1,9,2,2,2,2,9,1,1,9,2,2,1,1,9,2,2,1,1,1,2,1,0,2,2,9,2,9,2,9,2,9,2,2,2,2,2,2,1,9,2,5,5,1,9,2,5,1,9,2,5,1,9,2,9,2,5,1,9,2,2,28,29],"f":[0,0,0,0,0,0,0,[[1,1]],[[2,2],2],[[2,2]],[[2,2]],[[2,2]],[2],[[[1,[3]],4,3],[[6,[5]]]],0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[[6,[[7,[[2,[3]]]],5]]]],[[[1,[3]]],[[6,[[1,[3]],5]]]],[[[1,[[0,[8,3]]]]],[[1,[[0,[8,3]]]]]],[[[9,[[0,[8,3]]]]],[[9,[[0,[8,3]]]]]],[2,2],[[]],[[]],[[]],[[],[[1,[[0,[10,3]]]]]],[[],[[9,[[0,[10,3]]]]]],[[],2],[11],[11],[11],[11],[11],[11],[11],[11],[[],[[6,[2,12]]]],[[],[[6,[2,12]]]],[[],[[6,[2,12]]]],[11],[11],[11],[11],[[[1,[[0,[13,3]]]],1],14],[[[9,[[0,[13,3]]]],9],14],[[2,2],14],[[[1,[3]]],[[6,[3,5]]]],[2,15],0,[2,2],0,[[5,16],17],[[5,16],17],[[[1,[[0,[18,3]]]],16],17],[[[9,[[0,[18,3]]]],16],17],[[2,16],[[6,[19]]]],[12,5],[[]],[[]],[[]],[[]],[11,2],[[11,[21,[20]]],2],[[],22],[2],[[2,11]],[[],11],[[],11],[[],11],[[],11],[[]],[[]],[[]],[[]],[2,14],[2,23],[2,24],0,[[[1,[3]],[7,[[2,[3]]]],3],[[6,[5]]]],[[[1,[[0,[13,3]]]],1],14],[[[9,[[0,[13,3]]]],9],14],[[2,2],14],[2],[11,[[1,[3]]]],[[7,3],[[1,[3]]]],0,[2,11],0,[[[1,[3]]]],0,[[11,11],[[6,[5]]]],[11,2],[[11,11],[[6,[[1,[3]],5]]]],[[11,11],[[21,[[7,[[2,[3]]]]]]]],[[2,11,11,11],2],[[2,11,11,11]],[[[9,[3]],25],[[6,[12]]]],[2,[[6,[12]]]],[[[9,[3]],25],[[6,[12]]]],[2,[[6,[12]]]],[[[9,[3]],25],[[6,[12]]]],[2,[[6,[12]]]],[[[9,[3]]],11],[2,11],[[2,2],2],[[2,2]],[[2,2]],[[2,2]],[2,[[21,[20]]]],[[]],[[]],[[]],[[],26],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],27],[[],27],[[],27],[[],27],[[[9,[3]]],11],[2,11],[[]],[[]],[[]],[[]],[[],2],0,0],"p":[[3,"VirtualPolynomial"],[3,"DenseMultilinearExtension"],[8,"PrimeField"],[8,"IntoIterator"],[4,"ArithErrors"],[4,"Result"],[3,"Rc"],[8,"Clone"],[3,"VPAuxInfo"],[8,"Default"],[15,"usize"],[4,"SerializationError"],[8,"PartialEq"],[15,"bool"],[4,"Option"],[3,"Formatter"],[6,"Result"],[8,"Debug"],[3,"Error"],[3,"Global"],[3,"Vec"],[15,"u64"],[3,"Iter"],[3,"IterMut"],[8,"Write"],[3,"String"],[3,"TypeId"],[13,"InvalidParameters"],[13,"SerializationErrors"]]},\
"hyperplonk":{"doc":"Main module for the HyperPlonk SNARK.","t":[8,16,16,16,16,14,10,10,10],"n":["HyperPlonkSNARK","Index","Proof","ProvingKey","VerifyingKey","build_mle","preprocess","prove","verify"],"q":["hyperplonk","","","","","","","",""],"d":["A trait for HyperPlonk SNARKs. A HyperPlonk is derived …","","","","","Build MLE from matrix of witnesses.","Generate the preprocessed polynomials output by the …","Generate HyperPlonk SNARK proof.","Verify the HyperPlonk proof."],"i":[0,4,4,4,4,0,4,4,4],"f":[0,0,0,0,0,0,[1,[[2,[0]]]],[1,[[2,[0]]]],[1,[[2,[3,0]]]]],"p":[[8,"Borrow"],[4,"Result"],[15,"bool"],[8,"HyperPlonkSNARK"]]},\
"poly_iop":{"doc":"","t":[3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,14,11,11,11,11,11,11,11,11,11,13,13,13,13,13,13,16,13,8,16,16,2,4,8,16,16,13,13,8,16,16,16,13,16,16,8,16,16,11,11,5,11,11,11,11,11,10,11,11,11,11,11,11,11,5,11,10,10,10,10,11,11,11,11,10,10,10,10,5,14,11,11,11,11,10,10,10,10,11,12,12,12,12,12,12,12,12,12],"n":["PolyIOP","__clone_box","as_any","as_any_mut","borrow","borrow_mut","clone","clone_into","default","deref","deref_mut","drop","eq","extract_sum","fmt","from","init","init_transcript","init_transcript","init_transcript","init_transcript","into","into_any","into_any_arc","into_any_rc","ne","prelude","prove","prove","prove","prove","to_bytes","to_owned","try_from","try_into","type_id","verify","verify","verify","verify","vzip","ArithmeticErrors","InvalidChallenge","InvalidParameters","InvalidProof","InvalidProver","InvalidVerifier","MultilinearExtension","PCSError","PermutationCheck","PermutationCheckSubClaim","PermutationProof","PolyIOP","PolyIOPErrors","ProductCheck","ProductCheckProof","ProductCheckSubClaim","SerializationErrors","ShouldNotArrive","SumCheck","SumCheckProof","SumCheckSubClaim","Transcript","TranscriptErrors","VPAuxInfo","VirtualPolynomial","ZeroCheck","ZeroCheckProof","ZeroCheckSubClaim","as_any","as_any_mut","bit_decompose","borrow","borrow_mut","deref","deref_mut","drop","extract_sum","fmt","fmt","from","from","from","from","from","identity_permutation_mle","init","init_transcript","init_transcript","init_transcript","init_transcript","into","into_any","into_any_arc","into_any_rc","prove","prove","prove","prove","random_permutation_mle","to_bytes","to_string","try_from","try_into","type_id","verify","verify","verify","verify","vzip","0","0","0","0","0","0","0","0","0"],"q":["poly_iop","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","poly_iop::prelude","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","poly_iop::prelude::PolyIOPErrors","","","","","","","",""],"d":["Struct for PolyIOP protocol. It has an associated type <code>F</code> …","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","Takes as input a struct, and converts them to a series of …","","","","","Verify that an MLE g(x) is a permutation of an MLE f(x) …","","","","","Arithmetic Error: {0}","Invalid challenge: {0}","Invalid parameters: {0}","Invalid Proof: {0}","Invalid Prover: {0}","Invalid Verifier: {0}","","PCS error {0}","A PermutationCheck w.r.t. <code>(f, g, perm)</code> proves that g is a …","","","","A <code>enum</code> specifying the possible failure modes of the …","A product-check proves that two n-variate multilinear …","","","An error during (de)serialization: {0}","Should not arrive to this point","Trait for doing sum check protocols.","","","","Transcript Error: {0}","","","A ZeroCheck for <code>f(x)</code> proves that <code>f(x) = 0</code> for all …","","","","","Decompose an integer into a binary vector in little endian.","","","","","","Extract sum from the proof","","","","","","","Returns the argument unchanged.","An MLE that represent an identity permutation: …","","Initialize the system with a transcript","Initialize the system with a transcript","Initialize the system with a transcript","Initialize the system with a transcript","Calls <code>U::from(self)</code>.","","","","Inputs:","Generate a proof for product check, showing that witness …","Generate proof of the sum of polynomial over {0,1}^<code>num_vars</code>","initialize the prover to argue for the sum of polynomial …","An MLE that represent a random permutation","Takes as input a struct, and converts them to a series of …","","","","","Verify that an MLE g(x) is a permutation of MLE f(x) over …","Verify that for witness multilinear polynomials f(x), g(x) …","Verify the claimed sum using the proof","verify the claimed sum using the proof","","","","","","","","","",""],"i":[0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,5,5,5,5,0,5,5,5,5,5,5,5,5,5,17,17,17,17,17,17,30,17,0,31,31,0,0,0,32,32,17,17,0,30,30,30,17,30,30,0,33,33,17,17,0,17,17,17,17,17,30,17,17,17,17,17,17,17,0,17,31,32,30,33,17,17,17,17,31,32,30,33,0,0,17,17,17,17,31,32,30,33,17,34,35,36,37,38,39,40,41,42],"f":[0,[1],[[],2],[[],2],[[]],[[]],[[[5,[[0,[3,4]]]]],[[5,[[0,[3,4]]]]]],[[]],[[],[[5,[[0,[6,4]]]]]],[7],[7],[7],[[[5,[[0,[8,4]]]],5],9],[[],4],[[[5,[[0,[10,4]]]],11],12],[[]],[[],7],[[]],[[]],[[]],[[]],[[]],[[[14,[13]]],[[14,[2,13]]]],[15,[[15,[2]]]],[16,[[16,[2]]]],[[[5,[[0,[8,4]]]],5],9],0,[[],[[18,[17]]]],[19,[[18,[17]]]],[19,[[18,[17]]]],[[],[[18,[17]]]],0,[[]],[[],18],[[],18],[[],20],[[],[[18,[17]]]],[21,[[18,[17]]]],[[],[[18,[17]]]],[4,[[18,[17]]]],[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],2],[[],2],[[22,7],[[23,[9]]]],[[]],[[]],[7],[7],[7],[[]],[[17,11],12],[[17,11],12],[24,17],[25,17],[26,17],[27,17],[[]],[7,[[16,[[28,[4]]]]]],[[],7],[[]],[[]],[[]],[[]],[[]],[[[14,[13]]],[[14,[2,13]]]],[15,[[15,[2]]]],[16,[[16,[2]]]],[19,[[18,[17]]]],[19,[[18,[17]]]],[[],[[18,[17]]]],[[],[[18,[17]]]],[7,[[16,[[28,[4]]]]]],0,[[],29],[[],18],[[],18],[[],20],[[],[[18,[17]]]],[21,[[18,[17]]]],[[],[[18,[17]]]],[[],[[18,[17]]]],[[]],0,0,0,0,0,0,0,0,0],"p":[[3,"Private"],[8,"Any"],[8,"Clone"],[8,"PrimeField"],[3,"PolyIOP"],[8,"Default"],[15,"usize"],[8,"PartialEq"],[15,"bool"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Global"],[3,"Box"],[3,"Arc"],[3,"Rc"],[4,"PolyIOPErrors"],[4,"Result"],[3,"IOPTranscript"],[3,"TypeId"],[3,"VPAuxInfo"],[15,"u64"],[3,"Vec"],[4,"ArithErrors"],[4,"SerializationError"],[4,"TranscriptErrors"],[4,"PCSError"],[3,"DenseMultilinearExtension"],[3,"String"],[8,"SumCheck"],[8,"PermutationCheck"],[8,"ProductCheck"],[8,"ZeroCheck"],[13,"InvalidProver"],[13,"InvalidVerifier"],[13,"InvalidProof"],[13,"InvalidParameters"],[13,"InvalidChallenge"],[13,"SerializationErrors"],[13,"TranscriptErrors"],[13,"ArithmeticErrors"],[13,"PCSError"]]},\
"transcript":{"doc":"Module for PolyIOP transcript. TODO(ZZ): move this module …","t":[3,13,13,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,14,11,11,11,11,11,11,11,11,11,11,12,12],"n":["IOPTranscript","InvalidTranscript","SerializationError","TranscriptErrors","append_field_element","append_message","append_serializable_element","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","deref","deref","deref_mut","deref_mut","drop","drop","fmt","fmt","from","from","from","get_and_append_challenge","get_and_append_challenge_vectors","init","init","into","into","new","to_bytes","to_owned","to_string","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","0","0"],"q":["transcript","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","transcript::TranscriptErrors",""],"d":["An IOP transcript consists of a Merlin transcript and a …","Invalid Transcript: {0}","An error during (de)serialization: {0}","A <code>enum</code> specifying the possible failure modes of the …","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Create a new IOP transcript.","Takes as input a struct, and converts them to a series of …","","","","","","","","","","","",""],"i":[0,3,3,0,2,2,2,3,2,3,2,2,2,3,2,3,2,3,2,3,3,3,3,2,2,2,3,2,3,2,2,0,2,3,3,2,3,2,3,2,3,2,13,14],"f":[0,0,0,0,[[[2,[1]]],[[4,[3]]]],[[[2,[1]]],[[4,[3]]]],[[[2,[1]]],[[4,[3]]]],[[]],[[]],[[]],[[]],[[[2,[[0,[5,1]]]]],[[2,[[0,[5,1]]]]]],[[]],[6],[6],[6],[6],[6],[6],[[3,7],8],[[3,7],8],[[]],[9,3],[[]],[[[2,[1]]],[[4,[1,3]]]],[[[2,[1]],6],[[4,[[10,[1]],3]]]],[[],6],[[],6],[[]],[[]],[[],[[2,[1]]]],0,[[]],[[],11],[[],4],[[],4],[[],4],[[],4],[[],12],[[],12],[[]],[[]],0,0],"p":[[8,"PrimeField"],[3,"IOPTranscript"],[4,"TranscriptErrors"],[4,"Result"],[8,"Clone"],[15,"usize"],[3,"Formatter"],[6,"Result"],[4,"SerializationError"],[3,"Vec"],[3,"String"],[3,"TypeId"],[13,"InvalidTranscript"],[13,"SerializationError"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
