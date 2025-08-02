//! Generated from czech.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    b_found_suffix: bool,
    i_p1: i32,
}

static A_0: &'static [Among<Context>; 16] = &[
    Among("", -1, 14, None),
    Among("á", 0, 1, None),
    Among("é", 0, 2, None),
    Among("í", 0, 3, None),
    Among("ó", 0, 4, None),
    Among("ú", 0, 5, None),
    Among("ý", 0, 6, None),
    Among("č", 0, 7, None),
    Among("ď", 0, 8, None),
    Among("ě", 0, 2, None),
    Among("ň", 0, 9, None),
    Among("ř", 0, 10, None),
    Among("š", 0, 11, None),
    Among("ť", 0, 12, None),
    Among("ů", 0, 5, None),
    Among("ž", 0, 13, None),
];

static A_1: &'static [Among<Context>; 125] = &[
    Among("bude", -1, 1, None),
    Among("budeme", 0, 1, None),
    Among("budete", 0, 1, None),
    Among("budeš", 0, 1, None),
    Among("budou", -1, 1, None),
    Among("budu", -1, 1, None),
    Among("buď", -1, 1, None),
    Among("buďme", 6, 1, None),
    Among("buďte", 6, 1, None),
    Among("byl", -1, 1, None),
    Among("chce", -1, 8, None),
    Among("chceme", 10, 8, None),
    Among("chcete", 10, 8, None),
    Among("chceš", 10, 8, None),
    Among("chci", -1, 8, None),
    Among("chtíc", -1, 8, None),
    Among("chtíce", 15, 8, None),
    Among("chtě", -1, 8, None),
    Among("chtěj", 17, 8, None),
    Among("chtěje", 18, 8, None),
    Among("chtějme", 18, 8, None),
    Among("chtějte", 18, 8, None),
    Among("chtějí", 18, 8, None),
    Among("chtějíc", 22, 8, None),
    Among("chtějíce", 23, 8, None),
    Among("chtěl", 17, 8, None),
    Among("chtěn", 17, 8, None),
    Among("jda", -1, 3, None),
    Among("jde", -1, 3, None),
    Among("jdeme", 28, 3, None),
    Among("jdete", 28, 3, None),
    Among("jdeš", 28, 3, None),
    Among("jdi", -1, 3, None),
    Among("jdou", -1, 3, None),
    Among("jdouc", 33, 3, None),
    Among("jdouce", 34, 3, None),
    Among("jdu", -1, 3, None),
    Among("jděme", -1, 3, None),
    Among("jděte", -1, 3, None),
    Among("je", -1, 1, None),
    Among("jeda", 39, 4, None),
    Among("jeden", 39, 4, None),
    Among("jedl", 39, 4, None),
    Among("jedouc", 39, 4, None),
    Among("jedouce", 43, 4, None),
    Among("jedí", 39, 4, None),
    Among("jez", 39, 4, None),
    Among("jezme", 46, 4, None),
    Among("jezte", 46, 4, None),
    Among("jsa", -1, 1, None),
    Among("jsem", -1, 1, None),
    Among("jsi", -1, 1, None),
    Among("jsme", -1, 1, None),
    Among("jsou", -1, 1, None),
    Among("jsouc", 53, 1, None),
    Among("jsouce", 54, 1, None),
    Among("jste", -1, 1, None),
    Among("jí", -1, 4, None),
    Among("jím", 57, 4, None),
    Among("jíme", 58, 4, None),
    Among("jíte", 57, 4, None),
    Among("jíš", 57, 4, None),
    Among("maje", -1, 2, None),
    Among("mají", -1, 2, None),
    Among("majíc", 63, 2, None),
    Among("majíce", 64, 2, None),
    Among("má", -1, 2, None),
    Among("mám", 66, 2, None),
    Among("máme", 67, 2, None),
    Among("máte", 66, 2, None),
    Among("máš", 66, 2, None),
    Among("měj", -1, 2, None),
    Among("mějme", 71, 2, None),
    Among("mějte", 71, 2, None),
    Among("měl", -1, 2, None),
    Among("pojď", -1, 3, None),
    Among("pojďte", 75, 3, None),
    Among("půjde", -1, 3, None),
    Among("půjdeme", 77, 3, None),
    Among("půjdete", 77, 3, None),
    Among("půjdeš", 77, 3, None),
    Among("půjdou", -1, 3, None),
    Among("půjdu", -1, 3, None),
    Among("sní", -1, 5, None),
    Among("sním", 83, 5, None),
    Among("sníme", 84, 5, None),
    Among("sníte", 83, 5, None),
    Among("sníš", 83, 5, None),
    Among("sněden", -1, 5, None),
    Among("snědl", -1, 5, None),
    Among("snědí", -1, 5, None),
    Among("sněz", -1, 5, None),
    Among("snězme", 91, 5, None),
    Among("snězte", 91, 5, None),
    Among("vida", -1, 6, None),
    Among("vidouc", -1, 6, None),
    Among("vidouce", 95, 6, None),
    Among("vidí", -1, 6, None),
    Among("vidím", 97, 6, None),
    Among("vidíme", 98, 6, None),
    Among("vidíte", 97, 6, None),
    Among("vidíš", 97, 6, None),
    Among("viděl", -1, 6, None),
    Among("viděn", -1, 6, None),
    Among("viz", -1, 6, None),
    Among("vizme", 104, 6, None),
    Among("vizte", 104, 6, None),
    Among("ví", -1, 7, None),
    Among("vím", 107, 7, None),
    Among("víme", 108, 7, None),
    Among("víte", 107, 7, None),
    Among("víš", 107, 7, None),
    Among("věda", -1, 7, None),
    Among("vědouc", -1, 7, None),
    Among("vědouce", 113, 7, None),
    Among("vědí", -1, 7, None),
    Among("věděl", -1, 7, None),
    Among("věděn", -1, 7, None),
    Among("věz", -1, 7, None),
    Among("vězme", 118, 7, None),
    Among("vězte", 118, 7, None),
    Among("šel", -1, 3, None),
    Among("šla", -1, 3, None),
    Among("šli", -1, 3, None),
    Among("šlo", -1, 3, None),
];

