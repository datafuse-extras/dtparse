
use tokenize::Tokenizer;

fn tokenize_assert(test_str: &str, comparison: Vec<&str>) {
    let tokens: Vec<String> = Tokenizer::new(test_str).collect();
    assert_eq!(tokens, comparison, "Tokenizing mismatch for `{}`", test_str);
}

#[test]
fn test_tokenize0() {
    let comp = vec!["Thu", " ", "Sep", " ", "25", " ", "10", ":", "36", ":", "28"];
    tokenize_assert("Thu Sep 25 10:36:28", comp);
}

#[test]
fn test_tokenize1() {
    let comp = vec!["Sep", " ", "10", ":", "36", ":", "28"];
    tokenize_assert("Sep 10:36:28", comp);
}

#[test]
fn test_tokenize2() {
    let comp = vec!["10", ":", "36", ":", "28"];
    tokenize_assert("10:36:28", comp);
}

#[test]
fn test_tokenize3() {
    let comp = vec!["10", ":", "36"];
    tokenize_assert("10:36", comp);
}

#[test]
fn test_tokenize4() {
    let comp = vec!["Sep", " ", "2003"];
    tokenize_assert("Sep 2003", comp);
}

#[test]
fn test_tokenize5() {
    let comp = vec!["Sep"];
    tokenize_assert("Sep", comp);
}

#[test]
fn test_tokenize6() {
    let comp = vec!["2003"];
    tokenize_assert("2003", comp);
}

#[test]
fn test_tokenize7() {
    let comp = vec!["10", "h", "36", "m", "28.5", "s"];
    tokenize_assert("10h36m28.5s", comp);
}

#[test]
fn test_tokenize8() {
    let comp = vec!["10", "h", "36", "m", "28", "s"];
    tokenize_assert("10h36m28s", comp);
}

#[test]
fn test_tokenize9() {
    let comp = vec!["10", "h", "36", "m"];
    tokenize_assert("10h36m", comp);
}

#[test]
fn test_tokenize10() {
    let comp = vec!["10", "h"];
    tokenize_assert("10h", comp);
}

#[test]
fn test_tokenize11() {
    let comp = vec!["10", " ", "h", " ", "36"];
    tokenize_assert("10 h 36", comp);
}

#[test]
fn test_tokenize12() {
    let comp = vec!["10", " ", "h", " ", "36.5"];
    tokenize_assert("10 h 36.5", comp);
}

#[test]
fn test_tokenize13() {
    let comp = vec!["36", " ", "m", " ", "5"];
    tokenize_assert("36 m 5", comp);
}

#[test]
fn test_tokenize14() {
    let comp = vec!["36", " ", "m", " ", "5", " ", "s"];
    tokenize_assert("36 m 5 s", comp);
}

#[test]
fn test_tokenize15() {
    let comp = vec!["36", " ", "m", " ", "05"];
    tokenize_assert("36 m 05", comp);
}

#[test]
fn test_tokenize16() {
    let comp = vec!["36", " ", "m", " ", "05", " ", "s"];
    tokenize_assert("36 m 05 s", comp);
}

#[test]
fn test_tokenize17() {
    let comp = vec!["10", "h", " ", "am"];
    tokenize_assert("10h am", comp);
}

#[test]
fn test_tokenize18() {
    let comp = vec!["10", "h", " ", "pm"];
    tokenize_assert("10h pm", comp);
}

#[test]
fn test_tokenize19() {
    let comp = vec!["10", "am"];
    tokenize_assert("10am", comp);
}

#[test]
fn test_tokenize20() {
    let comp = vec!["10", "pm"];
    tokenize_assert("10pm", comp);
}

#[test]
fn test_tokenize21() {
    let comp = vec!["10", ":", "00", " ", "am"];
    tokenize_assert("10:00 am", comp);
}

#[test]
fn test_tokenize22() {
    let comp = vec!["10", ":", "00", " ", "pm"];
    tokenize_assert("10:00 pm", comp);
}

#[test]
fn test_tokenize23() {
    let comp = vec!["10", ":", "00", "am"];
    tokenize_assert("10:00am", comp);
}

