# fizix Roadmap

## v0.1.x: Foundation ✅
- Unit<V, S, M, L, T, I, K, N, J> struct with typenum type-level dims
- Add/Sub/Mul/Div/Neg operators
- SI base & derived unit aliases (60+ prefix variants)
- Compile-time dimension mismatch rejection
- no_std, cross-crate operation

## v0.2.x: Scale & Usability ✅
- Type-level scale parameter S (SI prefixes as typenum integers)
- ScaleCast trait for runtime convert()/to_base()
- Display with ×10^S notation (S≠0) + {:#} alternate ASCII mode
- Prefix aliases: mm, km, mg, GHz, GPa, MV, kN, µL, etc.
- Custom value types (Vec3 via DotProduct/CrossProduct traits)

## v0.3.x: Polish
- [ ] Benchmark vs uom (compile time, binary size)
- [ ] serde feature
- [ ] Pow trait for integer exponentiation
- [ ] MulAssign, DivAssign
- [ ] More niche prefix aliases (via features?)

## v1.0: Maturity
- [ ] generic_const_exprs stabilization → optional const-generic backend
- [ ] Comprehensive documentation with physics examples

## Earlier attempts (nightly)

Archived. See ROADMAP.md history for details.
