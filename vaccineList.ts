import {expect} from 'chai';
// https://www.codewars.com/kata/vaccinenation/train/javascript
// Vaccinations for children under 5
// You have been put in charge of administrating vaccinations for children in your local area. Write a function that will generate a list of vaccines for each child presented for vaccination, based on the child's age and vaccination history, and the month of the year.

// The function takes three parameters: age, status and month
// The parameter 'age' will be given in weeks up to 16 weeks, and thereafter in months. You can assume that children presented will be scheduled for vaccination (eg '16 weeks', '12 months' etc).
// The parameter 'status' indicates if the child has missed a scheduled vaccination, and the argument will be a string that says 'up-to-date', or a scheduled stage (eg '8 weeks') that has been missed, in which case you need to add any missing shots to the list. Only one missed vaccination stage will be passed in per function call.
// If the month is 'september', 'october' or 'november' add 'offer fluVaccine' to the list.
// Make sure there are no duplicates in the returned list, and sort it alphabetically.
// Example input and output
// input     ('12 weeks', 'up-to-date', 'december')
// output    ['fiveInOne', 'rotavirus']

// input     ('12 months', '16 weeks', 'june')
// output     ['fiveInOne', 'hibMenC', 'measlesMumpsRubella', 'meningitisB', 'pneumococcal']

// input     ('40 months', '12 months', 'october') 
// output    ['hibMenC', 'measlesMumpsRubella', 'meningitisB', 'offer fluVaccine', 'preSchoolBooster']
// To save you typing it up, here is the vaccinations list

// fiveInOne : ['8 weeks', '12 weeks', '16 weeks'],
// //Protects against: diphtheria, tetanus, whooping cough, polio and Hib (Haemophilus influenzae type b)
// pneumococcal : ['8 weeks', '16 weeks'],
// //Protects against: some types of pneumococcal infection
// rotavirus : ['8 weeks', '12 weeks'],
// //Protects against: rotavirus infection, a common cause of childhood diarrhoea and sickness
// meningitisB : ['8 weeks', '16 weeks', '12 months'],
// //Protects against: meningitis caused by meningococcal type B bacteria
// hibMenC : ['12 months'],
// //Protects against: Haemophilus influenzae type b (Hib), meningitis caused by meningococcal group C bacteria    
// measlesMumpsRubella : ['12 months', '40 months'],
// //Protects against: measles, mumps and rubella
// fluVaccine : ['september','october','november'],
// //Given at: annually in Sept/Oct
// preSchoolBooster : ['40 months']
// //Protects against: diphtheria, tetanus, whooping cough and polio
const Unique = (arr: any[]) => [...new Set(arr)];

class DateParser {
    originalDate: VaccineDate;
    days: Days;
    constructor(date: VaccineDate) {
        this.originalDate = date;
        if (date.includes('months')) {
            this.handleMonths();
        } else {
            this.handleWeeks();
        }
    }

    getNumeric() {
        return Number(this.originalDate.split(' ')[0])
    }
    handleMonths() {
        this.days = this.getNumeric() * 30;
    }

    handleWeeks() {
        this.days = this.getNumeric() * 7;
    }
}

class AgeParser extends DateParser {
    originalAge: VaccineDate;
    constructor(age: VaccineDate) {
        super(age);
        this.originalAge = age;
    }
}

class StatusParser extends AgeParser {
    isBehindOnVaccinations: boolean;
    constructor(status: string) {
        const isBehindOnVaccinations = status !== 'up-to-date';
        if (isBehindOnVaccinations) {
            super(status);
        } else {
            super('0 weeks');
        }
        this.isBehindOnVaccinations = isBehindOnVaccinations;
    }
}

interface IVaccineRule {
    requiredVaccines: requiredVaccines;
    applyRule: ()=>void;
}

type VaccineDate = string;
type VaccineSchedule = VaccineDate[];
type Days = number;
type requiredVaccines = Vaccine[];

class VaccineBaseRule implements IVaccineRule {
    requiredVaccines: requiredVaccines;
    applyRule: () => void;
    constructor(applyRule) {
        this.requiredVaccines = [];
        this.applyRule = applyRule;
    }

    finish() {
        this.requiredVaccines.forEach(this.applyRule);
    }
}