#[test]
fn test_tokenize24() {
    let comp = vec!["10", ":", "00", "pm"];
    tokenize_assert("10:00pm", comp);
}

#[test]
fn test_tokenize25() {
    let comp = vec!["10", ":", "00", "a", ".", "m"];
    tokenize_assert("10:00a.m", comp);
}

#[test]
fn test_tokenize26() {
    let comp = vec!["10", ":", "00", "p", ".", "m"];
    tokenize_assert("10:00p.m", comp);
}

#[test]
fn test_tokenize27() {
    let comp = vec!["10", ":", "00", "a", ".", "m", "."];
    tokenize_assert("10:00a.m.", comp);
}

#[test]
fn test_tokenize28() {
    let comp = vec!["10", ":", "00", "p", ".", "m", "."];
    tokenize_assert("10:00p.m.", comp);
}

#[test]
fn test_tokenize29() {
    let comp = vec!["October"];
    tokenize_assert("October", comp);
}

#[test]
fn test_tokenize30() {
    let comp = vec!["31", "-", "Dec", "-", "00"];
    tokenize_assert("31-Dec-00", comp);
}

#[test]
fn test_tokenize31() {
    let comp = vec!["0", ":", "01", ":", "02"];
    tokenize_assert("0:01:02", comp);
}

#[test]
fn test_tokenize32() {
    let comp = vec!["12", "h", " ", "01", "m", "02", "s", " ", "am"];
    tokenize_assert("12h 01m02s am", comp);
}

#[test]
fn test_tokenize33() {
    let comp = vec!["12", ":", "08", " ", "PM"];
    tokenize_assert("12:08 PM", comp);
}

#[test]
fn test_tokenize34() {
    let comp = vec!["01", "h", "02", "m", "03"];
    tokenize_assert("01h02m03", comp);
}

#[test]
fn test_tokenize35() {
    let comp = vec!["01", "h", "02"];
    tokenize_assert("01h02", comp);
}

#[test]
fn test_tokenize36() {
    let comp = vec!["01", "h", "02", "s"];
    tokenize_assert("01h02s", comp);
}

#[test]
fn test_tokenize37() {
    let comp = vec!["01", "m", "02"];
    tokenize_assert("01m02", comp);
}

#[test]
fn test_tokenize38() {
    let comp = vec!["01", "m", "02", "h"];
    tokenize_assert("01m02h", comp);
}

#[test]
fn test_tokenize39() {
    let comp = vec!["2004", " ", "10", " ", "Apr", " ", "11", "h", "30", "m"];
    tokenize_assert("2004 10 Apr 11h30m", comp);
}

#[test]
fn test_tokenize40() {
    let comp = vec!["Sep", " ", "03"];
    tokenize_assert("Sep 03", comp);
}

#[test]
fn test_tokenize41() {
    let comp = vec!["Sep", " ", "of", " ", "03"];
    tokenize_assert("Sep of 03", comp);
}

#[test]
fn test_tokenize42() {
    let comp = vec!["02", ":", "17", "NOV", "2017"];
    tokenize_assert("02:17NOV2017", comp);
}

#[test]
fn test_tokenize43() {
    let comp = vec!["Thu", " ", "Sep", " ", "10", ":", "36", ":", "28"];
    tokenize_assert("Thu Sep 10:36:28", comp);
}

#[test]
fn test_tokenize44() {
    let comp = vec!["Thu", " ", "10", ":", "36", ":", "28"];
    tokenize_assert("Thu 10:36:28", comp);
}

#[test]
fn test_tokenize45() {
    let comp = vec!["Wed"];
    tokenize_assert("Wed", comp);
}

#[test]
fn test_tokenize46() {
    let comp = vec!["Wednesday"];
    tokenize_assert("Wednesday", comp);
}

#[test]
fn test_tokenize47() {
    let comp = vec!["Thu", " ", "Sep", " ", "25", " ", "10", ":", "36", ":", "28", " ", "2003"];
    tokenize_assert("Thu Sep 25 10:36:28 2003", comp);
}

#[test]
fn test_tokenize48() {
    let comp = vec!["Thu", " ", "Sep", " ", "25", " ", "2003"];
    tokenize_assert("Thu Sep 25 2003", comp);
}

