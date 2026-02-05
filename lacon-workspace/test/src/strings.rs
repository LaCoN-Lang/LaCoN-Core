#[allow(dead_code)]
pub static FILE_STRINGS: &'static &str = &" \
/|\\ ===================== \
/|\\ BASIC TYPES & STRINGS \
/|\\ ===================== \
 \
const a<String> = 'Just string' \
const b<String> = \"interpolated ${String.as_ref()}\" \
 \
let c = 18kg \
c++ \
c *= c * (c / 2) \
 \
local const d<Expr> = 10kJ / delta 2s \
 \
use schema \"./schema.slacon\" \
 \
 \
/|\\ ===================== \
/|\\ ENTITY DEFINITIONS \
/|\\ ===================== \
 \
entity_type \"steam-engine\" \
power_reproduction 15kW \
fuel \"solid\" \
fuel_consumption 100g / 1min \
 \
 \
[Marker:Output<Dictionary[]>] \
use schema \"./energy_generator.slacon\" \
 \
{ \
    entity-type: energy-generator \
    entity-name: solar-generator-mk1 \
    entity-description: Base solar generator for small stations \
    energy: { \
        power-reproduction: 50W \
        heat-generation: 1kJ / 1min \
    } \
    craft: [ \
        {component: metal-plate, count: 10}, \
        {component: solar-cell, count: 8}, \
        {component: circuit-board, count: 5} \
    ] \
    craft-time: 30s \
    cost: {requisition: 200, energy: 50} \
    size: {L: 2m, W: 1m, H: 1m} \
    weight: 150kg \
    durability: 100 \
} \
 \
{ \
    entity-type: energy-generator \
    entity-name: coal-generator \
    entity-description: Medium power coal generator for industrial usage \
    energy: { \
        power-reproduction: 200W \
        fuel: { \
            type: solid \
            consumption: 5kg / 1min \
            emission: [ \
                {compound: CO2, per-consumption: 4.5kg}, \
                {compound: SO2, per-consumption: 0.5kg} \
            ] \
        } \
        heat-generation: 150kJ / 1min \
    } \
    craft: [ \
        {component: steel-plate, count: 20}, \
        {component: combustion-chamber, count: 1}, \
        {component: coolant-pipe, count: 5} \
    ] \
    craft-time: 120s \
    cost: {requisition: 1000, energy: 200} \
    size: {L: 5m, W: 2m, H: 2m} \
    weight: 1200kg \
    durability: 500 \
} \
 \
 \
/|\\ ===================== \
/|\\ SPREAD / GENERATORS \
/|\\ ===================== \
 \
spread([\"steam-generator-mk2\", \"steam-generator-mk3\", \"steam-generator-mk4\", \"steam-generator-mk5\"], \
       [{...}, {...}, {...}, {...}]) \
as (generator, data) { \
    yield { \
        entity-type: energy-generator \
        entity-name: ${generator} \
        entity-description: ${data.description} \
        energy: { \
            type: liquid \
            consumption: ${data.consumption} \
            emission: ${data.emission} \
        } \
        ...data \
    } \
} \
 \
 \
/|\\ ===================== \
/|\\ FUNCTIONS & LOOPS \
/|\\ ===================== \
 \
const generatorsData = readData(\"./energy_generators.llacon\") \
 \
public function registerGenerator(generator<Dictionary>) { \
    GameAPI.registerEntity({ \
        type: generator[\"entity-type\"], \
        name: generator[\"entity-name\"], \
        description: generator[\"entity-description\"] ?? \"Description missing\", \
        energy: generator.energy, \
        craft: generator.craft, \
        craftTime: generator[\"craft-time\"], \
        cost: generator.cost, \
        size: generator.size, \
        weight: generator.weight, \
        durability: generator.durability \
    }) \
} \
 \
for gen in generatorsData { \
    registerGenerator(gen) \
} \
 \
 \
/|\\ ===================== \
/|\\ IDENTIFIERS & OPERATORS \
/|\\ ===================== \
 \
a = \"literal ${identifier} literal\" \
module::exported \
a := ssing \
 \
floor_ceil = floor(170.75 / 13.38) * ceil(5.55 - (13 - argument + 1 * 7)) \
 \
 \
_ \
__ \
na_me \
__na_me \
 \
1 ** 2 \
1 * 2 \
1 * * 2 \
 \
10 kW*s \
10 W*s \
10 kW*fs \
 \
10 m3 -> nL \
10 m3 -> n \
10 m3 -> n1 \
10 m3 -> 1L \
10 m3 -> nnL \
10 m3 -> 10nL \
 \
10kg*m/us \
10kg/m3 \
10     kg/m3 \
kg/m3 \
 \
 \
/|\\ ===================== \
/|\\ UNIT EDGE CASES \
/|\\ ===================== \
 \
key = 1m/s6 \
key = 1km/s6 \
key = 1km/ks6 \
key = 1Mm/us6 \
key = 1um/Ms6 \
key = 1percent \
key = 1kg \
key = 1g \
key = 1ug/m3 \
key = 1ug/um3 \
key = 1kg/cm3 \
key = 1kg/m3 \
 \
25 - 100 \
string == string \
 \
 \
/|\\ ===================== \
/|\\ NUMERIC LIMITS \
/|\\ ===================== \
 \
const a<Int8>   = 127 \
const b<UInt8>  = 255 \
const c<Int16>  = 32767 \
const d<UInt16> = 65535 \
const e<Int32>  = 2147483647 \
const f<UInt32> = 4294967295 \
const g<Int64>  = 9223372036854775807 \
const h<UInt64> = 18446744073709551615 \
 \
const a<Float16> = 32767.25 \
const b<Float32> = 2147483647.75 \
const c<Float64> = 9223372036854775807.5 \
 \
 \
/|\\ ===================== \
/|\\ TABLE SYNTAX \
/|\\ ===================== \
 \
const t<table> = @| \
    (3) \
    (\"User Name\", \"Age\", \"Attributes\") \
    (<String>, <Number>, <Dictionary>) \
    [ \
        (\"Alice\", 30, {height: 5.5ft, weight: 130lb}), \
        (\"Bob\", 25, {height: 6ft, weight: 180lb}), \
        (\"Charlie\", 35, {height: 5.8ft, weight: 150lb}) \
    ] \
|@ \
 \
const cell = t[\"User Name\"].row(2) \
const row  = t.findRow(row => row[\"User Name\"] == \"Alice\") \
const cell2 = t.findCell((row, colName, rowIndex, colIndex) => \
    row[colName] is String && row[colName] ~= \"i)ar?l\" \
) \
 \
 \
/|\\ ===================== \
/|\\ GENERATE / SPREAD \
/|\\ ===================== \
 \
generate (0x4E3 .. -10) as local let code_point { \
    yield unicode-${code_point} { \
        char \"\\u{${code_point}}\" \
    } \
} \
 \
spread ['one', 'two', 'three', 'four', 'five'] as name { \
    yield digit-${name} { \
        number-value: index \
    } \
} \
 \
 \
/|\\ ===================== \
/|\\ PHYSICAL VALUES \
/|\\ ===================== \
 \
let something = 20000uW \
let speed<Speed> = 278ft/s \
let time<Time> = 25us \
let length<Length> = 25pc \
let degree<Degree> = 45deg \
 \
const generator_power<ElectricPower> = 15kW \
const fuel_energy<Energy> = 1.5MJ \
const temperature<Temperature> = 25C \
const resistance<ElectricResistance> = 10000kOhm \
const area<Area> = 73.14m2 \
const volume<Volume> = 15m3 \
 \
const number2<Number>  = 0b1011110011 \
const number8<Number>  = 0o071 \
const number16<Number> = 0x348FABD1 \
const number32<Number> = 0tL1FF \
const number32b<Number> = 0cZYX \
 \
 \
/|\\ ===================== \
/|\\ SPECIAL VALUES \
/|\\ ===================== \
 \
const inf_mass  = Infinitykg \
const ninf_mass = -Infinitykg \
 \
 \
/|\\ ===================== \
/|\\ MULTILINE STRING \
/|\\ ===================== \
 \
text_data = \"\"\" \
    Line 1 \
    Line 2 \
    \"\"\".trim().to_upper()";

