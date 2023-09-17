use super::Index;

impl Index {
    /// BitBoard index constant for a field.
    pub const A1: Index = Index::new(0);
    /// BitBoard index constant for a field.
    pub const B1: Index = Index::new(1);
    /// BitBoard index constant for a field.
    pub const C1: Index = Index::new(2);
    /// BitBoard index constant for a field.
    pub const D1: Index = Index::new(3);
    /// BitBoard index constant for a field.
    pub const E1: Index = Index::new(4);
    /// BitBoard index constant for a field.
    pub const F1: Index = Index::new(5);
    /// BitBoard index constant for a field.
    pub const G1: Index = Index::new(6);
    /// BitBoard index constant for a field.
    pub const H1: Index = Index::new(7);

    /// BitBoard index constant for a field.
    pub const A2: Index = Index::new(8);
    /// BitBoard index constant for a field.
    pub const B2: Index = Index::new(9);
    /// BitBoard index constant for a field.
    pub const C2: Index = Index::new(10);
    /// BitBoard index constant for a field.
    pub const D2: Index = Index::new(11);
    /// BitBoard index constant for a field.
    pub const E2: Index = Index::new(12);
    /// BitBoard index constant for a field.
    pub const F2: Index = Index::new(13);
    /// BitBoard index constant for a field.
    pub const G2: Index = Index::new(14);
    /// BitBoard index constant for a field.
    pub const H2: Index = Index::new(15);

    /// BitBoard index constant for a field.
    pub const A3: Index = Index::new(16);
    /// BitBoard index constant for a field.
    pub const B3: Index = Index::new(17);
    /// BitBoard index constant for a field.
    pub const C3: Index = Index::new(18);
    /// BitBoard index constant for a field.
    pub const D3: Index = Index::new(19);
    /// BitBoard index constant for a field.
    pub const E3: Index = Index::new(20);
    /// BitBoard index constant for a field.
    pub const F3: Index = Index::new(21);
    /// BitBoard index constant for a field.
    pub const G3: Index = Index::new(22);
    /// BitBoard index constant for a field.
    pub const H3: Index = Index::new(23);

    /// BitBoard index constant for a field.
    pub const A4: Index = Index::new(24);
    /// BitBoard index constant for a field.
    pub const B4: Index = Index::new(25);
    /// BitBoard index constant for a field.
    pub const C4: Index = Index::new(26);
    /// BitBoard index constant for a field.
    pub const D4: Index = Index::new(27);
    /// BitBoard index constant for a field.
    pub const E4: Index = Index::new(28);
    /// BitBoard index constant for a field.
    pub const F4: Index = Index::new(29);
    /// BitBoard index constant for a field.
    pub const G4: Index = Index::new(30);
    /// BitBoard index constant for a field.
    pub const H4: Index = Index::new(31);

    /// BitBoard index constant for a field.
    pub const A5: Index = Index::new(32);
    /// BitBoard index constant for a field.
    pub const B5: Index = Index::new(33);
    /// BitBoard index constant for a field.
    pub const C5: Index = Index::new(34);
    /// BitBoard index constant for a field.
    pub const D5: Index = Index::new(35);
    /// BitBoard index constant for a field.
    pub const E5: Index = Index::new(36);
    /// BitBoard index constant for a field.
    pub const F5: Index = Index::new(37);
    /// BitBoard index constant for a field.
    pub const G5: Index = Index::new(38);
    /// BitBoard index constant for a field.
    pub const H5: Index = Index::new(39);

    /// BitBoard index constant for a field.
    pub const A6: Index = Index::new(40);
    /// BitBoard index constant for a field.
    pub const B6: Index = Index::new(41);
    /// BitBoard index constant for a field.
    pub const C6: Index = Index::new(42);
    /// BitBoard index constant for a field.
    pub const D6: Index = Index::new(43);
    /// BitBoard index constant for a field.
    pub const E6: Index = Index::new(44);
    /// BitBoard index constant for a field.
    pub const F6: Index = Index::new(45);
    /// BitBoard index constant for a field.
    pub const G6: Index = Index::new(46);
    /// BitBoard index constant for a field.
    pub const H6: Index = Index::new(47);

    /// BitBoard index constant for a field.
    pub const A7: Index = Index::new(48);
    /// BitBoard index constant for a field.
    pub const B7: Index = Index::new(49);
    /// BitBoard index constant for a field.
    pub const C7: Index = Index::new(50);
    /// BitBoard index constant for a field.
    pub const D7: Index = Index::new(51);
    /// BitBoard index constant for a field.
    pub const E7: Index = Index::new(52);
    /// BitBoard index constant for a field.
    pub const F7: Index = Index::new(53);
    /// BitBoard index constant for a field.
    pub const G7: Index = Index::new(54);
    /// BitBoard index constant for a field.
    pub const H7: Index = Index::new(55);

    /// BitBoard index constant for a field.
    pub const A8: Index = Index::new(56);
    /// BitBoard index constant for a field.
    pub const B8: Index = Index::new(57);
    /// BitBoard index constant for a field.
    pub const C8: Index = Index::new(58);
    /// BitBoard index constant for a field.
    pub const D8: Index = Index::new(59);
    /// BitBoard index constant for a field.
    pub const E8: Index = Index::new(60);
    /// BitBoard index constant for a field.
    pub const F8: Index = Index::new(61);
    /// BitBoard index constant for a field.
    pub const G8: Index = Index::new(62);
    /// BitBoard index constant for a field.
    pub const H8: Index = Index::new(63);

    /// List of all BitBoard index fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// let mut b = BitBoard::EMPTY;
    ///
    /// for i in Index::ALL_FIELDS {
    ///     assert!(b.has_bit(*i) == false);
    ///     b |= *i;
    /// }
    ///
    /// assert_eq!(b, BitBoard::UNIVERSE);
    /// ```
    #[rustfmt::skip]
    pub const ALL_FIELDS: &[Index] = &[
        Index::A1, Index::B1, Index::C1, Index::D1, Index::E1, Index::F1, Index::G1, Index::H1,
        Index::A2, Index::B2, Index::C2, Index::D2, Index::E2, Index::F2, Index::G2, Index::H2,
        Index::A3, Index::B3, Index::C3, Index::D3, Index::E3, Index::F3, Index::G3, Index::H3,
        Index::A4, Index::B4, Index::C4, Index::D4, Index::E4, Index::F4, Index::G4, Index::H4,
        Index::A5, Index::B5, Index::C5, Index::D5, Index::E5, Index::F5, Index::G5, Index::H5,
        Index::A6, Index::B6, Index::C6, Index::D6, Index::E6, Index::F6, Index::G6, Index::H6,
        Index::A7, Index::B7, Index::C7, Index::D7, Index::E7, Index::F7, Index::G7, Index::H7,
        Index::A8, Index::B8, Index::C8, Index::D8, Index::E8, Index::F8, Index::G8, Index::H8,
    ];
}
