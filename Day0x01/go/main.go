package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type ShouldUpdateIdxFunc func(currIdx int, newIdx int) bool

var ShouldUpdateIdx ShouldUpdateIdxFunc = func(currIdx int, newIdx int) bool {
	return currIdx > newIdx
}

var NumWordToDigit = map[string]int{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func GetDigit(s string, last bool) (int, int) {
	digit := -1
	digitIdx := -1

	subStringSearch := strings.Index
	shouldUpdateIdx := ShouldUpdateIdx
	if last {
		subStringSearch = strings.LastIndex
		shouldUpdateIdx = func(currIdx int, newIdx int) bool {
			return !ShouldUpdateIdx(currIdx, newIdx)
		}
	}

	for i := 1; i < 10; i++ {
		idx := subStringSearch(s, strconv.Itoa(i))
		if idx < 0 {
			continue
		}

		if digitIdx < 0 || shouldUpdateIdx(digitIdx, idx) {
			digit = i
			digitIdx = idx
		}
	}

	return digit, digitIdx
}

func GetNumWord(s string, last bool) (int, int) {
	digit := -1
	numWordIdx := -1

	subStringSearch := strings.Index
	shouldUpdateIdx := ShouldUpdateIdx
	if last {
		subStringSearch = strings.LastIndex
		shouldUpdateIdx = func(currIdx int, newIdx int) bool {
			return !ShouldUpdateIdx(currIdx, newIdx)
		}
	}

	for numWord, numDigit := range NumWordToDigit {
		idx := subStringSearch(s, numWord)
		if idx < 0 {
			continue
		}

		if numWordIdx < 0 || shouldUpdateIdx(numWordIdx, idx) {
			digit = numDigit
			numWordIdx = idx
		}
	}

	return digit, numWordIdx
}

func main() {
	filePath := "../document.txt"
	f, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("failed to Open file %s: %v\n", filePath, err)
	}

	calDigitsSum := 0
	calDigitsAndWordsSum := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		firstDigit, firstDigitIndex := GetDigit(line, false)
		lastDigit, lastDigitIndex := GetDigit(line, true)

		calDigitsSum += (firstDigit * 10) + lastDigit

		firstNumWord, firstNumWordIdx := GetNumWord(line, false)
		lastNumWord, lastNumWordIdx := GetNumWord(line, true)

		firstDigitOrNumWord := firstDigit
		if firstDigitIndex < 0 || (firstNumWordIdx >= 0 && firstDigitIndex > firstNumWordIdx) {
			firstDigitOrNumWord = firstNumWord
		}

		lastDigitOrNumWord := lastDigit
		if lastDigitIndex < 0 || lastDigitIndex < lastNumWordIdx {
			lastDigitOrNumWord = lastNumWord
		}

		calDigitsAndWordsSum += (firstDigitOrNumWord * 10) + lastDigitOrNumWord
	}

	fmt.Println(calDigitsSum)
	fmt.Println(calDigitsAndWordsSum)
}
