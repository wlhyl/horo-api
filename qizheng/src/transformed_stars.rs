use ganzhiwuxing::TianGan;
use lunar_calendar::LunarCalendar;

use crate::ten_gods::ToTenGods;
use crate::{PlanetName, ten_gods::TenGods};
use TransformedStar::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 行星的变曜
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub(crate) struct StarTransformedStar {
    /// 行星
    star: PlanetName,
    /// 变曜名
    transformed_star: TransformedStar,
    /// 变曜所管宫位
    transformed_star_house: &'static str,
    /// 变曜解释
    transformed_star_describe: &'static str,
    /// 十神
    ten_gods: TenGods,
}

/// 计算变星
#[inline]
pub(crate) fn transformed_stars(t: &LunarCalendar) -> Vec<StarTransformedStar> {
    let year_tian_gan = t.lunar_year_gan_zhi.gan();
    [天禄, 天暗, 天福, 天耗, 天荫, 天贵, 天刑, 天印, 天囚, 天权]
        .into_iter()
        .map(|ts| {
            let star = ts.planet(&t);
            let ten_gods = year_tian_gan.to_ten_gods(star.to_tian_gan().unwrap());
            StarTransformedStar {
                star,
                transformed_star: ts,
                transformed_star_house: ts.house(),
                transformed_star_describe: ts.describe(),
                ten_gods,
            }
        })
        .collect()
}

const PLANETS: [PlanetName; 10] = [
    PlanetName::火,
    PlanetName::孛,
    PlanetName::木,
    PlanetName::金,
    PlanetName::土,
    PlanetName::月,
    PlanetName::水,
    PlanetName::气,
    PlanetName::计,
    PlanetName::罗,
];

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy)]
pub(crate) enum TransformedStar {
    天禄,
    天暗,
    天福,
    天耗,
    天荫,
    天贵,
    天刑,
    天印,
    天囚,
    天权,
    // 科名,
    // 科甲,
    // 文星,
    // 魁星,
    // 官星,
    // 印星,
    // 催官,
    // 禄神,
    // 喜神,
    // 爵星,
    // 天马,
    // 地驿,
    // 天元禄,
    // 地元禄,
    // 人元禄,
    // 天经,
    // 地纬,
    // 天马0,
    // 地驿0,
    // 职元,
    // 局主,
    // 催官0,
}

