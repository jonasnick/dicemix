/* automatically generated by rust-bindgen */

extern "C" {
    /**
 * Solve function from protocol specification.
 *
 * Solves the equation system
 *   forall 0 <= i < n. sum_{j=0}^{n-1} out_messages[j]^{i+1} = sums[i]
 * in the finite prime field F_prime for out_messages[], and checks if my_message is in the solution.
 *
 * \param[out] out_messages    Array of n char buffers (allocated by caller) of length at least strlen(prime) + 1
 * \param[in]  prime           Prime of the finite field (not checked for primality)
 * \param[in]  my_message      Our own message
 * \param[in]  sums            Array of n power sums
 * \param[in]  n               Number of peers, must be at least 2 and not larger than prime
 *
 * \retval 0                   Success, the solution vector has been stored as hexadecimal strings in out_messages[],
 *                             sorted in ascending numerical order.
 * \retval RET_INVALID         sums is not a proper array of power sums or my_message is not in the solution.
 * \retval RET_INPUT_ERROR     Illegal input values.
 * \retval RET_INTERNAL_ERROR  An internal error occured.
 */
    pub fn solve(out_messages: *const *mut ::std::os::raw::c_char,
                 prime: *const ::std::os::raw::c_char,
                 my_message: *const ::std::os::raw::c_char,
                 sums: *const *const ::std::os::raw::c_char, n: usize)
     -> ::std::os::raw::c_int;
}
