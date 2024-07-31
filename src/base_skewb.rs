use std::ops::{BitAnd, Shl, Shr, Add, Sub, BitOr, BitXor};
use std::fmt::{Display,Debug};
use std::collections::HashSet;
use std::iter::FromIterator;
use const_for::*;

const fn array_contains(array: &[usize], to_check: i32) -> bool{
    const_for!(i in 0..array.len() => {
        if array[i] == to_check as usize{
            return true
        }
    });
    return false
    
}

const fn blank_pieces_corners(pieces_to_blacken: &[usize], csize_f: usize) -> u64{
    let mut val = 0;
    const_for!(i in (0..8).rev() => {
        val = val << csize_f;
        // if !pieces_to_blacken.contains(&i) {
        if !array_contains(pieces_to_blacken,i){
            val += (2<<(csize_f-1))-1;
        }
    });
    val
}

const fn blank_pieces_centers(pieces_to_blacken: &[usize], esize_f: usize) -> u64{
    let mut val = 0;
    const_for!(i in (0i32..6).rev() => {
        val = val << esize_f;
        // if !pieces_to_blacken.contains(&i) {
        if !array_contains(pieces_to_blacken,i){
            val += (2<<(esize_f-1))-1;
        }
    });
    val
}

pub trait BaseCube: Sized {
    const DBR: usize = 0;
    const DBL: usize = 1;
    const DFR: usize = 2;
    const DFL: usize = 3;
    const UFR: usize = 4;
    const UFL: usize = 5;
    const UBR: usize = 6;
    const UBL: usize = 7;

    const D: usize = 0;
    const F: usize = 1;
    const R: usize = 2;
    const L: usize = 3;
    const B: usize = 4;
    const U: usize = 5;

    const XSIZE: usize;
    const CSIZE: usize;
    const CORNER_ORINENTATION: bool;
    const CSIZE_F: usize = match Self::CORNER_ORINENTATION {
        true => Self::CSIZE + 2,
        false => Self::CSIZE,
    };
    const XSIZE_F: usize = Self::XSIZE;
    
    type Center: TryFrom<u64> + BitXor<Output = Self::Center> + BitOr<Output = Self::Center> + BitAnd<Output = Self::Center> + Shl<usize, Output = Self::Center> + Shr<usize, Output = Self::Center> + Add<Output = Self::Center> + Sub<Output = Self::Center> + Copy + Display;
    type Corner: TryFrom<u64> + BitXor<Output = Self::Corner> + BitOr<Output = Self::Corner> + BitAnd<Output = Self::Corner> + Shl<usize, Output = Self::Corner> + Shr<usize, Output = Self::Corner> + Add<Output = Self::Corner> + Sub<Output = Self::Corner> + Copy + Display;
    
    const RC_MASK: u64 = blank_pieces_corners(&[Self::UFR,Self::UBL,Self::DBR,Self::UBR],Self::CSIZE_F);
    const RX_MASK: u64 = blank_pieces_centers(&[Self::R,Self::U,Self::B],Self::XSIZE_F);

    const LC_MASK: u64 = blank_pieces_corners(&[Self::UFR,Self::DFL,Self::UBL,Self::UFL],Self::CSIZE_F);
    const LX_MASK: u64 = blank_pieces_centers(&[Self::U,Self::F,Self::L],Self::XSIZE_F);

    const FC_MASK: u64 = blank_pieces_corners(&[Self::DFR,Self::UFL,Self::UBR,Self::UFR],Self::CSIZE_F);
    const FX_MASK: u64 = blank_pieces_centers(&[Self::U,Self::R,Self::F],Self::XSIZE_F);

    const BC_MASK: u64 = blank_pieces_corners(&[Self::DBL,Self::UBR,Self::UFL,Self::UBL],Self::CSIZE_F);
    const BX_MASK: u64 = blank_pieces_centers(&[Self::U,Self::L,Self::B],Self::XSIZE_F);

    fn center(&mut self) -> &mut Self::Center;

    fn corner(&mut self) -> &mut Self::Corner;

    fn new() -> Self;

