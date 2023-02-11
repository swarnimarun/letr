#[macro_export]
macro_rules! letr {
    // --- start of avoid recursion within reasonable limits
    (
        $pattern1:pat_param = $val1:expr, $(else)? $ret1:expr;
        $pattern2:pat_param = $val2:expr, $(else)? $ret2:expr;
        $pattern3:pat_param = $val3:expr, $(else)? $ret3:expr;
        $pattern4:pat_param = $val4:expr, $(else)? $ret4:expr;
        $pattern5:pat_param = $val5:expr, $(else)? $ret5:expr;
        $pattern6:pat_param = $val6:expr, $(else)? $ret6:expr;
        $pattern7:pat_param = $val7:expr, $(else)? $ret7:expr;
        $($tts:tt)*
    ) => {
        let $pattern1 = $val1 else {
            return $ret1;
        };
        let $pattern2 = $val2 else {
            return $ret2;
        };
        let $pattern3 = $val3 else {
            return $ret3;
        };
        let $pattern4 = $val4 else {
            return $ret4;
        };
        let $pattern5 = $val5 else {
            return $ret5;
        };
        let $pattern6 = $val6 else {
            return $ret6;
        };
        let $pattern7 = $val7 else {
            return $ret7;
        };
        letr!($($tts)*);
    };
    (
        $pattern1:pat_param = $val1:expr, $(else)? $ret1:expr;
        $pattern2:pat_param = $val2:expr, $(else)? $ret2:expr;
        $pattern3:pat_param = $val3:expr, $(else)? $ret3:expr;
        $pattern4:pat_param = $val4:expr, $(else)? $ret4:expr;
        $pattern5:pat_param = $val5:expr, $(else)? $ret5:expr;
        $pattern6:pat_param = $val6:expr, $(else)? $ret6:expr;
        $($tts:tt)*
    ) => {
        let $pattern1 = $val1 else {
            return $ret1;
        };
        let $pattern2 = $val2 else {
            return $ret2;
        };
        let $pattern3 = $val3 else {
            return $ret3;
        };
        let $pattern4 = $val4 else {
            return $ret4;
        };
        let $pattern5 = $val5 else {
            return $ret5;
        };
        let $pattern6 = $val6 else {
            return $ret6;
        };
        letr!($($tts)*);
    };
    (
        $pattern1:pat_param = $val1:expr, $(else)? $ret1:expr;
        $pattern2:pat_param = $val2:expr, $(else)? $ret2:expr;
        $pattern3:pat_param = $val3:expr, $(else)? $ret3:expr;
        $pattern4:pat_param = $val4:expr, $(else)? $ret4:expr;
        $pattern5:pat_param = $val5:expr, $(else)? $ret5:expr;
        $($tts:tt)*
    ) => {
        let $pattern1 = $val1 else {
            return $ret1;
        };
        let $pattern2 = $val2 else {
            return $ret2;
        };
        let $pattern3 = $val3 else {
            return $ret3;
        };
        let $pattern4 = $val4 else {
            return $ret4;
        };
        let $pattern5 = $val5 else {
            return $ret5;
        };
        letr!($($tts)*);
    };
    (
        $pattern1:pat_param = $val1:expr, $(else)? $ret1:expr;
        $pattern2:pat_param = $val2:expr, $(else)? $ret2:expr;
        $pattern3:pat_param = $val3:expr, $(else)? $ret3:expr;
        $pattern4:pat_param = $val4:expr, $(else)? $ret4:expr;
        $($tts:tt)*
    ) => {
        let $pattern1 = $val1 else {
            return $ret1;
        };
        let $pattern2 = $val2 else {
            return $ret2;
        };
        let $pattern3 = $val3 else {
            return $ret3;
        };
        let $pattern4 = $val4 else {
            return $ret4;
        };
        letr!($($tts)*);
    };
    (
        $pattern1:pat_param = $val1:expr, $(else)? $ret1:expr;
        $pattern2:pat_param = $val2:expr, $(else)? $ret2:expr;
        $pattern3:pat_param = $val3:expr, $(else)? $ret3:expr;
        $($tts:tt)*
    ) => {
        let $pattern1 = $val1 else {
            return $ret1;
        };
        let $pattern2 = $val2 else {
            return $ret2;
        };
        let $pattern3 = $val3 else {
            return $ret3;
        };
        letr!($($tts)*);
    };
    (
        $pattern1:pat_param = $val1:expr, $(else)? $ret1:expr;
        $pattern2:pat_param = $val2:expr, $(else)? $ret2:expr;
        $($tts:tt)*
    ) => {
        let $pattern1 = $val1 else {
            return $ret1;
        };
        let $pattern2 = $val2 else {
            return $ret2;
        };
        letr!($($tts)*);
    };
    // --- end of avoid recursion
    // repeat case
    (
        $pattern:pat_param = $val:expr, $(else)? $ret:expr;
        $($tts:tt)*
    ) => {
        let $pattern = $val else {
            return $ret;
        };
        letr!($($tts)*);
    };
    // base case
    () => {};
}

#[cfg(test)]
mod test {
    #[test]
    fn simple() {
        enum Items {
            RevItem(Option<i32>),
            ForItem(u32),
        }

        fn foo(x: Items) -> i32 {
            letr! {
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Items::RevItem(ret) = x, else -1;
                Some(ret) = ret, else -1;
            };
            ret
        }
        assert_eq!(foo(Items::RevItem(Some(0))), 0);
        assert_eq!(foo(Items::ForItem(0)), -1);
    }
}