#[test]
fn test_tokenize49() {
    let comp = vec!["2003", "-", "09", "-", "25", "T", "10", ":", "49", ":", "41"];
    tokenize_assert("2003-09-25T10:49:41", comp);
}

#[test]
fn test_tokenize50() {
    let comp = vec!["2003", "-", "09", "-", "25", "T", "10", ":", "49"];
    tokenize_assert("2003-09-25T10:49", comp);
}

#[test]
fn test_tokenize51() {
    let comp = vec!["2003", "-", "09", "-", "25", "T", "10"];
    tokenize_assert("2003-09-25T10", comp);
}

#[test]
fn test_tokenize52() {
    let comp = vec!["2003", "-", "09", "-", "25"];
    tokenize_assert("2003-09-25", comp);
}

#[test]
fn test_tokenize53() {
    let comp = vec!["20030925", "T", "104941"];
    tokenize_assert("20030925T104941", comp);
}

#[test]
fn test_tokenize54() {
    let comp = vec!["20030925", "T", "1049"];
    tokenize_assert("20030925T1049", comp);
}

#[test]
fn test_tokenize55() {
    let comp = vec!["20030925", "T", "10"];
    tokenize_assert("20030925T10", comp);
}

#[test]
fn test_tokenize56() {
    let comp = vec!["20030925"];
    tokenize_assert("20030925", comp);
}

#[test]
fn test_tokenize57() {
    let comp = vec!["2003", "-", "09", "-", "25", " ", "10", ":", "49", ":", "41.502"];
    tokenize_assert("2003-09-25 10:49:41,502", comp);
}

#[test]
fn test_tokenize58() {
    let comp = vec!["199709020908"];
    tokenize_assert("199709020908", comp);
}

#[test]
fn test_tokenize59() {
    let comp = vec!["19970902090807"];
    tokenize_assert("19970902090807", comp);
}

#[test]
fn test_tokenize60() {
    let comp = vec!["2003", "-", "09", "-", "25"];
    tokenize_assert("2003-09-25", comp);
}

#[test]
fn test_tokenize61() {
    let comp = vec!["09", "-", "25", "-", "2003"];
    tokenize_assert("09-25-2003", comp);
}

#[test]
fn test_tokenize62() {
    let comp = vec!["25", "-", "09", "-", "2003"];
    tokenize_assert("25-09-2003", comp);
}

#[test]
fn test_tokenize63() {
    let comp = vec!["10", "-", "09", "-", "2003"];
    tokenize_assert("10-09-2003", comp);
}

#[test]
fn test_tokenize64() {
    let comp = vec!["10", "-", "09", "-", "03"];
    tokenize_assert("10-09-03", comp);
}

#[test]
fn test_tokenize65() {
    let comp = vec!["2003", ".", "09", ".", "25"];
    tokenize_assert("2003.09.25", comp);
}

#[test]
fn test_tokenize66() {
    let comp = vec!["09", ".", "25", ".", "2003"];
    tokenize_assert("09.25.2003", comp);
}

#[test]
fn test_tokenize67() {
    let comp = vec!["25", ".", "09", ".", "2003"];
    tokenize_assert("25.09.2003", comp);
}

#[test]
fn test_tokenize68() {
    let comp = vec!["10", ".", "09", ".", "2003"];
    tokenize_assert("10.09.2003", comp);
}

#[test]
fn test_tokenize69() {
    let comp = vec!["10", ".", "09", ".", "03"];
    tokenize_assert("10.09.03", comp);
}

#[test]
fn test_tokenize70() {
    let comp = vec!["2003", "/", "09", "/", "25"];
    tokenize_assert("2003/09/25", comp);
}

#[test]
fn test_tokenize71() {
    let comp = vec!["09", "/", "25", "/", "2003"];
    tokenize_assert("09/25/2003", comp);
}

#[test]
fn test_tokenize72() {
    let comp = vec!["25", "/", "09", "/", "2003"];
    tokenize_assert("25/09/2003", comp);
}

