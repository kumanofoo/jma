# JMA Library

## References
- [気象庁が提供する天気予報データの読み解き方](https://github.com/misohena/el-jma/blob/main/docs/how-to-get-jma-forecast.org)

##  Weather Code (automatic observed)
[配信資料に関する仕様 No.13301 ～地域気象観測データ～](https://www.data.jma.go.jp/suishin/shiyou/pdf/no13301)

「0 20 212 自動観測による天気」(p.23)
|Code|Symbol|Description|SVG|PNG|
|:--- | :--- | :--- | :---: | :---: |
|0|![0.bmp](img/0.bmp)| 晴 | ![CLEAR](https://www.jma.go.jp/bosai/forecast/img/100.svg) ![CLEAR](https://www.jma.go.jp/bosai/forecast/img/500.svg) | ![Clear](https://worldweather.wmo.int/images/24a.png) |
|1|![1.bmp](img/1.bmp)| 曇 | ![CLOUDY](https://www.jma.go.jp/bosai/forecast/img/200.svg) ![CLOUDY](https://www.jma.go.jp/bosai/forecast/img/200.svg) | ![Cloudy](https://worldweather.wmo.int/images/23a.png) |
|2|![2.bmp](img/2.bmp)| 煙霧 !| ![CLOUDY](https://www.jma.go.jp/bosai/forecast/img/200.svg) ![CLOUDY](https://www.jma.go.jp/bosai/forecast/img/200.svg) | ![Haze](https://worldweather.wmo.int/images/19.png) |
|3|![3.bmp](img/3.bmp)| 霧 | ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) | ![Mist](https://worldweather.wmo.int/images/17.png) |
|4|![4.bmp](img/4.bmp)| 降水またはしゅう雨性の降水 | ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) | ![Occasional Showers](https://worldweather.wmo.int/images/10.png) |
|5|![5.bmp](img/5.bmp)| 霧雨 | ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) | ![Drizzle](https://worldweather.wmo.int/images/15.png) |
|6|![6.bmp](img/6.bmp)| 着氷性の霧雨 | ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) | ![Freezing Rain](https://worldweather.wmo.int/images/13.png) |
|7|![7.bmp](img/7.bmp)| 雨 | ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) | ![Rain](https://worldweather.wmo.int/images/14.png) |
|8|![8.bmp](img/8.bmp)| 着氷性の雨 | ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) | ![Freezing Rain](https://worldweather.wmo.int/images/13.png) |
|9|![9.bmp](img/9.bmp)| みぞれ | ![SNOW,FREQUENT SCCATERED SHOWERS](https://www.jma.go.jp/bosai/forecast/img/403.svg) ![SNOW,FREQUENT SCCATERED SHOWERS](https://www.jma.go.jp/bosai/forecast/img/403.svg) | ![Sleet](https://worldweather.wmo.int/images/8.png) |
|10|![10.bmp](img/10.bmp)| 雪 | ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) | ![Snowfall](https://worldweather.wmo.int/images/6.png) |
|11|![11.bmp](img/11.bmp)| 凍雨 | ![SNOW,FREQUENT SCCATERED SHOWERS](https://www.jma.go.jp/bosai/forecast/img/403.svg) ![SNOW,FREQUENT SCCATERED SHOWERS](https://www.jma.go.jp/bosai/forecast/img/403.svg) | ![Hail](https://worldweather.wmo.int/images/3.png) |
|12|![12.bmp](img/12.bmp)| 霧雪 | ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) | ![Light Snow](https://worldweather.wmo.int/images/7.png) |
|13|![13.bmp](img/13.bmp)| しゅう雨または止み間のある雨 | ![SHOWERS THROUGHOUT THE DAY](https://www.jma.go.jp/bosai/forecast/img/302.svg) ![SHOWERS THROUGHOUT THE DAY](https://www.jma.go.jp/bosai/forecast/img/302.svg) | ![Light Showers](https://worldweather.wmo.int/images/12.png) |
|14|![14.bmp](img/14.bmp)| しゅう雪または止み間のある雪 | ![SNOWTHROUGHOUT THE DAY](https://www.jma.go.jp/bosai/forecast/img/402.svg) ![SNOWTHROUGHOUT THE DAY](https://www.jma.go.jp/bosai/forecast/img/402.svg) | ![Light Snow](https://worldweather.wmo.int/images/7.png) |
|15|![15.bmp](img/15.bmp)| ひょう | ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) ![SNOW](https://www.jma.go.jp/bosai/forecast/img/400.svg) | ![Hail](https://worldweather.wmo.int/images/3.png) |
|16|![16.bmp](img/16.bmp)| 雷 | ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) ![RAIN](https://www.jma.go.jp/bosai/forecast/img/300.svg) | ![Lightning](https://worldweather.wmo.int/images/2.png) |
|17-29| N/A | 保留 | |
|30| N/A | 天気不明 | |
|31| N/A | 欠測 | |

Symbol Image: https://www.jma.go.jp/bosai/amedas/img/{_code_}.bmp
SVG Image: https://www.jma.go.jp/bosai/forecast/img/{_code_}.svg