static A_2: &'static [Among<Context>; 170] = &[
    Among("ba", -1, 1, None),
    Among("oba", 0, 1, None),
    Among("tba", 0, 1, None),
    Among("itba", 2, 1, None),
    Among("nda", -1, 1, None),
    Among("ucha", -1, 1, None),
    Among("ka", -1, 1, None),
    Among("ika", 6, 1, None),
    Among("nka", 6, 1, None),
    Among("enka", 8, 1, None),
    Among("inka", 8, 1, None),
    Among("ěnka", 8, 1, None),
    Among("tka", 6, 1, None),
    Among("vka", 6, 1, None),
    Among("čka", 6, 1, None),
    Among("ečka", 14, 1, None),
    Among("íčka", 14, 1, None),
    Among("uška", 6, 1, None),
    Among("téka", 6, 1, None),
    Among("ala", -1, 1, None),
    Among("ála", -1, 1, None),
    Among("na", -1, 1, None),
    Among("ina", 21, 1, None),
    Among("tina", 22, 1, None),
    Among("ština", 23, 1, None),
    Among("ovina", 22, 1, None),
    Among("telna", 21, 1, None),
    Among("árna", 21, 1, None),
    Among("írna", 21, 1, None),
    Among("ovna", 21, 1, None),
    Among("izna", 21, 1, None),
    Among("ajzna", 21, 1, None),
    Among("ura", -1, 1, None),
    Among("tura", 32, 1, None),
    Among("ita", -1, 1, None),
    Among("ota", -1, 1, None),
    Among("ista", -1, 1, None),
    Among("tva", -1, 1, None),
    Among("ec", -1, 1, None),
    Among("inec", 38, 1, None),
    Among("obinec", 39, 1, None),
    Among("tec", 38, 1, None),
    Among("ovec", 38, 1, None),
    Among("řad", -1, 1, None),
    Among("vod", -1, 1, None),
    Among("ce", -1, 1, None),
    Among("ace", 45, 1, None),
    Among("ice", 45, 1, None),
    Among("nice", 47, 1, None),
    Among("enice", 48, 1, None),
    Among("nce", 45, 1, None),
    Among("ance", 50, 1, None),
    Among("ence", 50, 1, None),
    Among("ie", -1, 1, None),
    Among("erie", 53, 1, None),
    Among("ule", -1, 1, None),
    Among("se", -1, 1, None),
    Among("xe", -1, 1, None),
    Among("ze", -1, 1, None),
    Among("če", -1, 1, None),
    Among("graf", -1, 1, None),
    Among("ing", -1, 1, None),
    Among("log", -1, 1, None),
    Among("och", -1, 1, None),
    Among("ek", -1, 1, None),
    Among("ánek", 64, 1, None),
    Among("ínek", 64, 1, None),
    Among("ásek", 64, 1, None),
    Among("eček", 64, 1, None),
    Among("áček", 64, 1, None),
    Among("íček", 64, 1, None),
    Among("oušek", 64, 1, None),
    Among("ik", -1, 1, None),
    Among("ink", -1, 1, None),
    Among("unk", -1, 1, None),
    Among("uňk", -1, 1, None),
    Among("ák", -1, 1, None),
    Among("ík", -1, 1, None),
    Among("ník", 77, 2, None),
    Among("ovník", 78, 1, None),
    Among("tel", -1, 1, None),
    Among("ovatel", 80, 1, None),
    Among("itel", 80, 1, None),
    Among("ál", -1, 1, None),
    Among("nom", -1, 1, None),
    Among("ium", -1, 1, None),
    Among("ikum", -1, 1, None),
    Among("ivum", -1, 1, None),
    Among("an", -1, 1, None),
    Among("čan", 88, 1, None),
    Among("oun", -1, 1, None),
    Among("loun", 90, 1, None),
    Among("án", -1, 1, None),
    Among("ián", 92, 1, None),
    Among("ín", -1, 1, None),
    Among("ko", -1, 1, None),
    Among("isko", 95, 1, None),
    Among("ovisko", 96, 1, None),
    Among("tko", 95, 1, None),
    Among("átko", 98, 1, None),
    Among("ečko", 95, 1, None),
    Among("éčko", 95, 1, None),
    Among("íčko", 95, 1, None),
    Among("lo", -1, 1, None),
    Among("dlo", 103, 1, None),
    Among("no", -1, 1, None),
    Among("čno", 105, 1, None),
    Among("ivo", -1, 1, None),
    Among("tvo", -1, 1, None),
    Among("stvo", 108, 1, None),
    Among("ovstvo", 109, 1, None),
    Among("er", -1, 1, None),
    Among("or", -1, 1, None),
    Among("tor", 112, 1, None),
    Among("átor", 113, 1, None),
    Among("our", -1, 1, None),
    Among("měr", -1, 1, None),
    Among("ér", -1, 1, None),
    Among("iér", 117, 1, None),
    Among("atér", 117, 1, None),
    Among("as", -1, 1, None),
    Among("ismus", -1, 1, None),
    Among("met", -1, 1, None),
    Among("ant", -1, 1, None),
    Among("ent", -1, 1, None),
    Among("ment", 124, 1, None),
    Among("ot", -1, 1, None),
    Among("est", -1, 1, None),
    Among("ost", -1, 1, None),
    Among("nost", 128, 1, None),
    Among("out", -1, 1, None),
    Among("át", -1, 1, None),
    Among("ov", -1, 1, None),
    Among("iny", -1, 1, None),
    Among("oň", -1, 1, None),
    Among("áč", -1, 1, None),
    Among("ař", -1, 1, None),
    Among("toř", -1, 1, None),
    Among("ář", -1, 1, None),
    Among("kář", 138, 1, None),
    Among("ionář", 138, 1, None),
    Among("éř", -1, 1, None),
    Among("néř", 141, 1, None),
    Among("íř", -1, 1, None),
    Among("ýř", -1, 1, None),
    Among("yně", -1, 1, None),
    Among("kyně", 145, 1, None),
    Among("iště", -1, 1, None),
    Among("oviště", 147, 1, None),
    Among("ná", -1, 1, None),
    Among("ová", -1, 1, None),
    Among("ouš", -1, 1, None),
    Among("ýš", -1, 1, None),
    Among("né", -1, 1, None),
    Among("ovné", 153, 1, None),
    Among("řadí", -1, 1, None),
    Among("ní", -1, 1, None),
    Among("aní", 156, 1, None),
    Among("ení", 156, 1, None),
    Among("ění", 156, 1, None),
    Among("ání", 156, 1, None),
    Among("tí", -1, 1, None),
    Among("etí", 161, 1, None),
    Among("ětí", 161, 1, None),
    Among("oví", -1, 1, None),
    Among("sloví", 164, 1, None),
    Among("ctví", -1, 1, None),
    Among("ství", -1, 1, None),
    Among("čí", -1, 1, None),
    Among("áž", -1, 1, None),
];