#[test]
fn test_tokenize73() {
    let comp = vec!["10", "/", "09", "/", "2003"];
    tokenize_assert("10/09/2003", comp);
}

#[test]
fn test_tokenize74() {
    let comp = vec!["10", "/", "09", "/", "03"];
    tokenize_assert("10/09/03", comp);
}

#[test]
fn test_tokenize75() {
    let comp = vec!["2003", " ", "09", " ", "25"];
    tokenize_assert("2003 09 25", comp);
}

#[test]
fn test_tokenize76() {
    let comp = vec!["09", " ", "25", " ", "2003"];
    tokenize_assert("09 25 2003", comp);
}

#[test]
fn test_tokenize77() {
    let comp = vec!["25", " ", "09", " ", "2003"];
    tokenize_assert("25 09 2003", comp);
}

#[test]
fn test_tokenize78() {
    let comp = vec!["10", " ", "09", " ", "2003"];
    tokenize_assert("10 09 2003", comp);
}

#[test]
fn test_tokenize79() {
    let comp = vec!["10", " ", "09", " ", "03"];
    tokenize_assert("10 09 03", comp);
}

#[test]
fn test_tokenize80() {
    let comp = vec!["25", " ", "09", " ", "03"];
    tokenize_assert("25 09 03", comp);
}

#[test]
fn test_tokenize81() {
    let comp = vec!["03", " ", "25", " ", "Sep"];
    tokenize_assert("03 25 Sep", comp);
}

#[test]
fn test_tokenize82() {
    let comp = vec!["25", " ", "03", " ", "Sep"];
    tokenize_assert("25 03 Sep", comp);
}

#[test]
fn test_tokenize83() {
    let comp = vec![" ", " ", "July", " ", " ", " ", "4", " ", ",", " ", " ", "1976", " ", " ", " ", "12", ":", "01", ":", "02", " ", " ", " ", "am", " ", " "];
    tokenize_assert("  July   4 ,  1976   12:01:02   am  ", comp);
}

#[test]
fn test_tokenize84() {
    let comp = vec!["Wed", ",", " ", "July", " ", "10", ",", " ", "'", "96"];
    tokenize_assert("Wed, July 10, '96", comp);
}

#[test]
fn test_tokenize85() {
    let comp = vec!["1996", ".", "July", ".", "10", " ", "AD", " ", "12", ":", "08", " ", "PM"];
    tokenize_assert("1996.July.10 AD 12:08 PM", comp);
}

#[test]
fn test_tokenize86() {
    let comp = vec!["July", " ", "4", ",", " ", "1976"];
    tokenize_assert("July 4, 1976", comp);
}

#[test]
fn test_tokenize87() {
    let comp = vec!["7", " ", "4", " ", "1976"];
    tokenize_assert("7 4 1976", comp);
}

#[test]
fn test_tokenize88() {
    let comp = vec!["4", " ", "jul", " ", "1976"];
    tokenize_assert("4 jul 1976", comp);
}

#[test]
fn test_tokenize89() {
    let comp = vec!["7", "-", "4", "-", "76"];
    tokenize_assert("7-4-76", comp);
}

#[test]
fn test_tokenize90() {
    let comp = vec!["19760704"];
    tokenize_assert("19760704", comp);
}

#[test]
fn test_tokenize91() {
    let comp = vec!["0", ":", "01", ":", "02", " ", "on", " ", "July", " ", "4", ",", " ", "1976"];
    tokenize_assert("0:01:02 on July 4, 1976", comp);
}

#[test]
fn test_tokenize92() {
    let comp = vec!["0", ":", "01", ":", "02", " ", "on", " ", "July", " ", "4", ",", " ", "1976"];
    tokenize_assert("0:01:02 on July 4, 1976", comp);
}

#[test]
fn test_tokenize93() {
    let comp = vec!["July", " ", "4", ",", " ", "1976", " ", "12", ":", "01", ":", "02", " ", "am"];
    tokenize_assert("July 4, 1976 12:01:02 am", comp);
}

#[test]
fn test_tokenize94() {
    let comp = vec!["Mon", " ", "Jan", " ", " ", "2", " ", "04", ":", "24", ":", "27", " ", "1995"];
    tokenize_assert("Mon Jan  2 04:24:27 1995", comp);
}

