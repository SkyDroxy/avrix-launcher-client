import { NewsTag, NewsTagValue } from '@common/enums/NewsTag';

export interface NewsItem {
  id: number;
  image: string;
  title: string;
  month: string;
  tag: NewsTagValue;
  statusDot: boolean;
}

export { NewsTag };