    fn twist_corner(corner: Self::Corner) -> Self::Corner{
        let u = corner & (Self::Corner::try_from(3).ok().unwrap()  << Self::CSIZE);
        let o = (u+(Self::Corner::try_from(1).ok().unwrap() << Self::CSIZE)+((u&(Self::Corner::try_from(1).ok().unwrap() << (Self::CSIZE+1)))>> 1))&(Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        o + (corner&((Self::Corner::try_from(2).ok().unwrap() <<(Self::CSIZE-1))-Self::Corner::try_from(1).ok().unwrap() ))
    }

    fn twist_corner_c(corner: Self::Corner) -> Self::Corner{
        let u = corner & (Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        let t = u + (Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        let o = (t - ((t & (Self::Corner::try_from(1).ok().unwrap() << (Self::CSIZE+1)))>>1)) & (Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        o + (corner&((Self::Corner::try_from(2).ok().unwrap()<<(Self::CSIZE-1))-Self::Corner::try_from(1).ok().unwrap()))
    }

    #[inline(always)]
    fn base_move_c(&mut self, p1: usize, p2: usize, p3: usize, p4: usize, mask: Self::Corner, clock_wise: bool) -> Self::Corner{

        let blank_c = (Self::Corner::try_from(2).ok().unwrap()<< (Self::CSIZE_F-1) )-Self::Corner::try_from(1).ok().unwrap();
        let mut corners = *self.corner();
        let mut block1 = (corners >> (p1 * Self::CSIZE_F)) & blank_c;
        let mut block2 = (corners >> (p2 * Self::CSIZE_F)) & blank_c;
        let mut block3 = (corners >> (p3 * Self::CSIZE_F)) & blank_c;
        let mut block4 = (corners >> (p4 * Self::CSIZE_F)) & blank_c;
        if clock_wise{
            block1 = Self::twist_corner_c(block1);
            block2 = Self::twist_corner_c(block2);
            block3 = Self::twist_corner_c(block3);
            block4 = Self::twist_corner(block4);
        }
        else{
            block1 = Self::twist_corner(block1);
            block2 = Self::twist_corner(block2);
            block3 = Self::twist_corner(block3);
            block4 = Self::twist_corner_c(block4);
        }

        corners = corners & mask;
        corners = corners ^ ((block1 << (p2 * Self::CSIZE_F)) | (block2 << (p3 * Self::CSIZE_F)) |(block3 << (p1 * Self::CSIZE_F)) |(block4 << (p4 * Self::CSIZE_F)));
        corners
    }
    
    #[inline(always)]
    fn base_move_x(&mut self, p1: usize, p2: usize, p3: usize, mask: Self::Center) -> Self::Center{

        let blank_x = (Self::Center::try_from(2).ok().unwrap()<< (Self::XSIZE_F-1) )-Self::Center::try_from(1).ok().unwrap();
        let mut centers = *self.center();
        let mut block1 = (centers >> (p1 * Self::XSIZE_F)) & blank_x;
        let mut block2 = (centers >> (p2 * Self::XSIZE_F)) & blank_x;
        let mut block3 = (centers >> (p3 * Self::XSIZE_F)) & blank_x;

        // println!("xsize: {}", Self::XSIZE_F);
        // println!("blank: {}", blank_x);
        // println!("{}", mask);
        centers = centers & mask; 
        centers = centers ^ ((block1 << (p2 * Self::XSIZE_F)) | (block2 << (p3 * Self::XSIZE_F)) |(block3 << (p1 * Self::XSIZE_F)));
        // println!("{}", centers);
        // println!("{}", block3);
        centers
    }

    fn r(mut self) -> Self {
        *self.center() = self.base_move_x(Self::R,Self::U,Self::B, Self::Center::try_from(Self::RX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::UFR,Self::UBL,Self::DBR,Self::UBR, Self::Corner::try_from(Self::RC_MASK).ok().unwrap(), true);
        self
    }

    fn rp(mut self) -> Self {
        *self.center() = self.base_move_x(Self::R,Self::B,Self::U, Self::Center::try_from(Self::RX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::UFR,Self::DBR,Self::UBL,Self::UBR, Self::Corner::try_from(Self::RC_MASK).ok().unwrap(), false);
        self
    }

    fn l(mut self) -> Self {
        *self.center() = self.base_move_x(Self::U,Self::F,Self::L, Self::Center::try_from(Self::LX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::UFR,Self::DFL,Self::UBL,Self::UFL, Self::Corner::try_from(Self::LC_MASK).ok().unwrap(), true);
        self
    }

    fn lp(mut self) -> Self {
        *self.center() = self.base_move_x(Self::U,Self::L,Self::F, Self::Center::try_from(Self::LX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::UFR,Self::UBL,Self::DFL,Self::UFL, Self::Corner::try_from(Self::LC_MASK).ok().unwrap(), false);
        self
    }

    fn f(mut self) -> Self {
        *self.center() = self.base_move_x(Self::U,Self::R,Self::F, Self::Center::try_from(Self::FX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::DFR,Self::UFL,Self::UBR,Self::UFR, Self::Corner::try_from(Self::FC_MASK).ok().unwrap(), true);
        self
    }

    fn fp(mut self) -> Self {
        *self.center() = self.base_move_x(Self::U,Self::F,Self::R, Self::Center::try_from(Self::FX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::DFR,Self::UBR,Self::UFL,Self::UFR, Self::Corner::try_from(Self::FC_MASK).ok().unwrap(), false);
        self
    }

    fn b(mut self) -> Self {
        *self.center() = self.base_move_x(Self::U,Self::L,Self::B, Self::Center::try_from(Self::BX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::DBL,Self::UBR,Self::UFL,Self::UBL, Self::Corner::try_from(Self::BC_MASK).ok().unwrap(), true);
        self
    }

    fn bp(mut self) -> Self {
        *self.center() = self.base_move_x(Self::U,Self::B,Self::L, Self::Center::try_from(Self::BX_MASK).ok().unwrap());
        *self.corner() = self.base_move_c(Self::DBL,Self::UFL,Self::UBR,Self::UBL, Self::Corner::try_from(Self::BC_MASK).ok().unwrap(), false);
        self
    }

    fn perform_move(mut self, movee: u8) -> Self{
        self = match movee{
            1 => self.r(),
            2 => self.l(),
            5 => self.f(),
            6 => self.b(),
            21 => self.rp(),
            22 => self.lp(),
            25 => self.fp(),
            26 => self.bp(),
            0 => self,
            _ => unreachable!()
        };
        self
    }

    fn do_scramble(mut self, scramble:String) -> (Self,Vec<u8>) where Self: Debug {
        let split_scramble: Vec<_> = scramble.split_ascii_whitespace().map(|f|
            match f {
                "R" => 1,
                "L" => 2,
                "F" => 5,
                "B" => 6,
                "R'" => 21,
                "L'" => 22,
                "F'" => 25,
                "B'" => 26,
                _ => unreachable!()
            }).collect();
    
        for movei in &split_scramble{
            self = self.perform_move(*movei);
            println!("{}, {:?}",movei, self)
        }
        (self, split_scramble)
    }
    
}

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub struct Skewb{
    corners: u64,
    centers: u32,
}

impl BaseCube for Skewb{
    const XSIZE: usize = 3;
    const CSIZE: usize = 3;
    const CORNER_ORINENTATION: bool = true;

    type Corner = u64;
    type Center = u32;

    fn center(&mut self) ->  &mut Self::Center {
        &mut self.centers
    }

    fn corner(&mut self) -> &mut Self::Corner {
        &mut self.corners
    }

    fn new() -> Self{
        Self {
            corners: 247132686368,
            centers: 181896,
        }
    }
}

impl Skewb{
    pub fn construct() -> Self { // The way the numbers in new is calculated
        let mut c = 0;
        for i in (0..8).rev(){
            c = c << Self::CSIZE_F;
            c += i;
        }
        let mut x = 0;
        for i in (0..6).rev(){
            x = x << Self::XSIZE_F;
            x += i;
        }

        Self {
            corners: c,
            centers: x
        }
    }
}