#[test]
fn test_tokenize95() {
    let comp = vec!["04", ".", "04", ".", "95", " ", "00", ":", "22"];
    tokenize_assert("04.04.95 00:22", comp);
}

#[test]
fn test_tokenize96() {
    let comp = vec!["Jan", " ", "1", " ", "1999", " ", "11", ":", "23", ":", "34.578"];
    tokenize_assert("Jan 1 1999 11:23:34.578", comp);
}

#[test]
fn test_tokenize97() {
    let comp = vec!["950404", " ", "122212"];
    tokenize_assert("950404 122212", comp);
}

#[test]
fn test_tokenize98() {
    let comp = vec!["3", "rd", " ", "of", " ", "May", " ", "2001"];
    tokenize_assert("3rd of May 2001", comp);
}

#[test]
fn test_tokenize99() {
    let comp = vec!["5", "th", " ", "of", " ", "March", " ", "2001"];
    tokenize_assert("5th of March 2001", comp);
}

#[test]
fn test_tokenize100() {
    let comp = vec!["1", "st", " ", "of", " ", "May", " ", "2003"];
    tokenize_assert("1st of May 2003", comp);
}

#[test]
fn test_tokenize101() {
    let comp = vec!["0099", "-", "01", "-", "01", "T", "00", ":", "00", ":", "00"];
    tokenize_assert("0099-01-01T00:00:00", comp);
}

#[test]
fn test_tokenize102() {
    let comp = vec!["0031", "-", "01", "-", "01", "T", "00", ":", "00", ":", "00"];
    tokenize_assert("0031-01-01T00:00:00", comp);
}

#[test]
fn test_tokenize103() {
    let comp = vec!["20080227", "T", "21", ":", "26", ":", "01.123456789"];
    tokenize_assert("20080227T21:26:01.123456789", comp);
}

#[test]
fn test_tokenize104() {
    let comp = vec!["13", "NOV", "2017"];
    tokenize_assert("13NOV2017", comp);
}

#[test]
fn test_tokenize105() {
    let comp = vec!["0003", "-", "03", "-", "04"];
    tokenize_assert("0003-03-04", comp);
}

#[test]
fn test_tokenize106() {
    let comp = vec!["December", ".", "0031", ".", "30"];
    tokenize_assert("December.0031.30", comp);
}

#[test]
fn test_tokenize107() {
    let comp = vec!["090107"];
    tokenize_assert("090107", comp);
}

#[test]
fn test_tokenize108() {
    let comp = vec!["2015", "-", "15", "-", "May"];
    tokenize_assert("2015-15-May", comp);
}

#[test]
fn test_tokenize109() {
    let comp = vec!["Thu", " ", "Sep", " ", "25", " ", "10", ":", "36", ":", "28", " ", "BRST", " ", "2003"];
    tokenize_assert("Thu Sep 25 10:36:28 BRST 2003", comp);
}

#[test]
fn test_tokenize110() {
    let comp = vec!["2003", " ", "10", ":", "36", ":", "28", " ", "BRST", " ", "25", " ", "Sep", " ", "Thu"];
    tokenize_assert("2003 10:36:28 BRST 25 Sep Thu", comp);
}

#[test]
fn test_tokenize111() {
    let comp = vec!["Thu", ",", " ", "25", " ", "Sep", " ", "2003", " ", "10", ":", "49", ":", "41", " ", "-", "0300"];
    tokenize_assert("Thu, 25 Sep 2003 10:49:41 -0300", comp);
}

#[test]
fn test_tokenize112() {
    let comp = vec!["2003", "-", "09", "-", "25", "T", "10", ":", "49", ":", "41.5", "-", "03", ":", "00"];
    tokenize_assert("2003-09-25T10:49:41.5-03:00", comp);
}

#[test]
fn test_tokenize113() {
    let comp = vec!["2003", "-", "09", "-", "25", "T", "10", ":", "49", ":", "41", "-", "03", ":", "00"];
    tokenize_assert("2003-09-25T10:49:41-03:00", comp);
}