impl TransformedStar {
    #[inline]
    pub(crate) fn house(&self) -> &'static str {
        match self {
            TransformedStar::天禄 => "官禄",
            TransformedStar::天暗 => "相貌",
            TransformedStar::天福 => "财帛、福德、迁移",
            TransformedStar::天耗 => "兄弟",
            TransformedStar::天荫 => "妻妾",
            TransformedStar::天贵 => "男女",
            TransformedStar::天刑 => "奴仆",
            TransformedStar::天印 => "田宅",
            TransformedStar::天囚 => "疾厄",
            TransformedStar::天权 => "命宫",
        }
    }

    #[inline]
    pub(crate) fn describe(&self) -> &'static str {
        match self {
            TransformedStar::天禄 => {
                "仕人遇之，主食俸禄，庶人有之，变主享福。禄主当生入命宫，田财旺气大亨通，官星更在高强位，年少声名达圣聪。凡禄主与官禄并详。一宜在七强宫，二宜照命，三喜顺行，四喜庙旺，五要在当生年纳音长生、临官、帝旺宫，以上并得用，主大富贵。如在五弱宫，或入四杀位，更行留、逆，故为福不纯。主人淹滞不遂。"
            }

            TransformedStar::天暗 => {
                "此谓吉星化囚，须吉而不吉也。富贵因何福不荣，只缘命里暗伤星，高强皆是为凶恶，入陷弧高祸自轻。凡天暗星与相貌同推。此星最忌在官宫及官、魁、文星、身命适之，皆无发达。"
            }
            TransformedStar::天福 => {
                "德者，福之基，亦要人之有德者，必得福寿也。身宫及命福星临，庙旺高强享福深，若遇陷宫并恶曜，荣华逍遥铄祸难禁。凡天福与福德、财帛、迁移同详。宜照福德为上，身命次之，男女宫见之，为上吉，缘此宫与福德相对，故为上喜。在庙旺兼行顺段为福。如在陷弱宫及伏逆留段为浅，更宜消详身命要本，方可断之。"
            }
            TransformedStar::天耗 => {
                "唐玄宗时，天耗星盗贵妃玉环及香囊而去，帝梦中叱之，答曰：吾天耗也。天耗之星不可逢，生来财帛化为空，若临贵地并权禄，尚自区区待限通。凡天耗星与兄弟同推，此星耗财之神，故以兄弟当之，忌在田财二宫，另宫无甚害。"
            }
            TransformedStar::天荫 => {
                "主父母蒙贵封荫之下。荫星逢著有操持，须是高强庙旺时，福禄印权并贵会，官荣极品耀天墀。凡天荫星与妻妾同推，喜入生旺宫，得妻财。入死绝宫主多病。顺行则吉，逆行则凶。七强宫见之相宜，五弱宫不利。此星居迁移主外壻，在奴仆不宜正婚，居四正宫，主有妻财，恶星不犯，主夫妻偕老也。"
            }
            TransformedStar::天贵 => {
                "吉星夹之有贵，凶星拱夹则贱。身遇高强及印权，命宫三合更相联，贵多刑少居官禄，职位荣华禄更迁。凡天贵星与男女并详。此星居七强顺行、乐、庙、旺宫及当生贵人禄马之位，主生贵子。在迁移、奴仆、兄弟宫，主过房子，或在五弱，被恶曜刑破，多主伤克，更男女宫星陷，主绝嗣。男女星好还有子。"
            }
            TransformedStar::天刑 => {
                "谓犯徒流贼盗刺字于面、于肘者。天刑若陷最为恶，身命田宅怕逢著，限临必主身不全，黥面文身方免却。凡天刑星与奴仆同推。此星在闲极宫无恶曜相犯奴仆得力，或在七强及得地，主仆从奸狡。更有凶星入宫，小人无故相侵犯凌辱。此星宜弱不宜强，宜顺不宜逆，若是命主星，又不妨，无自刑之理。"
            }
            TransformedStar::天印 => {
                "喜官禄等星。生来须有皇恩命，官禄高强赖此星，若遇科名科甲贵，因茲食禄播王庭。凡天印星与田宅并详。此星喜居田宅宫及七强入庙生旺之地，主多产业。如留逆无气更在闲极陷地，俱不得祖业。田宅宫别有吉星临照亦能自创立。如田宅有忌星相犯，加命弱失陷，必无田宅。"
            }
            TransformedStar::天囚 => {
                "若遇阑干、贯索相并，主牢狱之患。天囚若在四刑宫，脓血伤残命夭终，若是寿星临照著，也须为福不为凶。凡天囚星与疾厄同推。此星怕入七强生旺及在逆段，或照命，或临身，并不相宜也。若是紫木星为囚星，然其性本善，不可便以凶忌为嫌，但戊癸人见之减力，终不为祸。"
            }
            TransformedStar::天权 => {
                "主有威权，掌生杀之职。权星遇贵在高强，纵有刑囚亦不妨，更遇合宫高格局，定须官到紫微郎。凡天权星与命宫并详。此星照命及伴身，若入庙顺行，主得贵人扶持，更与太阳福禄同宫，尤奇。"
            }
        }
    }

    #[inline]
    pub(crate) fn planet(&self, t: &LunarCalendar) -> PlanetName {
        match self {
            TransformedStar::天禄 => 计算_天禄(t),
            TransformedStar::天暗 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 1) % 10]
            }
            TransformedStar::天福 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 2) % 10]
            }
            TransformedStar::天耗 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 3) % 10]
            }
            TransformedStar::天荫 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 4) % 10]
            }
            TransformedStar::天贵 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 5) % 10]
            }
            TransformedStar::天刑 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 6) % 10]
            }
            TransformedStar::天印 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 7) % 10]
            }
            TransformedStar::天囚 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 8) % 10]
            }
            TransformedStar::天权 => {
                let p = 计算_天禄(t);
                let index = PLANETS.iter().position(|&x| x == p).unwrap();
                PLANETS[(index + 9) % 10]
            }
        }
    }
}

#[inline]
fn 计算_天禄(t: &LunarCalendar) -> PlanetName {
    match t.lunar_year_gan_zhi.gan() {
        TianGan::甲 => PlanetName::火,
        TianGan::乙 => PlanetName::孛,
        TianGan::丙 => PlanetName::木,
        TianGan::丁 => PlanetName::金,
        TianGan::戊 => PlanetName::土,
        TianGan::己 => PlanetName::月,
        TianGan::庚 => PlanetName::水,
        TianGan::辛 => PlanetName::气,
        TianGan::壬 => PlanetName::计,
        TianGan::癸 => PlanetName::罗,
    }
}

