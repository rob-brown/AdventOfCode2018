extern crate regex;

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::cmp::{min, max};

#[allow(dead_code)]
fn day1() {
  let inputs: Vec<i64> = vec!(7, 15, 18, -15, 8, -2, 4, -12, 19, 6, -19, -12, -4, -10, -5, 4, 7, -13, -12, 10, 3, 18, -10, -14, -14, 5, -10, 6, 8, -15, -3, -12, 4, -12, 17, -11, 7, 13, 9, -7, 13, 10, 11, -18, -4, 7, 17, 20, 18, 15, 8, 19, -3, 2, 17, 16, -9, -9, 16, 17, 14, -11, 4, 9, -1, -14, -2, -10, 16, 4, 3, 10, 17, -18, 3, 17, 19, -3, 10, -6, 16, -12, -14, -11, 3, -11, -5, 2, -4, -7, 17, 15, -5, 18, -15, 21, 6, -16, -5, 19, 9, 1, -4, 1, 14, 19, -11, 18, -13, 11, 6, -2, 16, 16, 9, -18, -13, 1, 18, -12, 16, 19, 11, -10, 15, -6, 11, 11, -8, -6, 9, 1, -15, 1, 19, -3, 1, 8, 1, -20, 6, -3, -6, -14, -13, -8, -9, 3, -16, 3, -18, 16, 9, 16, -2, 8, -9, -8, -9, 6, 15, 6, 12, 10, -18, 12, -1, -20, 7, -17, 16, -20, -20, -17, 6, -11, -19, 10, 4, 4, -16, 19, -4, -9, 15, 19, -15, 2, -16, -9, -7, -12, -20, 19, 15, 13, -11, -3, 5, 13, 6, -11, 14, 14, 10, -9, 4, -8, 2, 17, -21, -28, -17, -14, 8, 11, -7, 4, 14, 23, 28, -2, 12, 17, 23, 19, -9, -17, -10, 20, -16, -7, 5, -13, -6, 40, 11, -1, 12, -4, 21, 7, 8, 17, -16, -11, -6, -3, 16, 7, -16, 1, 13, -19, -11, -17, 12, 6, -3, 17, 4, 20, -1, -5, 1, -11, 18, 18, -8, 16, -2, 18, -13, -12, 10, -19, 7, -16, -14, -20, 18, -17, -5, 3, -12, -3, 7, 13, 21, -10, -18, -9, -8, 18, 18, 13, 19, -17, 15, -16, 6, -15, 20, 17, -20, -1, 3, 7, 4, -2, 19, -3, 17, 10, -13, -13, 1, -4, 17, 18, 1, -13, -10, 14, -16, 13, -9, 16, -13, 7, 5, 4, 25, -18, -2, -1, 27, -3, 24, -6, 19, 17, -7, -3, 15, 3, 18, 2, 3, -14, 16, 11, -14, 10, 2, -3, 14, 13, 19, -4, -4, -9, 7, 11, -19, -9, -9, 8, -3, 16, -19, -13, -1, 6, -8, -11, -8, -9, -10, -20, -10, 5, 4, 19, 6, -2, -5, 15, -7, 12, 18, -10, 3, 11, -17, -4, 8, -10, -2, 1, 15, 19, 13, 1, 12, -16, 18, 16, -17, 15, 13, 13, 18, -9, 10, 18, -3, -7, 9, -13, -12, 6, -16, -7, -3, -2, -11, -1, 6, 21, 3, 7, 14, 21, -2, -2, -6, -27, -20, -7, -20, -27, -2, 11, 21, 1, 10, -6, -24, 6, 7, -11, -10, -14, -21, -10, 8, -36, -8, 1, 11, -6, -18, -2, -3, -16, -5, -30, -14, -1, 5, -19, 13, 15, 26, -38, 1, -18, -1, 6, -16, -2, 72, -6, -106, -52, 35, 8, 64, 59, 47, 19, -11, 137, -40, 23, 27, 5, 47, 76, 164, 108, -314, -396, 72153, 12, -8, 15, -8, -4, 17, 4, 3, -18, 4, -14, -7, -8, -5, 6, 1, 15, 1, -4, -16, -10, -17, -11, 2, 6, -14, 9, 11, -15, -8, -11, 8, -15, -3, -10, 5, -9, -16, -5, -9, -8, -16, -11, -7, -4, -22, -16, 9, 23, 1, 21, 1, 3, -1, 8, 18, 19, 7, 10, -16, -9, 1, -5, 14, -12, 21, 8, 3, 15, -6, 18, 11, 8, -2, 10, 16, 9, -14, -16, 2, 17, -18, -20, 1, -12, 4, -12, -8, -10, 5, 4, -18, -8, -15, -17, 6, -1, -26, -11, 26, 25, 9, 20, 20, 12, -14, 11, -4, 12, 11, -14, -8, -5, 19, 20, 8, 17, 3, -2, -14, -11, 2, 12, -6, 5, 17, 13, 15, -5, 14, -5, 19, 13, 11, -15, 10, 7, 7, -13, -2, -2, -5, 1, -14, -9, -16, -7, 18, -14, 18, -20, -17, 28, 8, 16, -7, 10, -5, -9, 15, 20, 11, -1, 4, 4, -2, -13, 18, 11, 9, 4, 11, 17, 16, 19, -15, 16, 6, -8, -7, -15, -2, 9, -6, 9, -17, -19, 11, 18, 10, 1, 6, 15, 16, 14, -8, -9, 12, 2, -13, 9, -14, -7, 5, -4, 14, 15, -17, -16, 19, -12, -9, -4, 15, 18, 3, 11, 19, 18, 16, 3, 2, -13, -11, -4, -19, 3, -9, 1, 18, 14, 19, 15, 11, -18, -10, 8, -12, 20, 6, 2, 20, -18, -8, 17, -2, -1, 17, 20, 6, -17, -7, -12, -8, -8, 11, 21, 9, 10, 17, -8, 13, 10, -4, 10, -2, 6, -5, -18, -13, 6, -4, 3, -17, 16, -13, -10, 16, -24, -5, 18, 6, -15, -17, -4, 20, -26, -17, -4, -10, -13, -19, 3, 19, 5, -6, -4, 8, 20, 8, -13, -16, -17, -19, -15, -2, 3, -16, -1, 9, 10, -17, -10, -17, -3, 21, -16, -4, -21, 10, -20, -1, -9, -8, 13, -18, 4, 5, -17, -10, 6, -12, -7, -5, 4, -17, -15, 11, 18, 8, 20, 2, -14, 5, -15, 13, -14, 8, 18, 6, 14, 16, 4, 9, 7, 8, -22, 10, -9, 5, -15, -19, 1, 10, -27, -19, -16, -9, 12, 10, 18, -10, 5, 4, -22, -10, -17, -17, -14, 3, -2, -17, 24, -9, 12, 17, 9, -16, -1, 3, 23, -6, -24, -19, 13, -14, 9, 29, 11, 25, -18, 5, 11, 20, 27, -10, -15, 4, 5, -17, 26, 55, -6, -3, 4, -21, 47, 12, 20, -2, -12, 8, -2, -14, 19, 16, -9, -19, -14, 22, -17, 3, 2, -7, 21, -11, 21, 20, -6, 8, -17, -2, -19, -28, 42, 14, 21, 9, -11, 25, -4, 17, -15, 19, -72362); 

  let sum: i64 = inputs.iter().sum();
  // 505
  println!("Day 1:A = {:?}", sum);

  let mut frequency: i64 = 0;
  let mut frequencies: HashSet<i64> = HashSet::new();

  'outer: loop {
    for n in inputs.iter() {
      frequency += n;

      if frequencies.contains(&frequency) {
        // 72330
        println!("Day 1:B = {:?}", frequency);
        break 'outer;
      }

      frequencies.insert(frequency);
    }
  }
}

#[allow(dead_code)]
fn day2() {
  let inputs: Vec<&str> = vec!("auxwcbzrmdvpsjfgkrthnkioqm", "auxwcbzrmdvpsjfgbltonyijqe", "auxwcbzrmdfpsefgklthnoioqe", "auxwcbzrmdvpsjfgkluhnjisqe", "auxwcbzrmdvesjfgdzthnyioqe", "auxwcbzrmdvhsjfgklthnmijqe", "auxwcbzridvpsjfgkltxeyioqe", "ayxwcbzrgdvpsjfgklthiyioqe", "ajxwcbzrmdvpsjfgklkhnyiode", "auxwcbcrmdvpsjfqelthnyioqe", "auxwcbzrmsvpsjsgklthnyiope", "auxwcbzrmqvpsjzgklghnyioqe", "auxwcbzrmdvpsjtqklthxyioqe", "auxwcbzrmdopsjfdklthncioqe", "auxwcbzrmdvpsjfgkltmhyfoqe", "aixwcbzrmdvpsjfgllthdyeoqe", "vuxicbzrmdepsjfgklthnyioqe", "auxwcbbxmdkpsjfgklthnyioqe", "auxwcbzrgdvpsofaklthnyioqe", "auxycbzrmdvpsjfgklthnyuose", "aujwcbzrmdvprjfgkltcnyioqe", "auxwgbzrmdvpsjfgyzthnyioqe", "auxwcbzrmavpsjfgkltsnyiome", "auxwcbgrmdvpsjfgkdthnrioqe", "kuxwcbzrmdvpsyfgklthnyioue", "auxwcbzomdvpjdfgklthnyioqe", "auxwcbzrmdppsjfgklthvyifqe", "aunwubzrmdvpsjrgklthnyioqe", "auxwcbzrmoipsjfgklbhnyioqe", "auxwdbzrmdvpsjfgmlthnyioce", "auxwcbzjmsvpsjfiklthnyioqe", "auxwcbzrmwcpsjfcklthnyioqe", "auxwcbzfmdvprjfhklthnyioqe", "auxdcbzrgdvpsjfgklthnyxoqe", "wuxwbbzrmdvpsjfgklthnyiote", "auowcbjrmdvpsjfgklthnyfoqe", "auxwsbzrmdvpsjfglltcnyioqe", "quxwcbzrmdvpkjfgklthnyioqt", "vuxwcbzrudvpsjfgklthnyioqi", "puxwcbzrmdvgsjfgklthncioqe", "luxdcbzrmdvpsjfgkothnyioqe", "auxwcbzrmdvpsjfyklthfhioqe", "auxwcbqrmdvpsjfgkldhnyiote", "quxwcbzrmlvpsjfgklthnyioqi", "auxwcbzgmdvpsjfoklthnyiuqe", "auxwcbzrmdvpsbfgkltdjyioqe", "auxwcbzsmdrpsjfgklthpyioqe", "auxwcbzrmfvpsjfwklthnyiote", "auxbkpzrmdvpsjfgklthnyioqe", "auxwcbzrddvpsjfsklthnyroqe", "abxwcbzrmdvpsjfgkltdnyivqe", "awxwcbzrmvvpsjfgklthngioqe", "auxwcbzrmkvgsjfgkltcnyioqe", "auxwcbammdvpsjfgklthpyioqe", "auxwcbhrmdvpsjfgtlthnuioqe", "auxwcpzrmdvpbjogklthnyioqe", "auxwcbzrmdvpslfgklbhkyioqe", "auxwcbsrmdvpjjfgkldhnyioqe", "auxwcbzrmdqpsjfgauthnyioqe", "ydxwcbxrmdvpsjfgklthnyioqe", "auxwcbzrmdvpejfgklthnyyofe", "auxwchzrmxvpsjfgklthnyioqh", "auxwcbzrtdvpsjfgklxhnzioqe", "auxwcbyrmdvpsnfgklnhnyioqe", "auxwcbzrcdvpsjugklihnyioqe", "auxwcbzrddvpsjfgklhhnyiaqe", "aumwtbzrmdvpsjfgklthnyitqe", "auxucbzrmdvpsjfgklthwfioqe", "auxwcbzrmdvpzmfgkllhnyioqe", "auxwcbzrmdvpsjhgklthntiome", "buxwzbzrmdvpszfgklthnyioqe", "ouxwcbzsgdvpsjfgklthnyioqe", "auxwcbzrmdvpsjfskltgnyioqz", "auxwcbbrmdvpsjftklthnyioqu", "quxocbzrmdvpsjfgklthfyioqe", "acxwcbzrmdvpsjfgklfhnrioqe", "auxwcbzrmdnpsjfrkjthnyioqe", "wuxwybzrmdwpsjfgklthnyioqe", "auxwgbxrmdvpsjfghlthnyioqe", "atxwcbzrmdvnsjfgklthnyjoqe", "acxwcbzmmdvpsjfbklthnyioqe", "auxhcbzrmdvbsjbgklthnyioqe", "auxwlbzrfdvpsjfgxlthnyioqe", "auxwmbzrmdfpsjqgklthnyioqe", "auxwcbzrmdvpsgfgklahnyigqe", "auxwgbzrmdvpsjfgzldhnyioqe", "auxwcbzrmdvpydfgklthnyiohe", "auxwxbzrmdvpsjfsklchnyioqe", "auxqcbzrmdvpsjfgqlthnyiwqe", "auxwcozrmdvssbfgklthnyioqe", "auxvcczrmdvpsufgklthnyioqe", "auxwcbzrudvpsjfgklyhnyioxe", "aulwcbzrmdvpsjqgknthnyioqe", "auukcbzrmdvpsjfgklthtyioqe", "auxwcszimdvpsjfgklthnyigqe", "juxwcbzrbdvpsjfgklthnyboqe", "auxwcbzrmdvpjofgklthnyioqj", "auxwcbzrmdvpsjfgplfhnyione", "auxwcbzrmdhpsjfgkltknyeoqe", "luxwcqzrmdvpsjfgklthnbioqe", "uuxwcbzrmdvpsjfgkithnyiiqe", "auxwcbzrmdvpdjfgkrthnyeoqe", "auuwcbnrmdvpsjfgklthnjioqe", "auxwcnzrmdvpsjvgklthnyooqe", "auxwcbzcmdvpsjfcklthnyiose", "auxwcbzrldfpsjfgklthjyioqe", "auxwcizrmdvpsjfjklthnymoqe", "auxwcbtrmdvpsjfgtlphnyioqe", "amxwcbzrmdvksjfgklthnyiove", "auxwcbzrmdvpszfgkpthnyiuqe", "auxwcbzrmdvxdjfgkltqnyioqe", "auxwcbzrudvpsjfgklthnymiqe", "auxwcbirmdvfsjfgklmhnyioqe", "auwwcbzrndvprjfgklthnyioqe", "auxwcbormdgpsjfgklbhnyioqe", "auxwabzrmdupsjfgklthnyioqt", "auxvcbzrmdvpsjfgkltrmyioqe", "auxwcbzrmddpsjfsklthnyizqe", "auxwcczrmuvpyjfgklthnyioqe", "auxwcczrmdvpsnfgkpthnyioqe", "auxkcbzrmdvpsjfhklihnyioqe", "auxwcbzrmdvpsjfgklthnkijje", "auxwcbzcmdvpsjpgkldhnyioqe", "auxwcnzrudvpstfgklthnyioqe", "xuxwcbzrgdvusjfgklthnyioqe", "aaxwcbzrmdvpsjvgklthnyidqe", "auxwcbztmdvpsjfgklthnyhqqe", "auxwcbzrmfvpsjfgklthnyilfe", "auxwcbzrmdvksjfgklthjyioqq", "auxwcbzrmdzksjfgktthnyioqe", "auxwcbzrmfvpszfgklohnyioqe", "auxwckzamdvpsjfgklthnyioqs", "auxwcmzrhdvpsjfaklthnyioqe", "fuxwcbzrmdapsjfgklrhnyioqe", "avxwxbzrmdvpsjfgklthniioqe", "auxwubzrmevpsjfgkltpnyioqe", "fuxwcbzrgdvpsjfgklhhnyioqe", "duxwwbdrmdvpsjfgklthnyioqe", "audwcbzrmdvpnjcgklthnyioqe", "auxtcbzrmdvpsjmgklthnyyoqe", "aucwcbwrmdepsjfgklthnyioqe", "auxwcbzrudvpsjfpklthnyiose", "auxwcbzridvpsjfsklthxyioqe", "auxtcbzrmdvpscfgklyhnyioqe", "auxwcbzrmdvppjfgklthnyivee", "auxwdbzrmuvpskfgklthnyioqe", "auxwubzrmdvosjfgklthnyiope", "auxwcbzrmhnpsjfgklthnyimqe", "auxwcbzrmdqpwjfgkltpnyioqe", "auxwcbormdvpsjljklthnyioqe", "auxwcbzrmdjpsjfgkltjpyioqe", "auxwcbzrmdvpszfgklthkyizqe", "auxwcbzighvpsjfgklthnyioqe", "auxwcbzrmdlpsjfgcythnyioqe", "auxwcbzumdvpsjflklthnyimqe", "pdxwcbzrmdvpsjfgklthnyihqe", "auxwcbzrsdvpsjfgklhhvyioqe", "auxwcfzamdvpsjfgkmthnyioqe", "aexwcdzrmdvpsjogklthnyioqe", "auxxcbkrmavpsjfgklthnyioqe", "auxwcbzredvssjfgklthryioqe", "aupwqbzrmdvpsjfgklthnyioqc", "auxwcbzrmdvpkcagklthnyioqe", "auxwcbzrmdvwsbfgklthnlioqe", "aunwcbzxmdvhsjfgklthnyioqe", "auxwcbzrhddpsjfgklthnnioqe", "ouxwcbzrmdvtsifgklthnyioqe", "auxwcbzrmdqpsjfgklthnyfoqp", "auxwrbzrhdvpsjfgolthnyioqe", "auxwcbcqmdvpsjugklthnyioqe", "auxwcbzrqdvpsjhgklthnjioqe", "auxmcbzrmdvpsjfgmlthnyjoqe", "auxwcbzrmdvpsjfgzlthnycoqv", "auswcbzrmdvpsffgslthnyioqe", "auxwcbzrfdvpsjfrmlthnyioqe", "auxwcbzrmdvpsjngzlthnxioqe", "auxwcbzrmdvpsjfuqlthnyiyqe", "auxwzbzrrdvosjfgklthnyioqe", "auxwcbzdmdvpsjfikxthnyioqe", "guxwcbzrmdvpsjfgmlthnytoqe", "auxwcbzrmdvpspfgkytenyioqe", "auxvcbzrldvpsjfgklthnyhoqe", "auxwcbzrmavpckfgklthnyioqe", "autwcbzrmdvpsafgklthnyirqe", "auxwcbzrxuvpsjfgklthmyioqe", "auxwcbarmdppsjfgklthnywoqe", "anxvcbzrmdvpsjfgklthnyijqe", "auxwcbwrmdapsjngklthnyioqe", "abxwcbzrmdvpsjugkltgnyioqe", "auxwcbtrmdvpsjfgkltunyioue", "aujwcbzrmovpsjfgklthryioqe", "auxwcbzrydvpsjfgklthndikqe", "auxwcbzrmdvpsjfgklmrnyioqo", "auxwcbzrddvpsjfggithnyioqe", "auxwcbzrmdvpfjfaklthlyioqe", "fuxtcbzrmdvpsjfgklwhnyioqe", "tuxwcbzrjdvpsjfgjlthnyioqe", "auxwcbzrmdppsofgklthnyfoqe", "auxvclzamdvpsjfgklthnyioqe", "auxwcbzrmdvpsjfdklhhnzioqe", "auxwcbzrmsvpsvdgklthnyioqe", "arxfcbzrmdvpsvfgklthnyioqe", "auxzcbzrmdvpsjfgklthnhioqj", "auxwcbzrrdvpsjfgpltunyioqe", "auxuibzrmdvpwjfgklthnyioqe", "auxwcbzrwdqpsjfgklthnyooqe", "aujwcbzrmdvpsjvgklthxyioqe", "abxwcbzrmfvpsjfgklthnyxoqe", "aurwcbzrmdvpshfgklthnyhoqe", "auxwcbzjmdvpsjfgknthnycoqe", "auxwcbzrmdvpsjfgklmhxwioqe", "auxwcbzrmfvpsjfgklthnyiorq", "auxwcbormdvpsjfgklwhnlioqe", "auxwctzrmdvpsjfgklcknyioqe", "awxwcbzrmdvpsjfgvlthnyiome", "auxwcbzrmdvpsjfjklthnyixje", "auxwcsxrmdvpsjfgkltsnyioqe", "auxbmbzrmdvpsjfgklthnyioce", "auxwcbzrmdvpsjfukzthnytoqe", "aixwcbzrmdvpsjfgllthdyioqe", "auxwcbzrmdypsjfgklthnlioqy", "auxccbzrmdvpsjfgkltrnnioqe", "auxwcznrmdvpsjfgklthnykoqe", "auxwmqzrmdvpsjfgilthnyioqe", "auxwcbzrmdvpdyfgolthnyioqe", "auxwcbzrmdvpsjfgkmohnqioqe", "auxwcfzrmzvpsjfoklthnyioqe", "auxwjyzrmdvpsjfgulthnyioqe", "auxwcgzredvpsjfgkxthnyioqe", "wuxwcbtrmdvpsjfgklthnyiofe", "auxwcbzrmdopsgfgklthncioqe", "auxmcbzjmdvpsjfgklbhnyioqe", "auxwlbzrmdvpsjffklthgyioqe", "auxwcbzrmrvpsjfgqlthtyioqe", "kuxwhbzrmdvpsjfgklthgyioqe", "auxwcozrmdgpsjfgklthnydoqe", "auxwdbzrmdvpdjfgklthgyioqe", "auxwqbzrmdapsvfgklthnyioqe", "auqwcbzridvjsjfgklthnyioqe", "auxwckzrmdvpsjfoklthnyuoqe", "auxwcbzvmdvpsjfgklghnyiome", "auxtcbzrmdvpsjqgktthnyioqe", "auxwcbzrmdvesjfgkljhnnioqe", "auxwcbzrmpvpsqfgklthnqioqe", "auxwcbzrmdcpsqfgklthnzioqe", "yuxwcbzrmdvpsjggklthnlioqe", "auxwcbzradvpsjftklthoyioqe", "auxwcbzrmdvjujfgklmhnyioqe", "auxwcbzrmdvpsrfgklpinyioqe", "auxwobzrvqvpsjfgklthnyioqe");
  
  let mut two_counts = 0;
  let mut three_counts = 0;
  
  for string in inputs.iter() {
    let mut map: HashMap<char, i32> = HashMap::new();
    
    for character in string.chars() {
      let counter = map.entry(character).or_insert(0);
      *counter += 1;
    }
    
    for (_, value) in map.iter() {
      if *value == 3 {
        three_counts += 1;
        break
      }
    }
    
    for (_, value) in map.iter() {
      if *value == 2 {
        two_counts += 1;
        break
      }
    }
  }
  
  let checksum = two_counts * three_counts;
  
  // 6972
  println!("Day 2:A = {}", checksum);
  
  for string1 in inputs.iter() {
    for string2 in inputs.iter() {
      if string1 >= string2 {
        continue;
      }
      
      let mut difference_count = 0;
      
      for (char1, char2) in string1.chars().zip(string2.chars()) {
        if char1 != char2 {
          difference_count += 1;
        }
      }
      
      if difference_count == 1 {
        let mut result: String = "".to_owned();
        
        for (char1, char2) in string1.chars().zip(string2.chars()) {
          if char1 == char2 {
            result += char1.to_string().as_str();
          }
        }
        
        // aixwcbzrmdvpsjfgllthdyoqe
        println!("Day 2:B = {}", result);
      }
    }
  }
}