#[test]
fn test_tokenize114() {
    let comp = vec!["20030925", "T", "104941.5", "-", "0300"];
    tokenize_assert("20030925T104941.5-0300", comp);
}

#[test]
fn test_tokenize115() {
    let comp = vec!["20030925", "T", "104941", "-", "0300"];
    tokenize_assert("20030925T104941-0300", comp);
}

#[test]
fn test_tokenize116() {
    let comp = vec!["10", "-", "09", "-", "2003"];
    tokenize_assert("10-09-2003", comp);
}

#[test]
fn test_tokenize117() {
    let comp = vec!["10", ".", "09", ".", "2003"];
    tokenize_assert("10.09.2003", comp);
}

#[test]
fn test_tokenize118() {
    let comp = vec!["10", "/", "09", "/", "2003"];
    tokenize_assert("10/09/2003", comp);
}

#[test]
fn test_tokenize119() {
    let comp = vec!["10", " ", "09", " ", "2003"];
    tokenize_assert("10 09 2003", comp);
}

#[test]
fn test_tokenize120() {
    let comp = vec!["090107"];
    tokenize_assert("090107", comp);
}

#[test]
fn test_tokenize121() {
    let comp = vec!["2015", " ", "09", " ", "25"];
    tokenize_assert("2015 09 25", comp);
}

#[test]
fn test_tokenize122() {
    let comp = vec!["10", "-", "09", "-", "03"];
    tokenize_assert("10-09-03", comp);
}

#[test]
fn test_tokenize123() {
    let comp = vec!["10", ".", "09", ".", "03"];
    tokenize_assert("10.09.03", comp);
}

#[test]
fn test_tokenize124() {
    let comp = vec!["10", "/", "09", "/", "03"];
    tokenize_assert("10/09/03", comp);
}

#[test]
fn test_tokenize125() {
    let comp = vec!["10", " ", "09", " ", "03"];
    tokenize_assert("10 09 03", comp);
}

#[test]
fn test_tokenize126() {
    let comp = vec!["090107"];
    tokenize_assert("090107", comp);
}

#[test]
fn test_tokenize127() {
    let comp = vec!["2015", " ", "09", " ", "25"];
    tokenize_assert("2015 09 25", comp);
}

#[test]
fn test_tokenize128() {
    let comp = vec!["090107"];
    tokenize_assert("090107", comp);
}

#[test]
fn test_tokenize129() {
    let comp = vec!["2015", " ", "09", " ", "25"];
    tokenize_assert("2015 09 25", comp);
}

#[test]
fn test_tokenize130() {
    let comp = vec!["April", " ", "2009"];
    tokenize_assert("April 2009", comp);
}

#[test]
fn test_tokenize131() {
    let comp = vec!["Feb", " ", "2007"];
    tokenize_assert("Feb 2007", comp);
}

#[test]
fn test_tokenize132() {
    let comp = vec!["Feb", " ", "2008"];
    tokenize_assert("Feb 2008", comp);
}

#[test]
fn test_tokenize133() {
    let comp = vec!["Thu", " ", "Sep", " ", "25", " ", "10", ":", "36", ":", "28", " ", "BRST", " ", "2003"];
    tokenize_assert("Thu Sep 25 10:36:28 BRST 2003", comp);
}

#[test]
fn test_tokenize134() {
    let comp = vec!["1996", ".", "07", ".", "10", " ", "AD", " ", "at", " ", "15", ":", "08", ":", "56", " ", "PDT"];
    tokenize_assert("1996.07.10 AD at 15:08:56 PDT", comp);
}

#[test]
fn test_tokenize135() {
    let comp = vec!["Tuesday", ",", " ", "April", " ", "12", ",", " ", "1952", " ", "AD", " ", "3", ":", "30", ":", "42", "pm", " ", "PST"];
    tokenize_assert("Tuesday, April 12, 1952 AD 3:30:42pm PST", comp);
}

#[test]
fn test_tokenize136() {
    let comp = vec!["November", " ", "5", ",", " ", "1994", ",", " ", "8", ":", "15", ":", "30", " ", "am", " ", "EST"];
    tokenize_assert("November 5, 1994, 8:15:30 am EST", comp);
}

