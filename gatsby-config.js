/* eslint-disable no-undef */

/**
 * @type {import('gatsby').GatsbyConfig}
 */
module.exports = {
  plugins: [
    'gatsby-plugin-styled-components',
    'gatsby-plugin-typescript-checker',
    'gatsby-plugin-typescript',
    {
      resolve: 'gatsby-plugin-eslint',
      options: {
        test: /\.ts$|\.tsx$/
      }
    }
  ]
}