// @VirtualStar(10, "科名", "假士子用之，主名高位重，庶人有之亦有声望。" +
//         "此上谓之十干科名星，要在七强顺段、庙旺，则名位高也。若在在五弱之宫及留逆段，虽中科甲，名次低也。")
// internal fun v10(t: HoroDateTime, geo :GeoPosition, ephePath :String?): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return TianGan(g).wuXing.toString()
// }

// @VirtualStar(11,"科甲", "主登高科，亦要庙旺得力，必中。科甲之星对命宫，对宫宫主要强隆，如居庙旺登高第，陷时及第必难逢。要在七强顺段庙旺中高甲，居陷弱，又退留伏段，虽贵，甲第必低。")
// internal fun v11(t: HoroDateTime, geo :GeoPosition, ephePath :String?): String {
//     val cusps = swe_houses(t.jdUT, geo.lat, geo.long, 'P')
//     if (cusps.rc < 0) throw HoroCallException("swe_houses()错误。")

//     /*
//         0 戌
//         1 酉
//         2 申
//         3 未
//         4 午
//         5 巳
//         6 辰
//         7 卯
//         8 寅
//         9 丑
//         10 子
//         11 亥
//      */
//     return when (((cusps.ascmc[0] / 30).toInt() + 6) % 12 ) {
//         0, 7 -> "火"
//         1, 6 -> "金"
//         2, 5 -> "水"
//         3 -> "月"
//         4 -> "日"
//         8, 11 -> "木"
//         else -> "土"
//     }
// }

// @VirtualStar(12,"文星", "文旺身衰，仲尼不仕，身旺文衰，廉颇就武。要相均平，则贵。")
// internal fun v12(t: HoroDateTime, geo :GeoPosition, ephePath :String?): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "罗"
//         "乙" -> "计"
//         "丙", "戊" -> "金"
//         "丁" -> "火"
//         "己" -> "气"
//         "庚" -> "木"
//         "辛" -> "土"
//         "壬" -> "日"
//         else -> "月"
//     }
// }

// @VirtualStar(13, "魁星", "魁星者，阴阳和合相生而成魁，独不以土为魁者，以土愚浊故也。" +
//         "（录入者：依原文，魁星似乎是看人聪明与否，读书是否厉害。）")
// internal fun v13(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "月"
//         "乙" -> "日"
//         "丙" -> "罗"
//         "丁" -> "计"
//         "戊" -> "火"
//         "己" -> "金"
//         "庚" -> "木"
//         "辛" -> "孛"
//         "壬" -> "气"
//         else -> "水"
//     }
// }

// @VirtualStar(14, "官星", "官者，辅君之星也，君以之而在政事。官星者，乃十干官星之禄星。琴堂谓对禄是也。" +
//         "甲以辛为官，辛以气为禄，则甲人用气为官星。十干皆以此例，独阳君不与者，以其官，君之所授也。")
// internal fun v14(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "气"
//         "乙" -> "水"
//         "丙" -> "罗"
//         "丁" -> "计"
//         "戊" -> "孛"
//         "己" -> "火"
//         "庚" -> "金"
//         "辛" -> "木"
//         "壬" -> "月"
//         else -> "土"
//     }
// }

// @VirtualStar(15, "印星", "印者，国之玺也，君主掌之以信天下之人，苟非其人而勿与焉。" +
//         "印星者，五行相符合而为印。甲以木，丙以火，戊土，庚金，壬水是也。五阴干取日月罗计孛亦以相生而相类。" +
//         "然十一曜独气不与者，以其善柔而非罗计孛之比焉。")
// internal fun v15(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "木"
//         "乙" -> "日"
//         "丙" -> "火"
//         "丁" -> "月"
//         "戊" -> "土"
//         "己" -> "罗"
//         "庚" -> "金"
//         "辛" -> "计"
//         "壬" -> "水"
//         else -> "孛"
//     }
// }

// @VirtualStar(16, "催官", "仕人赖之以荐举。催官之星主迁官进职也。" +
//         "大抵，此星与禄主相为催克。如甲人以火为禄，而见金则催。乙人以孛为禄，而见水则催。丙人以木为禄，而见日则催。" +
//         "丁人以金为禄，而见罗则催是也。十一曜独火不与者，以其有太阳在焉。")
// internal fun v16(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "金"
//         "乙" -> "水"
//         "丙" -> "日"
//         "丁" -> "罗"
//         "戊" -> "木"
//         "己" -> "气"
//         "庚" -> "孛"
//         "辛" -> "土"
//         "壬" -> "月"
//         else -> "计"
//     }
// }

