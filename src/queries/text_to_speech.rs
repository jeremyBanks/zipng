mod vocabulary;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use super::traits::Request;
use super::traits::Response;
use super::Context;
use crate::blob::Blob;
use crate::blob::BlobId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextToSpeechRequest;

impl super::Request {
    pub fn voice_samples() -> Self {
        Self::TextToSpeech(TextToSpeechRequest)
    }
}

impl Request for TextToSpeechRequest {
    type Response = TextToSpeechResponse;

    fn query(&self, context: &mut Context) -> Self::Response {
        // get all known TTS voices, then for each, generate a sample
        // utterance and store it in the context blob store.

        let name = "Microsoft Bob - Canadian English";

        TextToSpeechResponse {
            samples: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextToSpeechResponse {
    samples: BTreeMap<String, TextToSpeechUtterance>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextToSpeechUtterance {
    text: String,
    blob: BlobId,
}

impl Response for TextToSpeechResponse {}

static SAMPLE_VOCABULARY: &str = {
    "\
the quick brown fox jumps over the lazy dog.

she sells seashells by the seashore.

sphinx of black quartz, judge my vow.

the chic sikh's sixty-sixth sheep is sick.

0 1 10 100 1000 1000000 1000000000 1000000000000 1023 1024 11.
12 127 128 13 14 15 16 17 18 19 2 20 200 2000 2047 2048 21 22.
23 24 25 255 256 26 27 28 29 3 30 31 32 4 40 4000 403 404 4096.
420 5 50 500 5000 511 512 6 60 63 64 69 7 70 8 80 8000 8192 9.
90 99 a aaron abigail able about above accent accept ace acid.
across act actually adam adams add addison admit adult adults.
affirmative afraid africa african after afternoon again.
against age ages ago agree ah ahead ahh ahmed ai aiden air.
airy alexander alfa algorithm alive all allen allow almost.
alone along alpha already alright also alth ough alvarez always.
am amanda amaze amelia american amy an ancestor ancestry and.
anderson andrew angela anger angry animal ann anna anne.
annihilate annoy another answer ant anthony any anymore anyone.
anything anyway apartment apex apparently appear apple.
application approach april aqua archie are area areas aren't.
arm around arrive arthur artificial as ascend ascii ashley.
asia asian ask asked asleep ass assault async asynchronous at.
attack attempt attention aubrey august aunt author autumn ava.
average avery avoid away aww b baby back bad bag bailey baker.
bakes ball band bank banks bar barbara barely base basic.
bathroom battery be beast beat beaucoup beautiful became.
because become bed bedroom been before began begin behind.
believe bell belong benjamin bennet beside besides best beta.
better betty between big billion binary bird birth bit bite.
bitmap black blink block blocked blonde blood bloom blue blush.
body bond book bore both bother bottle bottom bound box boy.
boyfriend brace braced bracket bracketed brad brain branch.
brandon bravo brb bread break breakfast breath breathe brenda.
brian bright bring britain british broke broken bronze.
brooklyn brooks brother brought brown brush buddy buffer build.
bull bundle burn burst bus bush business busy but butt buy by.
byte bytes c c'est cake calendar call calm came camera camila.
campbell can can't canada cannabis cap captain car carbon card.
cardinal care carefully cargo carol carolyn carpe carry carter.
case cast castillo cat catch catherine caught cause.
celebration celeste celestial cell center centre chain chair.
chance change chapter charles charlie charlotte chase chavez.
check cheek chest chi child children china chinese chloe chris.
christ christina christine christopher chrysalis chuckle.
circle city clark class clean clear click climb clock close.
clothes clown club clubs coaster cocaine code coffee cogito.
coil coin cold college collins colon color column com combat.
come comment compare compilation compiled complete completely.
compute computer computing concern config configuration.
confuse consider consistency consistent continue control.
conversation cook cool cooper core corn corner cosmic cosmos.
couch could couldn't count counter country couple course cover.
cower cox crack craft crate crazy cross crowd cruz cry crypto.
cryptographic cryptography crystal cup customer cut cute cyan.
cynthia d dad damn dance danger dangerous daniel dank dark.
dash data database date daughter david davis day days daze de.
dead deal dear death deborah debra december decide decimal.
deep default definite definitely delete deleted delta demo.
democratic demonstration dennis des descend descendant desk.
diamond diamonds diane diaz did didn't die diem different dig.
digest dinner direction disappear distance distances distant.
divide divided do doctor document documents does doesn't dog.
dolor don't donald donate done donna door dope dorothy double.
doubt douglas down downcast dozen drag draw dream dress drink.
drive drop drove dry du dungeon duration durations during.
dylan e each ear early earth earthquake easily easy eat echo.
ed eddy edge edward edwards efficient eg egg eggs eight eighth.
either elbow elbows elicit elijah elite elizabeth ella else.
emily emma empire empty encode encoding end engine engineer.
english enjoy enough enter entire enum eon epic epoch epsilon.
equation ergo eric erin error errors escape especially est et.
eta eternal ethan eu europe european evans evelyn even evening.
eventual eventually ever every everyone everything evil ex.
exactly example except excite exclaim exclusive excuse exp.
expect experience explain export expression extension eye.
eyebrow eyes f face fact fall family far fast father fault.
favorite fear february feel feet fell felt fetch fetching few.
fic ficis fiction ficus field fifth fight fighting figure file.
files filesystem fill finally find fine finger fingers finish.
finite fire first fit five fix flame flash flip floor florence.
flores fly focus follow food foot for force foreign forever.
forget form format fortnight forward foster fought found four.
fourth foxtrot frank free french freya friday friend fright.
from front frost frown frozen fruit fuck full fun function.
functional funny further g gabriel game gamer gaming gamma.
garcia garden gary gasp gave gay gaze gently george german get.
giant gif giggle girl girlfriend git give given glad glance.
glare glass go god gods gold golf gomez gone gonna gonzales.
good goods got gotten government grab grace gram grave gray.
great green greet gregory grey grin grip groan ground group.
grow grunt guard guess gun gutierrez guts guy h ha had hadn't.
hah haha hahaha hair half hall hallway hand handle hang hannah.
happen happy har hard harper harris harry harsh has hash.
hashing hate have haven't he he'd he's head health hear heard.
heart hearts heather heavy held helen helium hell hello help.
henry her here hernandez heroes herself hey hi hide high hill.
him himself his historical history hit hitman hmm hold home.
hope horror horse hospital hot hotel hour house how howard.
however hp html hug huge hughes huh hum human hundred hung.
hungry hurry hurt husband hydrogen hyphen i i'd i'll i'm i've.
ice icon idea identity if ignore imagine immediately immutable.
import important in incredible india indigo inert infinite.
infinity initial insert inside instead intelligence interest.
internet interrupt intersect intersection into invade iota.
iphone ipsum iron is isaac isabella isn't it it's its ivy j.
jack jacket jackson jacob james janet jank january jason jay.
jayden je jeans jeb jeffrey jenkins jennifer jeremiah jeremy.
jerk jerry jesse jessica jessie jesus jimenez joan job john.
johnson join joke jonathan jones jose joseph joshua joy joyce.
jpeg jpg judith julie juliette july jump june junk just justin.
k kappa karen katherine kathleen keep kelly kenneth kent kept.
kevin key kick kid kill kilo kilobyte kilogram kilometer.
kilometre kim kimberly kind king kingdom kiss kitchen kitten.
knee knew knock know known kyle l la ladder lady lake lambda.
land language large larry larva larvae last late laugh laura.
lauren lay layla le lead lean learn least leave led lee left.
leg legend legends legion lens leo leroy les lesbian less let.
letter lewis liam lie life lift light like lillian lily lima.
linda line linux lip lisa listen little live lmao load loaded.
loading local localhost lock locker logan logic logical lol.
long look lopez lorem lose lost lot loud love low lucas luke.
luna lunch lust m mac machina machine mad made madison magenta.
magic main mais make man mana manage many march margaret maria.
marijauna mark marriage marry martin martinez mary mason match.
math mathematics maths matter matthew may maybe me mean meant.
median meet megan melissa memory men mendoza mention merci.
merge met metal meter meth methamphetamine metre mia michael.
michelle microsecond middle might mike miles milestone miller.
milligram millimeter millimetre million millisecond mind mine.
minus minute mirror miss mitchell mode mom moment monday money.
monster month mood moon moore morales more morgan morning.
morris mort most mother mouth move movie mp mr mrs mu much.
muhammad mum mumble municipal municipality murphy mushrooms.
music must mutable mutter my myers myself n name nancy nano.
nanosecond natalie nathan nation ne near nearly neck need.
negative nelson nervous net never new next nguyen nibble nice.
nicholas nicole night nine ninth no noah nod noise non none.
normal nose not note nothing notice novel novella november now.
nu null number o obliterate obviously ocean october of off.
offer office often oh okay old oliver olivia omega omicron on.
once one online only onto open optimal optimise optimize or.
orange orbit order org ortiz oscar other our out outline.
outside oval over own owner p pack package pain paint pair.
pamela pants papa paper paragraph paren parent parentheses.
parenthesis parenthesized parents park parker parse parsed.
part partner party pas pass past patel patricia patrick paul.
pause pay peas people percent perez perfect perhaps period.
person peter peterson phi phillips phone pi pick picture pie.
piece pink piss pit place plan planet plant platinum play.
player playing please plus png pocket point points police pop.
porn pornography position possible post pot pound power.
practically preferences present press pretend pretty price.
prime pro probably problem produce product prof professor.
progress promise protest protocol prototype province.
provincial psi pub pull punch pupa puppy push put q quad.
quadruple quake quebec queen question questioned queue quick.
quickly quid quiet quietly quite quo qwerty r race rachel rain.
raise ramirez ramos ran rang rather ray raymond reach read.
reader ready real realize really reason rebecca recognize.
rectangle red redundancy redundant reed reference register.
relationship relax remain remember remind remove removed.
rename repeat reply report respond rest resting return reyes.
rho richard richardson ride right ring river rivera road.
robert roberts robinson rock rodriguez rofl roger rogers roll.
romeo ronald room root rope rose ross round row rub ruiz run.
rush rust ruth ryan s sad safe said samantha same samuel.
sanchez sanders sandra sarah sat saturday save saved saving.
saw say scare scared scarlett schema scheme school scott.
scream sea seal search seat second sed see seem seen self.
selfie semicolon send sense sensitive sent sentence september.
serious seriously service set settle seven seventh several sex.
sexual sexy shadow shake shallow shape share sharon she she'd.
she's shift ship shirley shirt shit shock shoe shook shop.
short shot should shoulder shouldn't shout shove show shower.
shrug shut sick side sierra sigh sight sign signature silence.
silent silver simply since sing single sir sister sit.
situation six sixth skin sky slam sleep slightly slip slow.
slowly small smart smartphone smell smile smirk smith smoke.
snap snort so sofia soft softly sol some somehow someone.
something sometimes somewhere son song soon sophia sorry sort.
sound source space spade spades spanish speak spend spent.
spice spicy spider spoke spot spouse spring sql sqlite square.
squared stair stand star stare start state states station stay.
steal steam steel. step stephanie stephen steven stewart stick.
still stomach stone stood stop store story straight strange.
strata street strike string strong struct structure struggle.
stuck student study stuff stupid such suck sudden suddenly.
suggest sum summer sun sunday sunny suppose sure surprise.
surround susan sweet sync synchronous t table take taken talk.
tall tango tap taylor teacher teal team tear tech technology.
teeth tell ten tenth terra terran terre text than thank that.
that's the their them themselves then there there's these.
theta they they'd they're thick thin thing think third this.
thomas thompson those though thought thousand three threw.
thrill throat through throw thrust thursday tie tight time.
times timothy tiny tire tired title to today todo toe toes.
together told tomorrow tone tongue tonight too took top torpor.
torres totally touch toward tower town trace tracing track.
trade trader trail train trait traitor trans transgender tree.
trillion trip triple trouble true trump trumpet trunk trust.
truth try tu tube tuesday tunnel turn turner tv twenty two.
tyler type u uh um un uncle under understand une unicode.
uniform unify union unit unite united unix until up upcast.
upon upsilon ur url us use user usual usually utf-8 v vegan.
veganism vegetable vegetarian vein very vet vibe vibes vibrate.
vibration victor victoria vie vine violet virginia visible.
visit visual voice void volume volumes w wait wake walk walker.
wall walter want war ward warm warn wars was washington wasn't.
watch watcher watching water watson wave way we we'll we're.
we've wear web wedding wednesday weed week weird well went.
were weren't wet what what's whatever when where whether which.
while whine whiskey whisper white who whole why wide wife will.
william williams willow wilson wind window windows windy.
winter wipe wish witch with within without wizard woke woman.
women won't wonder wood word wore work world worldwide worry.
worse would wouldn't wow wrap wreck wright write writer.
written wrong www x x-ray xe xi xp y yeah year yeet yell.
yellow yes yet you you'd you'll you're you've young your.
yourself youth z zachary zero zeta zoe zoey zone zones zulu.
"
};
