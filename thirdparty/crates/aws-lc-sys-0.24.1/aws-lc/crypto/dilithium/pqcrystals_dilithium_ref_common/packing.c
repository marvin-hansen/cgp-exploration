#include "params.h"
#include "packing.h"
#include "polyvec.h"
#include "poly.h"

/*************************************************
* Name:        pack_pk
*
* Description: FIPS 204: Algorithm 22 pkEncode.
*              Bit-pack public key pk = (rho, t1).
*
* Arguments:   - ml_dsa_params: parameter struct
*              - uint8_t pk[]: pointer to output byte array
*              - const uint8_t rho[]: byte array containing rho
*              - const polyveck *t1: pointer to vector t1
**************************************************/
void pack_pk(ml_dsa_params *params,
             uint8_t *pk,
             const uint8_t rho[SEEDBYTES],
             const polyveck *t1)
{
  unsigned int i;

  for(i = 0; i < SEEDBYTES; ++i) {
    pk[i] = rho[i];
  }
  pk += SEEDBYTES;

  for(i = 0; i < params->k; ++i) {
    polyt1_pack(pk + i*POLYT1_PACKEDBYTES, &t1->vec[i]);
  }
}

/*************************************************
* Name:        unpack_pk
*
* Description: FIPS 204: Algorithm 23 pkDecode.
*              Unpack public key pk = (rho, t1).
*
* Arguments:   - ml_dsa_params: parameter struct
*              - const uint8_t rho[]: output byte array for rho
*              - const polyveck *t1: pointer to output vector t1
*              - uint8_t pk[]: pointer to byte array containing bit-packed pk
**************************************************/
void unpack_pk(ml_dsa_params *params,
               uint8_t rho[SEEDBYTES],
               polyveck *t1,
               const uint8_t *pk)
{
  unsigned int i;

  for(i = 0; i < SEEDBYTES; ++i) {
    rho[i] = pk[i];
  }
  pk += SEEDBYTES;

  for(i = 0; i < params->k; ++i) {
    polyt1_unpack(&t1->vec[i], pk + i*POLYT1_PACKEDBYTES);
  }
}

/*************************************************
* Name:        pack_sk
*
* Description: FIPS 204: Algorithm 24 skEncode.
*              Bit-pack secret key sk = (rho, tr, key, t0, s1, s2).
*
* Arguments:   - ml_dsa_params: parameter struct
*              - uint8_t sk[]: pointer to output byte array
*              - const uint8_t rho[]: byte array containing rho
*              - const uint8_t tr[]: byte array containing tr
*              - const uint8_t key[]: byte array containing key
*              - const polyveck *t0: pointer to vector t0
*              - const polyvecl *s1: pointer to vector s1
*              - const polyveck *s2: pointer to vector s2
**************************************************/
void pack_sk(ml_dsa_params *params,
             uint8_t *sk,
             const uint8_t rho[SEEDBYTES],
             const uint8_t tr[TRBYTES],
             const uint8_t key[SEEDBYTES],
             const polyveck *t0,
             const polyvecl *s1,
             const polyveck *s2)
{
  unsigned int i;

  for(i = 0; i < SEEDBYTES; ++i) {
    sk[i] = rho[i];
  }
  sk += SEEDBYTES;

  for(i = 0; i < SEEDBYTES; ++i) {
    sk[i] = key[i];
  }
  sk += SEEDBYTES;

  for(i = 0; i < TRBYTES; ++i) {
    sk[i] = tr[i];
  }
  sk += TRBYTES;

  for(i = 0; i < params->l; ++i) {
    polyeta_pack(params, sk + i * params->poly_eta_packed_bytes, &s1->vec[i]);
  }
  sk +=  params->l * params->poly_eta_packed_bytes;

  for(i = 0; i <  params->k; ++i) {
    polyeta_pack(params,sk + i * params->poly_eta_packed_bytes, &s2->vec[i]);
  }
  sk +=  params->k * params->poly_eta_packed_bytes;

  for(i = 0; i < params->k; ++i) {
    polyt0_pack(sk + i * POLYT0_PACKEDBYTES, &t0->vec[i]);
  }
}