#[allow(dead_code)]
fn day3() {
  let mut map: HashMap<(i32, i32), i32> = HashMap::new();
  let mut file = File::open("../inputs/day3.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
  
  for capture in regex.captures_iter(&contents) {
    let x = capture[2].parse::<i32>().unwrap();
    let y = capture[3].parse::<i32>().unwrap();
    let offset_x = capture[4].parse::<i32>().unwrap();
    let offset_y = capture[5].parse::<i32>().unwrap();
    
    for dx in 0..offset_x {
      for dy in 0..offset_y {
        let point = (x + dx, y + dy);
        let counter = map.entry(point).or_insert(0);
        *counter += 1;
      }
    }
  }
  
  let mut result = 0;
  
  for (_, count) in map.iter() {
    if *count > 1 {
      result += 1;
    }
  }  
  
  // 118539
  println!("Day 3:A = {}", result);
  
  'outer: for capture in regex.captures_iter(&contents) {
    let id = &capture[1]; 
    let x = capture[2].parse::<i32>().unwrap();
    let y = capture[3].parse::<i32>().unwrap();
    let offset_x = capture[4].parse::<i32>().unwrap();
    let offset_y = capture[5].parse::<i32>().unwrap();
    
    for dx in 0..offset_x {
      for dy in 0..offset_y {
        let point = (x + dx, y + dy);
        
        if let Some(count) = map.get(&point) {
          if *count > 1 {
            continue 'outer; 
          }
        }
      }
    }
    
    // 1270
    println!("Day 3:B = {}", id);
    break;
  }
}

#[derive(Debug, Clone)]
struct Time {
  month: i32,
  day: i32,
  hour: i32,
  minute: i32,
}

impl Time {
  fn new(month: i32, day: i32, hour: i32, minute: i32) -> Time {
    Time { month: month, day: day, hour: hour, minute: minute }
  }
}

#[derive(Debug, Clone)]
struct LogItem {
  time: Time,
  entry: String,
}

impl LogItem {
  fn new(time: Time, entry: String) -> LogItem {
    LogItem { time: time, entry: entry }
  }
}

#[derive(Debug, Clone)]
struct GuardInfo {
  id: i32,
  total_time_asleep: i32,
  previous_sleep: Option<Time>,
  minute_asleep: HashMap<i32, i32>,
}

impl GuardInfo {
  fn new(id: i32) -> GuardInfo {
    GuardInfo { id: id, total_time_asleep: 0, previous_sleep: None, minute_asleep: HashMap::new() }
  }
}

