const path = require('path')
module.exports = ({ file }) => {
  return {
    plugins: [require('autoprefixer')],
  }
}
