extern crate regex;

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

#[allow(dead_code)]
fn day1() {
  let inputs: Vec<i64> = vec!(7, 15, 18, -15, 8, -2, 4, -12, 19, 6, -19, -12, -4, -10, -5, 4, 7, -13, -12, 10, 3, 18, -10, -14, -14, 5, -10, 6, 8, -15, -3, -12, 4, -12, 17, -11, 7, 13, 9, -7, 13, 10, 11, -18, -4, 7, 17, 20, 18, 15, 8, 19, -3, 2, 17, 16, -9, -9, 16, 17, 14, -11, 4, 9, -1, -14, -2, -10, 16, 4, 3, 10, 17, -18, 3, 17, 19, -3, 10, -6, 16, -12, -14, -11, 3, -11, -5, 2, -4, -7, 17, 15, -5, 18, -15, 21, 6, -16, -5, 19, 9, 1, -4, 1, 14, 19, -11, 18, -13, 11, 6, -2, 16, 16, 9, -18, -13, 1, 18, -12, 16, 19, 11, -10, 15, -6, 11, 11, -8, -6, 9, 1, -15, 1, 19, -3, 1, 8, 1, -20, 6, -3, -6, -14, -13, -8, -9, 3, -16, 3, -18, 16, 9, 16, -2, 8, -9, -8, -9, 6, 15, 6, 12, 10, -18, 12, -1, -20, 7, -17, 16, -20, -20, -17, 6, -11, -19, 10, 4, 4, -16, 19, -4, -9, 15, 19, -15, 2, -16, -9, -7, -12, -20, 19, 15, 13, -11, -3, 5, 13, 6, -11, 14, 14, 10, -9, 4, -8, 2, 17, -21, -28, -17, -14, 8, 11, -7, 4, 14, 23, 28, -2, 12, 17, 23, 19, -9, -17, -10, 20, -16, -7, 5, -13, -6, 40, 11, -1, 12, -4, 21, 7, 8, 17, -16, -11, -6, -3, 16, 7, -16, 1, 13, -19, -11, -17, 12, 6, -3, 17, 4, 20, -1, -5, 1, -11, 18, 18, -8, 16, -2, 18, -13, -12, 10, -19, 7, -16, -14, -20, 18, -17, -5, 3, -12, -3, 7, 13, 21, -10, -18, -9, -8, 18, 18, 13, 19, -17, 15, -16, 6, -15, 20, 17, -20, -1, 3, 7, 4, -2, 19, -3, 17, 10, -13, -13, 1, -4, 17, 18, 1, -13, -10, 14, -16, 13, -9, 16, -13, 7, 5, 4, 25, -18, -2, -1, 27, -3, 24, -6, 19, 17, -7, -3, 15, 3, 18, 2, 3, -14, 16, 11, -14, 10, 2, -3, 14, 13, 19, -4, -4, -9, 7, 11, -19, -9, -9, 8, -3, 16, -19, -13, -1, 6, -8, -11, -8, -9, -10, -20, -10, 5, 4, 19, 6, -2, -5, 15, -7, 12, 18, -10, 3, 11, -17, -4, 8, -10, -2, 1, 15, 19, 13, 1, 12, -16, 18, 16, -17, 15, 13, 13, 18, -9, 10, 18, -3, -7, 9, -13, -12, 6, -16, -7, -3, -2, -11, -1, 6, 21, 3, 7, 14, 21, -2, -2, -6, -27, -20, -7, -20, -27, -2, 11, 21, 1, 10, -6, -24, 6, 7, -11, -10, -14, -21, -10, 8, -36, -8, 1, 11, -6, -18, -2, -3, -16, -5, -30, -14, -1, 5, -19, 13, 15, 26, -38, 1, -18, -1, 6, -16, -2, 72, -6, -106, -52, 35, 8, 64, 59, 47, 19, -11, 137, -40, 23, 27, 5, 47, 76, 164, 108, -314, -396, 72153, 12, -8, 15, -8, -4, 17, 4, 3, -18, 4, -14, -7, -8, -5, 6, 1, 15, 1, -4, -16, -10, -17, -11, 2, 6, -14, 9, 11, -15, -8, -11, 8, -15, -3, -10, 5, -9, -16, -5, -9, -8, -16, -11, -7, -4, -22, -16, 9, 23, 1, 21, 1, 3, -1, 8, 18, 19, 7, 10, -16, -9, 1, -5, 14, -12, 21, 8, 3, 15, -6, 18, 11, 8, -2, 10, 16, 9, -14, -16, 2, 17, -18, -20, 1, -12, 4, -12, -8, -10, 5, 4, -18, -8, -15, -17, 6, -1, -26, -11, 26, 25, 9, 20, 20, 12, -14, 11, -4, 12, 11, -14, -8, -5, 19, 20, 8, 17, 3, -2, -14, -11, 2, 12, -6, 5, 17, 13, 15, -5, 14, -5, 19, 13, 11, -15, 10, 7, 7, -13, -2, -2, -5, 1, -14, -9, -16, -7, 18, -14, 18, -20, -17, 28, 8, 16, -7, 10, -5, -9, 15, 20, 11, -1, 4, 4, -2, -13, 18, 11, 9, 4, 11, 17, 16, 19, -15, 16, 6, -8, -7, -15, -2, 9, -6, 9, -17, -19, 11, 18, 10, 1, 6, 15, 16, 14, -8, -9, 12, 2, -13, 9, -14, -7, 5, -4, 14, 15, -17, -16, 19, -12, -9, -4, 15, 18, 3, 11, 19, 18, 16, 3, 2, -13, -11, -4, -19, 3, -9, 1, 18, 14, 19, 15, 11, -18, -10, 8, -12, 20, 6, 2, 20, -18, -8, 17, -2, -1, 17, 20, 6, -17, -7, -12, -8, -8, 11, 21, 9, 10, 17, -8, 13, 10, -4, 10, -2, 6, -5, -18, -13, 6, -4, 3, -17, 16, -13, -10, 16, -24, -5, 18, 6, -15, -17, -4, 20, -26, -17, -4, -10, -13, -19, 3, 19, 5, -6, -4, 8, 20, 8, -13, -16, -17, -19, -15, -2, 3, -16, -1, 9, 10, -17, -10, -17, -3, 21, -16, -4, -21, 10, -20, -1, -9, -8, 13, -18, 4, 5, -17, -10, 6, -12, -7, -5, 4, -17, -15, 11, 18, 8, 20, 2, -14, 5, -15, 13, -14, 8, 18, 6, 14, 16, 4, 9, 7, 8, -22, 10, -9, 5, -15, -19, 1, 10, -27, -19, -16, -9, 12, 10, 18, -10, 5, 4, -22, -10, -17, -17, -14, 3, -2, -17, 24, -9, 12, 17, 9, -16, -1, 3, 23, -6, -24, -19, 13, -14, 9, 29, 11, 25, -18, 5, 11, 20, 27, -10, -15, 4, 5, -17, 26, 55, -6, -3, 4, -21, 47, 12, 20, -2, -12, 8, -2, -14, 19, 16, -9, -19, -14, 22, -17, 3, 2, -7, 21, -11, 21, 20, -6, 8, -17, -2, -19, -28, 42, 14, 21, 9, -11, 25, -4, 17, -15, 19, -72362); 

  let sum: i64 = inputs.iter().sum();
  println!("Day 1:A = {:?}", sum);

  let mut frequency: i64 = 0;
  let mut frequencies: HashSet<i64> = HashSet::new();

  'outer: loop {
    for n in inputs.iter() {
      frequency += n;

      if frequencies.contains(&frequency) {
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
    
    println!("Day 3:B = {}", id);
    break;
  }
}

fn main() {
  day1();
  day2();
  day3();
}
