module.exports = {
  mode: "production",
  entry: {
    "testing-library": './src/testing-library.js',
  },
  output: {
    filename: '[name].js',
    path: __dirname + '/dist',
  },
}
