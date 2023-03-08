use domain::value_structs::hashtag::Hashtag;
use regex::Regex;
use std::collections::HashSet;

pub fn extract(text: &str) -> HashSet<Hashtag> {
    let regex = Regex::new(r"^#.+$").unwrap();

    text.split_whitespace()
        .filter(|segment| regex.is_match(segment))
        .map(Hashtag::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_structs::hashtag::Hashtag;

    #[test]
    fn can_extract_hashtags() {
        let text = r#"#ランチには、新しくオープンしたカフェで美味しいパンケーキを食べました！#foodie #cafe #pancakes
            今日は朝からジョギングしてスッキリ！健康的な生活って大事ですね。#fitness #healthylifestyle #jogging
            昨日のディナーは友達と一緒にイタリアンレストランで食べました。赤ワインも美味しかった！#dinner #italianfood #wine
            今日は久しぶりに自分へのご褒美にショッピング！新しい服をたくさん買っちゃいました♪#shopping #fashion #selfcare
            #先頭
            新しい映画が公開されたので、友達と映画館に行ってきました。面白かった！#movie #entertainment #friends
            昨日は家族でBBQをしました。天気も良くて楽しかった！#bbq #familytime #outdoor 途中で言葉を挟んでみる
            今日のランチは、オーガニックカフェで野菜たっぷりのサラダを食べました。体に優しい食生活を心掛けています。#organicfood #salad #healthylunch
            全角　#スペース　です
            今日はお祭りがあるので、友達と一緒に出かけてきました。屋台でたくさん食べちゃいました！　#festival #friends #food
            昨日は家でのんびり映画を観ました。自分だけの時間って良いですね。 #movie #relaxation #selfcare
            最近読んだ本が面白かったので、感想をブログに書きました。みんなにもおすすめしたいです！#CAPITALLETTERS #bookreview #blogging #recommendation
            #recommendation 
            asdf
            #にほんご
            #絵文字いり📅 asd"#;

        let matches = vec![
            Hashtag::from(
                "#ランチには、新しくオープンしたカフェで美味しいパンケーキを食べました！#foodie",
            ),
            Hashtag::from("#cafe"),
            Hashtag::from("#pancakes"),
            Hashtag::from("#healthylifestyle"),
            Hashtag::from("#jogging"),
            Hashtag::from("#italianfood"),
            Hashtag::from("#wine"),
            Hashtag::from("#fashion"),
            Hashtag::from("#selfcare"),
            Hashtag::from("#先頭"),
            Hashtag::from("#entertainment"),
            Hashtag::from("#friends"),
            Hashtag::from("#familytime"),
            Hashtag::from("#outdoor"),
            Hashtag::from("#salad"),
            Hashtag::from("#healthylunch"),
            Hashtag::from("#スペース"),
            Hashtag::from("#festival"),
            Hashtag::from("#friends"),
            Hashtag::from("#food"),
            Hashtag::from("#movie"),
            Hashtag::from("#relaxation"),
            Hashtag::from("#selfcare"),
            Hashtag::from("#bookreview"),
            Hashtag::from("#blogging"),
            Hashtag::from("#recommendation"),
            Hashtag::from("#にほんご"),
            Hashtag::from("#絵文字いり📅"),
        ];

        let hashset = HashSet::from_iter(matches);

        assert_eq!(extract(text), hashset);
    }
}
