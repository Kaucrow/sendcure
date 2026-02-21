import swaggerJSDoc from 'swagger-jsdoc';
import { SwaggerTheme, SwaggerThemeNameEnum } from 'swagger-themes';

const theme = new SwaggerTheme();

const swaggerSpecOptions = {
  swaggerDefinition: {
    openapi: '3.0.0',
    info: {
      title: 'Sendcure Main Server API',
      version: '0.1',
    },
    components: {
      securitySchemes: {
        bearerAuth: {
          type: 'http',
          scheme: 'bearer',
          bearerFormat: 'PASETO',
          description: 'Enter the Bearer Token (PASETO) here.'
        }
      }
    }
  },
  apis: [
    './src/app.ts',
    './src/routes/**/*.ts',
    './src/business-objects/**/*.ts'
  ]
};

export const swaggerDocs = swaggerJSDoc(swaggerSpecOptions);

export const swaggerUIOptions = {
  customCss: theme.getBuffer(SwaggerThemeNameEnum.GRUVBOX),
};