static A_3: &'static [Among<Context>; 136] = &[
    Among("a", -1, 17, None),
    Among("ba", 0, 17, None),
    Among("ga", 0, 11, None),
    Among("ka", 0, 17, None),
    Among("la", 0, 3, None),
    Among("na", 0, 17, None),
    Among("ra", 0, 17, None),
    Among("ata", 0, 17, None),
    Among("va", 0, 17, None),
    Among("eb", -1, 1, None),
    Among("ec", -1, 17, None),
    Among("e", -1, 17, None),
    Among("ce", 11, 17, None),
    Among("le", 11, 3, None),
    Among("ete", 11, 6, None),
    Among("ěte", 11, 15, None),
    Among("če", 11, 2, None),
    Among("g", -1, 17, None),
    Among("ech", -1, 17, None),
    Among("lech", 18, 3, None),
    Among("nech", 18, 7, None),
    Among("atech", 18, 17, None),
    Among("vech", 18, 12, None),
    Among("ách", -1, 13, None),
    Among("bách", 23, 1, None),
    Among("kách", 23, 17, None),
    Among("nách", 23, 5, None),
    Among("rách", 23, 4, None),
    Among("vách", 23, 8, None),
    Among("ích", -1, 17, None),
    Among("cích", 29, 17, None),
    Among("ních", 29, 17, None),
    Among("tích", 29, 10, None),
    Among("zích", 29, 17, None),
    Among("i", -1, 17, None),
    Among("ci", 34, 17, None),
    Among("ami", 34, 13, None),
    Among("bami", 36, 1, None),
    Among("kami", 36, 16, None),
    Among("nami", 36, 5, None),
    Among("rami", 36, 4, None),
    Among("vami", 36, 8, None),
    Among("emi", 34, 17, None),
    Among("ěmi", 34, 15, None),
    Among("němi", 43, 14, None),
    Among("ni", 34, 17, None),
    Among("ti", 34, 10, None),
    Among("eti", 46, 6, None),
    Among("ěti", 46, 15, None),
    Among("ovi", 34, 17, None),
    Among("govi", 49, 11, None),
    Among("kovi", 49, 9, None),
    Among("zi", 34, 17, None),
    Among("k", -1, 17, None),
    Among("ek", 53, 17, None),
    Among("el", -1, 3, None),
    Among("em", -1, 17, None),
    Among("cem", 56, 2, None),
    Among("gem", 56, 11, None),
    Among("kem", 56, 17, None),
    Among("lem", 56, 3, None),
    Among("nem", 56, 7, None),
    Among("etem", 56, 6, None),
    Among("ětem", 56, 15, None),
    Among("vem", 56, 12, None),
    Among("um", -1, 17, None),
    Among("kum", 65, 17, None),
    Among("těm", -1, 10, None),
    Among("ám", -1, 13, None),
    Among("bám", 68, 1, None),
    Among("kám", 68, 16, None),
    Among("nám", 68, 5, None),
    Among("rám", 68, 4, None),
    Among("vám", 68, 8, None),
    Among("ím", -1, 17, None),
    Among("ním", 74, 17, None),
    Among("tím", 74, 10, None),
    Among("ům", -1, 17, None),
    Among("cům", 77, 2, None),
    Among("gům", 77, 11, None),
    Among("kům", 77, 17, None),
    Among("lům", 77, 3, None),
    Among("nům", 77, 7, None),
    Among("atům", 77, 17, None),
    Among("vům", 77, 12, None),
    Among("en", -1, 17, None),
    Among("o", -1, 17, None),
    Among("bo", 86, 1, None),
    Among("ko", 86, 17, None),
    Among("lo", 86, 17, None),
    Among("no", 86, 17, None),
    Among("ro", 86, 4, None),
    Among("vo", 86, 17, None),
    Among("r", -1, 4, None),
    Among("at", -1, 17, None),
    Among("u", -1, 17, None),
    Among("bu", 95, 1, None),
    Among("gu", 95, 11, None),
    Among("ku", 95, 17, None),
    Among("lu", 95, 3, None),
    Among("nu", 95, 17, None),
    Among("ou", 95, 13, None),
    Among("bou", 101, 1, None),
    Among("kou", 101, 16, None),
    Among("nou", 101, 5, None),
    Among("rou", 101, 4, None),
    Among("vou", 101, 8, None),
    Among("ru", 95, 4, None),
    Among("vu", 95, 17, None),
    Among("ev", -1, 17, None),
    Among("y", -1, 17, None),
    Among("by", 110, 1, None),
    Among("gy", 110, 11, None),
    Among("ky", 110, 17, None),
    Among("ly", 110, 3, None),
    Among("ny", 110, 17, None),
    Among("ry", 110, 4, None),
    Among("aty", 110, 17, None),
    Among("vy", 110, 17, None),
    Among("ň", -1, 17, None),
    Among("eň", 119, 17, None),
    Among("ě", -1, 17, None),
    Among("bě", 121, 1, None),
    Among("ně", 121, 17, None),
    Among("tě", 121, 17, None),
    Among("vě", 121, 8, None),
    Among("ť", -1, 10, None),
    Among("é", -1, 17, None),
    Among("ové", 127, 17, None),
    Among("gové", 128, 11, None),
    Among("í", -1, 17, None),
    Among("ní", 130, 17, None),
    Among("ů", -1, 17, None),
    Among("ců", 132, 2, None),
    Among("gů", 132, 11, None),
    Among("ků", 132, 17, None),
];

static A_4: &'static [Among<Context>; 77] = &[
    Among("ina", -1, 2, None),
    Among("ova", -1, 1, None),
    Among("ovic", -1, 1, None),
    Among("in", -1, 2, None),
    Among("ino", -1, 2, None),
    Among("ovo", -1, 1, None),
    Among("ův", -1, 1, None),
    Among("cí", -1, 1, None),
    Among("ací", 7, 1, None),
    Among("icí", 7, 1, None),
    Among("ní", -1, 4, None),
    Among("bilní", 10, 1, None),
    Among("ální", 10, 1, None),
    Among("ární", 10, 1, None),
    Among("tní", 10, 1, None),
    Among("ntní", 14, 1, None),
    Among("ivní", 10, 1, None),
    Among("ovní", 10, 1, None),
    Among("ační", 10, 1, None),
    Among("eční", 10, 1, None),
    Among("iční", 10, 1, None),
    Among("oví", -1, 1, None),
    Among("čí", -1, 1, None),
    Among("ší", -1, 1, None),
    Among("ejší", 23, 6, None),
    Among("ější", 23, 6, None),
    Among("nější", 25, 1, None),
    Among("ovnější", 26, 1, None),
    Among("tější", 25, 1, None),
    Among("štější", 28, 3, None),
    Among("ijštější", 29, 3, None),
    Among("ovštější", 29, 1, None),
    Among("ovější", 25, 1, None),
    Among("ký", -1, 3, None),
    Among("cký", 33, 1, None),
    Among("ický", 34, 1, None),
    Among("inký", 33, 2, None),
    Among("ilinký", 36, 1, None),
    Among("oulinký", 36, 1, None),
    Among("ouninký", 36, 1, None),
    Among("ounký", 33, 1, None),
    Among("ský", 33, 3, None),
    Among("ijský", 41, 1, None),
    Among("ánský", 41, 1, None),
    Among("ovský", 41, 3, None),
    Among("ičký", 33, 2, None),
    Among("ičičký", 45, 1, None),
    Among("oučký", 33, 1, None),
    Among("lý", -1, 1, None),
    Among("ičelý", 48, 1, None),
    Among("ilý", 48, 1, None),
    Among("ný", -1, 4, None),
    Among("ovaný", 51, 1, None),
    Among("cný", 51, 1, None),
    Among("ený", 51, 1, None),
    Among("telný", 51, 1, None),
    Among("itelný", 55, 4, None),
    Among("tný", 51, 1, None),
    Among("utný", 57, 1, None),
    Among("ečný", 51, 1, None),
    Among("ičný", 51, 1, None),
    Among("ěný", 51, 1, None),
    Among("atý", -1, 1, None),
    Among("inkatý", 62, 1, None),
    Among("natý", 62, 5, None),
    Among("itý", -1, 2, None),
    Among("ovitý", 65, 1, None),
    Among("ičitý", 65, 1, None),
    Among("istý", -1, 1, None),
    Among("utý", -1, 1, None),
    Among("vý", -1, 1, None),
    Among("avý", 70, 1, None),
    Among("kavý", 71, 1, None),
    Among("lavý", 71, 1, None),
    Among("ivý", 70, 2, None),
    Among("livý", 74, 1, None),
    Among("ový", 70, 1, None),
];