/*************************************************
* Name:        unpack_sk
*
* Description: FIPS 204: Algorithm 25 skDecode.
*              Unpack secret key sk = (rho, tr, key, t0, s1, s2).
*
* Arguments:   - ml_dsa_params: parameter struct
*              - const uint8_t rho[]: output byte array for rho
*              - const uint8_t tr[]: output byte array for tr
*              - const uint8_t key[]: output byte array for key
*              - const polyveck *t0: pointer to output vector t0
*              - const polyvecl *s1: pointer to output vector s1
*              - const polyveck *s2: pointer to output vector s2
*              - uint8_t sk[]: pointer to byte array containing bit-packed sk
**************************************************/
void unpack_sk(ml_dsa_params *params,
               uint8_t rho[SEEDBYTES],
               uint8_t tr[TRBYTES],
               uint8_t key[SEEDBYTES],
               polyveck *t0,
               polyvecl *s1,
               polyveck *s2,
               const uint8_t *sk)
{
  unsigned int i;

  for(i = 0; i < SEEDBYTES; ++i) {
    rho[i] = sk[i];
  }
  sk += SEEDBYTES;

  for(i = 0; i < SEEDBYTES; ++i) {
    key[i] = sk[i];
  }
  sk += SEEDBYTES;

  for(i = 0; i < TRBYTES; ++i) {
    tr[i] = sk[i];
  }
  sk += TRBYTES;

  for(i=0; i < params->l; ++i) {
    polyeta_unpack(params, &s1->vec[i], sk + i * params->poly_eta_packed_bytes);
  }
  sk += params->l * params->poly_eta_packed_bytes;

  for(i=0; i < params->k; ++i) {
    polyeta_unpack(params, &s2->vec[i], sk + i * params->poly_eta_packed_bytes);
  }
  sk += params->k * params->poly_eta_packed_bytes;

  for(i=0; i < params->k; ++i) {
    polyt0_unpack(&t0->vec[i], sk + i * POLYT0_PACKEDBYTES);
  }
}

/*************************************************
* Name:        pack_sig
*
* Description: FIPS 204: Algorithm 26 sigEncode.
*              Bit-pack signature sig = (c, z, h).
*
* Arguments:   - ml_dsa_params: parameter struct
*              - uint8_t sig[]: pointer to output byte array
*              - const uint8_t *c: pointer to challenge hash length SEEDBYTES
*              - const polyvecl *z: pointer to vector z
*              - const polyveck *h: pointer to hint vector h
**************************************************/
void pack_sig(ml_dsa_params *params,
              uint8_t *sig,
              const uint8_t *c,
              const polyvecl *z,
              const polyveck *h)
{
  unsigned int i, j, k;

  for(i=0; i < params->c_tilde_bytes; ++i) {
    sig[i] = c[i];
  }
  sig += params->c_tilde_bytes;

  for(i = 0; i < params->l; ++i) {
    polyz_pack(params, sig + i * params->poly_z_packed_bytes, &z->vec[i]);
  }
  sig += params->l * params->poly_z_packed_bytes;

  /* Encode h */
  for(i = 0; i < params->omega + params->k; ++i) {
    sig[i] = 0;
  }

  k = 0;
  for(i = 0; i < params->k; ++i) {
    for(j = 0; j < N; ++j) {
      if(h->vec[i].coeffs[j] != 0) {
        sig[k++] = j;
      }
    }

    sig[params->omega + i] = k;
  }
}

/*************************************************
* Name:        unpack_sig
*
* Description: FIPS 204: Algorithm 27 sigDecode.
*              Unpack signature sig = (c, z, h).
*
* Arguments:   - ml_dsa_params: parameter struct
*              - uint8_t *c: pointer to output challenge hash
*              - polyvecl *z: pointer to output vector z
*              - polyveck *h: pointer to output hint vector h
*              - const uint8_t sig[]: pointer to byte array containing
*                bit-packed signature
*
* Returns 1 in case of malformed signature; otherwise 0.
**************************************************/
int unpack_sig(ml_dsa_params *params,
               uint8_t *c,
               polyvecl *z,
               polyveck *h,
               const uint8_t *sig)
{
  unsigned int i, j, k;

  for(i = 0; i < params->c_tilde_bytes; ++i) {
    c[i] = sig[i];
  }
  sig += params->c_tilde_bytes;

  for(i = 0; i < params->l; ++i) {
    polyz_unpack(params, &z->vec[i], sig + i * params->poly_z_packed_bytes);
  }
  sig += params->l * params->poly_z_packed_bytes;

  /* Decode h */
  k = 0;
  for(i = 0; i < params->k; ++i) {
    for(j = 0; j < N; ++j) {
      h->vec[i].coeffs[j] = 0;
    }

    if(sig[params->omega + i] < k || sig[params->omega + i] > params->omega) {
      return 1;
    }

    for(j = k; j < sig[params->omega + i]; ++j) {
      /* Coefficients are ordered for strong unforgeability */
      if(j > k && sig[j] <= sig[j-1]) {
        return 1;
      }
      h->vec[i].coeffs[sig[j]] = 1;
    }

    k = sig[params->omega + i];
  }

  /* Extra indices are zero for strong unforgeability */
  for(j = k; j < params->omega; ++j) {
    if(sig[j]) {
      return 1;
    }
  }
  return 0;
}