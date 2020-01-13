// https://www.codewars.com/kata/paginationhelper/train/javascript

const { expect } = require("chai");

/**
 * 
 *  For this exercise you will be strengthening your page-fu mastery. You will complete the PaginationHelper class, which is a utility class helpful for querying paging information related to an array.

    The class is designed to take in an array of values and an integer indicating how many items will be allowed per each page. The types of values contained within the collection/array are not relevant.
 * 
 * @param collection 
 * @param itemsPerPage 
 */

// TODO: complete this object/class

// The constructor takes in an array of items and a integer indicating how many
// items fit within a single page
class PaginationHelper {
    pages = [];
    itemsPerPage = 0;


    constructor(collection: any[], itemsPerPage: number) {
        this.itemsPerPage = itemsPerPage;
        this.pages = this.addToPages(collection);
    }
    addToPages(collection: any[]) {
        return collection.reduce((acc, cur, _, arr) => {
            const latestPage = acc[acc.length-1];
            if (latestPage.length >= this.itemsPerPage) {
                acc.push([cur]);
                return acc;
            } else {
                latestPage.push(cur);
                return acc;
            }
        }, [[]]);
    }

    // returns the number of items within the entire collection
    itemCount() {
        return this.pages.reduce((acc, curPage) => acc+curPage.length, 0);
    }

    // returns the number of pages
    pageCount() {
        return this.pages.length;
    }

    // returns the number of items on the current page. page_index is zero based.
    // this method should return -1 for pageIndex values that are out of range
    pageItemCount(pageIndex: number) {
        const pgCount = this.pageCount();
        if (pageIndex < 0 || pageIndex > pgCount) return -1;
        return this.pages[pageIndex] && this.pages[pageIndex].length || -1;
    }

    // determines what page an item is on. Zero based indexes
    // this method should return -1 for itemIndex values that are out of range
    pageIndex(itemIndex: number) {
        if (itemIndex === 0 && this.itemCount() !== 0)
            return 0;
        if (this.itemCount() < itemIndex || itemIndex < 0 || this.itemCount() === 0)
            return -1;
        return this.itemCount() % itemIndex;
    }
}


var helper = new PaginationHelper(['a', 'b', 'c', 'd', 'e', 'f'], 4);

expect(helper.pageCount()).to.equal(2); //should == 2
expect(helper.itemCount()).to.equal(6); //should == 6
expect(helper.pageItemCount(0)).to.equal(4); //should == 4
expect(helper.pageItemCount(1)).to.equal(2); // last page - should == 2
expect(helper.pageItemCount(2)).to.equal(-1); // should == -1 since the page is invalid

// pageIndex takes an item index and returns the page that it belongs on
expect(helper.pageIndex(5)).to.equal(1); //should == 1 (zero based index)
expect(helper.pageIndex(2)).to.equal(0); //should == 0
expect(helper.pageIndex(20)).to.equal(-1); //should == -1
expect(helper.pageIndex(-10)).to.equal(-1); //should == -1