static A_5: &'static [Among<Context>; 111] = &[
    Among("ina", -1, 11, None),
    Among("ova", -1, 9, None),
    Among("ích", -1, 8, None),
    Among("ých", -1, 10, None),
    Among("hých", 3, 5, None),
    Among("chých", 4, 4, None),
    Among("kých", 3, 3, None),
    Among("ckých", 6, 2, None),
    Among("ských", 6, 1, None),
    Among("iných", 3, 11, None),
    Among("rých", 3, 6, None),
    Among("ových", 3, 9, None),
    Among("ími", -1, 8, None),
    Among("ými", -1, 10, None),
    Among("hými", 13, 5, None),
    Among("chými", 14, 4, None),
    Among("kými", 13, 3, None),
    Among("ckými", 16, 2, None),
    Among("skými", 16, 1, None),
    Among("inými", 13, 11, None),
    Among("rými", 13, 6, None),
    Among("ovými", 13, 9, None),
    Among("ini", -1, 11, None),
    Among("ovi", -1, 9, None),
    Among("ém", -1, 10, None),
    Among("hém", 24, 5, None),
    Among("chém", 25, 4, None),
    Among("kém", 24, 3, None),
    Among("ckém", 27, 2, None),
    Among("ském", 27, 1, None),
    Among("rém", 24, 6, None),
    Among("ím", -1, 8, None),
    Among("ým", -1, 10, None),
    Among("hým", 32, 5, None),
    Among("chým", 33, 4, None),
    Among("kým", 32, 3, None),
    Among("ckým", 35, 2, None),
    Among("ským", 35, 1, None),
    Among("iným", 32, 11, None),
    Among("rým", 32, 6, None),
    Among("ovým", 32, 9, None),
    Among("in", -1, 11, None),
    Among("ého", -1, 10, None),
    Among("hého", 42, 5, None),
    Among("chého", 43, 4, None),
    Among("kého", 42, 3, None),
    Among("ckého", 45, 2, None),
    Among("ského", 45, 1, None),
    Among("rého", 42, 6, None),
    Among("ího", -1, 8, None),
    Among("ino", -1, 11, None),
    Among("ovo", -1, 9, None),
    Among("ému", -1, 10, None),
    Among("hému", 52, 5, None),
    Among("chému", 53, 4, None),
    Among("kému", 52, 3, None),
    Among("ckému", 55, 2, None),
    Among("skému", 55, 1, None),
    Among("rému", 52, 6, None),
    Among("ímu", -1, 8, None),
    Among("inu", -1, 11, None),
    Among("ou", -1, 10, None),
    Among("hou", 61, 5, None),
    Among("chou", 62, 4, None),
    Among("kou", 61, 3, None),
    Among("ckou", 64, 2, None),
    Among("skou", 64, 1, None),
    Among("inou", 61, 11, None),
    Among("rou", 61, 6, None),
    Among("ovou", 61, 9, None),
    Among("ovu", -1, 9, None),
    Among("ův", -1, 9, None),
    Among("iny", -1, 11, None),
    Among("ovy", -1, 9, None),
    Among("ině", -1, 11, None),
    Among("ově", -1, 9, None),
    Among("á", -1, 10, None),
    Among("há", 76, 5, None),
    Among("chá", 77, 4, None),
    Among("ká", 76, 3, None),
    Among("cká", 79, 2, None),
    Among("ská", 79, 1, None),
    Among("rá", 76, 6, None),
    Among("é", -1, 10, None),
    Among("hé", 83, 5, None),
    Among("ché", 84, 4, None),
    Among("ké", 83, 3, None),
    Among("cké", 86, 2, None),
    Among("ské", 86, 1, None),
    Among("ré", 83, 6, None),
    Among("í", -1, 10, None),
    Among("cí", 90, 3, None),
    Among("čtí", 90, 2, None),
    Among("ští", 90, 1, None),
    Among("aví", 90, 7, None),
    Among("ří", 90, 6, None),
    Among("ejší", 90, 10, None),
    Among("řejší", 96, 6, None),
    Among("ější", 90, 10, None),
    Among("čtější", 98, 2, None),
    Among("štější", 98, 1, None),
    Among("rší", 90, 6, None),
    Among("šší", 90, 4, None),
    Among("žší", 90, 5, None),
    Among("ý", -1, 10, None),
    Among("hý", 104, 5, None),
    Among("chý", 105, 4, None),
    Among("ký", 104, 3, None),
    Among("cký", 107, 2, None),
    Among("ský", 107, 1, None),
    Among("rý", 104, 6, None),
];

static A_6: &'static [Among<Context>; 15] = &[
    Among("t", -1, 1, None),
    Among("at", 0, 1, None),
    Among("ovat", 1, 1, None),
    Among("zat", 1, 1, None),
    Among("ct", 0, 1, None),
    Among("et", 0, 1, None),
    Among("it", 0, 1, None),
    Among("dit", 6, 1, None),
    Among("sit", 6, 1, None),
    Among("stit", 6, 1, None),
    Among("nout", 0, 1, None),
    Among("sknout", 10, 1, None),
    Among("ět", 0, 1, None),
    Among("rát", 0, 1, None),
    Among("řít", 0, 1, None),
];