#[allow(dead_code)]
pub static FILE_STRINGS_CYR: &'static &str = &" \
/|\\ ============================================================================= \
/|\\ СТАТЬЯ: ТАЙНЫ ДРЕВНЕАНГЛИЙСКОЙ ПИСЬМЕННОСТИ И ПОЭТИКИ \
/|\\ ============================================================================= \
 \
$author_name Гемини \
 \
/|\\ --- Лексический фундамент (Переменные-корни) --- \
$root-ice īs \
$root-land land \
$root-sea sæ \
$root-man mann \
$root-gift giefu \
$root-glory wuldor \
$root-secret rūn \
$cjk-note [Примечание_для_Gemini]=Этот_блок_отключит_курсив_японскими_иероглифами_日本語 \
 \
/|\\ --- Рунический алфавит (Футорк) через Unicode --- \
$rune-fehu \u{16A0} \
$rune-uruz \u{16A2} \
$rune-thurisaz \u{16A6} \
$rune-ansuz \u{16A8} \
$rune-raidho \u{16B1} \
$rune-kenaz \u{16B2} \
 \
/|\\ --- Основное тело статьи --- \
article_content @( \
    Введение в англосаксонскую лингвистику \
    -------------------------------------- \
    Древнеанглийский язык (Old English) — это мир сложных метафор, называемых «кеннингами». \
    В этом тексте мы исследуем, как базовые концепции соединяются в поэтические образы. \
 \
    1. Магия Рун и Секреты \
    Слово \"$root-secret\" (тайна) напрямую связано с руническим письмом. \
    Каждая руна имела имя и сакральное значение: \
    - $rune-fehu (Fehu): Означает богатство или скот. \
    - $rune-ansuz (Ansuz): Означает уста или божество \u{16A8}. \
    - $rune-thurisaz (Thurisaz): Великан или шип \u{16A6}. \
 \
    Если мы объединим корень «$root-secret» с понятием мастера, мы получим «rūn-wita» — \
    того, кто посвящен в тайны. В кодировке Unicode эти символы занимают диапазон \u{16A0}-\u{16FF}. \
 \
    2. Ландшафты и Стихии \
    Англосаксы жили в суровом климате. Понятие «$root-ice» (\u{16C1}) было повсеместным. \
    Посмотрите на эти сложные слова: \
    - Поле льда: $root-ice~$root-land \
    - Человек суши: $root-land~$root-man \
    - Слава земли: $root-glory~$root-land \
 \
    Использование тильды в Lacon позволяет нам собирать эти слова без пробелов, \
    имитируя германскую агглютинацию: $root-sea~$root-man (моряк/человек моря). \
 \
    3. Юникод-артефакты в текстах \
    Часто в манускриптах встречаются указатели и разделители: \
    \u{2042} (Asterism) — использовался для разделения глав. \
    \u{2E3B} (Three-Em Dash) — длинное тире для пауз в элегиях. \
    \u{261E} (Pointing Finger) — «посмотри сюда». \
 \
    4. Сравнение кодировок (CJK Test) \
    Для корректной работы нашего движка важно, чтобы декораторы не ломали CJK-символы. \
    $cjk-note \
    Пример японского перевода термина rūn: \u{53E4}\u{4EE3}\u{306E}\u{795E}\u{79D8} (Kōdai no shinpi). \
 \
    5. Технические метаданные статьи \
    Статус обработки: [Завершено \u{2705}] \
    Версия парсера: \u{2318} v2.1 \
    Хеш-сумма: \u{2211} (alpha-omega) \
) \
 \
/|\\ Дополнительные параметры публикации \
metadata { \
    author $author_name \
    encoding \"UTF-8\" \
    tags [history, linguistics, $root-secret, runes] \
    formatting-check { \
        line-height-test 1.135em \
        font-family \"Sans-Serif\" \
        is-premium true \
    } \
}";
