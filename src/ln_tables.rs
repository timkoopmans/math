use crate::decimal::FixedPoint;
use crate::decimal::Integer;
use checked_decimal_macro::*;
use std::ops::{Div, Mul, Add, Sub};
use ndarray::{arr2, Array2};

impl FixedPoint {
    pub fn ln_tables(self) -> Option<(Self, bool)> {
        let x: u128 = self.get();

        assert!(x > 0, "must be greater than zero");

        let scale: u128 = 10u128.checked_pow(FixedPoint::scale() as u32)?;

        let ln_2_decimal = FixedPoint::new(693_147_180_559u128);
        let (bit_length, negative) = self.bit_length()?;
        let bit_length_decimal = FixedPoint::from_decimal(bit_length);

        let max = FixedPoint::new(2u128.pow(bit_length.get() as u32).checked_mul(scale)?);
        let max = if negative {
            // x^-n = 1/x^n
            let one = FixedPoint::from_integer(1);
            one.div(max)
        } else {
            max
        };

        let (s_0, t_0, lx_0) = self.log_table_value(self, max, 0);
        let (s_1, t_1, lx_1) = self.log_table_value(s_0, t_0, 1);
        let (s_2, t_2, lx_2) = self.log_table_value(s_1, t_1, 2);
        let (s_3, t_3, lx_3) = self.log_table_value(s_2, t_2, 3);
        let (s_4, t_4, lx_4) = self.log_table_value(s_3, t_3, 4);
        let (s_5, t_5, lx_5) = self.log_table_value(s_4, t_4, 5);
        let (s_6, t_6, lx_6) = self.log_table_value(s_5, t_5, 6);
        let (s_7, t_7, lx_7) = self.log_table_value(s_6, t_6, 7);
        let (s_8, t_8, lx_8) = self.log_table_value(s_7, t_7, 8);
        let (_s_9, _t_9, lx_9) = self.log_table_value(s_8, t_8, 9);

        let lx_sum = lx_0
            .checked_add(lx_1)
            .unwrap()
            .checked_add(lx_2)
            .unwrap()
            .checked_add(lx_3)
            .unwrap()
            .checked_add(lx_4)
            .unwrap()
            .checked_add(lx_5)
            .unwrap()
            .checked_add(lx_6)
            .unwrap()
            .checked_add(lx_7)
            .unwrap()
            .checked_add(lx_8)
            .unwrap()
            .checked_add(lx_9)
            .unwrap();

        let lx_sum_decimal = FixedPoint::new(lx_sum);

        let result = if negative {
            ln_2_decimal
                .mul(bit_length_decimal)
                .sub(lx_sum_decimal)
        } else {
            ln_2_decimal
                .mul(bit_length_decimal)
                .add(lx_sum_decimal)
        };

        Some((result, negative))
    }

    pub fn bit_length(self) -> Option<(Integer, bool)> {
        if self.get() == 0 {
            return Some((Integer::new(0), false));
        }

        let (log10, neg_num) = self.log10()?;
        let (log10_2, neg_den) = FixedPoint::from_integer(2).log10()?;
        let negative = neg_num != neg_den;

        // int(log10(x)/log10(2))
        let value = if negative {
            Integer::from_decimal_up(log10.div(log10_2))
        } else {
            Integer::from_decimal(log10.div(log10_2))
        };

        Some((value, negative))
    }

    fn log_table_value(
        self,
        s_value: FixedPoint,
        t_value: FixedPoint,
        log_table_col: usize,
    ) -> (FixedPoint, FixedPoint, u128) {
        let s_value = s_value.div(t_value);
        let place_value = 10u128.checked_pow((log_table_col + 1) as u32).unwrap();
        let f_value = FixedPoint::new(place_value);
        let t_value = s_value.mul(f_value).div(f_value);

        // let log_table_row: usize = t_value.mul(f_value).sub(f_value).unwrap().into();
        // let log_table_row = log_table_row.checked_sub(1);
        let log_table_row: usize = t_value.mul(f_value).sub(f_value).get() as usize;
        let log_table_row = log_table_row.checked_sub(1);

        let mut lx_value = 0u128;

        // Ensure within array of shape [9, 12]
        let log_table_row_range = 0..9;
        let log_table_col_range = 0..12;

        match log_table_row {
            Some(log_table_row) => {
                if log_table_row_range.contains(&log_table_row)
                    && log_table_col_range.contains(&log_table_col)
                {
                    lx_value = self.log_table(log_table_row, log_table_col);
                }
            }
            None => lx_value = 0,
        }

        (s_value, t_value, lx_value)
    }