static A_7: &'static [Among<Context>; 345] = &[
    Among("a", -1, 3, None),
    Among("ka", 0, 6, None),
    Among("la", 0, 3, None),
    Among("ala", 2, 16, None),
    Among("rala", 3, 2, None),
    Among("ovala", 3, 1, None),
    Among("zala", 3, 4, None),
    Among("ela", 2, 8, None),
    Among("řela", 7, 11, None),
    Among("ila", 2, 13, None),
    Among("dila", 9, 15, None),
    Among("sila", 9, 7, None),
    Among("stila", 9, 14, None),
    Among("zila", 9, 13, None),
    Among("kla", 2, 6, None),
    Among("skla", 14, 9, None),
    Among("nula", 2, 5, None),
    Among("ěla", 2, 10, None),
    Among("na", 0, 5, None),
    Among("ena", 18, 16, None),
    Among("zena", 19, 15, None),
    Among("čena", 19, 6, None),
    Among("řena", 19, 11, None),
    Among("šena", 19, 7, None),
    Among("žena", 19, 13, None),
    Among("skna", 18, 9, None),
    Among("ěna", 18, 10, None),
    Among("stěna", 26, 14, None),
    Among("štěna", 26, 16, None),
    Among("ána", 18, 12, None),
    Among("rána", 29, 2, None),
    Among("ována", 29, 1, None),
    Among("zána", 29, 4, None),
    Among("ra", 0, 11, None),
    Among("era", 33, 2, None),
    Among("ta", 0, 3, None),
    Among("ata", 35, 5, None),
    Among("nuta", 35, 5, None),
    Among("sknuta", 37, 9, None),
    Among("ouc", -1, 3, None),
    Among("kouc", 39, 6, None),
    Among("nouc", 39, 5, None),
    Among("sknouc", 41, 9, None),
    Among("rouc", 39, 11, None),
    Among("erouc", 43, 2, None),
    Among("íc", -1, 16, None),
    Among("díc", 45, 15, None),
    Among("jíc", 45, 3, None),
    Among("ajíc", 47, 12, None),
    Among("ejíc", 47, 8, None),
    Among("ujíc", 47, 1, None),
    Among("síc", 45, 7, None),
    Among("stíc", 45, 14, None),
    Among("zíc", 45, 13, None),
    Among("žíc", 45, 4, None),
    Among("ouce", -1, 3, None),
    Among("kouce", 55, 6, None),
    Among("nouce", 55, 5, None),
    Among("sknouce", 57, 9, None),
    Among("rouce", 55, 11, None),
    Among("erouce", 59, 2, None),
    Among("íce", -1, 16, None),
    Among("díce", 61, 15, None),
    Among("jíce", 61, 3, None),
    Among("ajíce", 63, 12, None),
    Among("ejíce", 63, 8, None),
    Among("ujíce", 63, 1, None),
    Among("síce", 61, 7, None),
    Among("stíce", 61, 14, None),
    Among("zíce", 61, 13, None),
    Among("žíce", 61, 4, None),
    Among("je", -1, 3, None),
    Among("aje", 71, 12, None),
    Among("eje", 71, 8, None),
    Among("uje", 71, 1, None),
    Among("me", -1, 16, None),
    Among("eme", 75, 3, None),
    Among("jeme", 76, 3, None),
    Among("ujeme", 77, 1, None),
    Among("neme", 76, 5, None),
    Among("skneme", 79, 9, None),
    Among("ereme", 76, 2, None),
    Among("čeme", 76, 6, None),
    Among("řeme", 76, 11, None),
    Among("žeme", 76, 4, None),
    Among("jme", 75, 3, None),
    Among("ejme", 85, 16, None),
    Among("ujme", 85, 1, None),
    Among("erme", 75, 2, None),
    Among("sme", 75, 7, None),
    Among("zme", 75, 13, None),
    Among("ňme", 75, 5, None),
    Among("čme", 75, 6, None),
    Among("ďme", 75, 15, None),
    Among("něme", 75, 5, None),
    Among("skněme", 94, 9, None),
    Among("stěme", 75, 14, None),
    Among("áme", 75, 12, None),
    Among("íme", 75, 16, None),
    Among("díme", 98, 15, None),
    Among("síme", 98, 7, None),
    Among("stíme", 98, 14, None),
    Among("zíme", 98, 13, None),
    Among("žme", 75, 4, None),
    Among("ne", -1, 5, None),
    Among("skne", 104, 9, None),
    Among("ere", -1, 2, None),
    Among("se", -1, 7, None),
    Among("te", -1, 16, None),
    Among("ete", 108, 3, None),
    Among("jete", 109, 3, None),
    Among("ujete", 110, 1, None),
    Among("nete", 109, 5, None),
    Among("sknete", 112, 9, None),
    Among("erete", 109, 2, None),
    Among("čete", 109, 6, None),
    Among("řete", 109, 11, None),
    Among("žete", 109, 4, None),
    Among("jte", 108, 3, None),
    Among("ejte", 118, 16, None),
    Among("ujte", 118, 1, None),
    Among("erte", 108, 2, None),
    Among("ste", 108, 7, None),
    Among("zte", 108, 13, None),
    Among("ňte", 108, 5, None),
    Among("čte", 108, 6, None),
    Among("ďte", 108, 15, None),
    Among("něte", 108, 5, None),
    Among("skněte", 127, 9, None),
    Among("stěte", 108, 14, None),
    Among("áte", 108, 12, None),
    Among("íte", 108, 16, None),
    Among("díte", 131, 15, None),
    Among("síte", 131, 7, None),
    Among("stíte", 131, 14, None),
    Among("zíte", 131, 13, None),
    Among("žte", 108, 4, None),
    Among("ze", -1, 13, None),
    Among("če", -1, 6, None),
    Among("ře", -1, 11, None),
    Among("avše", -1, 5, None),
    Among("nuvše", -1, 5, None),
    Among("že", -1, 4, None),
    Among("ji", -1, 3, None),
    Among("uji", 143, 1, None),
    Among("li", -1, 3, None),
    Among("ali", 145, 16, None),
    Among("rali", 146, 2, None),
    Among("ovali", 146, 1, None),
    Among("zali", 146, 4, None),
    Among("eli", 145, 8, None),
    Among("řeli", 150, 11, None),
    Among("ili", 145, 13, None),
    Among("dili", 152, 15, None),
    Among("sili", 152, 7, None),
    Among("stili", 152, 14, None),
    Among("zili", 152, 13, None),
    Among("kli", 145, 6, None),
    Among("skli", 157, 9, None),
    Among("nuli", 145, 5, None),
    Among("ěli", 145, 10, None),
    Among("ni", -1, 5, None),
    Among("eni", 161, 16, None),
    Among("zeni", 162, 15, None),
    Among("čeni", 162, 6, None),
    Among("řeni", 162, 11, None),
    Among("šeni", 162, 7, None),
    Among("ženi", 162, 13, None),
    Among("skni", 161, 9, None),
    Among("ěni", 161, 10, None),
    Among("stěni", 169, 14, None),
    Among("štěni", 169, 16, None),
    Among("áni", 161, 12, None),
    Among("ráni", 172, 2, None),
    Among("ováni", 172, 1, None),
    Among("záni", 172, 4, None),
    Among("ti", -1, 3, None),
    Among("ati", 176, 5, None),
    Among("sti", 176, 14, None),
    Among("nuti", 176, 5, None),
    Among("sknuti", 179, 9, None),
    Among("ři", -1, 11, None),
    Among("avši", -1, 5, None),
    Among("nuvši", -1, 5, None),
    Among("j", -1, 3, None),
    Among("ej", 184, 16, None),
    Among("uj", 184, 1, None),
    Among("l", -1, 3, None),
    Among("al", 187, 16, None),
    Among("ral", 188, 2, None),
    Among("oval", 188, 1, None),
    Among("zal", 188, 4, None),
    Among("el", 187, 8, None),
    Among("řel", 192, 11, None),
    Among("il", 187, 13, None),
    Among("dil", 194, 15, None),
    Among("sil", 194, 7, None),
    Among("stil", 194, 14, None),
    Among("zil", 194, 13, None),
    Among("kl", 187, 6, None),
    Among("skl", 199, 9, None),
    Among("nul", 187, 5, None),
    Among("ěl", 187, 10, None),
    Among("ám", -1, 12, None),
    Among("ím", -1, 16, None),
    Among("dím", 204, 15, None),
    Among("sím", 204, 7, None),
    Among("stím", 204, 14, None),
    Among("zím", 204, 13, None),
    Among("en", -1, 16, None),
    Among("zen", 209, 15, None),
    Among("čen", 209, 6, None),
    Among("řen", 209, 11, None),
    Among("šen", 209, 7, None),
    Among("žen", 209, 13, None),
    Among("ěn", -1, 10, None),
    Among("stěn", 215, 14, None),
    Among("štěn", 215, 16, None),
    Among("án", -1, 12, None),
    Among("rán", 218, 2, None),
    Among("ován", 218, 1, None),
    Among("zán", 218, 4, None),
    Among("lo", -1, 3, None),
    Among("alo", 222, 16, None),
    Among("ralo", 223, 2, None),
    Among("ovalo", 223, 1, None),
    Among("zalo", 223, 4, None),
    Among("elo", 222, 8, None),
    Among("řelo", 227, 11, None),
    Among("ilo", 222, 13, None),
    Among("dilo", 229, 15, None),
    Among("silo", 229, 7, None),
    Among("stilo", 229, 14, None),
    Among("zilo", 229, 13, None),
    Among("klo", 222, 6, None),
    Among("sklo", 234, 9, None),
    Among("nulo", 222, 5, None),
    Among("ělo", 222, 10, None),
    Among("eno", -1, 16, None),
    Among("zeno", 238, 15, None),
    Among("čeno", 238, 6, None),
    Among("řeno", 238, 11, None),
    Among("šeno", 238, 7, None),
    Among("ženo", 238, 13, None),
    Among("ěno", -1, 10, None),
    Among("stěno", 244, 14, None),
    Among("štěno", 244, 16, None),
    Among("áno", -1, 12, None),
    Among("ráno", 247, 2, None),
    Among("ováno", 247, 1, None),
    Among("záno", 247, 4, None),
    Among("to", -1, 3, None),
    Among("ato", 251, 5, None),
    Among("nuto", 251, 5, None),
    Among("sknuto", 253, 9, None),
    Among("er", -1, 2, None),
    Among("s", -1, 7, None),
    Among("at", -1, 5, None),
    Among("nut", -1, 5, None),
    Among("sknut", 258, 9, None),
    Among("u", -1, 3, None),
    Among("ju", 260, 3, None),
    Among("uju", 261, 1, None),
    Among("nu", 260, 5, None),
    Among("sknu", 263, 9, None),
    Among("ou", 260, 3, None),
    Among("jou", 265, 3, None),
    Among("ujou", 266, 1, None),
    Among("nou", 265, 5, None),
    Among("sknou", 268, 9, None),
    Among("erou", 265, 2, None),
    Among("čou", 265, 6, None),
    Among("řou", 265, 11, None),
    Among("žou", 265, 4, None),
    Among("eru", 260, 2, None),
    Among("ču", 260, 6, None),
    Among("řu", 260, 11, None),
    Among("žu", 260, 4, None),
    Among("av", -1, 5, None),
    Among("nuv", -1, 5, None),
    Among("ly", -1, 3, None),
    Among("aly", 280, 16, None),
    Among("raly", 281, 2, None),
    Among("ovaly", 281, 1, None),
    Among("zaly", 281, 4, None),
    Among("ely", 280, 8, None),
    Among("řely", 285, 11, None),
    Among("ily", 280, 13, None),
    Among("dily", 287, 15, None),
    Among("sily", 287, 7, None),
    Among("stily", 287, 14, None),
    Among("zily", 287, 13, None),
    Among("kly", 280, 6, None),
    Among("skly", 292, 9, None),
    Among("nuly", 280, 5, None),
    Among("ěly", 280, 10, None),
    Among("eny", -1, 16, None),
    Among("zeny", 296, 15, None),
    Among("čeny", 296, 6, None),
    Among("řeny", 296, 11, None),
    Among("šeny", 296, 7, None),
    Among("ženy", 296, 13, None),
    Among("ěny", -1, 10, None),
    Among("stěny", 302, 14, None),
    Among("štěny", 302, 16, None),
    Among("ány", -1, 12, None),
    Among("rány", 305, 2, None),
    Among("ovány", 305, 1, None),
    Among("zány", 305, 4, None),
    Among("ty", -1, 3, None),
    Among("aty", 309, 5, None),
    Among("nuty", 309, 5, None),
    Among("sknuty", 311, 9, None),
    Among("z", -1, 13, None),
    Among("ň", -1, 5, None),
    Among("č", -1, 6, None),
    Among("ď", -1, 15, None),
    Among("ě", -1, 10, None),
    Among("dě", 317, 15, None),
    Among("stě", 317, 14, None),
    Among("á", -1, 12, None),
    Among("eš", -1, 3, None),
    Among("ješ", 321, 3, None),
    Among("uješ", 322, 1, None),
    Among("neš", 321, 5, None),
    Among("skneš", 324, 9, None),
    Among("ereš", 321, 2, None),
    Among("češ", 321, 6, None),
    Among("řeš", 321, 11, None),
    Among("žeš", 321, 4, None),
    Among("áš", -1, 12, None),
    Among("íš", -1, 16, None),
    Among("díš", 331, 15, None),
    Among("síš", 331, 7, None),
    Among("stíš", 331, 14, None),
    Among("zíš", 331, 13, None),
    Among("dí", -1, 15, None),
    Among("jí", -1, 3, None),
    Among("ají", 337, 12, None),
    Among("ejí", 337, 8, None),
    Among("ují", 337, 1, None),
    Among("sí", -1, 7, None),
    Among("stí", -1, 14, None),
    Among("zí", -1, 13, None),
    Among("ž", -1, 4, None),
];

