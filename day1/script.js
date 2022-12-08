const fs = require('fs')


const data = fs.readFileSync('data.txt', 'utf-8')
const dataList = data.split('\r\n')


let total = []
let currentSum = 0

for (let el of dataList) {
    if (el === '') {
        total.push(currentSum)
        currentSum = 0
        continue
    }

    currentSum += parseInt(el)
}

const result = total.sort()[0]

console.log(result);