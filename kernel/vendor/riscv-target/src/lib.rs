extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Target {
    pub bits: u32,
    pub base_extension: char,
    pub extensions: HashSet<char>,
    pub suffix: String,
    pub vendor_os: String,
}

const EXTENSION_ORDER: &str = "eimafdqlcbjtpvn";

lazy_static! {
    static ref TARGET_REGEX: Regex =
        regex::Regex::new("riscv(\\d+)([a-z]+)(:?[^-]*)-(.*)").unwrap();
    static ref REGISTERED_EXTENSIONS: HashSet<char> = {
        let mut exts = HashSet::new();
        for e in EXTENSION_ORDER.chars() {
            exts.insert(e);
        }
        exts
    };
}

impl Target {
    pub fn from_target_str(target_str: &str) -> Self {
        let target_captures = TARGET_REGEX
            .captures(target_str)
            .expect("RISC-V target doesn't match the pattern 'riscv(\\d+)([a-z]+)-(.*)'");

        let bits = u32::from_str(&target_captures[1]).unwrap();
        let mut target_flags: HashSet<char> = target_captures[2].to_lowercase().chars().collect();
        let suffix = target_captures[3].to_owned();
        let vendor_os = target_captures[4].to_owned();

        let mut base_extension = 'e';

        if target_flags.contains(&'g') {
            target_flags.remove(&'g');
            target_flags.extend("imafd".chars());
        }
        if target_flags.contains(&'e') {
            target_flags.remove(&'e');
            base_extension = 'e';
        }

        if target_flags.contains(&'i') {
            target_flags.remove(&'i');
            base_extension = 'i';
        }

        Self {
            bits,
            base_extension,
            extensions: target_flags,
            suffix,
            vendor_os,
        }
    }

    fn is_g_extension(e: char) -> bool {
        e == 'm' || e == 'a' || e == 'f' || e == 'd'
    }

    pub fn retain_extensions(&mut self, extensions: &str) {
        let has_g = extensions.contains('g');
        self.extensions.retain(|&e| {
            (has_g && Self::is_g_extension(e)) || extensions.contains(e)
        })
    }

    pub fn remove_extensions(&mut self, extensions: &str) {
        for e in extensions.chars() {
            match e {
                'e' => {}
                'i' => self.base_extension = 'e',
                'g' => self.remove_extensions("imafd"),
                e => {
                    self.extensions.remove(&e);
                }
            }
        }
    }

    pub fn add_extensions(&mut self, extensions: &str) {
        for e in extensions.to_lowercase().chars() {
            match e {
                'e' => {}
                'i' => self.base_extension = 'i',
                'g' => self.add_extensions("imafd"),
                e => {
                    self.extensions.insert(e);
                }
            }
        }
    }

    pub fn has_extension(&self, extension: char) -> bool {
        match extension.to_ascii_lowercase() {
            'e' => true,
            'i' => self.base_extension == 'i',
            'g' => {
                self.base_extension == 'i' && "mafd".chars().all(|e| self.extensions.contains(&e))
            }
            e => self.extensions.contains(&e),
        }
    }
}

impl ToString for Target {
    fn to_string(&self) -> String {
        let mut unrecognized_extensions: Vec<char> = self
            .extensions
            .difference(&REGISTERED_EXTENSIONS)
            .cloned()
            .collect();
        unrecognized_extensions.sort();
        let unrecognized_extensions: String = unrecognized_extensions.iter().collect();

        let mut recognized_extensions = String::new();

        let has_g = self.has_extension('g');
        let base_extension = if has_g {
            'g'
        } else {
            self.base_extension
        };

        for e in EXTENSION_ORDER.chars() {
            if !(has_g && Self::is_g_extension(e)) && self.extensions.contains(&e) {
                recognized_extensions.push(e);
            }
        }

        format!(
            "riscv{}{}{}{}{}-{}",
            self.bits,
            base_extension,
            recognized_extensions,
            unrecognized_extensions,
            self.suffix,
            self.vendor_os
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Target;

    #[test]
    fn simplest_case() {
        let target = Target::from_target_str("riscv32ima-unknown-none-elf");
        assert_eq!(target.bits, 32);
        assert_eq!(target.base_extension, 'i');
        assert_eq!(target.extensions, "ma".chars().collect());
        assert_eq!(target.suffix, "");
        assert_eq!(target.vendor_os, "unknown-none-elf");

        assert_eq!(target.to_string(), "riscv32ima-unknown-none-elf");
    }

    #[test]
    fn order_is_canonicalized() {
        let target = Target::from_target_str("riscv32ami-unknown-none-elf");
        assert_eq!(target.bits, 32);
        assert_eq!(target.base_extension, 'i');
        assert_eq!(target.extensions, "ma".chars().collect());
        assert_eq!(target.suffix, "");
        assert_eq!(target.vendor_os, "unknown-none-elf");

        assert_eq!(target.to_string(), "riscv32ima-unknown-none-elf");
    }

    #[test]
    fn general_purpose_extensions_are_expanded() {
        let target = Target::from_target_str("riscv32gc-unknown-none-elf");
        assert_eq!(target.bits, 32);
        assert_eq!(target.base_extension, 'i');
        assert_eq!(target.extensions, "mafdc".chars().collect());
        assert_eq!(target.suffix, "");
        assert_eq!(target.vendor_os, "unknown-none-elf");

        assert_eq!(target.to_string(), "riscv32gc-unknown-none-elf");
    }

    #[test]
    fn unknown_extensions_are_sorted_and_appended_after_known_ones() {
        let target = Target::from_target_str("riscv32jimzaxc-unknown-none-elf");
        assert_eq!(target.bits, 32);
        assert_eq!(target.base_extension, 'i');
        assert_eq!(target.extensions, "jmzaxc".chars().collect());
        assert_eq!(target.suffix, "");
        assert_eq!(target.vendor_os, "unknown-none-elf");

        assert_eq!(target.to_string(), "riscv32imacjxz-unknown-none-elf");
    }

    #[test]
    fn retain_present_extensions() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.retain_extensions("c");

        assert_eq!(target.to_string(), "riscv32ic-unknown-none-elf");
    }

    #[test]
    fn retain_missing_extensions() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.retain_extensions("fd");

        assert_eq!(target.to_string(), "riscv32i-unknown-none-elf");
    }