static A_8: &'static [Among<Context>; 6] = &[
    Among("hle", -1, 1, None),
    Among("koli", -1, 1, None),
    Among("si", -1, 1, None),
    Among("mo", -1, 1, None),
    Among("koliv", -1, 1, None),
    Among("ky", -1, 1, None),
];

static A_9: &'static [Among<Context>; 9] = &[
    Among("bb", -1, 1, None),
    Among("dd", -1, 1, None),
    Among("ff", -1, 1, None),
    Among("gg", -1, 1, None),
    Among("mm", -1, 1, None),
    Among("nn", -1, 1, None),
    Among("pp", -1, 1, None),
    Among("rr", -1, 1, None),
    Among("tt", -1, 1, None),
];

static G_vowel: &'static [u8; 34] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 4, 18, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64];

static G_consonant: &'static [u8; 36] = &[119, 223, 119, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 64, 0, 128, 128, 8, 0, 0, 16];

static G_aeiou: &'static [u8; 3] = &[17, 65, 16];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    if !env.go_out_grouping(G_vowel, 97, 367) {
        return false;
    }
env.next_char();    if !env.go_in_grouping(G_vowel, 97, 367) {
        return false;
    }
env.next_char();    context.i_p1 = env.cursor;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            among_var = env.find_among(A_0, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("a");
                }
                2 => {
                    env.slice_from("e");
                }
                3 => {
                    env.slice_from("i");
                }
                4 => {
                    env.slice_from("o");
                }
                5 => {
                    env.slice_from("u");
                }
                6 => {
                    env.slice_from("y");
                }
                7 => {
                    env.slice_from("c");
                }
                8 => {
                    env.slice_from("d");
                }
                9 => {
                    env.slice_from("n");
                }
                10 => {
                    env.slice_from("r");
                }
                11 => {
                    env.slice_from("s");
                }
                12 => {
                    env.slice_from("t");
                }
                13 => {
                    env.slice_from("z");
                }
                14 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_exception(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    among_var = env.find_among(A_1, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    if env.cursor < env.limit {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("byt");
        }
        2 => {
            env.slice_from("mit");
        }
        3 => {
            env.slice_from("jit");
        }
        4 => {
            env.slice_from("jist");
        }
        5 => {
            env.slice_from("snist");
        }
        6 => {
            env.slice_from("videt");
        }
        7 => {
            env.slice_from("vedet");
        }
        8 => {
            env.slice_from("chtit");
        }
        _ => ()
    }
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_nouns_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_2, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        match among_var {
            1 => {
                env.slice_del();
                context.b_found_suffix = true;
            }
            2 => {
                env.slice_del();
                env.ket = env.cursor;
                if !env.eq_s_b(&"š") {
                    break 'lab0;
                }
                env.bra = env.cursor;
                env.slice_from("ch");
                context.b_found_suffix = true;
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_nouns_endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_3, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        match among_var {
            1 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ba");
                env.cursor = c;
            }
            2 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ec");
                env.cursor = c;
            }
            3 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "lo");
                env.cursor = c;
            }
            4 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ra");
                env.cursor = c;
            }
            5 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "na");
                env.cursor = c;
            }
            6 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "e");
                env.cursor = c;
            }
            7 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "no");
                env.cursor = c;
            }
            8 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "va");
                env.cursor = c;
            }
            9 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ek");
                env.cursor = c;
            }
            10 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "tě");
                env.cursor = c;
            }
            11 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "g");
                env.cursor = c;
            }
            12 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "vo");
                env.cursor = c;
            }
            13 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "a");
                env.cursor = c;
            }
            14 => {
                env.slice_del();
                let v_2 = env.limit - env.cursor;
                if !env.in_grouping_b(G_aeiou, 97, 117) {
                    break 'lab0;
                }
                env.cursor = env.limit - v_2;
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ň");
                env.cursor = c;
                let v_3 = env.limit - env.cursor;
                if !env.eq_s_b(&"y") {
                    break 'lab0;
                }
                env.cursor = env.limit - v_3;
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ně");
                env.cursor = c;
                let v_4 = env.limit - env.cursor;
                if !env.in_grouping_b(G_consonant, 98, 382) {
                    break 'lab0;
                }
                env.cursor = env.limit - v_4;
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "eň");
                env.cursor = c;
                context.b_found_suffix = true;
            }
            15 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ě");
                env.cursor = c;
            }
            16 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ka");
                env.cursor = c;
            }
            17 => {
                env.slice_del();
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_adjectives_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_4, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        match among_var {
            1 => {
                env.slice_del();
                context.b_found_suffix = true;
            }
            2 => {
                env.slice_del();
                if !r_alternance_i(env, context) {
                    break 'lab0;
                }
                context.b_found_suffix = true;
            }
            3 => {
                env.slice_del();
                'lab1: loop {
                    let v_2 = env.limit - env.cursor;
                    'lab2: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"ž") {
                            break 'lab2;
                        }
                        env.bra = env.cursor;
                        env.slice_from("h");
                        break 'lab1;
                    }
                    env.cursor = env.limit - v_2;
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"š") {
                        break 'lab0;
                    }
                    env.bra = env.cursor;
                    env.slice_from("ch");
                    break 'lab1;
                }
                context.b_found_suffix = true;
            }
            4 => {
                env.slice_del();
                'lab3: loop {
                    let v_3 = env.limit - env.cursor;
                    'lab4: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"ž") {
                            break 'lab4;
                        }
                        env.bra = env.cursor;
                        env.slice_from("h");
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_3;
                    'lab5: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"š") {
                            break 'lab5;
                        }
                        env.bra = env.cursor;
                        env.slice_from("ch");
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_3;
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"č") {
                        break 'lab0;
                    }
                    env.bra = env.cursor;
                    env.slice_from("k");
                    break 'lab3;
                }
                context.b_found_suffix = true;
            }
            5 => {
                env.slice_del();
                env.ket = env.cursor;
                if !env.eq_s_b(&"č") {
                    break 'lab0;
                }
                env.bra = env.cursor;
                env.slice_from("k");
                context.b_found_suffix = true;
            }
            6 => {
                env.slice_del();
                'lab6: loop {
                    let v_4 = env.limit - env.cursor;
                    'lab7: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"č") {
                            break 'lab7;
                        }
                        env.bra = env.cursor;
                        env.slice_from("k");
                        break 'lab6;
                    }
                    env.cursor = env.limit - v_4;
                    'lab8: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"čť") {
                            break 'lab8;
                        }
                        env.bra = env.cursor;
                        env.slice_from("ck");
                        break 'lab6;
                    }
                    env.cursor = env.limit - v_4;
                    'lab9: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"šť") {
                            break 'lab9;
                        }
                        env.bra = env.cursor;
                        env.slice_from("sk");
                        break 'lab6;
                    }
                    env.cursor = env.limit - v_4;
                    'lab10: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"ž") {
                            break 'lab10;
                        }
                        env.bra = env.cursor;
                        env.slice_from("h");
                        break 'lab6;
                    }
                    env.cursor = env.limit - v_4;
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"š") {
                        break 'lab0;
                    }
                    env.bra = env.cursor;
                    env.slice_from("ch");
                    break 'lab6;
                }
                context.b_found_suffix = true;
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_adjectives_endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_5, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        match among_var {
            1 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ský");
                env.cursor = c;
            }
            2 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "cký");
                env.cursor = c;
            }
            3 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ký");
                env.cursor = c;
            }
            4 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "chý");
                env.cursor = c;
            }
            5 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "hý");
                env.cursor = c;
            }
            6 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "rý");
                env.cursor = c;
            }
            7 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "avý");
                env.cursor = c;
            }
            8 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "í");
                env.cursor = c;
            }
            9 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ův");
                env.cursor = c;
            }
            10 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ý");
                env.cursor = c;
            }
            11 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "in");
                env.cursor = c;
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_verbs_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 116 as u8) {
            break 'lab0;
        }

        if env.find_among_b(A_6, context) == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        context.b_found_suffix = true;
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_verbs_endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_7, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ovat");
                env.cursor = c;
            }
            2 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "rát");
                env.cursor = c;
            }
            3 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "t");
                env.cursor = c;
            }
            4 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "zat");
                env.cursor = c;
            }
            5 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "nout");
                env.cursor = c;
            }
            6 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ct");
                env.cursor = c;
            }
            7 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "sit");
                env.cursor = c;
            }
            8 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "et");
                env.cursor = c;
            }
            9 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "sknout");
                env.cursor = c;
            }
            10 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "ět");
                env.cursor = c;
            }
            11 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "řít");
                env.cursor = c;
            }
            12 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "at");
                env.cursor = c;
            }
            13 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "zit");
                env.cursor = c;
            }
            14 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "stit");
                env.cursor = c;
            }
            15 => {
                env.slice_del();
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, "dit");
                env.cursor = c;
            }
            16 => {
                env.slice_del();
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_adverbs_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((37782048 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab0;
        }

        if env.find_among_b(A_8, context) == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        env.slice_del();
        context.b_found_suffix = true;
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_end_vowel(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.in_grouping_b(G_vowel, 97, 367) {
        return false;
    }
    env.bra = env.cursor;
    env.slice_from("");
    env.ket = env.cursor;
    if !env.in_grouping_b(G_vowel, 97, 367) {
        return false;
    }
    env.bra = env.cursor;
    env.slice_from("");
    return true
}

fn r_alternance_i(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            env.ket = env.cursor;
            if !env.eq_s_b(&"č") {
                break 'lab1;
            }
            env.bra = env.cursor;
            env.slice_from("k");
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        env.ket = env.cursor;
        if !env.eq_s_b(&"š") {
            return false;
        }
        env.bra = env.cursor;
        env.slice_from("ch");
        break 'lab0;
    }
    return true
}

fn r_end_double(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1401044 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_9, context) == 0 {
        return false;
    }
    env.cursor = env.limit - v_1;
    env.ket = env.cursor;
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    env.slice_del();
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_found_suffix: false,
        i_p1: 0,
    };
    'lab0: loop {
        let v_1 = env.cursor;
        'lab1: loop {
            if !r_exception(env, context) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = v_1;
        'lab2: loop {
            let v_2 = env.cursor;
            'lab3: loop {
                if !env.hop(4) {
                    break 'lab3;
                }
                break 'lab2;
            }
            env.cursor = v_2;
            break 'lab0;
        }
        env.cursor = v_1;
        let v_3 = env.cursor;
        r_mark_regions(env, context);
        env.cursor = v_3;
        context.b_found_suffix = false;
        env.limit_backward = env.cursor;
        env.cursor = env.limit;
        r_nouns_suffixes(env, context);
        'lab4: loop {
            'lab5: loop {
                if !context.b_found_suffix {
                    break 'lab5;
                }
                break 'lab4;
            }
            r_nouns_endings(env, context);
            break 'lab4;
        }
        'lab6: loop {
            'lab7: loop {
                if !context.b_found_suffix {
                    break 'lab7;
                }
                break 'lab6;
            }
            r_nouns_suffixes(env, context);
            break 'lab6;
        }
        'lab8: loop {
            'lab9: loop {
                if !context.b_found_suffix {
                    break 'lab9;
                }
                break 'lab8;
            }
            r_adjectives_suffixes(env, context);
            break 'lab8;
        }
        'lab10: loop {
            'lab11: loop {
                if !context.b_found_suffix {
                    break 'lab11;
                }
                break 'lab10;
            }
            r_adjectives_endings(env, context);
            break 'lab10;
        }
        'lab12: loop {
            'lab13: loop {
                if !context.b_found_suffix {
                    break 'lab13;
                }
                break 'lab12;
            }
            r_adjectives_suffixes(env, context);
            break 'lab12;
        }
        'lab14: loop {
            'lab15: loop {
                if !context.b_found_suffix {
                    break 'lab15;
                }
                break 'lab14;
            }
            r_adverbs_suffixes(env, context);
            break 'lab14;
        }
        'lab16: loop {
            'lab17: loop {
                if !context.b_found_suffix {
                    break 'lab17;
                }
                break 'lab16;
            }
            r_verbs_endings(env, context);
            break 'lab16;
        }
        'lab18: loop {
            'lab19: loop {
                if !context.b_found_suffix {
                    break 'lab19;
                }
                break 'lab18;
            }
            r_verbs_suffixes(env, context);
            break 'lab18;
        }
        let v_4 = env.limit - env.cursor;
        r_end_double(env, context);
        env.cursor = env.limit - v_4;
        let v_5 = env.limit - env.cursor;
        r_end_vowel(env, context);
        env.cursor = env.limit - v_5;
        let v_6 = env.limit - env.cursor;
        r_end_double(env, context);
        env.cursor = env.limit - v_6;
        env.cursor = env.limit_backward;
        break 'lab0;
    }
    let v_7 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_7;
    return true
}
