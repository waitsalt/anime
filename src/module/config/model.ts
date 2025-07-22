import { ServiceConfig } from "../service/model";
import { ThemeConfig } from "../theme/model";

interface Config {
  service: ServiceConfig;
  theme: ThemeConfig;
}

export { type Config };
