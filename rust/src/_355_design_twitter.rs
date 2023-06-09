use std::collections::{HashMap, HashSet};

struct Tweet {
    tweet_id: i32,
    posted_by_user_id: i32,
}

struct Twitter {
    tweets: Vec<Tweet>,
    users: HashMap<i32, HashSet<i32>>, // User to users they follow
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            tweets: Default::default(),
            users: Default::default(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push(Tweet {
            tweet_id,
            posted_by_user_id: user_id,
        })
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let max_tweets = 10;
        let mut result = Vec::with_capacity(max_tweets);
        let followees = self.users.get(&user_id);
        for tweet in self.tweets.iter().rev() {
            if tweet.posted_by_user_id == user_id
                || (followees.is_some() && followees.unwrap().contains(&tweet.posted_by_user_id))
            {
                result.push(tweet.tweet_id);
                if result.len() >= max_tweets {
                    break;
                }
            }
        }
        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_insert(HashSet::new())
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users.entry(follower_id).and_modify(|x| {
            x.remove(&followee_id);
        });
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5); // User 1 posts a new tweet (id = 5).
        assert_eq!(twitter.get_news_feed(1), vec![5]); // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
        twitter.follow(1, 2); // User 1 follows user 2.
        twitter.post_tweet(2, 6); // User 2 posts a new tweet (id = 6).
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]); // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
        twitter.unfollow(1, 2); // User 1 unfollows user 2.
        assert_eq!(twitter.get_news_feed(1), vec![5]); // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
    }
}
