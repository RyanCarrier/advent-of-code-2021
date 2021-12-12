package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Heights [][]int
type BasinMap [][]bool
type Point [2]int

func (bm *BasinMap) get(p Point) bool {
	return (*bm)[p[0]][p[1]]
}
func (bm *BasinMap) set(p Point) {
	(*bm)[p[0]][p[1]] = true
}

func (h *Heights) getBasinSizes() []int {

	basinSizes := []int{}
	bmd := make([][]bool, len(*h))
	for i := 0; i < len(*h); i++ {
		bmd[i] = make([]bool, len((*h)[0]))
	}
	bm := BasinMap(bmd)
	//init done
	for _, p := range h.getLow() {
		bm.set(p)
		nextSet := []Point{p}
		var basinTempSize int
		basinSize := 0
		for len(nextSet) > 0 {
			nextSet, basinTempSize = h.basinNeighbours(nextSet, &bm)
			basinSize += basinTempSize
		}
		basinSizes = append(basinSizes, basinSize+1)
	}
	return basinSizes
}

func (h *Heights) basinNeighbours(points []Point, bm *BasinMap) ([]Point, int) {
	nextSet := []Point{}
	count := 0
	for _, p := range points {
		for _, f := range h.getNeighbours(p) {
			if h.get(f) == 9 || bm.get(f) || h.get(f) < h.get(p) {
				continue
			}
			bm.set(f)
			count++
			nextSet = append(nextSet, f)
		}
	}
	return nextSet, count
}

func (h *Heights) getRisk() int {
	total := 0
	for _, l := range h.getLow() {
		total += 1 + h.get(l)
	}
	return total
}

func (h *Heights) get(p Point) int {
	return (*h)[p[0]][p[1]]
}

func (h *Heights) getLow() []Point {
	lowList := []Point{}
	lx := len(*h)
	ly := len((*h)[0])
	for x := 0; x < lx; x++ {
		for y := 0; y < ly; y++ {
			if h.lowestPoint(Point{x, y}) {
				lowList = append(lowList, Point{x, y})
			}
		}
	}
	return lowList
}

func (h *Heights) lowestPoint(p Point) bool {
	pv := h.get(p)
	for _, dp := range h.getNeighbours(p) {
		if h.get(dp) <= pv {
			return false
		}
	}
	return true
}

func (h *Heights) getNeighbours(p Point) (validPoints []Point) {
	lx := len(*h)
	ly := len((*h)[0])
	for _, transform := range [][]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}} {
		x, y := p[0]+transform[0], p[1]+transform[1]
		if x >= 0 && x < lx && y >= 0 && y < ly {
			validPoints = append(validPoints, Point{x, y})
		}
	}
	return
}

func main() {
	now := time.Now()
	basin := importData()
	fmt.Println("part1:", basin.getRisk())
	basinSizes := basin.getBasinSizes()
	sort.Ints(basinSizes)
	product := 1
	for _, size := range basinSizes[len(basinSizes)-3:] {
		product *= size
	}
	fmt.Println("part2:", product)
	fmt.Printf("took %dms\n", time.Since(now).Milliseconds())

}

func importData() Heights {
	lines := strings.Split(strings.TrimSpace(data), "\n")
	x := len(lines)
	y := len(lines[0])
	b := make([][]int, x)
	for i := range b {
		b[i] = make([]int, y)
	}
	for i, line := range lines {
		for j, c := range line {
			b[i][j], _ = strconv.Atoi(string(c))
		}
	}
	return b
}
