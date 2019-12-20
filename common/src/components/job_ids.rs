use crate::components::char::JobId;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::Display;
use strum_macros::EnumCount;
use strum_macros::EnumIter;
use strum_macros::EnumString;

impl JobSpriteId {
    pub fn from_job_id(job_id: JobId) -> JobSpriteId {
        match job_id {
            JobId::WIZARD => JobSpriteId::WIZARD,
            JobId::CRUSADER => JobSpriteId::CRUSADER,
            JobId::SWORDMAN => JobSpriteId::SWORDMAN,
            JobId::ARCHER => JobSpriteId::ARCHER,
            JobId::ASSASSIN => JobSpriteId::ASSASSIN,
            JobId::KNIGHT => JobSpriteId::KNIGHT,
            JobId::SAGE => JobSpriteId::SAGE,
            JobId::ALCHEMIST => JobSpriteId::ALCHEMIST,
            JobId::BLACKSMITH => JobSpriteId::BLACKSMITH,
            JobId::PRIEST => JobSpriteId::PRIEST,
            JobId::MONK => JobSpriteId::MONK,
            JobId::GUNSLINGER => JobSpriteId::GUNSLINGER,
            JobId::ROGUE => JobSpriteId::ROGUE,
            JobId::RANGER => JobSpriteId::RANGER,
            JobId::TargetDummy => panic!(),
            JobId::HealingDummy => panic!(),
            JobId::MeleeMinion => panic!(),
            JobId::RangedMinion => panic!(),
            JobId::Turret => panic!(),
            JobId::Guard => panic!(),
            JobId::Barricade => panic!(),
        }
    }
}
//½ÅÆÄÄÚÅ©·Ç¼¼ÀÌ´Õ_H_¿©
// male: ¿©
// female:

//CRUSADER: Å©·ç¼¼ÀÌ´õ
//SWORDMAN: °Ë»ç
//ARCHER: ±Ã¼ö
//ASSASSIN: ¾î¼¼½Å
//ROGUE: ·Î±×
//KNIGHT: ±â»ç
//WIZARD: À§Àúµå
//SAGE: ¼¼ÀÌÁö
//ALCHEMIST: ¿¬±Ý¼ú»ç
//BLACKSMITH: Á¦Ã¶°ø
//PRIEST: ÇÁ¸®½ºÆ®
//MONK: ¸ùÅ©
//GUNSLINGER: °Ç³Ê
//HUNTER: ÇåÅÍ