#[allow(dead_code)]
fn day4() {
  
  let mut file = File::open("../inputs/day4.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let entry_regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();
  let guard_regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();

  let mut map: HashMap<i32, GuardInfo> = HashMap::new();
  let mut current_guard = 0;
  
  let mut log_items: Vec<LogItem> = Vec::new();
  
  for capture in entry_regex.captures_iter(&contents) {
    let month = capture[2].parse::<i32>().unwrap();
    let day = capture[3].parse::<i32>().unwrap();
    let hour = capture[4].parse::<i32>().unwrap();
    let minute = capture[5].parse::<i32>().unwrap();
    let time = Time::new(month, day, hour, minute);
    let message = capture[6].to_string();
    let entry = LogItem::new(time, message);
    log_items.push(entry);
  }
  
  log_items.sort_by(|a, b| 
    if a.time.month == b.time.month {
      if a.time.day == b.time.day {
        if a.time.hour == b.time.hour {
          a.time.minute.cmp(&b.time.minute)
        }
        else {
          a.time.hour.cmp(&b.time.hour)
        }
      }
      else {
        a.time.day.cmp(&b.time.day)
      }
    }
    else {
      a.time.month.cmp(&b.time.month)
    }
  );

  let mut sleepiest_guard: Option<GuardInfo> = None;
  
  for item in log_items.iter() {
    if item.entry == "wakes up" {
      let guard = map.get_mut(&current_guard).unwrap();
      
      if let Some(time) = guard.previous_sleep.clone() {
        guard.previous_sleep = None;
        let time_asleep = item.time.minute - time.minute;
        guard.total_time_asleep += time_asleep;
        
        let mut current_minute = time.minute;
        
        while current_minute != item.time.minute {
          let counter = guard.minute_asleep.entry(current_minute).or_insert(0);
          *counter += 1;
          
          current_minute = (current_minute + 1) % 60;
        }
        
        if let Some(sleepy) = sleepiest_guard.clone() {
          if guard.total_time_asleep > sleepy.total_time_asleep {
            sleepiest_guard = Some(guard.clone());
          }
        }
        else {
          sleepiest_guard = Some(guard.clone());
        }
      }
    }
    else if item.entry == "falls asleep" {
      let guard = map.get_mut(&current_guard).unwrap();
      guard.previous_sleep = Some(item.time.clone());
    }
    else if let Some(captures) = guard_regex.captures(&item.entry) {
      current_guard = captures[1].parse::<i32>().unwrap();
      let _ = map.entry(current_guard).or_insert(GuardInfo::new(current_guard));
    }
  }
  
  {
    let guard = sleepiest_guard.unwrap();
    let mut minute_most_slept = 0;
    let mut times_slept_in_minute = 0;
    
    for (minute, times) in guard.minute_asleep {
      if times > times_slept_in_minute {
        times_slept_in_minute = times;
        minute_most_slept = minute;
      }
    }
    
    // 76357
    println!("Day 4:A = {}", guard.id * minute_most_slept);
  }

  {
    let mut sleepiest_guard_id = 0;
    let mut minute_most_slept = 0;
    let mut times_slept_in_minute = 0;
    
    for (_, guard) in map {
      for (minute, times) in guard.minute_asleep {
        if times > times_slept_in_minute {
          times_slept_in_minute = times;
          minute_most_slept = minute;
          sleepiest_guard_id = guard.id;
        }
      }
    }
    
    // 41668
    println!("Day 4:B = {}", sleepiest_guard_id * minute_most_slept);
  }
}

#[allow(dead_code)]
fn day5() {
  let input = "JjDdoODgGdNnIiTtJjFfnsSNsVUaAugGpPLAalvVHRrhvhoOZzQqHLhjJSsOoJjSaAsrqkBbKQRtTHlSqQeEioOICcUuHmMhPVvJjpJjDdhHUuSLllswWSLsJjFxXbBVvaAfuUoaFPpPpWwZzfAOrKkRxXnNqIJjKkiJCcjVvrRTIHhNnipPWwvVtLlBbMmrReEAaQwWlLSLldjgAattTTGZzJqUnNVvuQlLDsEDdLlzuWwUZDdCceFRrNpPnRrfFGLlgXWwxiAaILkKDYydlYqQytdDTtNnMFCcBbTQqtfsSmTaAyCcYhSsGgkDgBbfFGdKPpmMjuUJHfkuYyUXnNxDdyYlLOoQqmMuUuUPpYrRSrRnEeNOosyYSosSNMmnOOojJsFfYyEuUhMmTtzZQLhHFflmMkKJBMmbDdjoPpxXLwWlBbOQhHIiLlXxqqxXHjoOmMJdlLYyAariqQITtRHhoODiIzZqZzxXFfQlQOoqLtpPpeEPNLlncldDLCTIieEdZzoaAOWwIihHBbhHyIiIiYVvRrxXqQiIQDzZdqtTLlPccCCpPpHhWwiYyoOIBbVfFfhHFvWoYyfpPaAFbwWPTDdtrRpDGfXDdxFQqtTxXfFgxEZzZzeQqBbZzXWBbwdzZByYAuUgGaTKkcCtnNOlkKxXLFrRMmUiIFEefcUiIuCgGOoIioUujeEIiJjxXiIyYGrRyYoOJjgeRrEEeEelLKkbBzBbZJFTtfbBAkKqQASsGgaFOoIipqtTQPfxFfTrRaABYOYZzyoymMMmbAfzLKkEeCcxXlXxjpPeEJVvzZZmMIihHoOFsSamRrMhHBqQHhBPpbbBtTFAantTNnNfoObsqQTtSsyRXxrYSsSBrRHhRrAabPILlilPmBbZFfkHhuUTRrXxtKxXhHzMdDHLkeEfFKlPwFeEfWpDdOohoQqnNOpWwTNnPpoOtLyYPpTeEuUtzTrRtZBbeEBbIoeFfLYuUylOoEfFySsdDYOViIvisSmDdiINtdDWwTLUutTlPnNhyYHXxkKbBvVpJjqdDQnpPwWqQAaFgFfGDdecVvXIiGgzZxCGgZgGzWwqQqQwWyYnNmnNMtTUEeuEIVUoOooOzsHhCcSPpZHhOjNnYyOPpoGgUuJujaAGgXxJifRrFInNUuisSxXIKNxXnkPpTQqoBbOtuUdDgGrRbBgGUuIDdhHiHcHLlhCCcWhHeNrRiIHHhhngGEGgNnnFfNKkAacCUuPpMLlmcCwdRrerjJVvREDPpaEeuUAnNhNnQGJrRjgjJBwWbqfFGgWwaqQAPpPdDpPMaAmGaAyYcCgkKxnNkKPpsvNnVNnyfRrFYzZEehHSmEeXDdxCpEeHhPcoOFiIqQOorLlPqQYypRGgptTVvmAaMCcyYYyrRkVvTtOvMmVoaWwAKZzhHqQEpPslLqQSsSFfGgpwWXoOxGgPCYiIVvybNnyYxXqkKQBYPpyqQcTigGIsSisuUwWoOSIXxWwiIdlLWfFZwWznNwiGSsgKkVvkRrvGgVIieEbBwoOWxXKImMDgGtjJGwWgChHclmsSMLMJjmSXxsAafFtTLiIeEGgPfFmMpYynNAaNnoOPphHpPjJyqQwWPrRpgGYZzyNnCMmeESsXxbBciOoqQLlfTtFgGNnqQtTPXIHhixpEeRryFDdfCcyFfYYkKWqQwhHMrRmJjiZzKkIbLlmIugGzZUiMyyYPuLsoOsSzZXxzZSfmMPpeENQqnlHhLscbBCGBfFbgJjSQqYyqQklLKpPneEpPNUuPptxBbXTjJFvhHfFpcyYPPpVEevFAaTwPBbwWZzpGgaWwwWHXxhAWtfiILlxXJKkjvVpCJjPpvVjkpPJjiIgGNnYysSKJGgJjXxYrJjfFRyxfKkFqQwtJjwWTQhirRIHoOqveEVRvVrvFuUfgcCGZzIikKAaDdDbPpPpBDdolLYyMyYmeuUdsSDEbBRrhHOEUucCfuvVDdiIUFeMmSIisrZzeEgGRdYyVORrowJjdDjJknRrNpzZnNyYPKVQqvdDJcCGKkWwIrRigvVTtcwuUWzZyoOieyYEIYpPFKkfiIuUEeZiINGgZzEEeVvTYypPQAapPqIiaAJjDUudlQqLwWtHqQZzWwFfqJjRoOFfrQLbBlDIidnNiIhyYoyGgYJKkjJjOeyIgGilVoOlLCgGEecvLvQqVgsAapuEedDUlLOorRsSEexXyYHGgoOhFoPpOpPnBFfbeENcECcdDeUxxTtXXuNnuUWwTJSsBbIdDijnNrRnPpJrRHhlLTnsSDdNLbBqQoOlsNTtNndBbDXxVvmMnSGNzaAZuDdUEeBbsxXNnSjJZuUzkKvViIcCWdbBDZYyzdDIrRhHLEedDlFjJuURrfOoNniCcIdDiPpXxFfwkFfSfFsKnRrwWNbPHhpBkuSsUKbBRcCQFfiIqsSjJGgkKHhDdDddsScCIiXxawWAYAaylhHLDUwNnBbWNqQvVvCcVneEubBTtePpIicCEhGgRrfFgGsVVUuvPLEeNnlpPpoOvnNNnSHChHcdDDQqUubgGBdxXzZwWRhDWwKkdyYXdDxVvHxXZzxXwLlsSsSZzpnNPqWwQgGQqsSELlezzZCuhQjrRJqEEeekxKkXKHtnrsPpxXeAaqQvVVvEShzZKkZzHSsRHhYvVyigxXFlLftTKTtYyONnVvokWuUwLlhHYnNgGPoyYvVVvqQhHOpxLgGlsSXKjJkZwWqjJfMmzZyYmMFzcCyuUYTEetEGgrRZCcRrzKJjTtZzqlSsLQJjDdTtCaAMmCccqQEeoOrsgGSRQqlLdFfYNnyDFrvVNnVvrRRDTtqQdFfORrfFoXsSsSxPpVItTicCnNVvgDkKdGDLlbrwWRBLlShEeHWwtTxXRrXxCyYnGgNcsVvBbhGSsgDNJjnEkKeuUnLlNcKTLlrHhRtiIGgNyYCcbBnGzZEegqQmJjuhsSHUMsSsCKkcSjJaFfEeAaAEesSJqQVvjMuABbYTthHWwhHyaUwWDdTtGgnNRrDqQdNwVKkvwWOogHhGWntwztTLlbmTHhtMBZgGWdqgGQGgCcoKkOsSvVraEeDgGeEMmdGEAaegAJjRCrhHeEZxXLlzvdDVRZziIciITtPHQPpqRrhZzpPpLlFfDoMmNeEyaAYnsSOjJjJyYyYWwlIiPZzZzXxpfRrFzZmjNnnNuUbBgGJMLqQeEPvVTtJrRrnNYzZyocCTtORjrsSRyXxYtTuUPtTtTohFfyYHOYLAalfWTwWtDsSvVdRrAawWdmwWaAPPpbOoBmNnMpIvVxuUXvVNnDiIdXxAaKRrJjhHgGkHnOoFfNhuUVvyYtTazgsJhHjATtbBafFZzIifFfjgGuUJnNfmMFXxcFfCFpmWUuwMzZiIhwyYQqWBbBbwWBbBRyYrmHhuUrRhHxGCdDcgoOlLsSXMNOonsdMmDuUDeEdeESsWcCnNwQGCcIHhAQGHwNWwnwWYcJVvjCyqQWhxbBhgpFfPGHXXxEemMeEnpPNZzVvAagZzqmMWwoOyYBbwKkpPWNnuMmeEUuaAoOUHhHoOTtfaAUbBTtgjJqQxXGLlgOLloBbEeXxXxaAinNzEeDZzdZwzZWYyLlZzhHFfRrPpVuUvpPWwyYJjIiNnGHhZzNoOFMfFmfjJnKkVEOhHHJjhHFfhYyXMmGFfBbgbBxRIiRTtrrFLlfoBbbBVvvyYSYyEeEFfXxdDHhEvVgGmMeetTKknNWwQNnqjJkKTfFOOooZztSsBbpEeDdsnGgNhGgHZdDzSRCsSHhcOoFfEsSuUjcjyYGgWsSwKksdDbBSAaDKkMVvgGmdssSoOtTLfZzRrvVtTcMmCLlfFDMmdiIwWFYMmLuJjUhHTtPpGgyIxXinYyClLcGgGgxTmMDLldtdDxXXqQRKWwkoOCXzLlZxRriIxXrRjJHTQqtheExXcCkJjKPFfpyezjJCcZEhHCAacGgzZqMxacnNCaIVvitTtcCTGSsSstzZTpPDdCiqQcCIcBbdDanGgNAMmApPOoPFOlLoiIfFuUhmMyBbaAZzxXPpLPplbHhdLlyYIifrgGJFzZiIfyWsSGgwYaDdAdDEezJjvVHSXxzXxJNnjZHDdhKkvtTiIYPpyQqQqVvlLvVkHXxhtTjJWCcwXxwIUuBbVOikKIoOojJXpZzPoOxvSsiyYXwWTtqhHQTYyTmBbBfFYQmjJsSzZTtTLlMmPptuUuGdDNnRcCoOLcRgpkKkKDdhHbBjsjJSdDAaVvuUJPGqQlLVvPLlnNSpPszZrRpTZzZzvVbBqQtUumMJuPpKgPlLpGkJjiIwWaABDdYyueEaAhHSeWYywStqQTsIFEefEXxcCeiENnyDdUugGVvdDtGjJcCgCcGQqzIOoLhHliAaBbZHhpHhPCCpPcOfFLlqQoPRrpBbwWtTkuUAKkoOaeaAkKEXxGgsiIJlLhHaAYcCyUuNniILlzmtPpFfByyYInNiLlQqYUueGgMmEDMxXmdDdyBbuUAalLaAlLyBnNvVbSKfyFUufYFktxXVmhHMeEjWtVvwfeEFWNnNnsSTHhviIQqcCamaAMIiDdAVXLGgdDyYlUudmMeIiEOorRSslkrRpvzZkKKkVPIioRrGgOKLSiIIiPpGUugcCsUucCqjJQtuomMOdDvVwNiITYySstnOoHTIinCcmMXxZzZzNOotSswAaWaAyYhWKktoZzbBLljkrQqVBHhyYbZzaRrzZAwWvQEeuUqRgGfFKFOsSobBfkrlRrPpmjJYyMOSIiGgsoKjJylHhLYnvVBbNaAkBbQwWRrRCcNnsUnHhLleENgGuUuXuUktTKCTttTslLScaAfFxBmMbWwHdDiIHhqQOohtktTKTXyYbBrjJRaZlLmhCcuRrTtlMGghHjJcvnNVkKvaRrKkAsSHhYBrRnNiyYOoIbgGnNyjRrtTJopPkKfFlLXAjJJUumMacxXmMCqQAfFIixpPwWeuUiIERrQqcLlPpgQqlLIeEivVaALXfFRrtTxDdFsSfNeEnaAEHhPpqQrRKXxkeXxnNBVItTixXYyoHhOrRvNnliHhLlJjImMxXEeFlLPNfFnpTtDddDrIiRascCSADdzZPPpoOIRrNCcnifFUddoOBbDDJyYCcwWjZfFzdDGgdQmMcCAGyYgdDjJaQqQqNnwWKJjHlLhkhHvRSdDoOxEemTtIiRoOrbpPFfBTIitMWwxXEUuXOoCcxwhdDHrRGgXxWvVyVvYLlELlCOjJkedDmMEmsSKkeELqQlPanNvvVVAfSsGRrgFoGgGscCSowQNnEeqWlLZzpfdDRrFeSSssESLlRAaroODdtuFpPfUxXudDUhHrWjJwRhMmLlHpzZlpPnNLIiYyBzvoOVvVeryYRECcZHCcrRhgGQqzZAALlKkOiIRCUufiRXqQIiTvVtMmxLlSWvVwUusiISJRejJEeEOeEHrXxLlrEFbBNnfPTKQCcyYjJVvlCcLJjqbQXxBblwWiIzZLWVvwpKcuUpPCuvVRUuxXrOopnNoxXpPiIiIhHBUuIiXxPpYaTtAyjGgwBbeEyYiEeItPVvPdDGgpDbINnhwWzZdLICcizZbBaAjRvVPpriIZztTAuBiCcaAIEeboDdOKkMmUyYSilkKLJYyjVvJjoOIExsrtlLhHTRDnLlodDeEfrRrRPkKCcpbBHSshFZPpzIBXxKkbKkcCglmMLcCnKkQqqQPkKpXxxWwXSslLaeEuUBbUurDXxdZqnfFNoOiIvVAKuUCczXxsSUGgdFixXIIiRrfzDdZDvaAqEeQxxuHhFfUXNnZRroOqkzZKcxyYyYpPRLlrXCljNnJLTtlVMmvVuUvbvVBqaAJjQcCPdDpUuRfZZzOiIoznpPNFQiTtHhTtIqODzZdVvaUYyCcqSsfFqYyQSsQEeqQKkqLlLlYCcyhHNnNnBbuNBbnUNnjzjJudPpDCDdgqQGcPpfFIbBijtTJOhwxXkKnYyHhqQUujSsUtTWwnpPvVNyYuymMiIYxXsSbgKkMmMmQSAasEVvelLjJkKRrYtCcTyfYyKkjWLlDduflZzLnNRrmMLRrUuwhHWfhHBbFIilUuAaFSzFfeEZdDLlQqsFfIiSJjsPpeESswGgrmbBMjiILWNnwlRrMqhnNHvVQYylLICcimqVNnvZYyzTtnsSNnCsSBboOiIolLiIKlLdGWkWwYyKaAquUEBeEjJKkqQIiuyYUJjkKDdndzZDZzNWwdIiNvViIjJeEnEfFTxXtkKiIeBzJjpPtfFaHhlLtTuUDdTGgPptAruUwWqqQlIiQqEeiIPgGpiEBbtTleiIEUujJaALlZOozdsSiIlmMzZXpPhHxQWkSeEsKeyPRrLlpwWXsKAaAabBNnNBRjJrbDdnCOockxXsLlSASIisGgwZLIixeEEehHXPpEVvQqeAoOlLkKewWzhHApPsSZYyocCBoOnNulLEcCEeeZzBOobupPUlLQqmMQKkqXxHhhHfFxXZwWXwWZbBzSVnAarRIXRFGgfrZzxiXaAxSZzSsXWbBEvVSsewxsfFOUukKobBIYSkKhHsyqqQrNZzTaZLxXZzVUAazcCZGgTtgoORuUrmMaAGpPSqQDdsrRBPpCcjJdZzDmMSsRrYmZzMMTaAJjPpchVvHCPXZZzznNlmoOncCGgkDdkKKGgeeuUaAqYCciIyVUuBbeElPqQpXxLNnxnSsNiJuOoUbBGgidoOWwUzZoOMmPCcpOoLlqQEeuBbUBTlfFLKCcNlLniIIiTunTQubBPpUeQqWwEjzmMsnEFfHrRZaAFBwWNnwCVvTtEkXSsXWwxjJxKDFYyoVvOIivVyYrlLRoOfOSshHbPpZzlOoNngGnbBNKkClZxXxIiekKnGxXlLeEnRrNiIdDHnNmMPpONVWeEwHsZVpPvhHUuWwuUoOsSWwtMlLWHhhHwbBCJjbcCBQiIGgaAPZhGgSsyYHbBpxFfWBbwhCcHDdSszGfuUFRiIrdDieEeVYyvBXxHoOhwWVjJvVmxXpKuUkFfMmKMdDNnmXcCxkPkKjLluxFfXISsiJgGjicDdhWwjzZCcJsqQBbSHrKknNdZzRbBOoZxXzxlLoiIJHhjOoPcChHdOoWwlLDgGXTtxwyYTtinNsSYyFftTjZzJfFAOWsSwolLPNnMmuULLlWwaAVvYybBUtTspPMmHhdDsSSQMmZjJAIyyYhHYJjLlReEeEhHoOdwWZzDFfhXxTHHhhEKGNpPnguFfUjsSuUuUcOfFwozZOeEWWwqcCQZzeEEepPRBbBbaACpDdVvPifFNyYAOoanFfFiIQqfIaqSsQloOLfFOoFfIqVNnvQiTtuLlJjZzCcWwtVvzZpPVvXWwdMmjEeJDKiXxtpPOvVBXxbvcSsjIiJCiIxFKksSoIiOLrQqRxozaAZOjJIdxOlLwziIcCiaPixXjJjkKHhaAxXRrGgLlsQxXKkSsqeaAEkKyYBbxHnNhXbBKsSoOiBDdNnKkbIEZzqQeKXqQxUEeWwHhUPpAahHuOozbBZHqeaAESslLQbMmBzCaAjJRrkKduUcCQkMMmGvhcMmKkSsOBbrRocCfZzrREWwmMoOeogGKkZFjQqmTtMwWEkKSdDEeDvlLVfnNHTtMujoOmMfARrasTtSFKkZzJRrUHJxzZXjJOfFkKwKGMmgkXxGgWlTvVWwhQqHhbHhOWworRdONiImLlVBbzZvlLNRDdriOovVIndWwoKZfNnFzSslLggGGxXDbkAGgaKlWzfFZwphHOrxSEeMmQdYyDttQqoOMsOSyYcOokWwKjAlLaRhHicCvwWVIrzTtcCpPezZjJMmEYXxCxXcKkxXDxXdmMKkFiRreEfFeEIOoDoOdjJMzwWggkKQlLnoOeCcEDFfdSsfrRFrRzyYmtTMPUHdviIxXYyjJVQqDHdDAaArPhHDdlLehHHhDEedFoOujJUfOopPjJGRrgPRvVyyYWfFZlOoLJjzwYsSuLeEjVFfGgvbXxuUjPpjBlLbQqJKQqXxvEeLlVnNtTQTFfxXZRIiBWwuUNcCfQZzqpeAeEacCEYyPlLdaAovIYyZfXNnvVqQOTtmUuFfDdDzZdZIilLYyzpPkmMPpPprRrRiGgIEAzOoZqEWwvVJjememMEmMFfUuiIukKUCcyQSsqHhdNnLVDdvxgGYEpfFtTPNneJjMBboOSsZzcCMKgZoOzGoZHiIHhuUMmXKmMTtxXkIiOAizZRiIPYyTICcIiifFfzZFtNIPpqQinpdxAaYfFeEHhoCcpkTCaXxAMJyAaYtTnNnNjvFmMfVaAKkvVZjJzmMAaJjmltTwWfRmCaVvAaUuASsIisAwWfFaDdGjzwWiIZvVvVMQvSsfxXugGvVUxXOqQoVvhHNGPzZnNpgEeMIifFmaHXgZkrRZzKZzKkzRxXYydDrGfgAdDeEQabCJjLgohKVvkHhCwWUCcuyYiICcIwMdWxnNZzuUXzJjBaAYRryZiatThLlNoFOofOnPpPphHHbBPRrSpPvVYaJjEeWQqiVvIUuwWfFQqUHBTteEtTcFfGDdQqkKgEDdeNnoOCbMmtTjwWMmJOoCPpnMmNcnNeawWAQqURruESlLmTtDZzBbdzZLllOoZzLHhqQaAjYPpxAuQbBvOZzjkrRKJoYyiIfFjSsJfFVqBbRLlRtufHqQhWYhHfFSsQqyYgGiWwIdDdDyqQymbYVvyCcBpuGWRrwgswWyYxlBbmaAPIvHhVipPcgYyiIGJjCkKpfFgGfIipPslsDSMmrRsoBLzXxMmZlGZzLlXaiIjZoOIXKOoWUuMmvVwiIkWNnwijJXvnMvvVaAWwxXBGgGWwGEegvwGnIiyPpYNxHhQqXgwVnNvwllLLvVJZzCcjiWwICcXcuBsSbvKZXxUuzYGQxXqoOUqSsDLlMNnGfNnQqFijdpPTRrpPWpHYyhPaACchHbWwBLqsbxXBeEJnUuNEBbejYyBQqoObbrRMmBVOdBbDUIiSsuqQUbBFrRFSZnmMNjHuKiIkUhFrpjJPpMmsEuuhHvQqftmMsUaAGEpPeEguUlLsSVNnvAZzaAaCfVxXvEeUuFxadDnNurbRrBwWbWMMHhZzNnTGgMmDlLdpYBbyoOCmMcikCcKaABbYcCzZBbyIsSvVEaAOojKaAEeBbQqkoEeOAawUudDTWwSsoLWTtJjwltTicCMmIZzqQZmMYygRrGTtngeaAhCcGgHEGniIvtTVhoOxXuJqQoOoKWseWdDgMFfdsSIiTtHhETtlLjQqrTtLlfFoOgGRkKOyYDmMJjPzZUuHhYCcyLRqgYyGQXLvvzvtTldPiItTvJjKqQqEdLMmyYlDyoOxXYubBVvUwWtTzZRTXxQqdIiZzDDdwJLleMmTtbBISwWoZzWwOoOHhzQqZHhvtThJjXxCzZcXRxXrnNuULlpPcJjnNNbBTtnSsTYyHDPAacCArNcbBCaAiNvVdDpHhdhHKkLlIixCyLsEZIizdkKDvVbcKkNUojJOpPPpofFEeIOeFfTtEOooicHgGVvhoOeEZxXzoOQcqBbQpPCZzYTtvnjIizZUzZvVxPpyYTuUtmhHnzZvlAdDxPpduUXxXxDdDJYlvVLlsiIKkLljFfJrRvkKceECgJejFfJPRrrRpHBbBbxOsWSscWwKkAaQeUeELpPCaAzZclrmMqQxXBbFfVGgVvpPyYvIizlLZXIiZzzZeBbWKkwneENFrROjJoRmPpMXxrCcqQiltTNUunLjvlLVnhHNaATtVZyIzZiBbpjKkHhJjCSscRYnNFfUiqQIHsSafFAhvErRXRgGpusSUULlyHhaAYnwWNCERrjJEeecaAQFYylLfbBOkHhddDYAapPiIaAyiFfIyYWqTKDlLiIcPaAsSBoObhNNnntZNOorRnTaWFfcpPSYyUGYygaAlkKLiIKkkKGgXxNnqjJqpPpcCrkYyKnNbBVvViibYWwiIvVxSAamMsmMSoOsNnyYkKGgxsPpZzstTSTHZWwzdNnAaDhrmMRmqQLsMmWQqwJjjNnpPpnJOojLlaDdNqxdDXiZzIVeNnNnAmMaAtHhTMmaApPaEvQHhTtZIiTtzmyRexXEdzLcChJrzEemdwaATtVbmMBYyrgIMulUcCIiuqMOoJjiIwWxXnNsSqQMgfFGNCwwWWcbGgtPpEeaoOALlTBUuhHISsCcZYyBJjjGguIiqQkKXpKNnkcqQCPcBQAsIqQNnLXBdDbjGgxzZzZXQyYhXHbcCOVeRrEEexXEAaeTtThEWvVWsIcfFCiTzZtJNnjymtTMAmKhHypPMmqQYkEGgkUuKZmhMmubBUaAAaqQprSsRPPuUpxDdXiPGgbEwWMmdDeEKkDdbRuNnUaZzlLZzTUutIiGgMmDKkAhiIhHaiIARrHyYlMmLYyadMmMbRdDKGGMmcAGgbmKkMUudgGbBjFfJgDdGJjUuUuNyYDdnmMmINniOZzzZIiYSsyeEYnNizdDAaZIUuytTmMfeENNVvnnZzsUuSGUuglLKGgCQqckMmEPpeZnjNniItTJSXNyYZzAanlLkKQdjJDqLoOKxrRXklsoODdCwWjUoOgGKkTtTbBtMjFfaEHhGHeEkgGPHhmMeEHEuMmaABbUpPGgHYKkCcoOiIMmVjJlLIiTwUuIizZHhdkTtcCYvxCcXSaAarxFfXYhHQqaWwLHXUuKXxdLIiZzzPpZoaAiEvVeAMmaCJbBMGgyXYIiyPsGgEbBcEeJxmsSyYQPJIijaAYLsSWkKAuUiIWwpIjwWTtbDgGdtTvqQVBZpJfFjPQLZGgzlHzZhCcqJuUjrTDFYTtmaAMWwyUuSsFgZzaKIikUvcrRhHCkHMEeGgkKcFfkyyRrgjJnNGlLYqeEQgVvGiICcYKiyizZuvVfNnTPptJjypPigfcColIilLupPzLlZUjJYhqQcFfCwWDiIsdqQVhHagGjZzOEeOOoCSsccCmMoRRazwZzWkwWTnNHSsFOLQqlnZPpzNrOoRoqdDQqPQqpjJQfZNntTqQZzMdDmHkKSbBZzABbkkVfFiLlIipmRrVvTGgzZzZxXEIwEUGguDoOMnwWcuUwWtXxxMmXHnNhHbRXxrLlwWbBsSsXzPppebKkBvbBUoOOolbNnBSseZDdhHKkgXhHjJAwRlEeWPXEeUIuUiMAammMgwWYyGIWiIwPuKkZhEeHePHheeiZbuZzVvwdDxyYVYzZWwCcJjxwAnCSvkKVODuBAabTMmjuPpamMAAaPHhpMvVpbIqkEeKQkGkKgwiIWIMmIizZWsScCOuUBWSsgGJhHjWftTFwRHhSsUcCucUuCrEeCxXcVFfXjiIjnNGghHpYyfqQFaAPJYnAYVMmbWweEsKyHwHhWGTtQQmMFfuUqnaaXxhWwrRiDjJuvpcBbpyYuUfiCcIvkVfNnPVooOfFMYcCcRrkRYyVVYydJFQqZXucfmMRHhrprFfvVzbBuQqbskRRrrWoOYBoOtCnxXeEcCzZNnsQXxqpUuPtTDuUAadcCjuUjJJqnNMcDMMLlvVlLmtcFfAaiIvVriYuURrsSWiIAaNOonJjyhHFfjJJSOgAIiaVvmzOowMmoRXNnCuUczgGUuJlLPrHhwqbRmFfDdrPtIfFEegjLnNUulpDjtzMmZeEQclLnNHweawWksrwiIWEIitqQTazcyuUIIiaAiwyvNnVcQqCsSHdDhYSDdZzsWzqQZpIHhmMESsgGWROokkCYVgzZGvmMyVvNnZoOzDdoEeFJjfOoJxXjxXxyYXOJQqEogGfFbBOmsShMPpHOfQqhAdnNnEeMSsmFNlhHdvVDLtSBbsBhkKHbGTVvyXAaxYoOPpnNDdPTtpPRrPsSJDdcCfFjoOphHHxXjJCIPpXdEekkPpjJKZzLlHpfFBarRACSscCtTBZzRrygGYbUuUucXjJxSjJTfyYgGwWwPoxFfEexKPxKkwWXpcrCLHdyEeYoOiRrANnaIvfVvUsSTtAacCuXLIilGJBqQpkKPdUufLlFMoOHhSVvsmyFffIiHyYSskJyDfApitTIzZoOdDGyYgyYXtTxXhHxXvUFfuVvksCaAaLlqdDQAYXRrpuUPxaAfwWFBkbgGwWIilLsEQqJjUUrlLwWRUIioOuOIixrRWwcCXaoqqDrRdQxXRrzZiOFfomJjMAadDWrPkKZNzBnNbBbZUQJJDgGdjjBbMmqVvPpDdMmcIGgtXxjhuUQpFfcCAhHMmSsotTSsQqIMmiOoJjbjJXxXSFvVfbMWuUjJFfzZoNnPpOlLlLPpleELGgPpVvLvcfFVvcCCUuZmMnNzWweEvTXxtuUHpPCtTcCOocJHlLWyYGgzSSdDsEbGgzZcCBETtpfAaFscUuCffqQFzPueFfuUEIyqQnbEMSsOoEejFqIiQfJGgYytTjDhsSyYxskNnKSXUuWwPpMmBnNbjJSrwWiQqIRspPGuTOlLlwtTtTiLlYzkKZPpQqyIZzZwWzPBbeEpHEGaAkKguUiIfiIpPLqbOwTLHPpQqOohbBfWwFPiKQqkvVXzZEeqQfzFAGZpVjPqdDQxXSssSnNzZeUuckYdDWdpsuUtTSWwORIiVEeBbXXxmMBWwZvVjAvHhVxGggSYykCcKkoAaOKsYyswizAaZUuYBQqZwWrRGgzjJLyyYAaEeBbhsATtaShbBHQwWZzeFJQUEeuqgJjceRrmMEpPAEeyHrPpRATtHhsTVvGgyrnNzZRWNnwZIMFBbfoOKndyYlKPpxXWwEZzVXxvIilYyGLlofxXqQNnmMxXYUuyJjFPpWBHhAdXkKfxXuZzVssAhLlNgGbBuUnwWcWYywCnaAeEeIiENUzZfxXMmNnFuLlLStiGgdDIuUUMegWwhwWBbusNnSNgUuilLNkKYyviwATtPFfJjpaNupLqQUuKPpklCcxvVMmXqBbQPNneWwEUxXnCwWwcCIiWiSrFvXxBFfhHQaAAgGBbrRAaKkbxXJjBaAPpGgaGBbmYCFMmfZRraaAUutTVNnwbrUZzUgfnNPrwWDdUueEFuUussQVOovzqQZzXxZtgdeEjgjWbBGgwJIxXLGgldrnNRmaOosSnZzNyBSTtQHhbBSPpNnsdDyYAaSvVRhaARAzgGazZLwXZzxWCclbBIiaANnWGTtXittwWUEelLYyTlLNddQqtTJjyPpqGlJdWRrtTUuwVoOyYBIDdYybxWLTkKeAaTFYyfnlLTbwWBbpPLuUlvYrRyQqOvDdzaAZUEeuzHhoqQGWwASsUuaxHeEHLVXToOgZzGSvVPuHcmMEehGgNzpPZKkIoOoUuKkuuUeEyTUuRrtYxXdDaVqQiNyvVroORiIOFfYjJyFVAaLkKRrmMuUrzZBbCcLlGXxoeESsknxXNuUDdSsAanNOsSIioGgMiOfKuQpuXxUUuKkfzCbgGBYyAaedjKkSbFaAmMrBbRfBsLlQSVEeHyYtAsBbeOoUueEELpPPIijpPMmNsqdDsSQSiIBbUuOZzoCPvjJuujJvuUVhHXxMmHhKkStTEaAcCHyYkEeaAXqkSfFSNVvrLTfqQlgGLlxoNdxHAaRWweEXxeRrdsSkQqZzOdDgGoLfFYujSsJnNMmboOgyBbGnNoegBbQncfTtgLLjJfKgeEGxXVlAhCSsczxXXxLlTSstTMbgGHhDsDdqQTtRrCcLphcCHnxItTaIMmmhHMMEerRmLlkpPYpPRbSsvVfdDFWIRkvVKuyGtQqtTnNEeTlLvrqQRvGgVyuUUkgGwWZJjzFffLDfZwWwRrWwIYhHyjKXxkJipPzuAaPpdJjDlTtLhSsMSshAadnGgNDzwWZxKkqQnUuwLlEeEfkxXlHhyYnmjJvVYUGaAdDeEhqQFfvqYyQqiIQhHVmFKkJjxXfaAfpKJdDuckmMYICYyLlquHAJLlEeNnbkSNnWwQNnnNqsoOrbxXBoafigCYPpTobThHNnDduUGgtAacCyqgrRctTnNlcCLRAkuIlsSjJLlXmMeExJoOjuKZzkUgGtTTDdvPaApVtLljQqJuUJuUkpKkkKCeENnKkxmMGeEgXckKgCVpPvqFdVvvVpPktVIfgGkXxpDSscENUuaEbRgEeJjKGWwyWOMlEvLcClVdmMDFiIkzZKzXxZtcCbBZzdkKIVvkHhuNnVHhvcCKJsSweNnhHXxDdLlwamAaSAarRsqfFXGTtgxBIPCcpBjQqJbMhUukgzZGKHWtTwYyXEUXxuEZlwWLCcGgsrROoSHhzngQqmMjJGJdDHhovVTtOwTlXENCnNcLlzZoOnzISFfsMNfFnHHLFfkKlhRMmIiQqQqYmFfqQvVMynkoeoOqPTtLpTvVtrJjnNRssYnNdDkWwDzIkOohKJBbjWVvUoOtTuwWZJswBXxbHGguLlGzdGqUuVYMWwrcQqFfRwEebBNnpQqDtTLKmMkLlWYVblLLlYyKaxsBbsSecCEybFfhHBvVfaAFKkYwiVVPpSRqJjNnwfFbSdXbzUuwWhHLEVSxlLsSZnNkKNnGSrRYNnsSvFaAiIOqnWtTKIiNnjhHrRKJVvjWwifKUukcgYyUplFfLDQLlqcCSAaaAfaAFDdsuUEqiIZhKdDkKxXFSsfxVvPocCOKnssuUvVdDhHbBaAmJwHiIExoFfdfvVFgGDqGgEezZEcidDIHeESsmkKuUZLgLlGwvbwbYyBWNHiPpKkWqQyJsIinKkNBDLlMmGXLlCdzZmMeEdDfsSVWGkKPpMmLlAgGsSLIihHGgLluQlLkKqQqzkWwjfFSszZJjThWwwJFKkfgGDdFftpEeMmjxCcXBbdDMXxOnMmWwHhSVeEZGAaHhlLgnhHzvXEYtZMmzruUKkVictbBBmMOosSbTvPpPpEEhIZdDLlFfViIeNJCjJcgGpPZzsaDdlLASQKKMLLXExYyYyFfvVImqWkKLqJqQKGIiCcgyzZmBTMmtqQuUFKGZiPTtLliIKkGuXqQxXxXiIXxxMqQyOwWZiIcmFUQqxrjMmJkKOrRoVFohHeEQqSseECcrRKkOfKmvVTCctMQcyYCvIiIiMmGgKbBkwWjSmMGXxTcCaAwfFcHTtdDhWYySsBbZfAOoKkavVFfFkjJgRrGiYyzGZuYyfyAVjJvaYFUhrpkXMLldDmyoOYnBbNSsWQqsCUuYycHhtTNDdlLoBbGgnNnnNNehWDdLcxXWFGTxXtABAasSbaZYeEyRmNzZDyEoOjJSTtrRsHhyYLlVtTIDhbbBYyxyYHkBbyLNnfpyvydRrvUlLuJjuyYSRrRIFfZPiNnjsSqHhZzQOoJIFfBPdDpyYbbSstPSJuUCulLYybWLVvyiXFfxlLIqQOyYyNnrRYLlozvVpAfFZzZzmzZDdMTyYipvVPLlCcfmLXQqjJAabqQRwMgVYnNjlLYlLflLeEYoWUNnrRuwdEYyeTtCdUkrRqeExXQXxQapPslTtLRDtTdaZziIbjlaAZfFzJjIzaSAaUjWwJcAairRaZJjNfVvMzZmvVGIidDrXxcrgOhSsLlQGgqZzHXqRrhOogGfpPdwHheccCCgHhaAaLlGbBBRrbGFfHGanNnkKgGdDnXUuOqQoAaFGeEAaAjJPYiFkDDlLuMhHWyLlKkJsShHmwpDeEiZzITtqFjKTWSsGbEyYeYDRPEKkKMmMmbBwLeGgGXbmQFWTtwEerRfqGgaPpwQqCcWvxsSXcCWGgEEeLNPpxfcGfFgsSnNnNCbBuDIqtQqusBbSUbBRXxEerWdDEUnOlrZEyvVmBuHhxDdQUkKhHuUuMmdMmrRDqYaAyePknNKlQfFXxbOqLlQbBVbENIiJgzKEeKkpbfFBcyFFfkUuKfUfSHmMSIdwjSemiQkKMDKwWZzpSxXsWVmMeEWyYoqDaAJsmKkuloqjRTLHPhHpgGubBRrFmMBxUupPpqQmMnNlZzDiULlxmnZjWwxXJenNpPEziIPUuEepbWUpxXPaAuCeEiIwWcbBHhfFaAJziIqQKtTkZDCcdXxQqlLhJjoaizYydDZeOoEXxwWqQNdDbBGaAwWgoXxlhMtSsTyPEepWrRwYMURFpvlLVPuFfFUgGuQqfpPXxVvwWAaLaUgGXRfFoIdjJdBbDDhHaHkKSxHNTAixXUykKYlLuLJXxBEmMuUOoqTtQefLyYbBlUowWnFnNKhHkSsGLbBlMmgLlpTleJApABHkWwoEFYywWRvVthHTOFfDioKoOCcTtNWkQIUQquhcCqQHPHhcmhHZeEScDgPpSDdIiLCcjXxJWcClLhbSssSJjpPBoZzebBkdjnNaDNyOoYHbGLlgGgBXxfIiFmrMpnHTtHxCcXcgQxkaAoOveEValLAEeKAbBGyYgVGgHDyPDZrYfFfjzVPsSpvTtZYrSvVOmtVVvvTWwiLlXxRkKrmjWwWHhWvxXkEeKtTvxpPTMUwWFaBbFOlLIGlHJGgSHDdhsjWwhqQZmMsuUDsSdtJjTNHVvjoORYijBpsSRrbrigrRAaehHgcjZzJMtKEeHDKBeEbYybmMFbQqKAakcbOutnNTUonOQqGKkbBgRqQUHVvjPpJhItTiWgGWwnoOcyYfFsNwJjwsSWWJKkxcfFdYKkyfFseExXkrtTFfyoOsadDASEeoOfWySteApmyYzjduRyYbxeEjJTLwbBWGgHhEuCcPpHrcCFdnHfFhnNLlgIiGNvVGgTkKFftlWTaWquWwALRrRwWCQqyQqAqbthERbTtKHhvVkUomCcMmMKFVbBSDdLlqAZzbBeULlqQKjZWjblaGgcPIipdDurRUBbMmVPZouaJjXxUoepPIgHOoYTnyQquUeQqocQtPpuUTqpPCcCbBtTRqQCOXnldIVvirWwaoOfFHgxXSwmXadncEeRrDHvVuLGEefFUjNnrYyRrzfiIUWoUmMufoOpMmNyYSEeyMmrRUMmHIuUueEnNocZzfFGoOfFocXCcHhmMhYyHWwckKacFshRjLBbaIiDdreiPLlpDduUdjJFfpnQqjNuQqrRGpAjJiIPXgGQfFTtqaABbaIVvBnRrzZZhkKClrRRPIiOyYsSJjOophHRmMFfYyOTtHrkJjvvsSVziIvIiTRfFrKLlkvVZjMJtTxTtqUumMHqcZdDzCwlLMcidRBbcCddjJLpiPpkBtkDTbBIqjJNjNjHhJDXdUckKmtZUNkKXxnMmuMbBVoAqaADzyTJpcBEFGOodDQqiIpfmMFPYymPpDQqUuYydMkKWwaAHqHjbXYTtYyyxlDgIwWKkNnJjPNxXnpwWvEeVYnvVNzZnNwISszLlJxIFMVvPparRZzZTVvGkSsKYhHHDlLtXVvXxNvbGzZRAaRcCrivwWphHaAAaHSshRrXRrxTSSpaxXAJjAszPpZFuVkNOeGMmgXSzZDApatTOCcKeUuNnZzodDkKOfMvXYyIixvHhVVbGgGgsSXKkVFHhJfFcClNxXQtGgjNnwsScCYBbyPcsNNnBbPVchlsCczEerqzcnNCoOCyYFPRrpWwRruxXUMmlCrXxReEcLQqdDAbIiTtfiIaAFSPGGggUuDsSPffiCNncwwpPaGiIjDdIgGiWwantTOLlcCoNdRuvgGVUQAOomsSMcCDdLsSjBXvGgVbBxkKBNFffHhpMmuUdBNnzbBOwWnNRtTrbyYBsqQSuVvUqeFhHJVMbxXzZiIPXFfXIixxrbBHsSvqQxJjmMNeEqlLTrEyYCcXxUQqkYyKoqVvpPzvbBvVjJEeVHGocCOghmuFTtdQTrRXxthauAaPpsNBTtbnKoTKksSYytOvVwWuZzvVwSIVguaUugGAUFfNlIWwFhGvVNwrRGgPpEeiSsaGhUvNnhzJalLKXsZztTUzmHhMbIiOoXEeMNJvAaOpPGgoMAlLsfFDVvnRqjaAJNnDnJkjJxXUSMozqBOjuxXUJfFnMOKkoqNQqxWCYKMeEVxXeEOzZoyYnrRNamQqsvVlfFLQqBzzoXzZxOLdlQqrROecCtbDoOMGvFJOoeoOPUuHFfrGmPOkKWKebBEZzOacoBACTLKkRrlgOEeYyxBbcXxIiVJjzZjJEFIuBbrZzPpRxbvZnNgMmqrRsSzZWwWwKoRqAaUdLdbnbQqFfnvVvKFaARnlLlGfwSkKsrkKTGPgGJUuQqjiDdILRrVvlNVvnlsSHhNpBfCqKGRrKNURrivxGUfRrNnFsSGXxpWUCYwKSeNFfdlLGZzgMSsSNwWwnNnNrnzWgjJeqQEppPgdDwOAaCxXgGNnIitTXxoOFfoWaANWAgGRrbbBBGggSRocQfyYeoaAhcKkOeESSxXslLCcKimiLDCoOPpBWkNFfnNFEyPpPoONBKDdqQVKkLBwWvVBbHivVIPpJGgehHzZeEdDyYWwGPpkDnIMYYgdDsSPlmOeEkKMTtPpTyAaYrRbBilFfCcLIcXxCnNGgaZRrxXzmAwWYyAogGITtiJjHxjJXHHabqpPbaXxIGFrOGgMncGlRvVrbbzHhZYyBFddDIKkwWVggcGCcGByktQiIqsMujqQLOmMglUqQuZCoSsOYhHrcCTtCcgGRIoOGeshHaBbApVOoCcREerZfFteRgDTNKksSiyYDdIntdJGnSspCSscGgiIPwIjlmxOSUuOGgJLljpcTtRFnpPzYRrNnzGPrLlRvVxTtyobBzZiIJjEbHjJhgYeZURgZzvVsCcTtBJoqOogKfFJAKkMTtdHhPJjkuUPiIauRrUlvVaVFFrNNnKoOokyzZPpySsjwWhqtBIVvrbCdDtSWwsTVIpXNoqhvitTJPpOopMmXxlLpxRrOnXNnwWxoOrUuQqpPQqCdFtuiHhqAIVeRQquwWUWJjwEpRrPEepVvqxXQaAwiIZFfQqLdAGgapZzOoORrJjVDplLeEgcGgRlcCzFfpIiGQqAeJiIXrmHiLgGIivDBbdEehjJquMmPpodDzTtZJuUGbiTtgGDdlonlLNUAHUxyYPRxXBqQbDUGWwgubFTyYrQMdIkZzwNniLnNlIJjWtTULluArSshtSxXojMGcDlLBYySMoOokKvdHwIqvVJjwKVnNCcPlLtgDTMkrYqpPQpomMFfsByjJYbeEsrRpCeEniJjmMbKkKgwWyyYTtYiITCcdtCcPQqNNvoOVdbBkZXEexjJzKKeLlomUmMLcqvVtgGERrDUuRBjZVhhigGlLbBGgJqdCPZzpqQPYEBMWmkKnXbBzZxpCclyYBIibkKjCmCcPpSqQqQSsGzJhHJjtTmhLYaAPiBshHSEFbBDToWhCNkIPpiHyYYAafigNnjKHhvmwDWtToaYyAOwpzHhZPRrbBLnYpPOodpCcJsiISViIyXyYzVtuUOIioRfFVrTtRvPHhpITvtFriIrqLVvlIRrigvBwSzadGKmYEVMmmxXpPatlLTsSgCcGmfFqIiPJjpQMmPeEdjJDpMYEeQAahwWiDdLlPONpPkvgGwjFosRKkraAdDSdZnYpTiItGtTyUJjuCcYeLlgGnNRtjJgUmNrMmuURajUYoiXuUmgAKXBqkKvVmMywWRrRfFUuIrmGQqoBkpqhgbBWwrBbOacCYyAofIvVkKxXLqQDdsxlsVvQqyYSaALEeyEkKEGgfFMrhHzZGcocCVvGQbBIiDxrRrRCcVAahfiITtOonjrxSSKOoRrTtfPupPUgjHsWwmMSXxWLBWuUIiwvcCXxVNnMVIivdJjNnxBvwKbwLRrlWiIvmMVVtOHBHMrRmhxSseBGSLDdlNcsSsQzZtTpPwWFbCiIKkmPpAdkdrROTNjjLJtNIlbRHIERrorROePTtpJoOfZBYyXoMMqSaVvAszwWZQfFjJEezZTtjyYfSMdqmRdDTHAauwWMlLmHmNnPplLSeHhrmMctaFQqfFpVIpkChUmXCczZEDMmyVvtaUqQrMmRumRolLvVZZzoNOFEefIeEhyYWeEgGwWwkmMKsSNqyveEQqQqSsNnuDWwdXKKEIvVEemMVUHtBbTRRGrhHaARjJgraAnNrtWGmMZqQWHhEeZzlMGgJeafFAEgGFfStjJYyeHjzSsdnRwWhAzvHhFexfFXIiEwQqgvFssBGiIWTLlkOsCrjJGnNPpVXBKiIqerrKkKyMOFCmMNxYyoQCSmgSHhlcPJOoQCpPqQiFpPfaiIgGANbBXxSmKzTNglqQLGMmWwnNRrIvHSsbqQZCexrtzdcTiLlGgSshpPMGDdUAFjyrRYoUsXxSRhjJkKtTzFfZdvVDwuUWVXvaAeEgMmGVoOQVkLXxlCcIbBrRiLHXYylLfFGLaAkZzKtqQDYydIiFfTtAbnntThHhmJntkLlQhNLUoVgPpzIaAwmMwILXdDxhEtUuNyPLVdbBDgGvVQoXCcxuUqApqQurLupXxaAfUXXDpPfGnVlgLlFBeMoweEHhgYbtsgGUutTPJjpPenJjQgiptIOokbaAcpPxSxXsrwWbKkHfFGZzDdgiTtIifFeESgGsgPagGASsLlVvowjnNnVlLjJrRvGRGjJtfuJFfjbUnNuBXxIXSOocCkjkjLAAaayPpCcHhvdDfrdUbBBcCcKqhWPpwFfDiIYyeSsEFfqhBGQiQDdqIqBVfHhCglwWRPybBDrLSzmMWIiKEQjxEeOoSsxXbVnNDdboOBqCvRKCsCQiIqKkcScLlDjmYyMInPsNzLvNLCIeYhKkHWFutbBNnwWHIVcCqQVljQfFeNnApjPlzZLjJmMpUjeHlfnNyzpKwbBfMzZctjcCIiKkyRCEOoyDJSHhSsVhvlLrmJjKkMRxPpwoOsSCEXOxXcCAaOtToptmMrRTuCPorPpUuVXNnCPpmvrRTWwtVPFCrSKkHhAaNkgGdDxXvNnvVvsGgPdWKkwYIiNzQqmkjoOBbgZzkLlKGgRhXxcCXjJxrxvhHVXRZzeElrRXmMWqxJuvKNljZOJjoPfXkxXfKMmadJjpaALDdMmOxXeEUuoOQcCqpMmqXxQKkwWDtAarLNwWndMfooOROorqhHYygtTGzTbBxZLoWOJiWwxXIjxXWwzmMkEeyJMmteEcWuQQqidDpPkKBPpecFfhqFlrRCNRpPRmMrHzZhkyeRZzgHAahNTtEezdaADkKrRTCcXpGgOoPhsWjASHnNZzTiIUuYXssdDMtTKDdyMmbBVEYyHZzheLoOUHzZdVhuQaAOcNkPoYXxrmMFbbUutTEeSNZzHhPfqNnOfFoGmMgRFxXcCFVponmMTWdXMyYyOSnSWwllEdcCTkKtHUTtEAaJOBeESrTtxTkKoGwWJVSspGgCAnNdRCWtTwDQqdWQvAHAaIzCcoHUTtcBxXiVvkKwRHhWcuUCwtVQqqfFSOzORrlSsLoZhtGsSjJAEuUeaWRyeEYQyfFjfgydDXxpbBPqXkfFUdDovmlWzZKkyrRYmMXYQGgOrRMmqpxeEYKvWjxXYyNfAGcCdUFZcCWmrMReEnNraKuWwLyZzcoOJmMkpHYAJjnNRjJrFfXNkbBKQqKBBbNbByYhHqHhUuKkQgmMGLaqxYytOdDdRrXxYyDoWyEWEeBrnNhJjMqNzPKkxkIyTtoMCmMcQqJrkKRjmOYbBBbHrRKlxfRnKYlvVlJjNnLNwWHNCcoOqhPEBaYyuUjTArRMmfhHcPKvtmhQqpPHMCrWruUbqEVQqvaKkpmeAyjJYaEPpVGgLlTtvmvVBiIkdDFQYSUVvKkuFfsLvVpPlyXTHhmgGMtKiQqGdDgepPBVvbpPmMLYnTtgAVzYWhVPYwWRoFnMhFfGgGgSUuiIlVnkKvlKoBBbbHhCXPOdzIsSeEZtTSxXuUXxXxzcIxFfFsSfUuTBbcQMmGGgOxXoVvoOOpZsAaBpoOPyjBZzjJbeWUCOARngilyvVVjEuwvVHKxZzkKUtefFefOZCmNNnsEevoTOpdDPzbZzkRCEepPlNyHhZmfrXbXSOoZeEmVvyefIPpKFkKfiIbgdfvVCcnAawWAraARIQqJBpOowPpVnWzkhHXPpYhyYNqgGWQFveEYPWaJQtTtTdDyqQDocRbsNTtnSdgjjlhtDcwWCdNmeEMpIMLMkrPtIiTPppRnhkmXZzfFISscCtTwZzQlLRrHGgiIMYqgGDnNdgnxYyXUzYIWsNpNnPLAasSkKcyYCEeJODoZeaAEvsdZGfDvVjUzSXxlRrLRlEazZPvJjIiYdvreWvYypvhSsjJUuwWjJiiaRtTrbBAouCciqSPzCcZpUWwzgXQNLMkGgKmbBSRrtTHXzsjkAaLXvVlLnLFfrjlJqmnVvgZIJJjRKJjkMXFfrzSAWSJjNDdSsKDhHXxPpGgXPeaAkdDDnueuhHQoPCvVWgGwginCVvCoodKyGCnZAayYTtWwzYypgQqGVvPYErFfMsBbkbbsmlMmLCcMoluGcNOLlmMMmjsIDaAAzsZRNFfndYQqgtqpapPWnNwZBfFOyYgxXGbGxXTDaFfdqLleTixXbBrGgIikMjZzzAferRhHUuyRrfFHIpPifupPUGYywwWHZtPCcHtTOlVqQqOZGtKfNuEKUxDMNnjmrxsSXuaAUUXELAHlDdLtjJjJKkuUbBNPBbDIAaYMmxXmaPppPAMsqQrJvRjJjxQvxqQXhYXxOafZEeECoBbrcCRJjJIiQqjOrRqULlxJVPpBRrjZeKOhytTdBotTmMAyYHhTMmRRrriItDdkiIIiCUAaPpqQpiHhyYSIMssDJjSwWKAOEOoAaBIxXsEeZmMdCclbVLoEpVvcKUukCeEhHUFlPmIEeiVvBbRKVfZzQFFKQqryiLlExPWxrRgGvVUXMmWPpmMwafFAQVvgGqvVMmxucCcsSLOAauUwesKHFffuUScKGxgGxQthfFCIiQOoQgGhHOUNnbFpecHyKJSjJUmMuNnnOMdDdrAyNzUWADdQCcqacFpPdDFvMiegkDULtpLwuALluryYKkjJRlQqJjLmdDvuUcsJTOnIiablBRSHZrrJjQqZzlLRiIRBbSiIsGYZXxZZzTtDWXAaxwrFfkjJkKohmMgGOBWiOluULiAaiEjJtTyYhNnqQMPkKrRSoViGYXxyiIFfcCgsSkKuUDWoptTUThhCQsYLlzuUljZzKkOovjJOUDwWdtFaDPSWZzzFfzcCZAwQqUnrBgGEXLlmMPpDdAzmVvKvSsFbBGyYgfpPqZFBBblTqWwGTQLlCcqtdcCJGctgxrRJjlPMCCccDdXxKmXxiRcCQqmIyYajgLdDBxUCQqcmrRcXxSswWohHOkuWVeNKLExXnNIvViWYQSeynNtDDytFMOhavJjQqawWAlLKkVPpsSvZNlvVgGPlJbOoBPpbJInVvtpcQnNWebTydzZdIhdDHVYARRrrHpYUumMaABOoCbIdtCTIhVhHsSpdgGbIWGOqLCcbzFfSsgWwFfjvVXcHaSsklBbcueEkQvVbBjjJHhdQTtsSmQqCsSPFxUXTcwWClgikKIiEkkJEKaBbAkwWMzIRIhqhgRBbdDVvULXAuUaxVYyDKiIyYeEEGgjJjpdiHhFTtfcPpizCBckKxXFfuhJjUJGggrRmuUsobvcTtvJWwxXeOosOdDoxdDExHhHNcEvggAwrAasasSABitXEdmbBEeJjZnbIaoyYOGQymMayYPpbvVRduYYyGoOjCBhVwlwmAqQaMxUXEeucXxbTtPpBbcQqWpPwWMmrVvHTtGIicaafEFvVZzvFwZwCTfwDeXxDoDwBbgGWKFqbBQfltTtTbGgPTsSjUHwrrwfkuWAgOWwjvVFidjbYqfFpTZOnhztiOoXwWXqyxkQHhVKkqoqQfFftfbBbEeRmMzLlTXyYxwWRriUUDfVtTEvVKkRFFhYmCcOovVMmdOoOoRrBoBmMmKuUccrfFNXxenNtKkTEOgGoMXnwizTtLGglwWNdFEcHhGIdDrQbGQyYSHhshoPWsxunxsSZxEGpVuxXglLJeEjGEYmQumatEUPBVmMgGJHUuhqdDPrXxpPpPPHhrDdShXMmDtUMmsSgGqNQJjTgSLyJjKKSskWYWtgGTRfFHjJmMaIiAkKixCkqadegVdNcNXCcZPktFuYyvCcoOsFDAEteGDdXxDKkIXGpgXnyWbtvMmrNnRBrHhuMPoefFEevYyaoeJjCbBQgmSsDdocEEqWwtTxoLUFZzzXtCesShHPpaTvleBIwWoqRrQialJaIiAbHMTzZEethHuBWnZSsofyZrQUDgrRHhGUgJCBoRrRoqGNOSsdAHocRtNqlbOoBzJcxFLlfAaWoObKlggXSloyYadyYtTUjoOnqOZiCIgpPGMAOodrrJYMdCTpPtdABbZowWKdIFNnfiuRQNjJtTdDJkmMZYygGVvlKSjJsSskFXdPyYmfBSQqsxXOosKvrZoQPbuUwwLlSUKCcKgvDdSWUuyYvVwIxlhJjHlEZzaAaIiKpPtCcTLlLCclkkfDdeEsmMjwcDNbPDdDdpBTteqQEnoOdRrClfxOtTIehgueEDQEeTbTCrRaAHaZAFPpfwWxXvHhtTVMfeuUjqAaQTaAqQIILUaQvVjJXyYCcmMaPQqAzdbBAELlbEbUtGIvVCUpPupSgWRcDdlLUZzucOoCavLbcaVKmIiMkgZeEzxOokFvxXVfwWMmKOVvFfPdDpIkSbIoZzYyRrOTQqQqblLkKRPxTRoDLlqtiIiQqIsCczPMjMIczkNnKyWwXxKhHwZzEQAaqpqWtUnebEeVAfxuNjJgwiHADdqcPQqpMmucGWpQnNFcSYnYyLmRQzZqrMmJPpjMMoOHhJbBOaAcImlnWwRrqQicSenBXWCrZzAYjIRCcCcrVviIlxXFmMuUwWBSTyFfMmXxvhSsHDnSsQVvqnsRRrrSemFhhMfFluUreEfuajbOSDVFYVFWwDYytQGgJvZsSxXVOsFJFfBqbBPpvZEAaeCbBuzZUjtMjYyJXSHpNQxXqwUulLmcCTnKFOTYgTkuUabWiWwRnNeIVlLRdWpGEwMzdaFOonkEcRvVvbBrbEeBRbBoSlLsPpOoOVrCeKNfADNnZmWegPwUuDrviQqErQqOUuyoOYoxFfRrXfKkFIwBAKNnwWtGytofkNgGtMUuWnPUzZuhsxmTJrRzZcqQzVQbGSTtsTtgGgjfSovVUuvLlxXwWzguUGVjqTTtyYdfvyfvdsoBJAUAaFXjJxRuUwWQqLmHrRHfMENNdVYVzRrZkRMmrGSZzsgQqVvKFfvtsbfLiJyaRjpPJcwxbNEsCINQEerMmVvRqsSnNpPLMXxiCovVRQqrAuUQqaFfqdlLDQjvVmlNysCfqPwgGdDgCUMmCQLlahIWGzZaAnUXFavBENubyqQYBTOhnNHIirRowrRQPeZzWkrRYZCimJxXmpZSSsTQXxLlyYdOrtXpNyYnHhrBuUhHtxOoXoOiBsKgQqWwGsSbTtSsBijJwWoXJjyYGeEOovLlACQqBlVACrwGsPVvcZUuzVvkKigTuBeBeaDfFpPZtTapAAamMDdJjqQpPxBbqAulsSAaiYyiTteELloOPptixXIeUuEJnRrNEFmSsfFnNazALlhrRctBtqdGmMgdTtDUGHEidDoXeEFLYyWJSvAaVFKAeLhHLXirRgoOWwGsVGkkuhHsWWBIipVJjvqOzRrRVkSTtSlLWwsbFaAMpDxfLzvVKjnqrUIxxXXiDXAaxkOzaLvVlDdDchHDmyjREeAaRDOobBayXeEuLlUbBUuxYmicIzoQlLSsNJuZzDAPpOLsxGGLkBwvcCVAaXCjZcCLaAQnTrCLlYyOhaDonVvyEeYgiIQOWwrGgObcjGuduqRBbzoOYFOzNwbUXkKxEemhBjLAIOibEEeLVtASsEcTxZJjYyWwdZzDfulOXhHtTQeeCOwmMWMGqiIcEkKOAVEOnNpmULlRoObVTBaAXxwYGgNBbxGPgBbjJxidgFfEZzztTZTeaMbBmdfKkSVUfTKpzNKknaAoOxsSpPwWnCRrnDvPpGEXxDAQKcXIDdhrwywffFFYykYlsGtqnQZzpPgGuTdMmxHsRpRrUuMmRpJjQjOoFfvtTbpueTAMGgUqWiIUuwMyevWwVxXUvPgeXzXNUMmXScLlzZCwpOHqWwQqgkIicCgwWDdGKBqRigWwCefDNnnZIWNxZzmZpPznvIiSsVXxRzWUuwZCgGYyCkMbOMmbDMyHffrevFduuXxItAaXxZHhrBFTFfcCFOQvqPpKXIVviYNnQxUuXiIxxITZHNoztPZzQyBJDIiIfIiJoGauUwUWwKFWRRWhuJtpOoXxBLkxXdOdEdWKkFtjJcJjWzWvVfVfeFAACvVRrghRwCrZzRrRGgZBbzBCURrOoxuXQqTTttWbaABLqQWvqQbBHbceEJgyIhtTcCHTtiUDrBAiWwaiIAIYqOogAiZzBNVHhvOozSsMDexTIUubFfrRSRWaGorbjvqQVJBROVvGVeCnhXeXSMmEjVTtUuCVBOSuUMlLGjuZzHUeECbcZIOoCMmlLvSsVIDPJekdvHhhHuUlurGHFfqQMmQHiriZuyYBwWbBbUmejKTtKeIgGhHGLtxufFXfprRclLMZzHhPpqDJqKMtTmFfUzZCLbjLlJSpPsBlLKSPpsAhDdCtTxXxHDdhAaJpnNPGZvVBlQogwiBtTDPveEHitcTuCjJcUDiBcbgZzoOTQNCcoOnqYyttTGyPhPpHhawWyviDZzDvVYtBEvVwqCPTNijIMcCmiBjLpLnzlLVAHTtomfTFfYddTYEsqHzZhywuUelLQqlMmknpPEvwZzUaAKoOCMuXblIiGJWXxJjwAitPpTEeMrIMkfWwTtFmpYyLxXTtmMXGTCgjCcDqQAausSUgQtLcCbfzQsaASrRmMPpVOokMZapPxedDbRNuWatTvVZeEwspdAoOfJjTzZuXxoVSshHZhHzJLZySqcHSsHtuPOwdIoOvOsLlxXpmEgGaaAhjJgFfUuUuGHAeHvWwPptAaBbTVfFeIIoIwbDdoPpiIHOYyKRFfdzJjzyqQgYyTtkvVKzhsvVrbLBAWwwUufFWNotjfFUuSCVMajJAkjJKIiqQbBAaQqOoUatTUWMmXxcClPwWiITlkbBKudKGEIYymVfQqAJvVjafCNnwFfuZnYaDdRDmoVvNvVsjkKskYUuhCEPyxXYIWwiffFBpPuoqDdqcfFFEjJOoSseUufTtHTqXXgFKkfsSkCsFhjJkeESEWAQdDqBMmbaolCOoQkKuUGgqXwpXHheIYRYyVvkffqYqyYBbQOoqQPpypRrPFvkKkrMpLcTtQqCfuZzPeOlEevBLEaEIieKkAeDzSiUubeoavVksdTtZzSJjSmisCcIPucjJOoKLIuUilwWjJaObDBbzZlLYHotTkEzJbqQDdYyvqQzZjXBbhHuQcezFrRAqQoyHYylLdDxoOXVKklLqXJXxHCdMmOoDchrVjRSyiIidPAappwWneEpPoOzZcHaAhCThalexzZkKjJZzJjuSslLsSRMJBbmdXukeUnFuUxXQrRqkTaAgzOSsPpoiIoQvLbBohpzZnNTzhWMmgrRFhkKZzDdYEFaGgtTZJWwxXmKRItEQDAdjJtBbgBobIizAPQTGyDrzqQSZbBadiSJxXoFpAFfiIaPfnCgULrROSlLBBKStTmRjJeyNqQcgYpPxtTzZXiIkDbaABlCcKkLOOqQccdDNIGcBbWwLlmMpOqUEeEssSSDdAaUNdKyYaAEIipxdkStTsnWwswasYyDdZxXPpyYgGRNnxoOmtTaArjizGNMQBbjQqLiIJRlNhHvVUuxlKJPpmMSrRZxhslnlLqiIxGZbBOousQIUudDUIiOIIHVPnNVwrRERVWXxwDdDGgyVpAFfeQqLrsZsSuJQqdFgRrztTDSVzOGgdojMnNNnmlkKnmMSwiDdyQqZuNGQymCcCchqZYyNtTnzWiLlxMaAkKKHEdGgBbuAaUDeNVvKmzRrZyYujJwXxMmWUEelmicCkKnNiIUtwWTuPBbtTHWwjJFqQfhnmMTHhHLJJGDxXPpMmBrCOdrRWwVvnNYoOkUSsuKqaSsAjAkKJjwpyVfqqQKkwQtTnHyxKZLlRrwNvWPaAoOmMbjwWilLaCcVvhHcCNZzWwtTFDdDGBxXkiFYyEYMRFfwWrzsxuUBxRFMzYUunvaABbSsVLcroOKZzpPYyBZotOVxXSHhpcCPnMcRWwzZdDrzoFEiIeEErRQqTukKXkhWUeJvYLUuIGNraocpPuBbwEjJJuUOfFoYDdbSzPlLopPgNnqCBsSbVvtXiCZsziZDiIopmSsMxcOkLkKVNvIiLsHJjmCcNqQfmMOrypvHwyZvaGNylAuUaxXEIilLIkxYcCyqfLlKxXIiMmOjJobhHTtMMPAeQBRMmwRcTVkpCFqQatNnJAbepHQnhDlLAadnlkKLlLRCcrLylLkNrFeEXLnXxNkjkKJhiPpKXpZiInQmHRPpbBihHIbtTweUuYXxwufsSFUTXQApPpsSPlqQxsSXnbfFkmnNMnxRrRrIiayhPKjCqQYlTAatUCCcpPckjJAmRVvMwLlzKWwkQqfFfuDgaFrRntTJwRrFfVkyXMmBbWwkKPQXxrRBboqyxmMWwwLqQMIiVvVOuKxLlQYGFJgGYqxXrwOogSOuULKklosTHokKsQvTrWiIIfNnFbCuhOZiuUQqhaUuOoVqKkyYwcrDnNGgacCcPvMuUvVmsSRrjNngOtXOuUoRsFfbojeuhhHDeWwLLfFsNsoKkYmzZxDwtNOPvffrKkQhHFFuUfpoOoOtTtTNnnsIiyzZYBBffFCqQgGcFfRyhHOpKnCoqUHvDhulvsgGiIoOSQqYkSsmSFfLltTSLlxyyYtRwWiIaArhQqsaJwSHTtxtUuvAaVvVdDXxZngGzZGrEYKrPpncAaLfQHCExXDdbIqUFfwCTjYMmNnESsIieKZRrgGJjiEeNNnnIowOlzXQqtZQSsOFmDkWwKRrlRJjFfuUSiIsbBTdPQgGcCqoXgmMGxEeAaloOPDAKkkFKxFUupzYyrRJlLLnkVUjXQwxMmrRLXyYxIiTtlLHrRraAGJuxwGgWXUKDdMrkKRZnyeEOoNnDpCcPiRruUICcpSOMmoVVTtUuKaAnsRjCcJcgGfFcCfpMcxvuUROpchHglLGjJUMrRmPmMHhOooxmMecWduUBbDcCXYNnyPsSpVHvqTtQsjdYdDmMecrLlmMwWYJTCmFWSsaAkPeEZYFLZzhEJuPvVkKpJpPPaEqJLlLkKvvlCcLihTnNUfwbBUuyEtTiclnVlZnXxSnNpNfFiJdbBkrVhHxwWZzXcQvBSsGgvVgGXJqekFfrBoObjJRqQwZswWlyUuYIihHRdYpvVvVrZzIiSsLGcFTtRrvAfFabQqVvgbHQdHPpFnNfQkZzCbSsuOoDQqeEMmRFVWwYlJtWwTKJKsWsYySwiIxymBbMYiXxRruUYyBbAaBbUeEFkKRPprTqQiIgrgSUuZzsNGgJWOLljUubBJpbRrIiBjJGIhBlLtTRfFXCBKAaiTPIGqUJjlLuNrSRrsRkLlKEjJpIiWwmMSTByGHhWOmEbpPfGLtxXTvNgFdxxEeyLlYuFJHhjrRQqNnSrRsPUlRHhGgUSsPaQmzZLlaAzNgGnZMoOSsOqKkvlpYcCnhHTebBHJjlCciWAaCcVvWiZGvnNOulnHqdDHhKTNjMHtTKkNNBaTWqQIiVvwSsyYlgvVxhlKvqwWSsrRxFfCcjJRrvHrnrRdDxHvVhnNXNbBuOJfaugmaAHdDZzPmMpItCcXxCDZTRdDXEmMczBhAaVgGJjintZkMsyYhHXxUunIqQcqLcCwWljXhHxpCLHhGgsGMsuUcquUfFCvVcOXmMqQncfbBEsSeomYkXxRRqQEQiIkbiIxvNngRcSoKtwgMmbSSfVjJfFGWfVOoZaHrZzNDZJhETsSsjmRrxXdqQDLwzZFfzDdAaOogwZzHhThuviIVgGviekkxiIUVYQVvnHionwWOzPpJjOTtrMsSATYdeORroxMuHcKPQRrqiUcCKkuvPfATCVvRQkKqECcstTMtTgGwWFfhbBmaAHhnNMbBUrRwWGkKgGghmMtrOoMQLlDmsiIYVvyYyjxXJFJgGSrRslLmmCadDAKkcIiOxYybzFwOoWjTkKtsSixEeXhVvruZzUZvVzBLinTjOosSFflnNJJntoDKLlmMDaMmMLlceEBbBRrfiHhItHhTqSjJCnoOsgbNnEXbnNhXxoTtTvBkWVbXDxXmtLlJvVjTeEgGVYyvblwMmhJucCUAaEeGpFJjkssMtTmQqXsXxSRJcCNNnLlFHvrRXdIUuiFfnNqDdgVGgvPpOCDdguUnzZoOPbBpNPpRjJnNTtmeeoOYZzQqXSsShPpHliFZzRGHQATtaPKbOgMRiJjrDdYEeQbeEYyxeYyEkawxXHhWGMiImMxISNnsOoOpAasSHhPyIilQqLuJvHAahVArRaAnfFMuGTrEgPyNzDzZOfJWVvVKnopIHqVvJjymGgMAxXMveyMdDkvVgDGgAZsZzWbVGMmQRRfTVIiWwtirTvZmMzZxmMYvLljCvVcPZzDyNjJlTtQqwWeEMmdWMhHVkJGIFyhtTKLlncHwkKOtdbBtTqQfqQebgGIpOoyuUlHpvDLZzldVKkGgmMPMjPpZglLsQqMcJUuOoRrnNLPhzZHXxAaiIXxNMwmbeToOtwCmLlMcWyprTthRNngGrHRcDQPrRpLANAtTanEealjmMITtHCcHvYyzJcCGGggNnFfbrZzdovVnNOeTTaAtQCXxlqIiQuMOEkkKcCDnnpTDnNAatGkBgGINcPSSXxmuUMOIihHIiPqNnQyRHhKiSsImtdQqGTGglLkKpvkHhWQiqQQqqQEeWhDVfFOmsOIiobdyYCgmJOsTHRaYyKFfiyYPTtiIpDmczZCWwqRYybBtfBiIdrpXRrunoUuONhauOnNLYyKVvsSkJjTtoOIGgBgjFfOUQHPNnzZpxXVlkKzZIhcCMRxjJaAjEatTRJjrapPAgPZLhHrCGeEPdHzZhvlLYyAaHfFhoPnNDrRlwWzrgGRWqlLQPeseESpPqQcCrdDOoEvyYyYnNveEVXxIiiWwaQbBIQqUTfDcQqRoONoXVgGkKitTIvPPfwWFjeEIYyVHQoOOuUnxPivqKkqwWQeEQcBRibTBbQHPpJYNnYKOJjkwWpPnIiyYRffvjJALAptTKOopDmuUajCckGQOjrRbyYyYPpSGruhHzEyGSsPpBrRekKOEeYXpgZyCcZQqNfrCPwpPrRPpXxWoiIsBboXMLJYyXxiWGmIiGgMWwoOgNcCguUjGrETUuXxzuUaAJjHhvPBbSEgaAdDyYfBbFiFfylhHLczLGJjNkKnoPplWwLlJLlUmXxSTKrRzZYbggCDdGGviDJjUukKfqQBLgGDdgVvHhgGCNmcCoRmMfgAaiABQBXxBbbBdDAWThHEeqVvQtwhhhORdDraFfaMAYytmoMxXLpGdDyymiNdKgqQNnGgfsSWwFEqQjhblvVvaAAakbnpkKnkKNYmMeGgGgXxfnKwwWdqQDbbBfFaATzNfFDdJjnZtcdUOoulIaAMkKIksogGCMlLmHOZzGnNgEPpsSFqCFfOrsvVGsSawnwWwMaJjAmOasSdDWwxXAcoOoWdrRvVvJjVDQqGPGAaDdwfFZNPoOpwWRQDdqWsSkKnsmDnciEeICEDdsZzRrkWycIjJiUumMuLlwPcCgxXJjvVyYugXVCcvLwWlvTtVOoVIunkgsSpPLlkQJjcVyYvFOobPCcnLoOHhpgtRgGrRuenNEUWCcFkKgXxLKuUkKkNbkKHzZhWzZwBbBkKGMuUmIigrYyMmUufbBQdDqkeEokKcCVMmvZzuUOVNBNBDDdlDuQrOoEeOTjTHhtJtpPdDknxXNQIbBiGIiKkzVBoOBJjLlOoaAMmbthHTXrnNRUsSifevCXYyoLlGFfaAtcbBapPbZzOybBVvYvVCAoyYkwopbBKAaWwkMgRAPpaKkhOVvoHhpHhLlEtTjfVgmDoOddBsSTEoLDlWwZZdDbNhHnbBSZzpPXZzxMLlAjJEeOovmhHlLMmnNlLkCLlLiIlcYyCcycMmwHhXnQmNDdvVoQqgGbQZOeEonNOmsuKeRDdrEjNCcdQGZqQzgaANtTLldSamVKkjnmyYWwxBmMFfZSsuSyYxklLAjZHVumMPpHgAIWnghHHUufWdNaAYyLlnDwcCiDdLnGvisWUkSUAHquUVvNeEayYAnDNnfiIUMZdZzDQOcCupPvNuUnBbVJjOoeRtQjJKXxknXVvbhHBVhRpBmRrWwvjfEuUUuQsSjJoZZzFfbGggGDkKOopPPAapzZPpPmMFTWwtsXxSnbqQiIHhfFdDyYbJlhHaDIidiIqrDnNAIiJgRrGcCgAWWIFFpdGgtuUTtTpsTtqlLQqQtaAEeEeTBaGgnNJDdRrjCVvcfcWwZrRyYQRKWwkZSLyOoYoOrRUuHEeiICvXpPxpnSCgGpQqWJTqAagGnbBeELjfvbBxEeBmVDdvpPFCcpPEkoiIAQqnNPadsxKsSkZzEomMNnnKvURrfSaPsstjGgKknTtIGgiNTvVtJCcUuZzPVTthHuUIrgVvBVnxTdhPpZzWwygtzAQqmfiXjTLltzZcCZiXxWyTCctFtTnNfFfiGdLNSYysnpPBbXIiiIxBCcJhQLlhgGgGGggfebCPjtYZHhVvHlLhdFflLDdGgEeQaOvjJmFfzTVvMCuDxhHdnsuUSOoJnvVrRQitdKTbuUKnrRlbBLNIWwicCPfFpCcmMIPlpPOoDdbBbwWBDDoOwaAOoWrDiIyVvYzZICmWQhQTvYyVtXjmJzPplLKVvrRSmMsGgkaAtXWwxLlVFfZVMmyYTuUtKRhSSslLsuUNnDdyYDdudDXxHhLTtlUotTrZzOopPPOoophHrLuUNndDcHzQsSqNbqQiAxpGgacCPxXgNnSsUyYXxyQkKqYncCJQqNPHhDIETtCcRAlJjAaJrmMHRrSgGgGfCwWAgGCxyYDdmMtTDdCOfFgCOxiIXXwWxUihurRUuYIisnPiIVvFOwMmuFvVZRSbBsJuggGJKkjlIiUuUuUWwvVvsSVhWwdCYJjyNDyYUuAxjJMWzZjJsEeJjQoJfFjOCcqNKknUuGhAkvVwsSWKRDeEbBOoLNrRxoPppPcrmMvVFfhHOElFfLCcYgGOoaGgoOAtTNGTuUtZzgtyAaBbhLDpPdlGyYFfwWqQiEOuArRUOtTzVvIiXxpGgvIiCNnEeAxXLLlBtTjJJpnNPwzJkrvVpPRuDdIiIoOiEusSUoEeOktTKaQsvwMmWfkiIOsSuHtThzZYyTtBreHTBlyYLQaxXYjCcJcrkKlafUunNFUrIiiIRpPcCqQATiItaQIikKwAtUuWwwLlLFfDxXfZzVvRhUQnNqeGgjJifFUuIesSEYyLllBBbbtXBEeHhNnrUDJZGgMPaEUuTsYRrwFYOGgowWRKShYycCjJHeEDCXjnSBIipPblLCNgGwInNiDdBbuwWGgKksSXxroNVIMgGmIiivBCBfBZzNdDnYywWkdhkfFTnNmkKCGkgGKEdDGIRNnBbBsSPbJIiIyrJhnnNZzwWSXxzLgirlLRXxnNobgGBfAfuaEfFeAIiXGgxmaAtRrRrvYyVFfXeEVVCcwwJfFEeUGgzZuUkKOouMRrVPGggGpmMEefFvmMSsIMosAaRrRyJRrFyRUuzdpYIiXIixaAdhvaXkKqGCzZhwWVFfvQqhNPmUuRbZzrRVvEePpjJBMCBbcxFfUuqQXSvVoQqOshSvVBjJbsnDATtaddAJDiIeEVvKXzZxQqszZSESLlsDdOHLlwdHBZzbhDDdleEKksGdCsKkzMCpiMlSHhsKuUkLmqKwnMmkOxzZAaXbBIdorDdmMfeOQJjqKgGDCccYyNnCwTtWdZVvoOzhbwSsWaPaSsjJxXSwWzZsjocCOCcEeCcEFftTYyYykKLlLtPqQfNxqQXOuffTtFEeFcCQqFbjlIWwatnIihXEeraARHhGlnNbBLbBrRdDgaAshuUABbiOYyrxuAMSsmnNmMLllzZUfrurRmmHbFfBLOcCnZzIAOHIijHhwBSDPpdseEIiNMSsKkbBeEXuxXIdLoOQqPSsXbMmfUhIiltrTnNtlIQqiLJQIiOLUMfFSjSsdQOwrRzZgGWwvwbBLlhHPkdmqIZzMUuiSiIHhHhsIhHWeEwEXxsKkRBbrJWDSkKdDUusiQqshsFuIikyYKYCPkZprRkKeEPGjJjneBsSvmMoOoBqLpEXxXSDBbdsUbMYezRLopPNxXuewTmMQidUcCcCSsZzFXPpnlemMwSsVeaAEAQwWeEqMBGgxgElCcrFfEeRWSskelLhHKkCcprGgMmxXdyBgNnwtDdkdYyDJfQdPWMcCjYqQHhwmAJTtvVjaaxXAhHkKURrddqQJjKfqQIyQqpaghZzqboOHhBQHtTCcfxNYyNHhNneBbEGglLAlLghsSgBbVVvzZOiInNkKkeEEeKHhYUuyKkooOvVvgABbGEWDFHQxXxQEeEeqoKyYkGRCRaAUvVzZKkhHvVufFdDgFnwWzeEAIEesSsSCuAasAZjJzUUuuZiBbLJByYZzwWArxXTVvtSPpAErReqKueEDcDOyFnrRlLkfFrRxXKNyJyvGfFmWrSsVvBxXQsSqCcxeElMFItDdaPZyEeYCcCcYldDwZtTzBvVUcjyYUyYufFVvXxspUuTBpzirsUVDYVPprRbBojJSsoOOYyYPFlYKdDIihKCckcCXFfbBBymMDdYHdwWivejJYddDMmfFnpFfPMrzgfwClwHERrOlmMLjJnFfSwORrMmoxYyKfFUuPRHskKNYynSzgZtVvTIGwWgKCczwCWtgsJIFfiFfVqkPpvBbErReEeRXYycCCcmMgGjdDkOoQqKJufMCIizojJwWYgGYyzZmlLUrRgSsjJGlLpPgqQpnNIzgIiDdkfbMUuCcYkjdDQlwQMiqQXexlwWhHluYyUbBoOmzZkkqZzWwjnEvziWwvVHeQqeMmLlyYaANWtTbBjJZzwnVCIvRTyGgkKbBSseYyyYxUuVCcSsZBbdDvVNxXzvsfFNomMmXxYyJPcCToORrtIiJjLDdlKkTjJWtTwjWXxZzuUHxXtPpnNJKZAacCzjJZBbUgdDGMmlbBnMoOmNaRrgwvbBSHhsVvFDcxFZzfCcuUgkKdDAadbvVGoOgpPSjyYYqohHOQwIhnwWBrRVPpWlzMPoOpZzzZBUubhCnNlLvjJVeyYzCcZQOXJjeWwlTVvlLUuYyXxtLhWjMSSRrNkpLlXkHHZzhzfFxXQeJVaWzZwARBbrvHhjdPuGlLPpCFxXIIikTtyYYydnNDJkwOoNQgGofJjVwtRrTCcWewWIiEncCiINyFfbnBbNBjJsgzlfFLXrRPpgGGgrSsRsvelZBCXxjJczuUNnZxXxrRDFfsSsBWQrsIKAaNvSsVnkivvIWnNvVSXYlLykKAkUuAaIiBvyGlLgwldPGHhgWrbBCRbBeEFzZfVvgpPrsyYSRwLlpPWMmarRAiIGZzmyvQgDRrZgUhBIibWlLSjzPpWwwkJjHKitTZdqQnWwuuUUyYNKySSuUPlprRQEOyYjJKhHNrTthmiInfFNiZQCcglLCcqQtTOoGdDqBlLbBbexLJjtWRMmrjwmMyYWHhgGNeVxEeXXxhHPpvXxejJQqKkxmizZjJqQbWwTtieEIecCUHhuESsQFfXxzZFfMAWIiEWjkVvmMuUUJjKikhHKDTwWfeLmowYgXxFAaBbfkGrBeMmAaAnzZeojJVvOCWwdLlhHPlLdDEVvoOeKFiCcvTUuKDfQcOBbcCoEeOoqQGPKMmjaALiUzaAZKqQarhigGIHvUuiIVCczZVeEvlJyYjLCGQGaupPKbBkUAgbBYbcCeEeEBBOtycGIPpFAORTtookKfFObBOKBjlLahUNnaaAASSTIitsJjKksQciyKCUdeEDEeFbvxXVBfdpPDLlhHWwjkmwWcCMPqzZQFMHgQqLIiUuluyMNLKFyYYnNyeWQtTqNXHlPpvVLmdqFfQDHJjfEkKmRvVrMiIeFcCVvUFKkfZWzFdUulFfFKwSsWuYSsDWwtTdbHhBVgYUrTtfFqQfEnNeFUTtuikgGKUufFlTtLwBiIEeryKoOiKkSscCAiFfWwkYSsUAauyKrRXNOofFPlnNTbBtSdKkPpBmtZTtTtHaLvkFWSswllGFCxhaAHXNKkqUuRnNrGEOgvVdpuUPnNDYGBLlhHXxUXxQqylKoODErGghXDneEODeEdXfFGgKkLhHFWujJNnUKkwtbBlfFRnssKQQnuUNpZzPqxmMKhesUUnYkKyNVpzZyYjJcUunuUJdDpPmMBbYymMplTbBcgqQGsSiICiSsItZziIrfFRSamMTyYhveEsqFfZTtzJDVvEcCccCZFJjuUPqUlLkFoTtLlIRrtTmqQKWwsSIKkvVitDdTOgRnNFlLQqflDdvfokKYJpPjnNnIvqQARAapPIXxMmPpiqUuQrUloOLpPOlLzndDNZRrigGnHLlvaAVuUCdDtThtrRjJaAVvBbJjCcTnNUrRpstOoxvlChHchTWwthAaXsyYBbXxSHhpYyPMmgfFDdOzaAZyphHPYDdZzZVoQqCcDdVxXBlLCcfFnNtfFNeExXzZtCcZzEtlBhHUubZzwQTtqXjJnNDdBULlukpPKiOobvWwDOZzhzZHWwojEeoVMmvOLZzBbWiLleEIWwiIwKksSgQYDIiDlLPbBpntuTTrRIxgoOrRwAHhZtTarIiHQnNqrsqsQqbKkzZYAVvNkKIinszZSDIidMDeEiuUIiTnNtGJDGSsBbTqSSUAafxXxXRpFGugGGguaARDdBNnPpEeWvXxAzcJjyMgDdDoOaAdIiqbqQgGVVvCcfRuUsIcoOWHhEuYyUeIVnIJjGbBEenUNnHNnGEQqxkCMmcKXmuTsrRRrlXozZwVvQqWPpOouUOxZznNvVpPXUuxXxHeEBbcJjLlCvVaSSvUFxeEDmMaQqbwOgLSsSseTtOovTtVqQkLzZDSsNKOokkFbBjgGJYyqQfmieEzyPpYuUfFTWwIUuitYSsLltpPPJjpSbBJjLladDherRESsYaGgsSMmpPCGBbjfQqxXuUEAaqHvVdDYlGgmMbyIWSGmMXaJAaBUubzbxvcCrWwoPDwnNuUtTyKCKkyYEjJbBpCcJviIPzghHXxhPpHawWfsDdiIsSSmMBbqQxJkKbBjuzZSsUnNXTtZFzZAaxIuUWwhHTCctpeEZzAalfFmgNnGMtCcMCcWwmWozZEeBQlFehIiTtmMjoOJdDTtSApPaDkKaAHQqhdNnPpsWLotwWUjJAZziIeEaAahVvHGgbBJRrjgfCcwWDTtdFHIidcKkbBCcoOUuCNmMmLlMnFqQYyfJeECmVeEIivMcvVmjJeuUBgGtDdCbBcZzfVvVvFTUudDNYHbkKBhwWiUGUuFsSfgpZFsSSPwWLlBQqbQqkWwHDdDdDdoOhHhXxKuUdDeeTtsZwhlLFfaDdAjnNCRrcjJhkKLlViIwWsSVlIeEilYyLwMmIimUtTurRaVvAOoVuUvBFfOoAasvVxzZBfFiIOawWPxXWwWwKmMVrRvoOvVmMXxyYvVynNYkqkKEeHJTioOgGCOjJocCuxXnzppPRUurRwSVvEesIFfQOTtACWwcoWwuSsuQqwQbBqqQyYWSseSBKbycSKVsSLlxPLlaFgGdfFYjBmrJjRMbUuKhFlLmMYVvHpPNnhgGhHDSsSsbfQqFIibyyYXxrRAaYBjKkgwWSzZsIDdiKkERreJdDjaZSszAxCcFVTtldUuDLbBrRDhUupljJLCcPlcRfFkKdDmeEzFfZMCkRrXuUXkKOpWFIieEKktsnNbcCPhgjJzgdDGwWZGKDdDVvxicWwbBhptUuglLTKkqQVvlOFfojJRrnNLnfYyNnwWNDYyRraiIHFoHhhsSShHsmKkuUIioOHMaAesvVSjclLKfFJjnNYyKrwekKijJmMaOoAPHhHhCcYqQCZASseRSCcKrAEWhCqTipHhPzZIJAadYyrzZRrIiRPJGarTtRXxFfAdDKkiTpbBRMrJjDrRsSdBIikKQWYyMmRPppcvLlVPpCblLlLAaYyFsSfuUBnDdLlNpPvVpPfFjsSZKkaVvTtAvVxrOWZMRYyrGoLlWwsjnNYbBwrRiiPtTxXpIEeIYyyIyYRCKKkkTuUmcCpPXxQnNqdCmQSNFfcTqjJFfQZvVzXxDdbOoywkAaZrRNnPpzfFkKYyKKkKSBUTtZRPkKFCJjeEGgWwRrMmUFOofBbxkKhHzYyfiINnlLjDvgGvrJjvJzZjdDVKfFCymdDWWwwOhHvpFeRrEvUucCNnKVFoOsSPEPpeClLPmTtPNnpMVUdmMIHAdDdlLvVDMDdlLmANqWiIwgCchYkHhUuWwSZcCzBBbvyfFaNVvyGrRgfFJqAaQxvwbowNnIsSilLuUJjsSXxcfFCiKoOGgeEiBPHToOthmKkUJjGgJuUtaAUdqQoscCcNasSsSWXLlyvXWjsSJUBzIOoEEoOAaQLlqSSsFfQqGgsjJbTtbBBpCyYcEzVUuvQqXRrOoxUpNlLQqzZjJnmMzZDdjJiWwUuqQqQuxpwLmMQcCFfsXxSoOqCcTtrRrWuUaGgsSxGnNBbNLlnzELsSJjuVEPlCcLvAaIiSsVZzOoZfFrXxRUfHGghFuxFfSnIikKFfNnNHhBYydDhHQqhpIiPHhTCMHWwhmNmGHhcClLgdeDdZzgGWzZOoFftOeEoToOiwWeJjtnNKkEeVvMqQRrPSsOoGgDdIUuiZaAPpzIveEfFKKayYcfFCsIcCBbvaAPpVihqQzFfhQqdDvVaAtekKEKZnNAuUkKrroJlLfKkMmFFiIfAoOvDsSKkNEeeEzZnCcMGgLlmSyYyYdIgGiHyLOeEFwIiyYRrWGOodxXmMXxzZDIYBbFUlIiLIYHZzJjhqQICPphHlLewWLlEmBbQqgGhKHhVexXlLESsuKHhyYkAGAJZzjaLlNnffhHdVvwWtTtDdUumMRzJnwWNsSoOWwiPawlyJFsVvHhShHfjDdOopqMkhHwWKXjBbCegGmMSCcmMpaKTtkAxYHhmjcIeugGUfFEOoOeEfFjwWAaJlIxXJjiDrRksStjrzZReEYYyymMZzJTxaAdDhlhHAXxvVyRAsVyKzZDWXxtlLvaOoxXAQBbWwaAqyhKkeTtaAhpKVvhglLeAJmuJccCSxnAakKhHlLHKkgGhNNnGgCcJtTDdjsNNXxnmMXxMmzFDdoCWwcTtjJvVJjEekKuUMDXxBarRCvVgbBvVgkGGgJjDdgryYzZBmCcpOoPMmDtTnzZNdDhHXHPNnpAahxdArBKkTSstulLCfFRrcBmTtLlMbUetTidDIBpIbhHpPByYHMjaJjAJzeMagOoGYmMSNnXxwBdDbwnNcCeeEAaoOZzHZztxXTtxXFfvnzZNoSsBcCWwLlqQhxHkKUvVuqJxPpXxWuUwliSaoLlOKuUkqYyRKbBAakaAtTtTrVvQqMmbIiCpPxXxhAaHXOYyFfoZvVzxUrRJbrCcRXxzvVidDaAnmiXxImQLUmvVfFiGRvWDtwKfFkWiKkITBqQTMmtbEeoOIiBpPTtSIisbMmMBKkbWwJjglLGPUupNnZtTRjHlCmMcZWwDEPperYUzEeZuoaAOMHhEecDdCgGnvVEeAepsSPEuxXUNaAIiPyYQqKkSsJSZzrRlFfMCctDdSuUGgXZzGgXyBIIIiYygGNnvnNiIYyRwtTWFSsVvfPQgGZzQbZzBHnNJjSshxaAXrRPpusUutTYyCiJjIZSszSswbBvAolLOacAaCVdpPDWaAwFfAaAeEtNgGnUupPzYySsTOXxoTtvQqSsuUhHVHKkHhpLlbBCdLlbBbhHBtTkXlLxFftzlLZRpJjoZzOPrwWwWQvVyYwzZDfFKoAXxaqmMBbxXLBbtTHhIihhHHfFlGgZzjJLlufbBFQqyYPrxYyeyYVuXCzpPZcCcxvVyIirlLJPYKkLlzAavjDdkCcKfTtFJPpgdDGRrJgGDdIfAavVYypeENnSLleEsPlLlGgLyZzYJjExRqQugGuUNhHaiIAnEnNSsqyYyYNnYyCwSJjoXhhHzZKHhkECcksSKjdDLlKDdkGVCcePpvVEFoNnVhHvsSOsSuUfeEMmVvLlNuflLbBFUnSLyjXMmIEeiFnNfAaacCZzLBbtTbBDBzZbdWnNwMzuUZWwtTmhHVNMjJcCrRXRfFruJNQqhHVyeEcCqliILCOunrRHhoOCBeSlhHbBYydDOonNJjHHdDhhmMYMmcnoONXyYDQqSsPbBnInRapdQqhSSsstCJptTlLPsCcSxoOXnNQqLnNljoKkOxwWHnHhNBbnNNnVWwrjJROUuFKkfoRrsiEjJjyoiwWsSjJlLmMIzZBbPpOYVvWZztMmhHgSsGlwWLxXreQnNkhHCgGcbBqyYQVpaADGjaAJMsSgGmbbIibBBtoOTvVlBbLkKlLEeoOjJFfkKVvURruzZBgdDGgjJdDXfFxyYLSgGvVsPpCtTSscSsVZVypPSsYiIQqBbDRrNndNWwnoODEedVBblFfxZwWGCcjtTJgzNnQOoWwqrtTlGgSsphoYyOHdoJsSTtevVMmDmuUoqQOEeLlbBGwqQESTtKTFuUPlLpidDpPIftcCkwkYyOxIiBbiYoOyIXzbBbBJjSsZzZtTBbucCeEUiIjbBhHnNUHRrNkKiIfFGZzgzZUwrRRRrrWuNzLlOiIdDiItWNnqQtTJeyYPtmhMmUDWwdbBSsvtTVgGrZzRkShHqQsnNKDdQqkKrRuFfEiIemMVvHmwwWxoOZzXGgxWwXaPpABRNCcnUPoONnpAiIeEwWXxXcBbyYGPpexKkcCkKoOXguIiSsSIUuirRsSHpPbBhTHupPUAzZahHfAaFhFqQMmVmMgGUmMUpPeXxeEqijJInNKkQSMmPRuUfmMJzFfsffulLoUudDobBOKkTcCtQBbnNqvSQGKkglwtDoOJIfFgmdFfQTtYyufbBhHFeEcJjCfFgFfKktTGgykVUCxEepPFfAayYJiIWwhHzZjNSsnLlNnzOooOZVvWTtWwrDdQMCcmqRWjJWwRrWwCchHWVsSVvHhphHPJjgGMmrtKkkKDHhBbYmMydaAWwzzZzPtTpQqZeEZTRgvRrVQqbVYTtcCAaymWwQqNVDdKXxmMxBbWwKkXCcAakxqQIxizJApxXEePvVzZxoOTtbESgdDGeBatTDOodAgfCzZcFLlbTtOyYdSLHhDdSnNnNFMcCLXSUPMovVOdDYVDdpPvwFUFYyzZGgMmpvRrLlVPfTTiIXxtGOogSsrrUaXyUuvVVvJPpCcMCcsgaAYyxXGkKSshXUuJjFfAaBbGgQSsqxuyYSswAyspjJAZzsQqSsSvVIzjgGJbXxZfFGSsgtTwDOomuGgcaACUWJKkjVvQsSqkKilFfqbBQvVWwWNnTtwLVvwWQQqqcHOWwGlcBwrRWAqnBUubNyvVYahUuqQHGyAaYFxdDLlhAXxnsSHhwDdHhUuWouUXxrROVvQqXDdWgGweExFPpVqAamJaAMlLHhQYyqmUugRrScpPMsSrFLiIjJMRrmmKKkkMATtactiSsIKQqPhHOyUuBbNnhHAaXDDdNnrWwIYyavYyVojJzZeSiIszZExhHhhHrYyRzpPxiIYgGySsXcCOhHnNuuUUTthNnuUHbBdzZDkfFhwzZuUVvWHmyYmyiIWwIIxXdDifFiXFfMOomuUlDdNYynNnDYKbBJqQjkMybBzZBbYfFQwWaFfZzBbeKDzZdiIMqQOooWwPpxoOFziZzVOfFDmMFnbrhHAalLRrztKkqTttTkjJpPJBYyKkJWHhpPpPYTtywllLUHhQqTsSaAtrpRrnNgGECcPpLlzZpRtTaBRPprbiIGAagtTTthyYZzhuiRrIjWKkwJpZNhHqGhHJjcCaAGiIZtJjTUumbBizOoZIftTyZzwWUuMhHDdmaAZzlLrKkRNiIxXnoOZJJqQAajClEvVPpeWwLloOLfIiFsCcoSiInNxXtTEemhIEeeEiPpdDHQbBqTTiIqRrsXRoPLBDdFVvUuWwzZYyfXzEMmeZxosFfSOdkGnNgRrOpPVvyYxOmMOncCNooXDXxHhOhHznqQNZzZCcrRiIXxoVvUunxaAXNnNaApPJjNqQFfnMnoDGGgyYUBbuyYgRrlLCCcdDmkKIipPIiMhHZzWwlBbLclLFwWIMmifBHENnkRrVvKehzZHRrtAcKkeEjJsSeECCgGcaCcLcCDdoAZzaOojfZLlzjJFhHhJjmCcxXOohFiINnKpPqQkDddtTsqQefFJfzBTFfrRtpPbXxuxhHSsXvVRrKkUOFiICBbuUpgGJjwWPHJjIiiCcZzuUvNtnNTWwnVPfFaApIQqCcCcVejJEgmdDoUuOnNKqeEfJjFOKkoDzZcpPAaZiILlMmwWVvKbiIBkUgGuhGxXqDdQNnGgrWdVvDwbBkKJjRuUtTAaNKkngukSskQqHhuHhUSJIpUuAIymMYVvBbacCApPJjhbBHZVvWXxdDtTobFkrRKfBXDiZzvZzVXlfXUuqQVqQoUuIvViTrlLRrRiVvImVNnrRIivMXxIyDdYPpkxiIZzRwWBbrTiIHhDFiXxmMKHhrbBRkIZzfdUQYyqlLoWPMmpWwwsKkPpeGgAaqaAvVQNnLmMlyYQqElLSwWOKksqTtfFwQqWQSAaEPpgGedDxXnNANncroCBbLlZzJTtketSsHDQqdrZziaysSYSyYhHsAazsSqTrRtuFfOonNlpwPpWayYaAzZIWpQqQqsShqQHQtTUuqOXrfFRlaALIirDQqMmRCstHhTSbvVrRBNnBbIfTtFoOUHhmMJHXPpxkGgaAEbBeKSshaAMvmMbkLlKEaAMJjmiIILkKuUlbBgZDdXxXPsWRrwSzpOoFsSfGuUtdDYylLaAwWTaAMmmMNeEpPngqclkKLIruOoYyUrRusbBSjJUTtRKkkKGgFfVvaAHhXxyYimEpPejJTBbzSiIhlLvgGnohRrMzZmgfFEeNjJEXXgGxFfzukKULcLyYBHhoiISsdecWbfIiFfvVJsSjJfFjzheNSRVvrdDsSmJjDdMXbBKKkaAkxpUoqQuUOufFJjZzmNnMoorROomgoOGMZzOMmOTtrAamhHMwWeERLluUjbPpbBBniINCcJHhnOXxoNBbyYhHvaAVbBSsPZJtTqbtiaAIFfTtTWAawSsBBzZbKkFfLlLltAKoOkaNUtkOotbSsdDWwqkKQZzdDtkKTbBIiuOoYymCcMDIWwjIngGNcCHhXvYPpKkyNnlLWwAZzZzxcWVvlLwmMCiIXailLIQkKaLlUuGgpPADdEwWyuUYPpENpPkKMTtEeLxptmWwyrRbuvCgGclzAtgGDdniITtRQinNfFHhrCcKkRhHvVByYbKOQqtTokfFhnNHWwouUGgSsOtvVSsTWCcwkKNnZzZzmMWwWqQKvVFfkoOwNvsuUOoMmzaAZvVmbBGgbQqBQfEeIiFqlLsSqeEfFQMyYGggGoOcCUudDdDxZzzlLUBbbOzAaaZGgThZztTHEtrRTetEbBalIiIizjJIimMQqWRraSxYEDdOoKkwHhqzRsSrvVZLZzgGacgIiGCNnADMmoqoOQPpOdVvrRDLhHxXpPeIfFmMeGgEvVpYyPmvVMQqVvXxBblLuLlSsaACcQqxXUvgGVyYqQHhhHJjLqjJTtQfFUuQYyRZzzpUuPjJEewWKkZWwoAaOYyTwWCcnNhHECceZYMZzonNOoOmqfSsFQDdxtZzTXwWyvAasGgmMSaAnNhHyskapPRrAXxNjmMJVvnKSsfFPpKkSNaAqQnNNnlLsSFTtfFfnCcnNIirgGRYVKbBkEebKkSsDsSuUGgbMmWwVveQsSTtZzNndDiIUuDdoOKcChHkwoOgDefFEkEeJjOVeFfHhEvcCYyRbDdBrcIiNUbBWwuQKtTHhdDkxXxXoOgGhwWHbBJXxRJjbBWNsSnQRrqBbXdxsSXDBbRraAxSvVNDEedlLTtfFfFnWwSssRrURMmrwuOoWwJpPwWjUJQGgqjJoOmMeztTZFfEFcCfFFfAazZqScCsdDINnisSmTtMGBrRJNpPiIWCMiIxXqfFtTGlLkKgQmciJlLjIsSHoUZJVvQrRVBbsSZzvuAojJPpZzsSrsSLbBoOQSEesFfzXVwWbeEBvDdVuZlcCBblWwLLkGgWwauAaUZzFfdYyDnzIiZZzNrRvVXxBUUHhhHuugGbyYtTyYdGgDHIAaihSsQjJUdAaGgoODueERxLlXzhHZiQiIqpPuUrRIDdnNrzCcFFffRXDuUdxXUuSsxANGiErReOAfFanJjNNLldSXWwetTPuaxOoXAUpLlsuBuEefZzFUbrRlLUkKepvVWwPEBWwuUpiIPTtKkOoqmMQbpPpdDWwwWPaoOpPFfJCcOYyogGRJjrlDdDfFFdDkKfHhmMHgGiBTtdmMBbpTVveNnEWJtTUubeEOzcCZzZPYmBAabZzUdDuMfFyUkKQqTtAaUuWwkJjWwPYyGIigqBkYytgSsxXViIwWvXnNEeGgxZzMmxXeEGMmpeRRXIixDWwddSsDyYBbtTmMhmMorjsLlrICcHIihFchHCcrUuoFflLReEragGViLluUAaIvaTtXxwWbQhHnNqCipPIHhDdcSsNvVnvJuUVvjTtVPFAaOkHhKofSPpDrRdPphHsyYJAwWaCsSFfiIAXMmxnNalLVvcSsbBUujbBDbBgGdWwTqyYQshHfFwWfFYTtzrBbRKkZyPoPtTpdyYDhHoOoZzOOOjJgTXxtObokKGgORrBprRMJjKuUocVveEeerRxOxXoEeRrtTFfmrRMXPdDpBpfVCcvmMFPkEyYeKuUbXaAdDTtHQqlLhLlszZrVwWiOoZzDdIPsSlLpwWPpDuDIcCidpacCQqwSAasWAfDdoCcOXZEezkKxLbuOoKkOoUqQYylLlGaTtACpEePBuUbGgBbXjAlLaOoacSsAaClCcBbHhWwLxOHhRVvrhgGkKHpPoWKkYyNnEYyePpxdDXwSTtsOlndDRrNOoLRrWwHhWwTtVCSaAtTsvVyYgGGgmLhsWwSgGQgGBbqJjHUoHhOHHxkKyYXhaNnAHGghHCcrwWRXhHxeZzEWwQqhIiHmMeEhUuMzsrfFRUuSAQqxxXsSSESsetTVvGgtIiTiTtIaAjJbBdZzHhIGgiDryYyYXxqtTcUuMmCAacOoCsrREHheSLrRXxRTttTKCcJOoQqYuUcCyOWwoSswWWYLlneEKkNywVBbvhHfFONnTiIAaJjdDUAfFaznoONZAaPQqrVUEeuQqpPvRpTtTeEVvyYOxXoOoOoOoULsSPDdIsSiplusSyYDFfdDAaxPpwJAaBbvIibBzhyYHIgrRGxXTPptiUuZLcvVCWwlTcGXxbBgUuaANnDLldCcCsLsSlwWYXxnRrNYzZlLrRdxXvVDWiIwSwWsJtTjUVvuFfbsSqQTtGAaPpgTQqNnUcCRruMZCaAcgGFfgqQCcGEfFerRGgAaPpkKOoOojdDkKmEeMyYpviIVnHjJhNPuUSsfFVvSGgrGUuMMmmghHRKZzfFBbGgPbQcyYCqBOZzorRoOnNpgGcWwjpPeEJXxCcofFOXxppQqPmMsSPAQmMFfsShHPpKiIkzZqaAaRrBrRbCcgSNnVvsHCcpPvVKkhTyEeYYvyYVsiIuFYyfYyiInNQqNchHqQCnUbBLdLlgGMmDVvzAafFZMmYypPWwWwleEuKkUWwcCUbSskKsSgzZGKkCcjJXFfRrxAadDnNUjNnHhOorClTKktjJGgbBDdVAavrgcCkKqQsSKSskeEOoPmMpeIxXiEFfoymLlMYVvWwrROkKUsLUuIZgGziNnrRxhHXgGoORrHhnNgqQdDGuUlFfVvZzSMqybSssSwLlWMOovZzdaADVxXhHQqHuNnNuUnUhFftsSdDxRrpcCPgGXtmMxWKIeEiVcIiQuKkUqCslLhzAaZiIEeowWOkKEeZxFfQIiRrqXKuUkeEEBbjJerRgGJjyYFfZzFfjGgvVRBbFtTXxZzgGYyneEzZhHQqWIWwXxixLltTXwRrNDsSQprRvVPxXDdFfTtqEeEeQqaHzdDZEehsMmPpPpSGQqgzZGgoOAZzBANJjlLSgGsBfFfFbxXsxXSnLxSsHhiICQqzZvVrbBFfAaRcXbBnNlaYlLHffMmzRrZQqIiOoOWwqQovVqQlLFNnyYpoOagBbuUHhWwoOqQtTiIAAUoOxXuyRrCcYXvVpPtTOoBjJbCczzOlLlLhHIioBbmMZWpaAPwZmGgQYcqCcJjdDpPQMmXxrUucCNWwnNDdxXYuUvPpVPpAHWwhaCcxzZCmMcxBbHjJhwvVWyYZMmMmzPpvVXXBbZzlWwYtTysSRTNntUrRiIurylIeeFfEEpPiGVvgpsSPSQqJiIjJwWVvZLlLgyYapPlLtTzZAGlzOopPNncCUlLQqNnGguCJMaAmjnNwWoOJePpdrRDoJjeEcCWAaeEwOrPfAaFcCsSsKkNnVrmMRjJvVgAaGZznNjcCIiJveEMmVhHWwSsWwevgzZcrqQuURCHDdgGTtkKYyhGgOZzoIwWaRrDdNnDdAGqQvVnNqQXWwJjJjxPkCcKNIihHnwWIirRpBHhbsSfFoOuFjJhwWaGgiyYUutEepdNnDRrPPpwWTgGkwWKVvCcMmgqYNnyQqSyYoObvViIHvVSGgjJsiSeEMmslLsHhSIOSsEBrRbeoFBbfPzZCYYyyEEeecSYyfFGAaRrFfZwaAWUudDxXjJCcjkKJoOEeqQMPSspfFaYyAmrRqQEoOeyyFfgGYYNOonAcSVvTtOoyYsCJjiEgGeSbBiIFfSiIsaAYXPpxyqQwWbyYBiIiIsSfFsIbBRriAXUuxaCcBbvVCccEeZMmYyzCMPpDqQCcqAaQWwQzZqiIXxwKqeZzhHEeEQcUuceEpPCKkCjJQNnqaAkFypLqQlaAXxfBbFnrRNpTUumWwKkvVoOSoOsklLlLCdDdrRlLsSBbmbBMDwIAvVKOokaieEWMxXmtTdHMmtTMiImRcCrDddUumseEwWSeEyYnrRWwNHhMnsJjSOotTrRNQqyEDdetTYwQKkqWkYygGKQqaArRSdDNnGgsRrZzvfHhgiIGkopPJjCeEuxXUcPpOnNeTtwWZQkKzZzcCLkKlpPLrRlyYTRrTtrRCclTtLtFfQpPqyHcCyYRrhGvVIrwWNnRGIigmMfFNTnNUwWuUdarRADczZJjCzVvKkfFZEdDecsSrRwKkHhWBbHhXxZWrtTcCraAJjnaJhHuUTtniINcCOoUYbByuwWYpPOoyMvVSsKkmjnNuKbBkzZUQqYxYyXfFkKyhHAJjSsNngGVvgIihYyHtDdeOoOYMmyoeEEJjHCchjNtWwJjTtUupPCvVRrkJjKeEtOouUTfOoxgbBcNnCQqGXxXlLPSvVNnGTtqQqCcQITtirRFiIfSPCcGgWwpsUNnZEecCzoOuuqtTfFEJswWSjbBdDVwWveQUYYyWwsSwWldDJjDdyMmMmyoOYiIguUGAaYMmWaApPFfwaYybBiIfFDdALZzEpdDPyiIjAatTbBiIHYyaArRQqdDMmhzQqZSsWwFfjwWtZvVzTJdRrPpDJEumRrMsSHhUeZzYQSsqOoCcJjoOeJjnfhHFVuUvIiztTeEClfFLpcNBuUCcbnWBbwqVvIizZvAasPpSwOTtoWxqQXfFGgwiIJaAjWwWBbcCVuUzZyYWwQkFfQqJjxXKzzAaZFTtNVvnjJZzfFfEedDwWUPpnWwNlLGguXxqQlLMmJjiIluUuUiIZzOoWwiIXxCcVvYIGgiaAZWFfwzZyYzhHPpNnyjZzJLDdYyfqQFZxXWwDdgDvVdQMmZziIqtVvTGgFfCcKAaHUqQuhkGnNBKkbgvjJVuUGNnhHClNnLoObhHHhBVvkzqDdZKkKihHmCcMIPQqNRrnoOjWWOokKXPHPpwWhAaJjuUpOUuogZzGcCpPZOofFgWwFfaSszZACcGzPIiKNnkcCxXKJjkkKwWmMvVftTFTtuUcYyMmYhHhHcCyTtCfFWqQwFfnNVRrlVbBWwvUlcqQCYyLsSaIiAaApYzgerREGrpPRrRZByCcYkKyYFfFfRMmrBbBpPbGgGiIPpgtTLlbtTBuUIWDdwGdDCcEegJjgfFLlGDdgmSsMBboNnOaAGUuYgGaASsfqQDdFYkvVKyAeEbiIBhHlLaDwWdAaleBbPfKkFZEeCIicPpjUXdDxaAuJcfOosStTFxXCoONnzfBbTtMtWZznNwTJiIjXJjMvyYVjJowWOfFmUuBtqQTchHCeEHhQZHhzqJjbmMpvTMCcmaxXAtpPjmxXMIiJmMXxqQRFfsSrWzBbFoOOBbofPpZHhwmiIMSspCcPEeiSoOsEJjCcXxefvIiVSsiIiIZydRrjJDsSYfFFfxqQJjXeEYBbyBHYyhbFfFWwDdfpPFFfOoOoWwfpPbBIYyizkoOgzZGAaKFfMppPcKkCIiYAayYoOyAagGBxXbIDdisFflLSRrGKiIgQqGiIYCcykeEKkjzZJixXIoOgtOrMmREVvxXeRiIrzdvWjJwVDvVZiLeEcCTtMmfHhFlIovVJjiYvbBwWEeVSsvVaAWwbBPeEOovVpPOAaVvcCopdDkKyYtTeQwtTWqENnQqyhHntTNxXMmxOoqQnwWMeEmEpPeNOoXtPpTnSsjJEeOoNpPUuQoOxvVXkzZKIxXAIiarEeMmuURyYnNiXTtxwxXYyWoOoOoOadVvDzZZaAMmzhJjHdDPiIpHyYaATthQpPzZOoWwfFnNaAfPpFpFdOoDnNTtfPxsbBSuUreERXBbNaAnqGghHtTxXAiVFfveEIIlLHhVviAatfNnFDdTlzZyYMnNmUufyYFzaAZLoOkKtTpPfcCFqFIifxXUGgZsSzupEXxvVDdLleCciSgpPGPpjJpPTSstqQUyIiXxYPpupPsOoSuUspPICMmcAzFfZLlPplLzZacOoUuVvfFpPGgTOotuOocCUCtTHlLrRxXFfPhWwHphPvfFuUQqDdVxXISsWwJRrjfbhHBzZvVQqDdkKtTFDKCcXxkrRdnEPpsSeHhcaAzZCSsfQqBbrRFNylLYyYSsJjxXnNtTtTaAiIuUDYxXydpAaHhPgGblLBuUjJBbFApPafdSsDbBRrUoOuDdtTmMiWshYyHjJSwIKkirRIhHxXDfQqFduUGgGgxXBhHKzZNnkuUbJjXLlvVahvBbVHhHWOowRRrrmMBbvVUiIumMOqQuImXGgxMkKVvNnpPQqOozvVWwDkKrRdYCcrRyzwWZCcwmMWzZZoOSsxXifmMwWSabBANnswguUMmzFfZGzOoZqQTMmkjHhUuJoOhHKCcDdQqtuaAkMmvVKUDoOYyjnZzsSNJhkMmKNnHTmwWKkEexTtXMCcowaAWOxXcYQKkqyPpQpPrRqCRrzZZztcCTlvVLtMmOoqQEeCpPcqQeDsSdhHIiYjJiIsSydqQDEevVOMAanNiImYJjyHOohodDERrLlefAaFdDgMmGyCqQzZcFfXxYmgGMyAFflwsgGYsSfFySWlLYbBiHhIIivVUuygGyYLiWVgGvmOvVoMwZzIXxWrRQqwamMiWwNnYyWIiwNnmMdZZzzQqDKkvVYyrRIPpQqaAOFsSfoToOtjJnNaAKUuqQVOovlLxAakKYOoyIIiUuiXxFfZMmsSnbBNzwWRrxXiIaAWGgwmqQCcTtdDwWfFMeEipPIXFLAaljtTXcCccCCIQqibBxwWJf".to_string();
  let mut stack: Vec<char> = Vec::new();
  
  for character in input.chars() {
    if let Some(c) = stack.pop() {
      if character != c && character.eq_ignore_ascii_case(&c) {
        continue;
      }
      else {
        stack.push(c);
        stack.push(character);
      }
    }
    else {
      stack.push(character);
    }
  }

  // 9822
  println!("Day 5:A = {}", stack.len());
  
  let mut shortest = stack.len();
  
  for ignored_char in "abcdefghijklmnopqrstuvwxyz".chars() {
    
    let mut stack: Vec<char> = Vec::new();
    
    for character in input.chars() {
      if character.eq_ignore_ascii_case(&ignored_char) {
        continue;
      }
      
      if let Some(c) = stack.pop() {
        if character != c && character.eq_ignore_ascii_case(&c) {
          continue;
        }
        else {
          stack.push(c);
          stack.push(character);
        }
      }
      else {
        stack.push(character);
      }
    }
    
    shortest = min(shortest, stack.len());
  }
  
  // 5726
  println!("Day 5:B = {}", shortest);
}

enum Location {
  Site(i32),
  Nearby(i32, i32),
  NoMansLand,
  Border(i32),
}

fn hamilton_distance(c1: (i32, i32), c2: (i32, i32)) -> i32 {
  let (x1, y1) = c1;
  let (x2, y2) = c2;
  i32::abs(x1 - x2) + i32::abs(y1 - y2)
}

#[allow(dead_code)]
fn day6() {
  
//  let inputs = [
//    (1, 1),
//    (1, 6),
//    (8, 3),
//    (3, 4),
//    (5, 5),
//    (8, 9)
//  ];
  let inputs = [
    (353, 177),
    (233, 332),
    (178, 231),
    (351, 221),
    (309, 151),
    (105, 289),
    (91, 236),
    (321, 206),
    (156, 146),
    (94, 82),
    (81, 114),
    (182, 122),
    (81, 153),
    (319, 312),
    (334, 212),
    (275, 93),
    (224, 355),
    (347, 94),
    (209, 65),
    (118, 172),
    (113, 122),
    (182, 320),
    (191, 178),
    (99, 70),
    (260, 184),
    (266, 119),
    (177, 178),
    (313, 209),
    (61, 285),
    (155, 218),
    (354, 198),
    (274, 53),
    (225, 138),
    (228, 342),
    (187, 165),
    (226, 262),
    (143, 150),
    (124, 159),
    (325, 210),
    (163, 176),
    (326, 91),
    (170, 193),
    (84, 265),
    (199, 248),
    (107, 356),
    (45, 340),
    (277, 173),
    (286, 44),
    (242, 150),
    (120, 230)
  ];
  
  let mut candidates: HashMap<i32, Option<i32> > = HashMap::new();
  let mut map: HashMap<(i32, i32), Location> = HashMap::new();
  let mut max_x = 0;
  let mut max_y = 0;
  
  // Sets up the area.
  for (n, (x, y)) in inputs.iter().enumerate() {
    candidates.insert(n as i32, Some(0));
    map.insert((*x, *y), Location::Site(n as i32));
    max_x = max(max_x, *x);
    max_y = max(max_y, *y);
  }
  
  // Calculates claims
  for x in 0..=max_x {
    for y in 0..=max_y {
      let point = (x, y);
      
      // Ignore the original sites.
      if map.contains_key(&point) {
        continue;
      }
      
      let mut nearest_neighbor: Option<(i32, i32)> = None;
      
      for (n, (px, py)) in inputs.iter().enumerate() {
        let distance = hamilton_distance(point, (*px, *py));
        
        if let Some((_, nearest_distance)) = nearest_neighbor {
          if distance < nearest_distance {
            nearest_neighbor = Some((n as i32, distance));
            if x == 0 || x == max_x ||  y == 0 || y ==  max_y {
              map.insert(point, Location::Border(n as i32));
            }
            else {
              map.insert(point, Location::Nearby(n as i32, distance));
            }
          }
          else if distance == nearest_distance {
            map.insert(point, Location::NoMansLand);
          }
        }
        else {
          nearest_neighbor = Some((n as i32, distance));
        }
      }
    }
  }
  
  // Calculate area sizes
  for (_, location) in map.iter() {
    match location {
      Location::Site(id) => {
        let entry = candidates.entry(*id).or_insert(None);
        if let Some(count) = *entry {
          *entry = Some(count + 1);
        }
      }
      Location::Nearby(id, _) => {
        let entry = candidates.entry(*id).or_insert(None);
        if let Some(count) = *entry {
          *entry = Some(count + 1);
        }
      }
      Location::NoMansLand =>
        continue,
      Location::Border(id) => {
        let entry = candidates.entry(*id).or_insert(None);
        *entry = None;
      }
    }
  }
  
  // Find the greatest area.
  let mut max_area = 0;
  
  for (_, entry) in candidates {
    if let Some(count) = entry {
      max_area = max(max_area, count);
    }
  }

  // 4829
  println!("Day 6:A = {}", max_area);
  
  
  // 46966
}

fn main() {
//  day1();
//  day2();
//  day3();
//  day4();
//  day5();
  day6();
}