    #[test]
    fn retain_both_missing_and_present_extensions() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.retain_extensions("cfd");

        assert_eq!(target.to_string(), "riscv32ic-unknown-none-elf");
    }

    #[test]
    fn retain_base_extension_i() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.retain_extensions("i");

        assert_eq!(target.to_string(), "riscv32i-unknown-none-elf");
    }

    #[test]
    fn retain_base_extension_e() {
        let mut target = Target::from_target_str("riscv32e-unknown-none-elf");
        target.retain_extensions("e");

        assert_eq!(target.to_string(), "riscv32e-unknown-none-elf");
    }

    #[test]
    fn retain_base_extension_no_upgrade() {
        let mut target = Target::from_target_str("riscv32e-unknown-none-elf");
        target.retain_extensions("i");

        assert_eq!(target.to_string(), "riscv32e-unknown-none-elf");
    }

    #[test]
    fn retain_base_extension_no_downgrade() {
        let mut target = Target::from_target_str("riscv32i-unknown-none-elf");
        target.retain_extensions("e");

        assert_eq!(target.to_string(), "riscv32i-unknown-none-elf");
    }

    #[test]
    fn retain_extension_g() {
        let mut target = Target::from_target_str("riscv32imacj-unknown-none-elf");
        target.retain_extensions("g");

        assert_eq!(target.to_string(), "riscv32ima-unknown-none-elf");
    }

    #[test]
    fn remove_missing_extensions() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.remove_extensions("fd");

        assert_eq!(target.to_string(), "riscv32imac-unknown-none-elf");
    }

    #[test]
    fn remove_base_extension() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.remove_extensions("i");

        assert_eq!(target.to_string(), "riscv32emac-unknown-none-elf");
    }

    #[test]
    fn remove_base_extension_downgrade() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.remove_extensions("e");

        assert_eq!(target.to_string(), "riscv32imac-unknown-none-elf");
    }

    #[test]
    fn remove_base_extension_e() {
        let mut target = Target::from_target_str("riscv32emac-unknown-none-elf");
        target.remove_extensions("e");

        assert_eq!(target.to_string(), "riscv32emac-unknown-none-elf");
    }

    #[test]
    fn remove_extension_g() {
        let mut target = Target::from_target_str("riscv32emacj-unknown-none-elf");
        target.remove_extensions("g");

        assert_eq!(target.to_string(), "riscv32ecj-unknown-none-elf");
    }

    #[test]
    fn add_base_extension_e() {
        let mut target = Target::from_target_str("riscv32imac-unknown-none-elf");
        target.add_extensions("e");

        assert_eq!(target.to_string(), "riscv32imac-unknown-none-elf");
    }

    #[test]
    fn add_base_extension_i_upgrades() {
        let mut target = Target::from_target_str("riscv32emac-unknown-none-elf");
        target.add_extensions("i");

        assert_eq!(target.to_string(), "riscv32imac-unknown-none-elf");
    }

    #[test]
    fn add_missing_extensions() {
        let mut target = Target::from_target_str("riscv32emac-unknown-none-elf");
        target.add_extensions("fd");

        assert_eq!(target.to_string(), "riscv32emafdc-unknown-none-elf");
    }

    #[test]
    fn add_present_extensions() {
        let mut target = Target::from_target_str("riscv32emac-unknown-none-elf");
        target.add_extensions("ma");

        assert_eq!(target.to_string(), "riscv32emac-unknown-none-elf");
    }

    #[test]
    fn add_extension_g() {
        let mut target = Target::from_target_str("riscv32emac-unknown-none-elf");
        target.add_extensions("g");

        assert_eq!(target.to_string(), "riscv32gc-unknown-none-elf");
    }

    #[test]
    fn has_base_extension_i() {
        let target = Target::from_target_str("riscv32imac-unknown-none-elf");

        assert!(target.has_extension('i'));
        assert!(target.has_extension('e'));
    }

    #[test]
    fn has_base_extension_e() {
        let target = Target::from_target_str("riscv32emac-unknown-none-elf");

        assert!(!target.has_extension('i'));
        assert!(target.has_extension('e'));
    }

    #[test]
    fn has_extension_if_present() {
        let target = Target::from_target_str("riscv32imac-unknown-none-elf");

        assert!(target.has_extension('c'));
    }

    #[test]
    fn has_extension_if_missing() {
        let target = Target::from_target_str("riscv32imac-unknown-none-elf");

        assert!(!target.has_extension('f'));
    }

    #[test]
    fn has_extension_g_if_missing() {
        let target = Target::from_target_str("riscv32imac-unknown-none-elf");

        assert!(!target.has_extension('g'));
    }

    #[test]
    fn has_extension_g_if_present() {
        let target = Target::from_target_str("riscv32imafd-unknown-none-elf");

        assert!(target.has_extension('g'));
    }
}
