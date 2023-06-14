#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[cfg(feature = "approx")]
    #[test]
    fn reganexec() {
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        if unsafe {
            tre_regcomp(
                preg.as_mut_ptr(),
                b"Hello!\0".as_ptr() as *const _,
                (REG_EXTENDED | REG_ICASE) as i32,
            )
        } != 0
        {
            panic!("tre_regcomp");
        }
        let preg = unsafe { preg.assume_init() };

        let params = regaparams_t {
            cost_ins: 1,
            cost_del: 1,
            cost_subst: 1,
            max_cost: 2,
            max_del: 2,
            max_ins: 2,
            max_subst: 2,
            max_err: 2,
            ..Default::default()
        };

        let mut amatch: regamatch_t = Default::default();

        if unsafe { tre_regaexec(&preg, b"Hullo!\0".as_ptr() as *const _, &mut amatch, params, 0) } != 0
        {
            panic!("tre_regaexec");
        }

        assert_eq!(amatch.cost, 1);
    }

    #[test]
    fn regexec() {
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        if unsafe {
            tre_regcomp(
                preg.as_mut_ptr(),
                b"Hello(, [[:alpha:]]+)?!\0".as_ptr() as *const _,
                (REG_EXTENDED | REG_ICASE) as i32,
            )
        } != 0
        {
            panic!("tre_regcomp");
        }

        let preg = unsafe { preg.assume_init() };

        let nmatch = 1;
        let mut pmatch: Vec<regmatch_t> = vec![regmatch_t { rm_so: 0, rm_eo: 0 }; 1];
        if unsafe {
            tre_regexec(
                &preg,
                b"Hello!".as_ptr() as *const _,
                nmatch,
                pmatch.as_mut_ptr(),
                0,
            )
        } != 0
        {
            panic!("tre_regexec");
        }

        assert!(pmatch[0].rm_so == 0, "Bad starting offset");
        assert!(pmatch[0].rm_eo == 6, "Bad ending offset");

        pmatch[0].rm_eo = 0;

        let nmatch = 2;
        pmatch.push(regmatch_t { rm_so: 0, rm_eo: 0 });
        if unsafe {
            tre_regexec(
                &preg,
                b"Hello, world!\0".as_ptr() as *const _,
                nmatch,
                pmatch.as_mut_ptr(),
                0,
            )
        } != 0
        {
            panic!("tre_regexec");
        }

        assert!(pmatch[0].rm_so == 0, "Bad starting offset");
        assert!(pmatch[0].rm_eo == 13, "Bad ending offset");
        assert!(pmatch[1].rm_so == 5, "Bad starting offset for match group");
        assert!(pmatch[1].rm_eo == 12, "Bad ending offset for match group");
    }
}
