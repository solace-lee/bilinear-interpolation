import image from './image.json' assert { type: 'json' }
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filenameNew = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filenameNew)

// JS原生处理
function main() {
  const { data } = image
  const newData = []
  Object.entries(data).forEach(o => {
    newData[Number(o[0])] = o[1]
  })
  newData.forEach((v, i) => {
    if (!v) {
      newData[i] = []
    }
  })
  const obj = {
    ...image,
    data: newData
  }
  fs.writeFileSync(path.resolve(__dirname, './RT_fmt.json'), JSON.stringify(obj))
}

main()