#[derive(
    EnumIter, EnumString, Debug, Display, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub enum JobSpriteId {
    NOVICE = 0,
    SWORDMAN = 1,
    MAGICIAN = 2,
    ARCHER = 3,
    ACOLYTE = 4,
    MERCHANT = 5,
    THIEF = 6,
    KNIGHT = 7,
    PRIEST = 8,
    WIZARD = 9,
    BLACKSMITH = 10,
    HUNTER = 11,
    ASSASSIN = 12,
    KNIGHT2 = 13,
    CRUSADER = 14,
    MONK = 15,
    SAGE = 16,
    ROGUE = 17,
    ALCHEMIST = 18,
    BARD = 19,
    DANCER = 20,
    CRUSADER2 = 21,
    MARRIED = 22,
    SUPERNOVICE = 23,
    GUNSLINGER = 24,
    NINJA = 25,
    XMAS = 26,
    SUMMER = 27,
    NoviceH = 4001,
    SwordmanH = 4002,
    MagicianH = 4003,
    ArcherH = 4004,
    AcolyteH = 4005,
    MerchantH = 4006,
    ThiefH = 4007,
    KnightH = 4008,
    PriestH = 4009,
    WizardH = 4010,
    BlacksmithH = 4011,
    HunterH = 4012,
    AssassinH = 4013,
    Knight2H = 4014,
    CrusaderH = 4015,
    MonkH = 4016,
    SageH = 4017,
    RogueH = 4018,
    AlchemistH = 4019,
    BardH = 4020,
    DancerH = 4021,
    Crusader2H = 4022,
    NoviceB = 4023,
    SwordmanB = 4024,
    MagicianB = 4025,
    ArcherB = 4026,
    AcolyteB = 4027,
    MerchantB = 4028,
    ThiefB = 4029,
    KnightB = 4030,
    PriestB = 4031,
    WizardB = 4032,
    BlacksmithB = 4033,
    HunterB = 4034,
    AssassinB = 4035,
    Knight2B = 4036,
    CrusaderB = 4037,
    MonkB = 4038,
    SageB = 4039,
    RogueB = 4040,
    AlchemistB = 4041,
    BardB = 4042,
    DancerB = 4043,
    Crusader2B = 4044,
    SupernoviceB = 4045,
    TAEKWON = 4046,
    STAR = 4047,
    STAR2 = 4048,
    LINKER = 4049,
    /*
    not used yet=
    Job_Gangsi	4050
    Job_Death_Knight	4051
    Job_Dark_Collector	4052
    */
    RuneKnight = 4054,
    WARLOCK = 4055,
    RANGER = 4056,
    ARCHBISHOP = 4057,
    MECHANIC = 4058,
    GuillotineCross = 4059,
    RuneKnightH = 4060,
    WarlockH = 4061,
    RangerH = 4062,
    ArchbishopH = 4063,
    MechanicH = 4064,
    GuillotineCrossH = 4065,
    RoyalGuard = 4066,
    SORCERER = 4067,
    MINSTREL = 4068,
    WANDERER = 4069,
    SURA = 4070,
    GENETIC = 4071,
    ShadowChaser = 4072,
    RoyalGuardH = 4073,
    SorcererH = 4074,
    MinstrelH = 4075,
    WandererH = 4076,
    SuraH = 4077,
    GeneticH = 4078,
    ShadowChaserH = 4079,
    RuneKnight2 = 4080,
    RuneKnight2H = 4081,
    RoyalGuard2 = 4082,
    RoyalGuard2H = 4083,
    RANGER2 = 4084,
    Ranger2H = 4085,
    MECHANIC2 = 4086,
    Mechanic2H = 4087,

    RuneKnightB = 4096,
    WarlockB = 4097,
    RangerB = 4098,
    ArchbishopB = 4099,
    MechanicB = 4100,
    GuillotineCrossB = 4101,
    RoyalGuardB = 4102,
    SorcererB = 4103,
    MinstrelB = 4104,
    WandererB = 4105,
    SuraB = 4106,
    GeneticB = 4107,
    ShadowChaserB = 4108,
    RuneKnight2B = 4109,
    RoyalGuard2B = 4110,
    Ranger2B = 4111,
    Mechanic2B = 4112,
    // 4113 ?
    FrogNinja = 4114,
    PecoGunner = 4115,
    PecoSword = 4116,
    // 4117 ?
    PigWhitesmith = 4118,
    PigMerchant = 4119,
    PigGenetic = 4120,
    PigCreator = 4121,
    OstrichArcher = 4122,
    PoringStar = 4123,
    PoringNovice = 4124,
    SheepMonk = 4125,
    SheepAco = 4126,
    SheepSura = 4127,
    PoringSnovice = 4128,
    SheepArcb = 4129,
    FoxMagician = 4130,
    FoxSage = 4131,
    FoxSorcerer = 4132,
    FoxWarlock = 4133,
    FoxWiz = 4134,
    // 4135 ?
    FoxHwiz = 4136,
    PigAlche = 4137,
    PigBlacksmith = 4138,
    SheepChamp = 4139,
    DogGCross = 4140,
    DogThief = 4141,
    DogRogue = 4142,
    DogChaser = 4143,
    DogStalker = 4144,
    DogAssassin = 4145,
    DogAssaX = 4146,
    OstrichDancer = 4147,
    OstrichMinstrel = 4148,
    OstrichBard = 4149,
    OstrichSniper = 4150,
    OstrichWander = 4151,
    OstrichZipsi = 4152,
    OstrichCrown = 4153,
    OstrichHunter = 4154,
    PoringTaekwon = 4155,
    SheepPriest = 4156,
    SheepHpriest = 4157,
    PoringNoviceB = 4158,
    // 4159 ?
    FoxMagicianB = 4160,
    OstrichArcherB = 4161,
    SheepAcoB = 4162,
    PigMerchantB = 4163,
    OstrichHunterB = 4164,
    DogAssassinB = 4165,
    SheepMonkB = 4166,
    FoxSageB = 4167,
    DogRogueB = 4168,
    PigAlcheB = 4169,
    OstrichBardB = 4170,
    OstrichDancerB = 4171,
    PoringSnoviceB = 4172,
    FoxWarlockB = 4173,
    SheepArcbB = 4174,
    DogGCrossB = 4175,
    FoxSorcererB = 4176,
    OstrichMinstrelB = 4177,
    OstrichWanderB = 4178,
    SheepSuraB = 4179,
    PigGeneticB = 4180,
    DogThiefB = 4181,
    DogChaserB = 4182,
    PoringNoviceH = 4183,
    // 4184 ?
    FoxMagicianH = 4185,
    OstrichArcherH = 4186,
    SheepAcoH = 4187,
    PigMerchantH = 4188,
    DogThiefH = 4189,
    SUPERNOVICE2 = 4190,
    Supernovice2B = 4191,
    PoringSnovice2 = 4192,
    PoringSnovice2B = 4193,
    SheepPriestB = 4194,
    FoxWizB = 4195,
    PigBlacksmithB = 4196,

    KAGEROU = 4211,
    OBORO = 4212,
    FrogKagerou = 4213,
    FrogOboro = 4214,
}
