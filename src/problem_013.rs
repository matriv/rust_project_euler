mod problem_013 {
    use substring::Substring;

    pub struct Euler;

    impl Euler {
        // ~ 20μs
        pub fn exec() -> String {
            let mut result: f64 = 0.0;
            let numbers: [f64; 100] = [
                "37107287533902102798797998220837590246510135740250"
                    .parse()
                    .unwrap(),
                "46376937677490009712648124896970078050417018260538"
                    .parse()
                    .unwrap(),
                "74324986199524741059474233309513058123726617309629"
                    .parse()
                    .unwrap(),
                "91942213363574161572522430563301811072406154908250"
                    .parse()
                    .unwrap(),
                "23067588207539346171171980310421047513778063246676"
                    .parse()
                    .unwrap(),
                "89261670696623633820136378418383684178734361726757"
                    .parse()
                    .unwrap(),
                "28112879812849979408065481931592621691275889832738"
                    .parse()
                    .unwrap(),
                "44274228917432520321923589422876796487670272189318"
                    .parse()
                    .unwrap(),
                "47451445736001306439091167216856844588711603153276"
                    .parse()
                    .unwrap(),
                "70386486105843025439939619828917593665686757934951"
                    .parse()
                    .unwrap(),
                "62176457141856560629502157223196586755079324193331"
                    .parse()
                    .unwrap(),
                "64906352462741904929101432445813822663347944758178"
                    .parse()
                    .unwrap(),
                "92575867718337217661963751590579239728245598838407"
                    .parse()
                    .unwrap(),
                "58203565325359399008402633568948830189458628227828"
                    .parse()
                    .unwrap(),
                "80181199384826282014278194139940567587151170094390"
                    .parse()
                    .unwrap(),
                "35398664372827112653829987240784473053190104293586"
                    .parse()
                    .unwrap(),
                "86515506006295864861532075273371959191420517255829"
                    .parse()
                    .unwrap(),
                "71693888707715466499115593487603532921714970056938"
                    .parse()
                    .unwrap(),
                "54370070576826684624621495650076471787294438377604"
                    .parse()
                    .unwrap(),
                "53282654108756828443191190634694037855217779295145"
                    .parse()
                    .unwrap(),
                "36123272525000296071075082563815656710885258350721"
                    .parse()
                    .unwrap(),
                "45876576172410976447339110607218265236877223636045"
                    .parse()
                    .unwrap(),
                "17423706905851860660448207621209813287860733969412"
                    .parse()
                    .unwrap(),
                "81142660418086830619328460811191061556940512689692"
                    .parse()
                    .unwrap(),
                "51934325451728388641918047049293215058642563049483"
                    .parse()
                    .unwrap(),
                "62467221648435076201727918039944693004732956340691"
                    .parse()
                    .unwrap(),
                "15732444386908125794514089057706229429197107928209"
                    .parse()
                    .unwrap(),
                "55037687525678773091862540744969844508330393682126"
                    .parse()
                    .unwrap(),
                "18336384825330154686196124348767681297534375946515"
                    .parse()
                    .unwrap(),
                "80386287592878490201521685554828717201219257766954"
                    .parse()
                    .unwrap(),
                "78182833757993103614740356856449095527097864797581"
                    .parse()
                    .unwrap(),
                "16726320100436897842553539920931837441497806860984"
                    .parse()
                    .unwrap(),
                "48403098129077791799088218795327364475675590848030"
                    .parse()
                    .unwrap(),
                "87086987551392711854517078544161852424320693150332"
                    .parse()
                    .unwrap(),
                "59959406895756536782107074926966537676326235447210"
                    .parse()
                    .unwrap(),
                "69793950679652694742597709739166693763042633987085"
                    .parse()
                    .unwrap(),
                "41052684708299085211399427365734116182760315001271"
                    .parse()
                    .unwrap(),
                "65378607361501080857009149939512557028198746004375"
                    .parse()
                    .unwrap(),
                "35829035317434717326932123578154982629742552737307"
                    .parse()
                    .unwrap(),
                "94953759765105305946966067683156574377167401875275"
                    .parse()
                    .unwrap(),
                "88902802571733229619176668713819931811048770190271"
                    .parse()
                    .unwrap(),
                "25267680276078003013678680992525463401061632866526"
                    .parse()
                    .unwrap(),
                "36270218540497705585629946580636237993140746255962"
                    .parse()
                    .unwrap(),
                "24074486908231174977792365466257246923322810917141"
                    .parse()
                    .unwrap(),
                "91430288197103288597806669760892938638285025333403"
                    .parse()
                    .unwrap(),
                "34413065578016127815921815005561868836468420090470"
                    .parse()
                    .unwrap(),
                "23053081172816430487623791969842487255036638784583"
                    .parse()
                    .unwrap(),
                "11487696932154902810424020138335124462181441773470"
                    .parse()
                    .unwrap(),
                "63783299490636259666498587618221225225512486764533"
                    .parse()
                    .unwrap(),
                "67720186971698544312419572409913959008952310058822"
                    .parse()
                    .unwrap(),
                "95548255300263520781532296796249481641953868218774"
                    .parse()
                    .unwrap(),
                "76085327132285723110424803456124867697064507995236"
                    .parse()
                    .unwrap(),
                "37774242535411291684276865538926205024910326572967"
                    .parse()
                    .unwrap(),
                "23701913275725675285653248258265463092207058596522"
                    .parse()
                    .unwrap(),
                "29798860272258331913126375147341994889534765745501"
                    .parse()
                    .unwrap(),
                "18495701454879288984856827726077713721403798879715"
                    .parse()
                    .unwrap(),
                "38298203783031473527721580348144513491373226651381"
                    .parse()
                    .unwrap(),
                "34829543829199918180278916522431027392251122869539"
                    .parse()
                    .unwrap(),
                "40957953066405232632538044100059654939159879593635"
                    .parse()
                    .unwrap(),
                "29746152185502371307642255121183693803580388584903"
                    .parse()
                    .unwrap(),
                "41698116222072977186158236678424689157993532961922"
                    .parse()
                    .unwrap(),
                "62467957194401269043877107275048102390895523597457"
                    .parse()
                    .unwrap(),
                "23189706772547915061505504953922979530901129967519"
                    .parse()
                    .unwrap(),
                "86188088225875314529584099251203829009407770775672"
                    .parse()
                    .unwrap(),
                "11306739708304724483816533873502340845647058077308"
                    .parse()
                    .unwrap(),
                "82959174767140363198008187129011875491310547126581"
                    .parse()
                    .unwrap(),
                "97623331044818386269515456334926366572897563400500"
                    .parse()
                    .unwrap(),
                "42846280183517070527831839425882145521227251250327"
                    .parse()
                    .unwrap(),
                "55121603546981200581762165212827652751691296897789"
                    .parse()
                    .unwrap(),
                "32238195734329339946437501907836945765883352399886"
                    .parse()
                    .unwrap(),
                "75506164965184775180738168837861091527357929701337"
                    .parse()
                    .unwrap(),
                "62177842752192623401942399639168044983993173312731"
                    .parse()
                    .unwrap(),
                "32924185707147349566916674687634660915035914677504"
                    .parse()
                    .unwrap(),
                "99518671430235219628894890102423325116913619626622"
                    .parse()
                    .unwrap(),
                "73267460800591547471830798392868535206946944540724"
                    .parse()
                    .unwrap(),
                "76841822524674417161514036427982273348055556214818"
                    .parse()
                    .unwrap(),
                "97142617910342598647204516893989422179826088076852"
                    .parse()
                    .unwrap(),
                "87783646182799346313767754307809363333018982642090"
                    .parse()
                    .unwrap(),
                "10848802521674670883215120185883543223812876952786"
                    .parse()
                    .unwrap(),
                "71329612474782464538636993009049310363619763878039"
                    .parse()
                    .unwrap(),
                "62184073572399794223406235393808339651327408011116"
                    .parse()
                    .unwrap(),
                "66627891981488087797941876876144230030984490851411"
                    .parse()
                    .unwrap(),
                "60661826293682836764744779239180335110989069790714"
                    .parse()
                    .unwrap(),
                "85786944089552990653640447425576083659976645795096"
                    .parse()
                    .unwrap(),
                "66024396409905389607120198219976047599490197230297"
                    .parse()
                    .unwrap(),
                "64913982680032973156037120041377903785566085089252"
                    .parse()
                    .unwrap(),
                "16730939319872750275468906903707539413042652315011"
                    .parse()
                    .unwrap(),
                "94809377245048795150954100921645863754710598436791"
                    .parse()
                    .unwrap(),
                "78639167021187492431995700641917969777599028300699"
                    .parse()
                    .unwrap(),
                "15368713711936614952811305876380278410754449733078"
                    .parse()
                    .unwrap(),
                "40789923115535562561142322423255033685442488917353"
                    .parse()
                    .unwrap(),
                "44889911501440648020369068063960672322193204149535"
                    .parse()
                    .unwrap(),
                "41503128880339536053299340368006977710650566631954"
                    .parse()
                    .unwrap(),
                "81234880673210146739058568557934581403627822703280"
                    .parse()
                    .unwrap(),
                "82616570773948327592232845941706525094512325230608"
                    .parse()
                    .unwrap(),
                "22918802058777319719839450180888072429661980811197"
                    .parse()
                    .unwrap(),
                "77158542502016545090413245809786882778948721859617"
                    .parse()
                    .unwrap(),
                "72107838435069186155435662884062257473692284509516"
                    .parse()
                    .unwrap(),
                "20849603980134001723930671666823555245252804609722"
                    .parse()
                    .unwrap(),
                "53503534226472524250874054075591789781264330331690"
                    .parse()
                    .unwrap(),
            ];
            for i in 0..numbers.len() {
                result += numbers[i];
            }
            return result.to_string().substring(0, 10).to_string();
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_013::problem_013::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!("5537376230", result);
        }
    }
}