// @VirtualStar(17, "禄神", "主俸禄。" +
//         "禄神者，正禄神遇之主食正俸禄。十一曜俱全而甲独兼木孛者，以其阳干之首也。")
// internal fun v17(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "木孛"
//         "乙" -> "水"
//         "丙" -> "计"
//         "丁" -> "罗"
//         "戊" -> "土"
//         "己" -> "火"
//         "庚" -> "金"
//         "辛" -> "气"
//         "壬" -> "日"
//         else -> "月"
//     }
// }

// @VirtualStar(18, "喜神", "主婚姻财喜之类。人命月逆行，喜神逆禄。十曜司禄而顺布，喜神随月以逆承，" +
//         "故禄之序，火孛木金土月水气计罗，而喜神之序，罗计气水月土金木孛火，是也。")
// internal fun v18(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "罗"
//         "乙" -> "计"
//         "丙" -> "气"
//         "丁" -> "水"
//         "戊" -> "月"
//         "己" -> "土"
//         "庚" -> "金"
//         "辛" -> "木"
//         "壬" -> "孛"
//         else -> "火"
//     }
// }

// @VirtualStar(19, "爵星", "主进爵除拜之星。爵者，地元爵也，自年支中出也。遇之进爵拜，最要官福身命有之，又喜在高强相遇。")
// internal fun v19(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[1].toString()
//     return when(g){
//         "子", "申" -> "土"
//         "亥", "未" -> "火"
//         "午", "丑" -> "水"
//         "卯" -> "气"
//         "寅", "巳" -> "木"
//         "酉", "戌" -> "金"
//         else -> "孛"
//     }
// }

// @VirtualStar(20, "天马", "主升迁驰之类。" +
//         "其法以驿马宫遁禄干所属为地驿，禄干化禄为天马。如申子辰人，马居寅，则遁甲禄在寅，以甲木为地驿，以甲火为天马。" +
//         "又寅午戌人，马居申，则遁庚禄居申，以庚金为地驿，以庚水为天马。余效此推。" +
//         "果老有天马地驿起例不同，以五虎遁官禄宫取二者，姑并详之，惟果老者为切。")
// internal fun v20(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[1].toString()
//     return when(g){
//         "申", "子", "辰" -> "火"
//         "寅", "午", "戌" -> "水"
//         "亥", "卯", "未" -> "木"
//         else -> "计"
//     }
// }

// @VirtualStar(21, "地驿", "主升迁驰之类。" +
//         "其法以驿马宫遁禄干所属为地驿，禄干化禄为天马。如申子辰人，马居寅，则遁甲禄在寅，以甲木为地驿，以甲火为天马。" +
//         "又寅午戌人，马居申，则遁庚禄居申，以庚金为地驿，以庚水为天马。余效此推。" +
//         "果老有天马地驿起例不同，以五虎遁官禄宫取二者，姑并详之，惟果老者为切。")
// internal fun v21(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[1].toString()
//     return when(g){
//         "申", "子", "辰" -> "木"
//         "寅", "午", "戌" -> "金"
//         "亥", "卯", "未" -> "火"
//         else -> "水"
//     }
// }

// @VirtualStar(22, "天元禄", "满用一星而得三用也。" +
//         "十曜天元化禄因，五遁迤前命位寻，何干化禄为天禄，最宜满用喜垣城。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v22(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     val tianGan = when(g){
//         "甲", "己" -> TianGan("丙")
//         "乙", "庚"-> TianGan("戊")
//         "丙", "辛"-> TianGan("庚")
//         "丁", "壬"-> TianGan("壬")
//         else -> TianGan("甲")
//     }
//     val firstHouse = getFirstHouse(t, geo, ephePath)
//     val lu = tianGan + (firstHouse - DiZhi("寅"))
//     return ganVirtualStars[lu - TianGan("甲")]
// }

// @VirtualStar(23, "地元禄", "取卦气逆命何干属某是。欲识地元推卦气，逆至命里即干神，遇有吉星临满用，定拟金殿玉阶行。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v23(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val guaQi = guaQi(t, geo, ephePath)
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     val firstHouse = getFirstHouse(t, geo, ephePath)
//     val lu = TianGan(g) + (guaQi - firstHouse)
//     return lu.wuXing.toString()
// }