#[test]
fn test_tokenize137() {
    let comp = vec!["1994", "-", "11", "-", "05", "T", "08", ":", "15", ":", "30", "-", "05", ":", "00"];
    tokenize_assert("1994-11-05T08:15:30-05:00", comp);
}

#[test]
fn test_tokenize138() {
    let comp = vec!["1994", "-", "11", "-", "05", "T", "08", ":", "15", ":", "30", "Z"];
    tokenize_assert("1994-11-05T08:15:30Z", comp);
}

#[test]
fn test_tokenize139() {
    let comp = vec!["1976", "-", "07", "-", "04", "T", "00", ":", "01", ":", "02", "Z"];
    tokenize_assert("1976-07-04T00:01:02Z", comp);
}

#[test]
fn test_tokenize140() {
    let comp = vec!["Tue", " ", "Apr", " ", "4", " ", "00", ":", "22", ":", "12", " ", "PDT", " ", "1995"];
    tokenize_assert("Tue Apr 4 00:22:12 PDT 1995", comp);
}

#[test]
fn test_tokenize141() {
    let comp = vec!["Today", " ", "is", " ", "25", " ", "of", " ", "September", " ", "of", " ", "2003", ",", " ", "exactly", " ", "at", " ", "10", ":", "49", ":", "41", " ", "with", " ", "timezone", " ", "-", "03", ":", "00", "."];
    tokenize_assert("Today is 25 of September of 2003, exactly at 10:49:41 with timezone -03:00.", comp);
}

#[test]
fn test_tokenize142() {
    let comp = vec!["Today", " ", "is", " ", "25", " ", "of", " ", "September", " ", "of", " ", "2003", ",", " ", "exactly", " ", "at", " ", "10", ":", "49", ":", "41", " ", "with", " ", "timezone", " ", "-", "03", ":", "00", "."];
    tokenize_assert("Today is 25 of September of 2003, exactly at 10:49:41 with timezone -03:00.", comp);
}

#[test]
fn test_tokenize143() {
    let comp = vec!["I", " ", "have", " ", "a", " ", "meeting", " ", "on", " ", "March", " ", "1", ",", " ", "1974"];
    tokenize_assert("I have a meeting on March 1, 1974", comp);
}

#[test]
fn test_tokenize144() {
    let comp = vec!["On", " ", "June", " ", "8", "th", ",", " ", "2020", ",", " ", "I", " ", "am", " ", "going", " ", "to", " ", "be", " ", "the", " ", "first", " ", "man", " ", "on", " ", "Mars"];
    tokenize_assert("On June 8th, 2020, I am going to be the first man on Mars", comp);
}

#[test]
fn test_tokenize145() {
    let comp = vec!["Meet", " ", "me", " ", "at", " ", "the", " ", "AM", "/", "PM", " ", "on", " ", "Sunset", " ", "at", " ", "3", ":", "00", " ", "AM", " ", "on", " ", "December", " ", "3", "rd", ",", " ", "2003"];
    tokenize_assert("Meet me at the AM/PM on Sunset at 3:00 AM on December 3rd, 2003", comp);
}

#[test]
fn test_tokenize146() {
    let comp = vec!["Meet", " ", "me", " ", "at", " ", "3", ":", "00", " ", "AM", " ", "on", " ", "December", " ", "3", "rd", ",", " ", "2003", " ", "at", " ", "the", " ", "AM", "/", "PM", " ", "on", " ", "Sunset"];
    tokenize_assert("Meet me at 3:00 AM on December 3rd, 2003 at the AM/PM on Sunset", comp);
}

#[test]
fn test_tokenize147() {
    let comp = vec!["Jan", " ", "29", ",", " ", "1945", " ", "14", ":", "45", " ", "AM", " ", "I", " ", "going", " ", "to", " ", "see", " ", "you", " ", "there", "?"];
    tokenize_assert("Jan 29, 1945 14:45 AM I going to see you there?", comp);
}

#[test]
fn test_tokenize148() {
    let comp = vec!["2017", "-", "07", "-", "17", " ", "06", ":", "15", ":"];
    tokenize_assert("2017-07-17 06:15:", comp);
}