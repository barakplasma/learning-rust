const assert = require('chai').assert

class CapitalizationStrategy {
  constructor(strategy) {
    this.strategy = strategy;
  }
  
  applyTitleCase(word) {
    return this.strategy(word);
  }
}

class NullCapitalizationStrategy extends CapitalizationStrategy {
  constructor()  {
    super(word => word);
  };
}

class RegularWordCapitalizationStrategy extends CapitalizationStrategy {
  constructor()  {
    super(word => word[0].toUpperCase() + word.slice(1).toLowerCase());
  };
}

class MinorWordCapitalizationStrategy extends CapitalizationStrategy {
  constructor()  {
    super(word => word.toLowerCase());
  };
}

class MinorWords {
  constructor(minorWords) {
    this.minorWordsArray = minorWords ? minorWords.toLowerCase().split(' ') : [];
  }
  isMinorWord(word) {
    return this.minorWordsArray.some(function (mw) { 
      return mw === word;
    }); 
  }
}

class TitleCaseSolution {
  constructor(minorWords) {
    this.MinorWords = new MinorWords(minorWords);
  }

  titleInvalid(title) {
    return typeof this.title === 'undefined' || this.title.length === 0;
  }

  titleCase(title) {
    this.title = title;
    if(this.titleInvalid()) return title;
    return title.toLowerCase().split(' ').map(function (word, index) {
      var strategy = this.getCapitalizationStrategy(word, index === 0);
      return strategy.applyTitleCase(word);
    }, this).join(' ');
  }
  
  getCapitalizationStrategy(word, isFirstWord) {  
    if (word.length === 0) {
      return new NullCapitalizationStrategy();
    }
    else if (isFirstWord || !this.MinorWords.isMinorWord(word)) {
      return new RegularWordCapitalizationStrategy();
    }
    else {
      return new MinorWordCapitalizationStrategy();
    }
  }
}

function titleCase(title, minorWords) {
  const tcs = new TitleCaseSolution(minorWords);
  
  return tcs.titleCase(title)
}

'use strict';
 
var ELIDED = '<..>';
 
function repr(thing, depth, max) {
    if (depth === undefined) { depth = 0; }
    if (max === undefined) { max = 2; }
 
    if (isPrim(thing)) {
        return showPrim(thing);
    }
 
    var type = typeof (thing), result;
 
    if (type === 'function') {
        result = 'function';
    } else if (Array.isArray(thing)) {
        result = showArray(thing, depth, max);
    } else if (thing.constructor === Object) {
        result = showObject(thing, depth, max);
    } else if (thing.constructor === String) {
        // show string objects the same way as string prims, not sure
        // if this is the morally correct course of action...
        result = showPrim(thing.toString());
    } else {
        result = thing.toString();
    }
    return result;
}
 
function isPrim(thing) {
    return thing === null || /^[snbu]/.test(typeof thing);
}
 
function showPrim(prim) {
    var type = typeof (prim), result;
 
    if (type === 'string') {
        result = JSON.stringify(prim);
    } else if (type === 'number' || type === 'boolean') {
        result = prim.toString();
    } else if (prim === null) {
        result = 'null';
    } else if (type === 'undefined') {
        result = 'undefined';
    }
    return result;
}
 
function showArray(arr, depth, max) {
    if (depth === undefined) { depth = 0; }
    if (max === undefined) { max = 2; }
 
    if (depth >= max) {
        return ELIDED;
    }
 
    var result = '[', i;
 
    // for loop instead of forEach to correctly handle sparse arrays
    for (i = 0; i < arr.length; i += 1) {
        if (i !== 0) {
            result += ', ';
        }
        // can't simply check if arr[i] is undef because there's a
        // distinction between arr[i] being set to undef and it never
        // being set at all
        if (arr.hasOwnProperty(i.toString())) {
            result += repr(arr[i], depth + 1, max);
        } else {
            result += '';
        }
    }
    result += ']';
    return result;
}
 
function showObject(o, depth, max) {
    if (depth === undefined) { depth = 0; }
    if (max === undefined) { max = 2; }
 
    if (depth >= max) {
        return ELIDED;
    }
 
    var result = '{';
    Object.keys(o).forEach(function (key, ix) {
        if (ix !== 0) {
            result += ', ';
        }
        result += key + ': ' + repr(o[key], depth + 1, max);
    });
    result += '}';
    return result;
}

function test(title, minorWords, expect) {
    var answer = titleCase(title, minorWords);
    if (answer instanceof String) {
        answer = answer.valueOf();
    }
    var message = 'Gave title=' + repr(title);
    if (minorWords !== undefined) {
        message += ', minorWords=' + repr(minorWords);
    }
    message += '. Expected ' + repr(expect) + ' but got ' + repr(answer) + '.';
    assert.equal(answer, expect, message);
}

test('', '', '');
test('aBC deF Ghi', undefined, 'Abc Def Ghi');
test('ab', 'ab', 'Ab');
test('a bc', 'bc', 'A bc');
test('a bc', 'BC','A bc');
test('First a of in', 'an often into', 'First A Of In');
test('a clash of KINGS', 'a an the OF', 'A Clash of Kings');
test('the QUICK bRoWn fOX', 'xyz fox quick the', 'The quick Brown fox');