// @VirtualStar(24, "人元禄", "以虎顺宫，何干属某干为是。人元虎遁来官禄，以干受克的为真，中间若有闲神杂，斟酌当加仔细论。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v24(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     val tianGan = when(g){
//         "甲", "己" -> TianGan("丙")
//         "乙", "庚"-> TianGan("戊")
//         "丙", "辛"-> TianGan("庚")
//         "丁", "壬"-> TianGan("壬")
//         else -> TianGan("甲")
//     }
//     // 命宫与官禄宫差3位
//     val wuxingName: Array<String> = arrayOf("木", "火", "土", "金", "水")
//     val n = wuxingName.indexOf((tianGan + 3).wuXing.toString())
//     return wuxingName[(n + 3) % wuxingName.size]
// }

// @VirtualStar(25, "天经", "二星用星不用宫，以虎顺，命干属某是经，以命支神属某是纬。" +
//         "星有天经地纬神，虎看过轮流到命程，地支支神名地纬，天干干主是天经。身命逢他来拱夹，经天纬地有才能。若值斗标来指破，一生名利算轻尘。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v25(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     val tianGan = when(g){
//         "甲", "己" -> TianGan("丙")
//         "乙", "庚"-> TianGan("戊")
//         "丙", "辛"-> TianGan("庚")
//         "丁", "壬"-> TianGan("壬")
//         else -> TianGan("甲")
//     }
//     val firstHouse = getFirstHouse(t, geo, ephePath)
//     return (tianGan + (firstHouse - DiZhi("寅"))).wuXing.toString()
// }

// @VirtualStar(26, "地纬", "二星用星不用宫，以虎顺，命干属某是经，以命支神属某是纬。" +
//         "星有天经地纬神，虎看过轮流到命程，地支支神名地纬，天干干主是天经。身命逢他来拱夹，经天纬地有才能。若值斗标来指破，一生名利算轻尘。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v26(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val firstHouse = getFirstHouse(t, geo, ephePath)
//     return firstHouse.wuXing.toString()
// }

// @VirtualStar(27, "天马", "天马地驿，果老所著也，与前诸家不同。" +
//         "若驿马有二属星者，则取左右夹拱用之。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v27(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     val tenthHouse = getFirstHouse(t, geo, ephePath) + 3
//     val tg = wuHu(TianGan(g)) + (tenthHouse - DiZhi("寅"))
//     return tianGanLu(tg).wuXing.toString()
// }

// @VirtualStar(28, "地驿", "天马地驿，果老所著也，与前诸家不同。" +
//         "若驿马有二属星者，则取左右夹拱用之。" +
//         "(录入者：流年此星数据不准，忽略。)")
// internal fun v28(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val tenthHouse = getFirstHouse(t, geo, ephePath) + 3
//     return yiMa(tenthHouse).wuXing.toString()
// }

// @VirtualStar(29, "职元", "主职分。" +
//         "卦气顺行至命程，干属化曜职元星，身命逢之居中显职，凡人遇此干才能。" +
//         "(录入者：流年此星数据不准，忽略。本命此星，《果老星宗》的计算方法有些模糊，此数据可能不能准。)")
// internal fun v29(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     val guaQi = guaQi(t, geo, ephePath)
//     val firstHouse = getFirstHouse(t, geo, ephePath)
//     val tg = TianGan(g) + (firstHouse - guaQi)
//     val n = tg - TianGan("甲")
//     return ganVirtualStars[n]
// }

// @VirtualStar(30, "局主", "职元六合推干化，即是一局主之辰，命身值此无空破，士庶逢之众仰钦。" +
//         "(录入者：流年此星数据不准，忽略。局主的数据依赖职元。)")
// internal fun v30(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val zhiYuan = v29(t, geo, ephePath)
//     val zhiYuanGan = TianGan("甲") + ganVirtualStars.indexOf(zhiYuan)
//     val juZhu = zhiYuanGan + 5
//     val n = juZhu - TianGan("甲")
//     return ganVirtualStars[n]
// }

// /*
// @VirtualStar(31, "催官", "")
// internal fun v31(t: HoroDateTime, geo :GeoPosition, ephePath :String?=null): String{
//     val g = getLunarCalendar(t, geo, ephePath).yearGanZhi[0].toString()
//     return when(g){
//         "甲" -> "金"
//         "乙" -> "水"
//         "丙" -> "日"
//         "丁" -> "罗"
//         "戊" -> "木"
//         "己" -> "气"
//         "庚" -> "孛"
//         "辛" -> "土"
//         "壬" -> "月"
//         else -> "计"
//     }
// }
// */