class MonthlyRules extends VaccineBaseRule {
    relevantVaccines: MonthlyScheduledVaccine[]
    month: string;
    constructor(applyRule, vaccines: MonthlyScheduledVaccine[], month: VaccineDate) {
        super(applyRule);
        this.relevantVaccines = vaccines;
        this.month = month;
        this.ruleStrategy();
    }

    ruleStrategy() {
        this.relevantVaccines.forEach(mv => {
            if(mv.months.includes(this.month)) {
                this.requiredVaccines.push(this.relevantVaccines[0])
            }
        })
    }
}

class AgeRules extends VaccineBaseRule {
    relevantVaccines: AgeScheduledVaccine[]
    constructor(applyRule, vaccines: AgeScheduledVaccine[], age: AgeParser) {
        super(applyRule);
        this.relevantVaccines = vaccines;
        this.ruleStrategy(age);
    }

    ruleStrategy(age: AgeParser) {
        this.relevantVaccines.forEach(
            v => {
                if(v.requiredAfter.map(d => d.days).includes(age.days)){
                    this.requiredVaccines.push(v);
                }
            }
        )
        this.finish();
    }
}

class Vaccine {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
}

class AgeScheduledVaccine extends Vaccine {
    requiredAfter: DateParser[];
    constructor(name: string, boosterDates: VaccineSchedule) {
        super(name);
        this.requiredAfter = this.parseSchedule(boosterDates);
        this.name = name;
    }

    parseSchedule(boosterDates: VaccineSchedule) {
        return boosterDates.map(bd => new DateParser(bd));
    }
}

class MonthlyScheduledVaccine extends Vaccine {
    months: string[];
    constructor(name, months: string[]) {
        super(name);
        this.months = months;
    }
}

const AgeScheduledVaccines = Object.entries({
    fiveInOne : ['8 weeks', '12 weeks', '16 weeks'],
    pneumococcal : ['8 weeks', '16 weeks'],
    rotavirus : ['8 weeks', '12 weeks'],
    meningitisB : ['8 weeks', '16 weeks', '12 months'],
    hibMenC : ['12 months'],
    measlesMumpsRubella : ['12 months', '40 months'],
    preSchoolBooster : ['40 months'],
}).map(v => new AgeScheduledVaccine(v[0], v[1]))

const MonthlyScheduledVaccines = Object.entries({
    'offer fluVaccine' : ['september','october','november']
}).map(v => new MonthlyScheduledVaccine(v[0], v[1]))

class Patient {
    age: AgeParser;
    requiredVaccines: requiredVaccines;

    status: StatusParser;

    constructor(age: string, status: string, month: string) {
        this.age = new AgeParser(age);
        this.requiredVaccines = [];
        this.status = new StatusParser(status);
        this.applyAgeVaccines();
        this.applyMonthlyVaccines(month);
        this.requiredVaccines = Unique(this.requiredVaccines);
    }

    applyVaccine = (vaccine: Vaccine) => {
        this.requiredVaccines.push(vaccine);
    }

    applyAgeVaccines() {
        const ageRules = new AgeRules(this.applyVaccine, AgeScheduledVaccines, this.age);
        if (this.status.isBehindOnVaccinations) {
            // act as if patient is younger as well
            const youngerVaccinations = new AgeRules(this.applyVaccine, AgeScheduledVaccines, this.status);
            youngerVaccinations.finish();
        }
        ageRules.finish();
    }

    applyMonthlyVaccines(month: string) {
        const monthlyRules = new MonthlyRules(this.applyVaccine, MonthlyScheduledVaccines, month);
        monthlyRules.finish();
    }
}

const vaccineList = (age: string, status: string, month: string) => {
    const patient = new Patient(age, status, month);
    return patient.requiredVaccines.map(rv => rv.name).sort();
}

const Test = {
    assertSimilar: (actual, expected, msg) => expect(actual, msg).to.deep.equal(expected)
}

Test.assertSimilar(vaccineList('12 weeks','up-to-date','december'), ['fiveInOne', 'rotavirus'], "Your list isn't returning what was expected.");
Test.assertSimilar(vaccineList('12 months','16 weeks','june'), ['fiveInOne', 'hibMenC', 'measlesMumpsRubella', 'meningitisB', 'pneumococcal'], "Your list isn't returning what was expected.");
Test.assertSimilar(vaccineList('40 months','12 months','october'), ['hibMenC','measlesMumpsRubella','meningitisB','offer fluVaccine','preSchoolBooster'], "Your list isn't returning what was expected.");