    // These can be calculated from an index expressed as:
    // 1.1	1.01	1.001	1.0001	1.00001	1.000001	1.0000001	1.00000001	1.000000001
    // 1.2	1.02	1.002	1.0002	1.00002	1.000002	1.0000002	1.00000002	1.000000002
    // 1.3	1.03	1.003	1.0003	1.00003	1.000003	1.0000003	1.00000003	1.000000003
    // 1.4	1.04	1.004	1.0004	1.00004	1.000004	1.0000004	1.00000004	1.000000004
    // 1.5	1.05	1.005	1.0005	1.00005	1.000005	1.0000005	1.00000005	1.000000005
    // 1.6	1.06	1.006	1.0006	1.00006	1.000006	1.0000006	1.00000006	1.000000006
    // 1.7	1.07	1.007	1.0007	1.00007	1.000007	1.0000007	1.00000007	1.000000007
    // 1.8	1.08	1.008	1.0008	1.00008	1.000008	1.0000008	1.00000008	1.000000008
    // 1.9	1.09	1.009	1.0009	1.00009	1.000009	1.0000009	1.00000009	1.000000009
    // with each column, row determined by the function:
    // INT(LN(index)*scale)
    // where scale is a predetermined precision e.g. 10^12
    fn log_table(self, row: usize, col: usize) -> u128 {
        let table: Array2<u128> = arr2(&[
            [
                95310179804,
                9950330853,
                999500333,
                99995000,
                9999950,
                999999,
                99999,
                9999,
                1000,
                100,
                10,
                1,
            ],
            [
                182321556793,
                19802627296,
                1998002662,
                199980002,
                19999800,
                1999998,
                199999,
                19999,
                1999,
                200,
                20,
                1,
            ],
            [
                262364264467,
                29558802241,
                2995508979,
                299955008,
                29999550,
                2999995,
                299999,
                29999,
                3000,
                300,
                30,
                3,
            ],
            [
                336472236621,
                39220713153,
                3992021269,
                399920021,
                39999200,
                3999991,
                399999,
                39999,
                4000,
                400,
                40,
                3,
            ],
            [
                405465108108,
                48790164169,
                4987541511,
                499875041,
                49998750,
                4999987,
                499999,
                49999,
                4999,
                500,
                50,
                5,
            ],
            [
                470003629245,
                58268908123,
                5982071677,
                599820071,
                59998200,
                5999982,
                599999,
                59999,
                6000,
                600,
                60,
                6,
            ],
            [
                530628251062,
                67658648473,
                6975613736,
                699755114,
                69997550,
                6999975,
                699999,
                69999,
                6999,
                700,
                70,
                6,
            ],
            [
                587786664902,
                76961041136,
                7968169649,
                799680170,
                79996800,
                7999968,
                799999,
                79999,
                7999,
                800,
                80,
                8,
            ],
            [
                641853886172,
                86177696241,
                8959741371,
                899595242,
                89995950,
                8999959,
                899999,
                89999,
                9000,
                900,
                90,
                8,
            ],
        ]);

        table[[row, col]]
    }

}

#[cfg(test)]
mod tests {
    use crate::decimal::{FixedPoint, Integer};
    use checked_decimal_macro::*;

    #[test]
    fn test_bit_length() {
        // 0 bit length == 0
        let d = FixedPoint::new(0);
        let (bit_length, negative) = d.bit_length().unwrap();
        assert_eq!(bit_length, Integer::new(0));
        assert_eq!(negative, false);

        // 10 bit length == 3
        let d = FixedPoint::from_integer(10);
        let (bit_length, negative) = d.bit_length().unwrap();
        assert_eq!(bit_length, Integer::new(3));
        assert_eq!(negative, false);

        // 0.900000000000 bit length == -1
        let d = FixedPoint::new(900000000000);
        let (bit_length, negative) = d.bit_length().unwrap();
        assert_eq!(bit_length, Integer::new(1));
        assert_eq!(negative, true);

        // 0.01 bit length == -7
        let d = FixedPoint::from_scale(1, 2);
        let (bit_length, negative) = d.bit_length().unwrap();
        assert_eq!(bit_length, Integer::new(7));
        assert_eq!(negative, true);

        // 0.000001 bit length == -20
        let d = FixedPoint::from_scale(1, 6);
        let (bit_length, negative) = d.bit_length().unwrap();
        assert_eq!(bit_length, Integer::new(20));
        assert_eq!(negative, true);

        // 18446744073709551615 bit length == 64
        let d = FixedPoint::new(18446744073709551615);
        let (bit_length, negative) = d.bit_length().unwrap();
        assert_eq!(bit_length, Integer::new(24));
        assert_eq!(negative, false);
    }

    #[test]
    fn test_ln_tables() {
        //  with integer and fractional digits
        // ln(2.25) = 0.8109302162163287639560262309286982731439808469249883952280
        {
            let decimal = FixedPoint::new(2250000000000u128);
            let actual = decimal.ln_tables();
            let expected = Some((FixedPoint::new(810930216138u128), false));
            assert_eq!(actual, expected);
        }

        //  with fractional digits only
        // ln(0.810930216138) = -0.209573275254525923995526530250450021440003921493434432564204599
        {
            let decimal = FixedPoint::new(810930216138u128);
            let actual = decimal.ln_tables();
            let expected = Some((FixedPoint::new(209573275322u128), true));
            assert_eq!(actual, expected);
        }

        // with very small fractional digits only, note this becomes lossy due to tables
        // ln(0.000000100000) = -16.11809565095831978812594018279054945320771042040141083223329530
        {
            let decimal = FixedPoint::new(100000u128);
            let actual = decimal.ln_tables();
            let expected = Some((FixedPoint::new(16_118084833430u128), true));
            assert_eq!(actual, expected);
        }
    }
}
