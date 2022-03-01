use std::collections::HashMap;

pub struct Font {
    pub scale_w: i32,
    pub scale_h: i32,
    pub page_files: Vec<String>,
    pub data: HashMap<u16, [i16; 8]>,
}

impl Font {
    pub fn new() -> Font {
        Font {
            scale_w: 256,
            scale_h: 64,
            page_files: get_pages(),
            data: get_data(),
        }
    }
}

fn get_pages() -> Vec<String> {
    let mut pages: Vec<String> = Vec::new();
    pages.push(String::from("assets/mgfw/alagard.png"));
    pages
}

#[rustfmt::skip]
fn get_data() -> HashMap<u16, [i16; 8]> {
    let mut data: HashMap<u16, [i16; 8]> = HashMap::new();

    //char id=32   x=42    y=37    width=3     height=1     xoffset=-1    yoffset=14    xadvance=5     page=0
    data.insert(32   ,[42    ,37    ,3     ,1     ,-1    ,14    ,5     ,0  ]);
    data.insert(33   ,[15    ,27    ,4     ,11    ,0     ,1     ,5     ,0  ]);
    data.insert(34   ,[240   ,24    ,7     ,5     ,0     ,1     ,8     ,0  ]);
    data.insert(35   ,[58    ,26    ,10    ,10    ,0     ,2     ,11    ,0  ]);
    data.insert(36   ,[0     ,0     ,8     ,14    ,0     ,0     ,9     ,0  ]);
    data.insert(37   ,[77    ,0     ,12    ,12    ,0     ,1     ,13    ,0  ]);
    data.insert(38   ,[111   ,0     ,11    ,11    ,0     ,1     ,12    ,0  ]);
    data.insert(39   ,[252   ,24    ,3     ,5     ,0     ,1     ,4     ,0  ]);
    data.insert(40   ,[16    ,0     ,6     ,14    ,0     ,1     ,7     ,0  ]);
    data.insert(41   ,[9     ,0     ,6     ,14    ,1     ,1     ,8     ,0  ]);
    data.insert(42   ,[224   ,24    ,6     ,6     ,0     ,1     ,7     ,0  ]);
    data.insert(43   ,[47    ,26    ,10    ,10    ,0     ,2     ,11    ,0  ]);
    data.insert(44   ,[248   ,24    ,3     ,5     ,0     ,10    ,4     ,0  ]);
    data.insert(45   ,[35    ,38    ,6     ,1     ,0     ,8     ,7     ,0  ]);
    data.insert(46   ,[21    ,39    ,2     ,2     ,1     ,10    ,4     ,0  ]);
    data.insert(47   ,[36    ,26    ,10    ,10    ,1     ,2     ,12    ,0  ]);
    data.insert(48   ,[218   ,12    ,7     ,11    ,0     ,1     ,8     ,0  ]);
    data.insert(49   ,[0     ,27    ,4     ,11    ,0     ,1     ,5     ,0  ]);
    data.insert(50   ,[242   ,12    ,6     ,11    ,0     ,1     ,7     ,0  ]);
    data.insert(51   ,[194   ,12    ,7     ,11    ,0     ,1     ,8     ,0  ]);
    data.insert(52   ,[197   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(53   ,[226   ,12    ,7     ,11    ,0     ,1     ,8     ,0  ]);
    data.insert(54   ,[176   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(55   ,[185   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(56   ,[86    ,13    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(57   ,[247   ,0     ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(58   ,[221   ,24    ,2     ,7     ,1     ,5     ,4     ,0  ]);
    data.insert(59   ,[85    ,25    ,3     ,10    ,0     ,5     ,4     ,0  ]);
    data.insert(60   ,[89    ,25    ,5     ,9     ,0     ,3     ,6     ,0  ]);
    data.insert(61   ,[9     ,39    ,7     ,4     ,1     ,6     ,9     ,0  ]);
    data.insert(62   ,[95    ,25    ,5     ,9     ,1     ,3     ,7     ,0  ]);
    data.insert(63   ,[69    ,26    ,7     ,10    ,0     ,2     ,8     ,0  ]);
    data.insert(64   ,[30    ,0     ,12    ,13    ,0     ,2     ,13    ,0  ]);
    data.insert(65   ,[59    ,14    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(66   ,[68    ,14    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(67   ,[77    ,13    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(68   ,[0     ,15    ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(69   ,[187   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(70   ,[30    ,14    ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(71   ,[95    ,13    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(72   ,[207   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(73   ,[20    ,27    ,4     ,11    ,0     ,1     ,5     ,0  ]);
    data.insert(74   ,[73    ,0     ,3     ,13    ,0     ,1     ,4     ,0  ]);
    data.insert(75   ,[10    ,15    ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(76   ,[202   ,12    ,7     ,11    ,0     ,1     ,8     ,0  ]);
    data.insert(77   ,[98    ,0     ,12    ,11    ,0     ,1     ,13    ,0  ]);
    data.insert(78   ,[146   ,0     ,10    ,11    ,0     ,1     ,11    ,0  ]);
    data.insert(79   ,[104   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(80   ,[113   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(81   ,[43    ,0     ,9     ,13    ,0     ,1     ,10    ,0  ]);
    data.insert(82   ,[20    ,15    ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(83   ,[50    ,14    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(84   ,[227   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(85   ,[135   ,0     ,10    ,11    ,0     ,1     ,11    ,0  ]);
    data.insert(86   ,[217   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(87   ,[123   ,0     ,11    ,11    ,0     ,1     ,12    ,0  ]);
    data.insert(88   ,[167   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(89   ,[157   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(90   ,[122   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(91   ,[58    ,0     ,4     ,13    ,1     ,1     ,6     ,0  ]);
    data.insert(92   ,[25    ,27    ,10    ,10    ,0     ,2     ,11    ,0  ]);
    data.insert(93   ,[68    ,0     ,4     ,13    ,0     ,1     ,5     ,0  ]);
    data.insert(94   ,[231   ,24    ,8     ,5     ,0     ,1     ,9     ,0  ]);
    data.insert(95   ,[24    ,39    ,10    ,1     ,-1    ,11    ,10    ,0  ]);
    data.insert(96   ,[17    ,39    ,3     ,3     ,2     ,1     ,6     ,0  ]);
    data.insert(97   ,[174   ,24    ,7     ,8     ,0     ,4     ,8     ,0  ]);
    data.insert(98   ,[131   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(99   ,[190   ,24    ,7     ,8     ,0     ,4     ,8     ,0  ]);
    data.insert(100  ,[140   ,12    ,8     ,11    ,0     ,1     ,9     ,0  ]);
    data.insert(101  ,[198   ,24    ,7     ,8     ,0     ,4     ,8     ,0  ]);
    data.insert(102  ,[210   ,12    ,7     ,11    ,0     ,1     ,8     ,0  ]);
    data.insert(103  ,[149   ,12    ,8     ,11    ,0     ,4     ,9     ,0  ]);
    data.insert(104  ,[177   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(105  ,[10    ,27    ,4     ,11    ,0     ,1     ,5     ,0  ]);
    data.insert(106  ,[23    ,0     ,3     ,14    ,0     ,1     ,4     ,0  ]);
    data.insert(107  ,[237   ,0     ,9     ,11    ,0     ,1     ,10    ,0  ]);
    data.insert(108  ,[5     ,27    ,4     ,11    ,0     ,1     ,5     ,0  ]);
    data.insert(109  ,[101   ,25    ,12    ,8     ,0     ,4     ,13    ,0  ]);
    data.insert(110  ,[137   ,24    ,9     ,8     ,0     ,4     ,10    ,0  ]);
    data.insert(111  ,[166   ,24    ,7     ,8     ,0     ,4     ,8     ,0  ]);
    data.insert(112  ,[158   ,12    ,8     ,11    ,0     ,4     ,9     ,0  ]);
    data.insert(113  ,[167   ,12    ,8     ,11    ,0     ,4     ,9     ,0  ]);
    data.insert(114  ,[182   ,24    ,7     ,8     ,0     ,4     ,8     ,0  ]);
    data.insert(115  ,[214   ,24    ,6     ,8     ,0     ,4     ,7     ,0  ]);
    data.insert(116  ,[249   ,12    ,5     ,11    ,0     ,1     ,6     ,0  ]);
    data.insert(117  ,[147   ,24    ,9     ,8     ,0     ,4     ,10    ,0  ]);
    data.insert(118  ,[157   ,24    ,8     ,8     ,0     ,4     ,9     ,0  ]);
    data.insert(119  ,[114   ,24    ,11    ,8     ,0     ,4     ,12    ,0  ]);
    data.insert(120  ,[126   ,24    ,10    ,8     ,0     ,4     ,11    ,0  ]);
    data.insert(121  ,[234   ,12    ,7     ,11    ,0     ,4     ,8     ,0  ]);
    data.insert(122  ,[206   ,24    ,7     ,8     ,0     ,4     ,8     ,0  ]);
    data.insert(123  ,[53    ,0     ,4     ,13    ,0     ,2     ,5     ,0  ]);
    data.insert(124  ,[27    ,0     ,2     ,14    ,2     ,1     ,5     ,0  ]);
    data.insert(125  ,[63    ,0     ,4     ,13    ,0     ,2     ,5     ,0  ]);
    data.insert(126  ,[0     ,39    ,8     ,4     ,0     ,6     ,9     ,0  ]);
    data.insert(162  ,[90    ,0     ,7     ,12    ,0     ,2     ,8     ,0  ]);
    data.insert(163  ,[77    ,25    ,7     ,10    ,0     ,2     ,8     ,0  ]);
    data.insert(165  ,[40    ,14    ,9     ,11    ,0     ,1     ,10    ,0  ]);


    data
}
