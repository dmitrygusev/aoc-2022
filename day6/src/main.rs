use std::collections::HashSet;

fn main() {
    let input = "rdzrddbgdbbqtbqbrrznznjzjjctcbtttrvrwwsvwssjfjcjnjzzqgzgzsggfddvnnrbbwgwfwgfgpgrprgrprmmqjmqmvmlmmgjmmgqmqppqgqlqmllszsmzzwfzfqzzgcccmggvfgvvzlvzlznnjfjsscrsrprcrscsqsfqfggrrhccpnpwphwhchfhtfhttzpzwzjzqzjjqjdqqnvqnqjqbqcbqccdllbcblbjljgjbgbhhgphghjhrrvbrvbrvvfggwbwnnghgwhhpbpgphpnpgpqqzvqzqtzzngzngnjnnffgbgbvvjvqvwwqnwqnnvrnnfgfttcftcfczzpbbdlbdbhbssrggtfgfbgfgfnffqsfqfcqqqwvvqpphfpfcfrrjmjhjrjbblcbbznzwwpbwwsvvwnvvzjvzzmllrpllqffplpwlpwwlpwwmnmrrlggvssjggdjjsffbtthlhsswvwjjbtbjbsswqsshppddcjjlrlttwgtwwcnwcncmcpmpnnhndnccqscqqzqpplglvvpggcvvhnhmhqqsvvhcvvjpphbhnndpdbdndjnnmccmjmhmrrlcrrfzfnndldffstffhqffhcffhbhwhzhnhrhprhhtqtztntnptnppjdpdqdzqqpzprrqbqmqhmqqmhqqjttlrrqbqnqdnnrqqtqfqjjdfjfdjjsnspptltlgttmctcgttcthcttdpppzphpnhhvthtccbvccpplpwwfgwfffhfbbrsbbgppnhpnhhwnnnrprcpcncffrqqpjprpddjnddnccqtcthhfqfnnnlrrzhhmvvczzlbzbrzbbjpjpbbjttmwtwbtbqqtssgdsdsmsmbsmsfftltmtwwsswbwdbdwbbblgbbqpbqbmbgmbgmbbvcvbbmrrldlpdpqqnvvrzzcddjsdjjfvjvvndnvvrsvrsvrsrmsswlswlssqllfmlffmwmttjgttzgtggghrhprppbgbdgdtgddvmdvvbmmchhjppczzglzglzglzgggsmsrrsttgddmvvhddvdhvvcfcvvglvlmvmdvmdvdcdvvgfftccljllmssrvvqhvvnlnplnlglrlvrvfvrrgjjzqjjzcjcvcsvvbppgrrbdrdsrddqsspwssdlltlcttgngzglgfgzfzhfzhfhnffmsspvsscjjrbbpgbbfgbffhggzpznzmnmcmzmdzmzjzddcsscgssdnddjbdjdtdzdwwshwhfhtthjhsjjfjvvdnnmzmtmmlrlblzzmmspsnpnbnvvczvzsszrrsgsbsrrlprrzfrrjdrdbdhdvvwrvvrfvrrvwrrqlqwwqjjpwwffdvffwfbwwdrwdrddhhhjghjhvhccgbcbvblllbwlwtlldbdzbbdpbbqmmglmllvwwzmzhhfnfcnfcfddvrvhvbvzvbvhvthtrtppsccggdgpghgfhhhnwnmmwfwtffpjfppmdpdhppvwvfvvhdhzhtzhtzzplpspmpnpzpgzgqgvvqcqcqmqffzszwzfwfzzcmchhmgmppgwwvffnnvhhttvtjvjgjljmmfcmmzmvvjfjggppqmpqqswsnwwdvwdvwdwjwrrrtprpqrrsddsshgssbbpbgpbbprprzppsnsbszszccddfcftcfczffchfcfjjffjwwsbbvfvpfprpbczqdnvrqrlhsrzvlvgzbqpwgpmgftzfvtlnlqrtmpjfstmsrbfjldtmhvvqwznrcflbvmsnbzcrjvbzgvvmwlzcwggwvhnpscnwhjldzjvmtfdgvptfhjtdtwfjntzqtswqgwvnfgqhqpdvdchfqjzfhgjmdstmchppcvgpdfrqbrhrvdvzbtnnrpqsnmnljjzgzqfnzmlvbzmwzfbfszvfsqdnqfstpjglwtcdjpmdfqzfcsngbcvvjvdzlnzndcggcdtdjpwgvfzlfdqpgbgfjjfvgtwwrgtrfcpfvmzvrwrftpfdprzmcmrpnjpfrdvbmlrfzrjcdzhvwbpgmbpdnqggjdpqttgnqbtjfmrglqbvcfjwghrqtcgjddhwpntjgtghmfgvjdhbzzgmfnrrrgvdmsfbhndfcwlbbtsdcbpgfpbvnwmpwdpjlvcbcgnwjzftsfqhvwdshzltrmqpcsngfzrvfwhffrcjzlfqjqdbcntdwhfrfnrzwftqhlfjpjqpngjqjcnfnrpmmbprdtzgsvsdjpzsnzzmzdfjplfhzqrnrwggcvrrmqzwlvslwvtvhbgwmjmnzftrbfhdrzszcvmdhgfvwgwgsgqtpwgvpvfrszczmsmstwhnhtnftmmmjblpchrnrdwlnhrvqtbwqwdchfjbcldmjmjzwlcngfgfvmblgmnwtwvjmzswzmpvdjtcfgcpvvqhrfnwcczrrrhwwvfrngwmlstqcvvrqrshwdvrgtgbffvlwtvwhvlcwwqgspnfmndbsbpvdbjwdlfntwrrmtgsdtwbfmjjcjfvsvltwdjmvswwpchpdtsjmbbgcgtddcbprwznsldmwflrgcrgfflzwllrzcrgdgqgsvrqmspqzsrzvppztrhlpsnqdlhmghdcrppvbljsnrjgcwhpvmlgcnswrjjbjltwnctqbqsffbcfpclhcbrnsjlmrstpngbvcfcbjstgvzwfgcsqbwgqqblsnmfddprrqmpqgfrjbhfptrdvltcrmtrcqgcdfpjhjptzngzqdghqhpsfwlmrwgfhldzpfrbtzzgsblcfmwztjmtjzgwrpttvwfhbntvsgnfvbpfpnscdspmcvncsqltzqnwczvdbtphwrdtjcszmhbpcfnvbrbgfpnrrhvhzlrglthpldrlfscpvpvttvjtfdrqpjvnwvmscdvclbnzfvppslzgmglrvvdvpsbsfhflnjbdnpqzjzlrfgwgjvwvpthjptftlzppmnmrrpfvdhgwfzfdnnbhqpwrzvvdgtcrflwfjgbmvbgsnqzqtmsvrdlfmlbqqdnfftljbtphnpqqrgbrlzrbbhlgvjpcdbmtvzlpqhbvjpmhpdmtrmjwvzrbfrdrdfmwsfvllljdwmnqlgcnzvpphwmlmstcsljvmljcjprtgmzfssgbtjlttssfzbcqbgnjnvrwzchtwtwdtfwngdflwzjrhzdlrrqfnsztvdbzqfwfzppqrghrhzsqtwqstsfspddpjrpffhgqvspzwmlzwhtzqqldqbwlsrqhlmvhhmzjpdsrgdcvqnfpldnmblgvvssrcdnjqvcgwtwmhqdcwtsqqhbntvjnlbljlqjqglggvpqncpdzztlvhhlghtrncfcdhjtzwjqdhlntjfrzccbnmglmnzwvplclvcmnsppqjhbggbzncqlcfnzbbzdjrvcdthcqwjzjvdbjddcbjchwfgbjhwqpzbpgsdtlwlphtvhwddjdbwbpsnqhnffqptcrljcqzzhszdfpdfsgflhwwsgbfcnwrbdqflnrcwddwwmfztlbswlzhtzzcllnvbtqgjsdzmhcpnnqpdpqgdntlfwgvddgqvhqhrbvstsmzrmgwslpdjlsbgthfhgnlftbqdnzsvcrrmllcjvdlqrzbvrbrjcbpttsvwcrnlvvbnjvfgldzmtflvmzqdgbnjcgctllrldgzltwlswmfbbwjmcqpldhhdsmqpbvnjprdqnvrbjhrjzqwqfrfqwngwtwjjmzdbqmpmvqrprjhnhnrlmlgpfwwzjhlgmbzdlpshcqpnlgrqvprbspmdznzzsvhdlzwmttpdnlrlqjllqnshjllvvsrblscjcmbcqlsgpcjlmmpgwrvjnjzvzfgvgghwqfjswjbjghmzcgdpsjwhbnzmbhtzgnchpbrmnfdbfscgzldpqmvprjpvcwtwdfjblfshffwqctdphhnhngsjlrqtqprpjhwqcbmhctqbpdtpzvbbfncfrcvmbfvqmbmqjvgtdvspfqfbqnmjwhzbcpcfgbhtllbsssqntfbmsmlwhjchgcsrvsfznbmspwwszqfnwzzljfnvcwnwmgfzqfvmwwwdjd";
    assert_eq!(1702, part1(input));
    assert_eq!(3559, part2(input));

    part1("123️⃣45");
}

fn part2(s: &str) -> usize {
    find_marker_offset(s, 14)
}

fn part1(s: &str) -> usize {
    find_marker_offset(s, 4)
}

fn find_marker_offset(s: &str, n: usize) -> usize {
    for i in n-1..s.len() {
        if all_different(&s, i, n) {
            return i + 1;
        }
    };
    return 0;
}

fn all_different(s: &str, offset: usize, n: usize) -> bool {
    HashSet::<char>
        ::from_iter(s.chars().skip(offset+1-n).take(n))
        .len() == n
 }